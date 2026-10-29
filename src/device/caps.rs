//! v3 控制面能力（caps）：开关量、厂商原始通道、执行模型分类骨架。
//!
//! 模块纪律（见 `docs/设备驱动抽象层v3.md` §5.4/§5.5/§5.10）：
//! - 能力 trait 一律继承 [`DeviceBase`]（**不要求 `Sync`**），供独占句柄直接实现；
//! - [`IoCtl`] 是末端逃生舱，不是通用控制面（每设备一个，op 由设备族定义）；
//! - 分类骨架（`SyncOp` 等）只作标注，带关联类型，不用于 `dyn`。

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

// ---------------------------------------------------------------------------
// 执行模型分类骨架：只作标注，带关联类型，**不用于 dyn**
// ---------------------------------------------------------------------------

/// 同步型：一次调用完成，结果即 `Outcome`。
pub trait SyncOp {
    type Outcome;
}

/// 轮询异步型：`start` 返回 `Op`，`poll` 轮询，`cancel` 取消。
pub trait PollingAsyncOp {
    type Op: Copy;
    type Outcome;
}

/// 事件流型：结果经事件通道持续送达。
pub trait EventSource {
    type Event: Copy;
}

/// 事务型：请求-应答 + 会话句柄 + 显式超时 + 可取消。
pub trait TransactionOp {
    type Session: Copy;
}
