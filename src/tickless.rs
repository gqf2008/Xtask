//! tickless 动态节拍总开关——**默认开启**,仅对 `Porting::tickless_supported()`
//! 的口生效;恒定节拍口(未覆盖本扩展面的 RISC-V/ARM 口)行为不变。
//!
//! 第 29 章:idle 任务在"无任务可跑"时不再热转,而是睡到最近的期限
//! (延时队列队首 / 软定时器堆顶取近),到点由一次性节拍中断实测时长
//! 跳账(`TICKS += el`);期间 `time::tick()` 冻结——它是运行时时钟,
//! 墙钟另看 `Porting::systick()`/`Instant`。系统忙碌阶段(任务在跑)
//! 回到恒定节拍(每拍一次),由移植层的中断路径保证。
//!
//! 重启开关的运行时语义:应用可临时关回恒定节拍(例如测试需要
//! "恒定节拍"作为阳性对照、或应用代码依赖固定拍长的行为),关闭后
//! 空闲回到自旋,与旧内核逐字一致。

use core::sync::atomic::{AtomicBool, Ordering};

static WANT_TICKLESS: AtomicBool = AtomicBool::new(true);

/// 启用/停用 tickless(默认开启)。
/// 停用后空闲自旋、恒定节拍——与旧内核行为一致。
/// 仅对 `tickless_supported()` 的口生效,其余口恒为恒定节拍
pub fn set_enabled(on: bool) {
    WANT_TICKLESS.store(on, Ordering::Release);
}

/// 应用是否已启用 tickless(idle 引擎据此决策)
pub(crate) fn enabled() -> bool {
    WANT_TICKLESS.load(Ordering::Acquire)
}
