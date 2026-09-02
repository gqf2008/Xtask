//! 可重入互斥锁(递归互斥锁,任务阻塞版)
//!
//! 回答"同一任务嵌套拿同一把锁"的场景：分层代码里外层函数已持锁、内层
//!  helper 又要拿同一把锁——普通 [`Mutex`] 会把自己睡死(认领失败后挂起的
//!  是自己),可重入锁认出"持锁者就是当前任务",直接放行并记一层深度。
//! 对应 FreeRTOS 的 recursive mutex(`xSemaphoreCreateRecursiveMutex`)。
//!
//! 与 [`Mutex`] 共用 [`LockCore`] 互斥内核(持有者账本 + 优先级序等待队列 +
//! 优先级继承),差别只在认领策略:可重入锁允许"同任务加深",普通锁
//! 拒绝(嵌套 = 自死锁,由 Rust 的类型系统把错误用法挡在文档层)。
//! 释放只有深度减到 0 才真正发生:还清账、继承优先级按**全链重算**落位
//! (从剩余持锁集合取最大紧迫度,不是一刀切回落到出生值——完整 PI)、
//! 唤醒队首——嵌套几层就得多放几层,放干净了别人才进得来。
//!
//! ⚠️ **健全性保留**(教学点,也是 std 始终没收 reentrant mutex 的原因)：
//! 同一任务可同时持有多个 guard,于是能造出两个指向同一数据的 `&mut T`——
//! Rust 的别名规则靠"一把锁同一时刻只发一个 guard"维系,可重入语义恰好打破它。
//! C/FreeRTOS 没有别名检查所以无所谓;parking_lot 的 `ReentrantMutex` 与本文档
//! 一样选择"提供能力 + 显著标注"。**别在同一任务里让两个 guard 的作用域交叠着
//! 写数据**(嵌套拿锁的正确用法是"每层函数自己 guard 自己那段",析构顺序天然
//! 不重叠)。跨任务的互斥不受此影响——账本保证任何时刻只有一个任务持有。

use crate::sync::lock_core::{self, LockCore};
use crate::task::yield_now as task_yield;
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

/// 可重入互斥锁。
///
/// `lock`：锁空闲 → 认领为持有者；持有者就是当前任务 → 深度 +1 立即返回；
/// 别人持有 → 任务进入 `Blocked` 在等待队列排队(按优先级),由持有者彻底
/// 释放时唤醒重试。guard 析构：深度 -1,减到 0 才真正释放。
pub struct ReentrantMutex<T> {
    /// 互斥内核:持有者账本(含递归深度)+ 优先级序等待队列 + PI
    core: UnsafeCell<LockCore>,
    /// 被保护数据
    data: UnsafeCell<T>,
}

// SAFETY: 与 mutex.rs 同一论证,仅多一本深度账——
// 1) owner/depth 的读写全部发生在 sync::free 临界区内(任务侧关中断串行;
//    SMP 下全局自旋跨核互斥,见 critical.rs/ch25),账本的"认领/加深/清空"
//    各自原子,任何时刻至多一个任务被记为持有者;
// 2) data 只在"账本记着我是持有者"期间被该任务触达——一个任务同一时刻只在
//    一个核上执行(is_current_any 纪律),持有者身份不会因跨核迁移而重复;
// 3) 丢失唤醒防护与信号量同款:持有者彻底释放(owner 置 None)的同一动作里
//    唤醒队首,等待者只在"认领失败+入队挂起"同一临界区内入队,两者无窗口;
// 4) 优先级继承的链与换桶与 mutex.rs 同款,均在同一临界区内。
// 因此 ReentrantMutex<T: Send> 的 Send/Sync 是 sound 的(guard 的单任务别名
// 保留见模块文档,那是 safe API 的使用约束,不影响本 impl)。
unsafe impl<T: Send> Send for ReentrantMutex<T> {}
unsafe impl<T: Send> Sync for ReentrantMutex<T> {}

impl<T> ReentrantMutex<T> {
    /// 常量构造:内核零分配零惰性初始化(与 [`Mutex::new`] 同)
    /// [`Mutex::new`]: crate::sync::mutex::Mutex::new
    pub const fn new(data: T) -> Self {
        Self {
            core: UnsafeCell::new(LockCore::new()),
            data: UnsafeCell::new(data),
        }
    }

    /// PCP 构造: 带**优先级天花板**(1..=16,数字小=优先级高)——启用
    /// 优先级天花板协议(拿锁即升到天花板;严格优于所有他人持锁天花板
    /// 才许拿空闲锁;声明责任在使用者,书稿第 27 章;与 PI 锁混用则协议
    /// 定理失效)。可重入语义不变:深度账本照记,天花板只在 0→1 与 →0
    /// 交界处生效。
    pub const fn with_ceiling(data: T, ceiling: u8) -> Self {
        Self {
            core: UnsafeCell::new(LockCore::with_ceiling(ceiling)),
            data: UnsafeCell::new(data),
        }
    }

