//! 可重入互斥锁（递归互斥锁，任务阻塞版）
//!
//! 回答"同一任务嵌套拿同一把锁"的场景：分层代码里外层函数已持锁、内层
//!  helper 又要拿同一把锁——普通 [`Mutex`] 会把自己睡死（计数已是 0，等的是
//! 自己），可重入锁认出"持锁者就是当前任务"，直接放行并记一层深度。
//! 对应 FreeRTOS 的 recursive mutex（`xSemaphoreCreateRecursiveMutex`）。
//!
//! 与 [`Mutex`] 的结构差别只在多一本"谁持有、持了几层"的账：
//! - 门闩（gate）：容量 1 的信号量，非持有者在此排队；**与互斥锁同款排队/唤醒**。
//! - 账本（owner）：持有者任务指针 + 递归深度，只在 `sync::free` 临界区内读写。
//!
//! ⚠️ **健全性保留**（教学点，也是 std 始终没收 reentrant mutex 的原因）：
//! 同一任务可同时持有多个 guard，于是能造出两个指向同一数据的 `&mut T`——
//! Rust 的别名规则靠"一把锁同一时刻只发一个 guard"维系，可重入语义恰好打破它。
//! C/FreeRTOS 没有别名检查所以无所谓；parking_lot 的 `ReentrantMutex` 与本文档
//! 一样选择"提供能力 + 显著标注"。**别在同一任务里让两个 guard 的作用域交叠着
//! 写数据**（嵌套拿锁的正确用法是"每层函数自己 guard 自己那段"，析构顺序天然
//! 不重叠）。跨任务的互斥不受此影响——账本保证任何时刻只有一个任务持有。

use crate::sync;
use crate::sync::semaphore::Semaphore;
use crate::task::scheduler::xworker as xw;
use crate::task::Task;
use core::cell::{OnceCell, UnsafeCell};
use core::ops::{Deref, DerefMut};

/// 持有者账本：哪个任务、递归了几层。只在 `sync::free` 临界区内读写。
#[derive(Clone, Copy)]
struct Owner {
    task: *mut Task,
    depth: usize,
}

/// 可重入互斥锁。
///
/// `lock`：锁空闲 → 认领为持有者；持有者就是当前任务 → 深度 +1 立即返回；
/// 别人持有 → 任务进入 `Blocked` 在门闩上排队，由持有者彻底释放时唤醒重试。
/// guard 析构：深度 -1，减到 0 才真正释放并唤醒门闩上一个排队者。
pub struct ReentrantMutex<T> {
    /// 门闩：非持有者排队用。容量必须钳到 1（计数只表示"有一张自由通行证"），
    /// 否则空闲期每次释放都白攒一个计数，新等待者要空转烧掉攒下的全部计数。
    /// 惰性初始化（构造要分配，不能进 const）——与 [`Mutex::sem`] 同款。
    gate: OnceCell<Semaphore>,
    /// 账本：`None` = 锁空闲。只允许"持有者任务"在持锁期间触达 data。
    owner: UnsafeCell<Option<Owner>>,
    /// 被保护数据
    data: UnsafeCell<T>,
}

// SAFETY: 与 mutex.rs 同一论证，仅多一本账——
// 1) owner 的读写全部发生在 sync::free 临界区内（任务侧关中断串行；SMP 下
//    全局自旋跨核互斥，见 critical.rs/ch25），账本的"认领/加深/清空"三步各自
//    原子，任何时刻至多一个任务被记为持有者；
// 2) data 只在"账本记着我是持有者"期间被该任务触达——一个任务同一时刻只在
//    一个核上执行（is_current_any 纪律），持有者身份不会因跨核迁移而重复；
// 3) 丢失唤醒防护与信号量同款：持有者彻底释放（owner 置 None）的同一动作里
//    post_isr 门闩，等待者只在门闩计数为 0 时才入队，两者无窗口。
// 因此 ReentrantMutex<T: Send> 的 Send/Sync 是 sound 的（guard 的单任务别名
// 保留见模块文档，那是 safe API 的使用约束，不影响本 impl）。
unsafe impl<T: Send> Send for ReentrantMutex<T> {}
unsafe impl<T: Send> Sync for ReentrantMutex<T> {}

