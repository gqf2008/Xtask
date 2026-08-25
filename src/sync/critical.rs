//! 可重入临界区——全内核互斥的唯一汇聚点。
//!
//! 形态即 SMP 的 irqsave(关本核中断 + 全局自旋),单核下行为与原来的
//! "关中断"逐点等价:
//!
//! - **关本核中断**由 `Porting::free` 提供(挡住本核 ISR;host 测试下是
//!   进程内互斥锁,提供真实互斥);
//! - **全局自旋**挡住别的核——单核恒立得,开销可忽略;SMP 落地时它就是
//!   跨核互斥的半边(ch25 失效清单一)。无原子 CAS 的目标(rp2040/M0+)
//!   暂为空桩:SMP 移植时由 SIO 硬件自旋锁补上,单核语义不受影响;
//! - **每核深度计数**保证嵌套安全:外层已持区时,内层直接复用,
//!   不重复关中断/抢锁。这同时解决了 host 端 `std::sync::Mutex` 不可重入
//!   的死锁隐患(F1 修复的前提:`wakeup` 内包临界区后,已在临界区内的
//!   调用方走嵌套路径)。目标端深度按 mhartid 分槽,每核独占一字。
//!
//! 内核代码一律经 `sync::free` 进临界区;`Porting::free` 只是本模块
//! 组合用的"关中断原语",不再被直接使用(唯一的例外历史遗留已清理)。

use crate::port::{Portable, Porting};
use bare_metal::CriticalSection;

#[cfg(test)]
thread_local! {
    /// host 测试:深度按线程隔离——`cargo test` 多线程并行,共享深度会让
    /// 别的线程把外层误认为"嵌套重入"而跳过互斥锁
    static CS_DEPTH: core::cell::Cell<usize> = core::cell::Cell::new(0);
}

/// 目标端:深度按核分槽——每核只读写自己的槽,计数器本身无跨核竞态;
/// 同核上 ISR 不会抢占持区任务(持区即关本核中断),故每核一个字即可
#[cfg(not(test))]
static mut CS_DEPTH: [usize; crate::port::MAX_HARTS] = [0; crate::port::MAX_HARTS];

/// 本核深度槽下标(mhartid;恒 < MAX_HARTS——_max_hart_id 闸保证)
#[cfg(not(test))]
#[inline]
fn hart() -> usize {
    debug_assert!((Porting::hart_id() as usize) < crate::port::MAX_HARTS);
    Porting::hart_id() as usize
}

#[inline]
fn depth() -> usize {
    #[cfg(test)]
    {
        CS_DEPTH.with(|d| d.get())
    }
    #[cfg(not(test))]
    // SAFETY: 本核独占的槽;读之后只用于"是否嵌套"判断。外层路径在关中断后
    // 读写,嵌套路径只在"本核已持区"(中断已被外层关)时为真——同核无竞态窗口
    unsafe {
        CS_DEPTH[hart()]
    }
}

#[inline]
fn enter() {
    #[cfg(test)]
    {
        CS_DEPTH.with(|d| d.set(d.get() + 1));
    }
    #[cfg(not(test))]
    // SAFETY: 同 depth()——本核独占槽,外层关中断后写、嵌套在中断已关时写
    unsafe {
        CS_DEPTH[hart()] += 1;
    }
}

#[inline]
fn leave() {
    #[cfg(test)]
    {
        CS_DEPTH.with(|d| d.set(d.get() - 1));
    }
    #[cfg(not(test))]
    // SAFETY: 同 enter();深度恒 ≥1 时才会 leave(配平由本模块结构保证)
    unsafe {
        CS_DEPTH[hart()] -= 1;
    }
}

/// 全局自旋锁:SMP 下挡住别的核;单核恒立得。
/// host 测试不启用——互斥已由 HostPorting 的进程内锁提供,测试 panic
/// 时若抱着自旋锁会让后续测试集体挂死。
#[cfg(all(not(test), target_has_atomic = "ptr"))]
static KERNEL_CS_LOCK: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

#[inline]
fn kernel_cs_acquire() {
    #[cfg(all(not(test), target_has_atomic = "ptr"))]
    {
        use core::sync::atomic::Ordering;
        while KERNEL_CS_LOCK
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }
    }
    // 无 CAS 目标(rp2040/M0+):单核空桩;SMP 需 SIO 硬件锁,移植时补
}

#[inline]
fn kernel_cs_release() {
    #[cfg(all(not(test), target_has_atomic = "ptr"))]
    {
        use core::sync::atomic::Ordering;
        KERNEL_CS_LOCK.store(false, Ordering::Release);
    }
}

/// 临界区保护(可重入)。
///
/// 外层:关本核中断 → 抢全局自旋 → 进区 → 放自旋 → 恢复中断;
/// 嵌套:本上下文已在区内,直接复用(深度 +1/-1)。
#[inline]
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(&CriticalSection) -> R,
{
    if depth() > 0 {
        enter();
        // SAFETY: depth>0 意味着本执行上下文的外层 free 正在持区——
        // 本核中断已关、全局自旋已持,构造 CriticalSection 的约定成立
        let r = f(unsafe { &CriticalSection::new() });
        leave();
        return r;
    }
    Porting::free(|cs| {
        kernel_cs_acquire();
        enter();
        let r = f(cs);
        leave();
        kernel_cs_release();
        r
    })
}

#[cfg(test)]
mod tests {
    use super::free;

    /// 阳性对照:嵌套 free 必须不死锁(host 端 std Mutex 不可重入,
    /// 没有深度计数这条测试直接挂死)。这是 F1(wakeup 内包临界区)
    /// 得以成立的地基——已在区内的调用方走嵌套路径。
    #[test]
    fn nested_free_does_not_deadlock() {
        let v = free(|_| free(|_| 42));
        assert_eq!(v, 42);
    }

    /// 回归:三层嵌套配平,出来后还能再进外层(深度泄漏则第二次外层
    /// 被误判成嵌套,host 上表现为跳过互斥锁——多线程下静默竞争)。
    #[test]
    fn depth_balances_across_nesting() {
        free(|_| {
            free(|_| {
                free(|_| ());
            });
        });
        assert_eq!(super::depth(), 0, "退出后深度必须归零(配平)");
        free(|_| ());
        assert_eq!(super::depth(), 0);
    }
}
