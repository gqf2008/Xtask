//! v3 控制面能力（caps）：开关量 + 厂商原始通道逃生舱。
//!
//! 模块纪律（见 `docs/设备驱动抽象层v3.md` §5.4/§5.10）：
//! - 能力 trait 继承 [`DeviceBase`]（**不要求 `Sync`**），供独占句柄直接实现；
//! - [`IoCtl`] 是末端逃生舱：每设备一个、不经 `as_*`，不会变成统一控制面；
//! - 只有设计、没有实现方的执行模型骨架（`SyncOp` 等）随 L2 范围收口移出内核
//!   （设计保留在 v3 §5.5，代码不预置空 trait）。
//!
//! 范围收口（2026-10-30）：SPI/I2C/PWM/ADC 等片内外设词汇交给 `embedded-hal`；
//! 内核只保留“把设备接进调度器”所需的能力面。

use core::ffi::c_void;

use crate::device::{DeviceBase, DeviceError};

/// 开关量能力：LED / 继电器 / 背光使能。
pub trait Switchable: DeviceBase {
    /// 打开（C2）。
    fn on(&self) -> Result<(), DeviceError>;
    /// 关闭（C2）。
    fn off(&self) -> Result<(), DeviceError>;
    /// 翻转（C2）。允许读回设备自己的输出寄存器，不允许另存影子副本。
    fn toggle(&self) -> Result<(), DeviceError>;
}

/// 厂商私有命令逃生舱（每设备一个）。
///
/// 只有持有具体设备（typed accessor）的调用方才能用到它；**不经 `as_*`**，
/// 因此不会变成一条"任何 `&dyn Device` 都能发任意 op"的万能控制面。
pub trait IoCtl: DeviceBase {
    /// # Safety
    ///
    /// `arg` 的类型 / 大小 / 对齐 / 生命周期由 `op` 的厂商契约决定；仅在本次调用
    /// 期间有效，**不得留存**。`op` 取 `u32`（厂商 `ulong cmd`）。错用即厂商驱动侧 UB。
    unsafe fn ioctl_raw(&self, op: u32, arg: *mut c_void) -> Result<isize, DeviceError>;
}
