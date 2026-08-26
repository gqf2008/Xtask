//! 互斥锁（任务阻塞版）
//!
//! 本文件回答"互斥"的两种粒度：
//! - [`free`](crate::sync::free)：**不放弃 CPU 的互斥**——可重入临界区
//!   （irqsave 形态：关本核中断+全局自旋+深度计数，见 critical.rs），
//!   保护"登记/取数"级代码（几行到几十行，微秒级）。第 20 章 UART 驱动、
//!   注册表、bus 全用它。
//! - [`Mutex`]：**愿意睡到拿锁为止的互斥**——拿不到的任务进入 `Blocked` 状态，
//!   由持锁者释放时唤醒。文件 I/O 这种毫秒级操作只能用它（关中断几毫秒会打死 systick）。
//!
//! 与 C 系 pthread_mutex 的对照（教学点）：它们同样把"锁"做成一个对象，但
//! Rust 的 `MutexGuard` 让**解锁只能由 guard 析构发生**——"另一个任务的锁"在
//! 安全代码里根本无法表达（guard 不被共享），双释放/他者释放的整类 bug 被类型系统
//! 直接排除；C 版本需要运行时检查（错误码 EPERM/EBUSY）兜底同样的场景。

use crate::sync;
use crate::sync::semaphore::Semaphore;
use core::cell::{OnceCell, UnsafeCell};
use core::ops::{Deref, DerefMut};

/// 任务阻塞互斥锁。
///
/// 实现即"一个二值信号量 + 一个受它保护的引用"：信号量计数 1 = 未锁、0 = 已锁，
/// `lock` = `wait`（空则任务 Blocked），guard 析构 = `post`（唤醒排队者）。
/// 与 [`Semaphore`] 的差别只在形式上多一层"谁拿了 guard 谁拥有数据"的所有权——
/// 拿不到锁的失败路径、等待队列、丢失唤醒防护全部复用信号量已验证的机制。
pub struct Mutex<T> {
    /// 信号量；构造要分配（Arc），不能进 const，用 `OnceCell` 推迟到首次加锁
    sem: OnceCell<Semaphore>,
    /// 被保护数据：只允许"持锁者"（拿到 `MutexGuard` 的任务）访问
    data: UnsafeCell<T>,
}

// SAFETY: 单核抢占模型下（SMP 经⑥全局自旋扩展,论证同构)——
// 1) MutexGuard 只发给"把信号量 1→0 成功"的任务（wait/try_wait 的 fetch_update
//    原子保证互斥），data 的任何时刻最多一个任务在读写；
// 2) sem 的队列访问全在 sync::free 临界区内（Semaphore 自身纪律）:任务侧关中断
//    串行、ISR 侧 post_isr 的借用也已收进同一把锁(ch25 ⑥),不存在裸并发借用;
// 3) "取信号+登记+挂起"在同一临界区（信号量纪律），不存在丢失唤醒窗口。
// 因此 Mutex<T: Send> 的 Send/Sync 是 sound 的——与 semaphore.rs、drv.rs 的
// unsafe impl 同一论证，只是把"队列"换成了"数据"。
unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    /// 常量构造：信号量惰性初始化（见 [`Mutex::sem`]）。
    /// 注意 `OnceCell` 里的分配发生在首次 `lock`/`try_lock`——**那时堆必须已 init**。
    pub const fn new(data: T) -> Self {
        Self {
            sem: OnceCell::new(),
            data: UnsafeCell::new(data),
        }
    }

    /// 惰性初始化信号量（计数 1 = 未锁）。**必须整体在 `sync::free` 内**：
    /// `OnceCell::get_or_init` 不是线程安全的——并发/重入 initialize 会 panic，
    /// 而内核是抢占式的，两个任务在 get_or_init 中途切换就会踩中；临界区屏蔽
    /// 中断后单核上不存在第二个执行上下文能观察到 InProgress。分配走全局
    /// allocator（自带自旋锁），临界区内分配不会自锁（见 allocator.rs）。
    fn sem(&self) -> &Semaphore {
        sync::free(|_| self.sem.get_or_init(|| Semaphore::with_signal(1)))
    }

    /// 加锁：空闲立即返回；被占用则任务进入 `Blocked` 挂起，由持锁者释放时唤醒。
    /// 禁止在 ISR 中调用（与 `Semaphore::wait` 同规）。
    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.sem().wait();
        MutexGuard { mutex: self }
    }

    /// 尝试加锁：非阻塞，拿不到返回 `None`（宿主回归与"不愿等"的场景）。
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        self.sem().try_wait().then(|| MutexGuard { mutex: self })
    }

    /// 解锁（仅 guard 析构调用）：计数 0→1 并唤醒一个排队的等待者。
    /// **不公开**——安全代码里手动调它意味着"guard 还在手上就解锁"，
    /// 会让第二个任务闯进数据（C 版的 EPERM 场景，Rust 版直接不提供）。
    fn unlock(&self) {
        self.sem().post();
    }
}

/// 互斥锁守卫：析构即解锁。
/// 独占语义由"只有守卫能触达 data"表达——任务间通过 `&mut T` 交接数据，
/// 编译期不允许在锁外传播。
pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: 持锁者（信号量 1→0 成功）独占访问；&self 引用共享期
        // 同样以持锁为前提，没有任何并发读者——见 Mutex 的 unsafe impl 论证。
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: 同 deref；&mut T 独占窗口内不可能有其他持锁者。
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}

#[cfg(test)]
mod tests {
    use super::Mutex;

    /// 编译门禁：`Mutex::new` 必须 const（静态 `OnceCell<Mutex<T>>` 依赖它；
    /// 全仓没有"运行时构造的 static 单例"先例，不可用也不许迁就）。
    const _: Mutex<u32> = Mutex::new(0);
    const _: Mutex<Option<u8>> = Mutex::new(None);

    /// 回归：加锁→写→解锁→再取锁 看到同一个值（信号充足路径，host 可测）。
    #[test]
    fn lock_unlock_roundtrip() {
        let m = Mutex::new(0);
        {
            let mut g = m.lock();
            assert_eq!(*g, 0);
            *g = 42;
        }
        let g = m.try_lock().expect("解锁后应能再取锁");
        assert_eq!(*g, 42, "值应保留");
    }

    /// 阳性对照：已锁时 try_lock 必须失败——漏掉"已锁检查"（如直接放行）
    /// 这条测试即红；双任务同时进临界区是互斥锁最致命的错误。
    #[test]
    fn try_lock_while_locked_fails() {
        let m = Mutex::new(1);
        let _g = m.lock();
        assert!(m.try_lock().is_none(), "已锁状态 try_lock 必须 None");
        drop(_g);
        assert!(m.try_lock().is_some(), "解锁后 try_lock 应成功");
    }

    /// 回归：guard 离开作用域即解锁（Drop 忘了解锁这条测试即红——
    /// 忘解锁 = 第二次 lock 永久阻塞，在真机上表现为任务饿死）。
    #[test]
    fn guard_drop_releases() {
        let m = Mutex::new(());
        {
            let _g = m.lock();
        }
        assert!(m.try_lock().is_some());
    }

    /// 回归：锁内修改对后续持锁者可见（DerefMut 路径）。
    #[test]
    fn guard_deref_mut_persists() {
        let m = Mutex::new(Vec::new());
        {
            let mut g = m.lock();
            g.push(7);
            g.push(8);
        }
        let g = m.try_lock().unwrap();
        assert_eq!(&*g, &[7, 8]);
    }
}