    /// 加锁：空闲/已持有立即返回；别人持有则任务 `Blocked` 睡到被唤醒后重试。
    /// 禁止在 ISR 中调用(挂起路径,与 `Mutex::lock` 同规)。
    pub fn lock(&self) -> ReentrantMutexGuard<'_, T> {
        loop {
            if lock_core::acquire(&self.core, true) {
                return ReentrantMutexGuard::new(self);
            }
            // 别人持有:已挂起入队(按优先级)。醒后重试认领——持有者可能已被
            // 抢先(barging),一律回到 acquire 让账本重新裁决
            task_yield();
        }
    }

    /// 尝试加锁：非阻塞。空闲/已持有返回 `Some(guard)`,别人持有返回 `None`。
    pub fn try_lock(&self) -> Option<ReentrantMutexGuard<'_, T>> {
        lock_core::try_acquire(&self.core, true).then(|| ReentrantMutexGuard::new(self))
    }

    /// 测试专用：读当前递归深度(0 = 空闲)。host 是单上下文(身份 null),
    /// 同一身份永远可重入,无法用 try_lock 观察"是否已释放"——深度探针直接读账本。
    #[cfg(test)]
    pub(crate) fn test_depth(&self) -> usize {
        unsafe { (&*self.core.get()).test_depth() }
    }
}

/// 可重入锁守卫：析构即解锁一层(深度 -1,减到 0 才真正释放)。
pub struct ReentrantMutexGuard<'a, T> {
    mutex: &'a ReentrantMutex<T>,
    /// `!Send + !Sync` 标记:与 [`MutexGuard`](crate::sync::mutex::MutexGuard)
    /// 同理——守卫不得跨任务移动,释放必须是持有者本人(递归深度账按持有
    /// 任务记)。裸指针非 Send/Sync,把约束钉进类型系统。
    _not_send: core::marker::PhantomData<*mut ()>,
}

impl<'a, T> ReentrantMutexGuard<'a, T> {
    #[inline]
    fn new(mutex: &'a ReentrantMutex<T>) -> Self {
        Self {
            mutex,
            _not_send: core::marker::PhantomData,
        }
    }
}

impl<T> Deref for ReentrantMutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: 持有者任务独占 data(账本唯一记录当前任务);&self 共享期
        // 同样以持锁为前提——见 ReentrantMutex 的 unsafe impl 论证。
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> DerefMut for ReentrantMutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: 同 deref。**注意可重入语义允许同一任务持有多个 guard**,
        // 别在交叠作用域里用两个 guard 同时改写(模块文档的健全性保留)。
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for ReentrantMutexGuard<'_, T> {
    fn drop(&mut self) {
        lock_core::release(&self.mutex.core);
    }
}

#[cfg(test)]
mod tests {
    use super::ReentrantMutex;

    /// 编译门禁：`ReentrantMutex::new` 必须 const(与 Mutex 同款静态单例诉求)。
    const _: ReentrantMutex<u32> = ReentrantMutex::new(0);
    const _: ReentrantMutex<u32> = ReentrantMutex::with_ceiling(0, 2);

    /// 回归：同一任务嵌套拿锁不睡死,深度随 lock/drop 逐层涨落——这是它与
    /// 普通 Mutex 的本质区别(普通 Mutex 第二次 lock 就把自己堵死)。
    #[test]
    fn relock_same_task_does_not_deadlock() {
        let m = ReentrantMutex::new(0);
        assert_eq!(m.test_depth(), 0, "新生成锁应空闲");
        {
            let mut g1 = m.lock();
            assert_eq!(m.test_depth(), 1);
            *g1 += 1;
            {
                let mut g2 = m.lock(); // 普通 Mutex 在此永久阻塞;可重入锁放行
                assert_eq!(m.test_depth(), 2, "嵌套拿锁深度应+1");
                *g2 += 1;
                assert_eq!(*g2, 2);
            } // g2 析构:深度 2→1,锁仍被 g1 持有
            assert_eq!(m.test_depth(), 1, "内层析构只退一层,锁未释放");
            *g1 += 1;
            assert_eq!(*g1, 3);
        } // g1 析构:深度 1→0,真正释放
        assert_eq!(m.test_depth(), 0, "最外层析构后才真正释放");
        let g = m.try_lock().expect("彻底释放后应能再取锁");
        assert_eq!(*g, 3, "嵌套修改应逐层可见且保留");
    }

    /// 阳性对照：深度没退干净就不算释放——三层拿锁逐层析构,深度探针必须
    /// 精确记录每一层;任何一层"忘退"(深度停在非 0)都意味着别的任务饿死。
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

    /// 回归：guard 析构即解锁一层,持锁期间修改对内层/后续持锁者可见
    /// (DerefMut 路径;与 Mutex 的 guard_deref_mut_persists 同款守护)。
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
            // 内层已析构,外层仍在:仍能继续写(持锁未断)
            g.push(3);
        }
        let g = m.try_lock().unwrap();
        assert_eq!(&*g, &[1, 2, 3]);
    }

    /// 阳性对照：try_lock 在已持有时可重入成功(同一身份)且深度照样+1——
    /// "拿不到返回 None"的路径只能由别的任务触发,host 单上下文测不到,
    /// 由 QEMU 执行级测试(examples/qemu_kernel_tests.rs)覆盖跨任务互斥。
    #[test]
    fn try_lock_reentrant_when_owned() {
        let m = ReentrantMutex::new(5);
        let _g = m.lock();
        let g2 = m.try_lock().expect("持有者 try_lock 应可重入");
        assert_eq!(*g2, 5);
        assert_eq!(m.test_depth(), 2);
    }
}