impl<T> ReentrantMutex<T> {
    /// 常量构造：门闩惰性初始化（见 [`ReentrantMutex::gate`]）。
    /// 注意 `OnceCell` 里的分配发生在首次 `lock`/`try_lock`——**那时堆必须已 init**。
    pub const fn new(data: T) -> Self {
        Self {
            gate: OnceCell::new(),
            owner: UnsafeCell::new(None),
            data: UnsafeCell::new(data),
        }
    }

    /// 惰性初始化门闩（计数 1 = 锁空闲，容量钳 1）。**必须整体在 `sync::free` 内**，
    /// 理由与 [`Mutex::sem`] 完全相同（`OnceCell::get_or_init` 非线程安全）。
    fn gate(&self) -> &Semaphore {
        sync::free(|_| {
            self.gate
                .get_or_init(|| Semaphore::with_signal_max_value(1, 1))
        })
    }

    /// 当前任务身份（裸指针）。调度器起跑前/host 测试为 null——视为"唯一上下文"，
    /// 单执行流下可重入语义照样成立（所有 lock 都是同一个 null 持有者）。
    #[inline]
    fn me() -> *mut Task {
        unsafe { xw::current_ptr() }
    }

    /// 尝试认领或加深（**在 `sync::free` 内**执行账本判断）：空闲 → 认领；
    /// 我就是持有者 → 深度 +1；别人持有 → false。返回 true 即拿到锁。
    fn try_acquire(&self) -> bool {
        let me = Self::me();
        sync::free(|_| {
            let owner = unsafe { &mut *self.owner.get() };
            match owner {
                None => {
                    *owner = Some(Owner { task: me, depth: 1 });
                    true
                }
                Some(o) if o.task == me => {
                    o.depth += 1;
                    true
                }
                _ => false,
            }
        })
    }

    /// 加锁：空闲/已持有立即返回；别人持有则任务 `Blocked` 睡到被唤醒后重试。
    /// 禁止在 ISR 中调用（会走 `Semaphore::wait` 的挂起路径，与 `Mutex::lock` 同规）。
    pub fn lock(&self) -> ReentrantMutexGuard<'_, T> {
        self.gate();
        loop {
            if self.try_acquire() {
                return ReentrantMutexGuard { mutex: self };
            }
            // 别人持有：在门闩上睡到被唤醒。醒后不自动静默认领——持有者可能已被
            // 抢先（barging），一律回到 try_acquire 让账本重新裁决，这同时吸收
            // 了"释放时 post 的通行证计数"，不会有丢失唤醒（见模块 SAFETY 3)。
            self.gate().wait();
        }
    }

    /// 尝试加锁：非阻塞。空闲/已持有返回 `Some(guard)`，别人持有返回 `None`。
    pub fn try_lock(&self) -> Option<ReentrantMutexGuard<'_, T>> {
        self.gate();
        self.try_acquire()
            .then(|| ReentrantMutexGuard { mutex: self })
    }

    /// 解锁（仅 guard 析构调用）：深度 -1，减到 0 才清空账本并唤醒门闩一个排队者。
    /// **不公开**——与 [`Mutex::unlock`] 同规，安全代码里手动解锁会让别的任务闯进数据。
    fn unlock(&self) {
        let release = sync::free(|_| {
            let owner = unsafe { &mut *self.owner.get() };
            match owner {
                Some(o) => {
                    debug_assert_eq!(
                        o.task,
                        Self::me(),
                        "可重入锁只能由持有者任务释放（guard 不可跨任务移交）"
                    );
                    o.depth -= 1;
                    if o.depth == 0 {
                        *owner = None;
                        true // 彻底释放 → 唤醒门闩
                    } else {
                        false // 仍持有（外层还在锁内），不唤醒
                    }
                }
                None => unreachable!("guard 存活期间账本必然记着持有者"),
            }
        });
        if release {
            // post_isr 而非 post：门闩容量钳在 1，post 在满时会把解锁任务自己
            // 挂到 notifiers 上（死锁）；post_isr 满即返回 Err 忽略，语义正确——
            // 计数已是 1 表示"通行证本就在架"，无需再放一张。
            let _ = self.gate().post_isr();
        }
    }

    /// 测试专用：读当前递归深度（0 = 空闲）。host 是单上下文（current_ptr=null），
    /// 同一身份永远可重入，无法用 try_lock 观察"是否已释放"——深度探针直接读账本。
    #[cfg(test)]
    pub(crate) fn test_depth(&self) -> usize {
        sync::free(|_| unsafe { (*self.owner.get()).map_or(0, |o| o.depth) })
    }
}

