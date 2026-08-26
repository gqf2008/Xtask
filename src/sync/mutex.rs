//! 互斥锁(任务阻塞版)
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
//!
//! 实现 = [`LockCore`](crate::sync::lock_core)（持有者账本 + 优先级序等待队列 +
//! 优先级继承 PI）+ 受它保护的 `T`。**为什么不是 max=1 的信号量**:计数模型
//! 没有"持有者",也就没有 PI——高优先级任务 H 等锁时把持锁的低优先级任务 L
//! 临时抬到 H 的优先级,否则中优先级任务 M 会一直跑在 L 前面(火星探路者事故)。
//! 设计推导与已知局限见 `lock_core.rs` 模块文档与书稿第 8 章。

use crate::sync::lock_core::{self, LockCore};
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

/// 任务阻塞互斥锁。
///
/// 内核是 [`LockCore`]：锁空闲 → 认领为持有者；别人持有 → 本任务 `Blocked`
/// 排队睡到被唤醒(等待队列**按优先级排序**,队首先醒);guard 析构 = 释放,
/// 唤醒队首,旧持有者的继承优先级由**全链重算**落位——从剩余持锁集合的
/// 队首等待者取最大紧迫度,而不是一刀切回落到出生值(完整 PI)。
/// 不可重入:同一任务嵌套拿同一把锁会把自己睡死(那是 [`ReentrantMutex`] 的
/// 领域,见 `src/sync/reentrant_mutex.rs`——可重入锁在账本上多记一层递归深度)。
/// [`ReentrantMutex`]: crate::sync::reentrant_mutex::ReentrantMutex
pub struct Mutex<T> {
    /// 互斥内核:零堆分配、const 构造(信号量的惰性初始化随之退役)
    core: UnsafeCell<LockCore>,
    /// 被保护数据：只允许"持锁者"（拿到 `MutexGuard` 的任务）访问
    data: UnsafeCell<T>,
}

// SAFETY: 单核抢占模型下（SMP 经⑥全局自旋扩展,论证同构)——
// 1) MutexGuard 只发给"认领成功"的任务(acquire 在 sync::free 内判定 owner,
//    原子互斥;等待队列按优先级序、释放只唤醒队首,任何时刻至多一个持有者),
//    data 的任何时刻最多一个任务在读写;
// 2) LockCore 的账本/队列访问全在 sync::free 临界区内:任务侧关中断串行、
//    SMP 下全局自旋跨核互斥,不存在裸并发借用(rw 不变量与 critical.rs 同构);
// 3) "认领失败+入队挂起"在同一临界区(lock_core::acquire),不存在丢失唤醒窗口;
// 4) 优先级继承的一切优先级字段修改(lock_core::recompute_inheritance /
//    place_priority → set_priority)都在同一临界区内,与调度器的就绪队列
//    换桶构成一个不可分割的事实。
// 因此 Mutex<T: Send> 的 Send/Sync 是 sound 的——与 semaphore.rs 同一论证
// 家族,只是把"队列"换成了"数据"、把"计数"换成了"持有者"。
unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    /// 常量构造：内核 `VecDeque::new` 是 const——**零分配、零惰性初始化**,
    /// 不必再等堆 init 后的首次加锁(信号量门闩方案的 OnceCell 随之退役)。
    pub const fn new(data: T) -> Self {
        Self {
            core: UnsafeCell::new(LockCore::new()),
            data: UnsafeCell::new(data),
        }
    }

    /// PCP 构造: 带上**优先级天花板**(取值 1..=16,数字小=优先级高,
    /// 同调度器约定)——这把锁启用**优先级天花板协议**:拿锁即升到天花板
    /// (规则 1);只有"当前优先级严格优于所有他人持锁天花板"的任务才许拿
    /// 空闲锁(规则 2,哪怕锁是空的——交叉持锁的死锁由此掐断)。
    /// ⚠️ **声明责任**:天花板必须覆盖**所有**实际使用者(任何使用者
    /// 优先级数字 ≥ ceiling),漏标的后果是协议性质失效,引擎按声明行事。
    /// 与 PI 锁混用时协议定理失效——按锁二选一(书稿第 26 章)。
    pub const fn with_ceiling(data: T, ceiling: u8) -> Self {
        Self {
            core: UnsafeCell::new(LockCore::with_ceiling(ceiling)),
            data: UnsafeCell::new(data),
        }
    }

    /// 加锁：空闲立即返回；被占用则任务进入 `Blocked` 挂起,由持锁者释放时唤醒。
    /// 禁止在 ISR 中调用（会走 `LockCore` 的挂起路径,与 `Semaphore::wait` 同规）。
    pub fn lock(&self) -> MutexGuard<'_, T> {
        loop {
            if lock_core::acquire(&self.core, false) {
                return MutexGuard { mutex: self };
            }
            // 没拿到锁:已挂起入队(按优先级)。醒后回来重试认领——可能被抢先
            // (barging),一律重试,这是"挂起-唤醒"模型的标准写法
            crate::task::yield_now();
        }
    }

    /// 尝试加锁：非阻塞，拿不到返回 `None`（宿主回归与"不愿等"的场景）。
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        lock_core::try_acquire(&self.core, false).then(|| MutexGuard { mutex: self })
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
        // SAFETY: 持锁者独占访问；&self 引用共享期同样以持锁为前提，
        // 没有任何并发读者——见 Mutex 的 unsafe impl 论证。
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
        lock_core::release(&self.mutex.core);
    }
}

#[cfg(test)]
mod tests {
    use super::Mutex;

    /// 编译门禁：`Mutex::new` 必须 const（静态 `Mutex<T>` 依赖它；
    /// 全仓没有"运行时构造的 static 单例"先例，不可用也不许迁就）。
    const _: Mutex<u32> = Mutex::new(0);
    const _: Mutex<Option<u8>> = Mutex::new(None);
    /// 编译门禁：PCP 构造同样必须 const(静态天花板锁的场景与 new 一致)。
    const _: Mutex<u32> = Mutex::with_ceiling(0, 2);

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
    /// host 是单上下文(null 身份),同身份再认领走的是"普通锁不许重入"
    /// 分支(与真实"别人持锁"同一结论)。
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
    /// 深度探针直读账本:host 上"再取锁成功"不足以区分"已释放"与"可重入"
    /// (同一个 null 身份)。
    #[test]
    fn guard_drop_releases() {
        let m = Mutex::new(());
        {
            let _g = m.lock();
            assert_eq!(
                unsafe { (&*m.core.get()).test_depth() },
                1,
                "加锁后持有深度应为 1"
            );
        }
        assert_eq!(
            unsafe { (&*m.core.get()).test_depth() },
            0,
            "guard 析构后必须彻底释放"
        );
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
