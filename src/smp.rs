//! SMP 总开关——**默认关闭,单核语义逐字不变**。
//!
//! 多核口(qemu_riscv)上从核在 `_mp_hook` 里停泊,只有应用显式
//! `enable()` 之后,调度器 `start()` 才会放行从核参与调度(ch25
//! 改造路线②)。不开的原因很直接:SMP 改变调度语义——"高优先级抢占"
//! 在双核下不再意味着严格串行(两个核可以各跑一个任务),为单核写的
//! 应用/测试(如 qemu_kernel_tests 的次序断言)在双核下不再成立。
//! 要不要付出这个语义代价,必须是应用的显式决定。

use core::sync::atomic::{AtomicBool, Ordering};

static WANT_SMP: AtomicBool = AtomicBool::new(false);

/// 开启 SMP:调度器启动时放行从核参与调度。
/// 必须在 `xtask::start()` 之前调用(从核在 start 时被放行)
pub fn enable() {
    WANT_SMP.store(true, Ordering::Release);
}

/// 应用是否已开启 SMP(移植层据此决定 core_count 与是否放行从核)
pub(crate) fn enabled() -> bool {
    WANT_SMP.load(Ordering::Acquire)
}