/// 可重入锁守卫：析构即解锁一层（深度 -1，减到 0 才真正释放）。
pub struct ReentrantMutexGuard<'a, T> {
    mutex: &'a ReentrantMutex<T>,
}

impl<T> Deref for ReentrantMutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: 持有者任务独占 data（账本唯一记录当前任务）；&self 共享期
        // 同样以持锁为前提——见 ReentrantMutex 的 unsafe impl 论证。
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> DerefMut for ReentrantMutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: 同 deref。**注意可重入语义允许同一任务持有多个 guard**，
        // 别在交叠作用域里用两个 guard 同时改写（模块文档的健全性保留）。
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for ReentrantMutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}

#[cfg(test)]
mod tests {
    use super::ReentrantMutex;

    /// 编译门禁：`ReentrantMutex::new` 必须 const（与 Mutex 同款静态单例诉求）。
    const _: ReentrantMutex<u32> = ReentrantMutex::new(0);

    /// 回归：同一任务嵌套拿锁不睡死，深度随 lock/drop 逐层涨落——这是它与
    /// 普通 Mutex 的本质区别（普通 Mutex 第二次 lock 就把自己堵死）。
    #[test]
    fn relock_same_task_does_not_deadlock() {
        let m = ReentrantMutex::new(0);
        assert_eq!(m.test_depth(), 0, "新生成锁应空闲");
        {
            let mut g1 = m.lock();
            assert_eq!(m.test_depth(), 1);
            *g1 += 1;
            {
                let mut g2 = m.lock(); // 普通 Mutex 在此永久阻塞；可重入锁放行
                assert_eq!(m.test_depth(), 2, "嵌套拿锁深度应+1");
                *g2 += 1;
                assert_eq!(*g2, 2);
            } // g2 析构：深度 2→1，锁仍被 g1 持有
            assert_eq!(m.test_depth(), 1, "内层析构只退一层，锁未释放");
            *g1 += 1;
            assert_eq!(*g1, 3);
        } // g1 析构：深度 1→0，真正释放
        assert_eq!(m.test_depth(), 0, "最外层析构后才真正释放");
        let g = m.try_lock().expect("彻底释放后应能再取锁");
        assert_eq!(*g, 3, "嵌套修改应逐层可见且保留");
    }

    /// 阳性对照：深度没退干净就不算释放——三层拿锁逐层析构，深度探针必须
    /// 精确记录每一层；任何一层"忘退"（深度停在非 0）都意味着别的任务饿死。
    #[test]
    fn depth_accounting_is_exact() {
        let m = ReentrantMutex::new(());
        let _g1 = m.lock();
        let _g2 = m.lock();
        let g3 = m.lock();
        assert_eq!(m.test_depth(), 3);
        drop(g3);
        assert_eq!(m.test_depth(), 2, "析构一层退一层");
        drop(_g2);
        assert_eq!(m.test_depth(), 1, "仍持有——外层 guard 还活着");
        drop(_g1);
        assert_eq!(m.test_depth(), 0, "全部析构才真正空闲");
    }

    /// 回归：guard 析构即解锁一层，持锁期间修改对内层/后续持锁者可见
    /// （DerefMut 路径；与 Mutex 的 guard_deref_mut_persists 同款守护）。
    #[test]
    fn guard_deref_mut_persists_across_nesting() {
        let m = ReentrantMutex::new(Vec::new());
        {
            let mut g = m.lock();
            g.push(1);
            {
                let mut g2 = m.lock();
                g2.push(2);
            }
            // 内层已析构，外层仍在：仍能继续写（持锁未断）
            g.push(3);
        }
        let g = m.try_lock().unwrap();
        assert_eq!(&*g, &[1, 2, 3]);
    }

    /// 阳性对照：try_lock 在已持有时可重入成功（同一身份）且深度照样+1——
    /// "拿不到返回 None"的路径只能由别的任务触发，host 单上下文测不到，
    /// 由 QEMU 执行级测试（examples/qemu_kernel_tests.rs）覆盖跨任务互斥。
    #[test]
    fn try_lock_reentrant_when_owned() {
        let m = ReentrantMutex::new(5);
        let _g = m.lock();
        let g2 = m.try_lock().expect("持有者 try_lock 应可重入");
        assert_eq!(*g2, 5);
        assert_eq!(m.test_depth(), 2);
    }
}
