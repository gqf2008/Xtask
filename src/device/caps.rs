//! v3 控制面能力（caps）：开关量、厂商原始通道、执行模型分类骨架。
//!
//! 模块纪律（见 `docs/设备驱动抽象层v3.md` §5.4/§5.5/§5.10）：
//! - 能力 trait 一律继承 [`DeviceBase`]（**不要求 `Sync`**），供独占句柄直接实现；
//! - 逃生舱与执行模型骨架**尚未接通**，一律收在 [`reserved`] 子模块里：
//!   `caps::SyncOp` 这类顶层路径取不到它们，用错了编译期就撞墙（见该模块文档）。
//!
//! 判断一个 trait 该放顶层还是 `reserved` 的唯一标准：**有没有实现方**。有实现方
//! （至少一个驱动 + 一条端到端调用）才进顶层；只有设计没有实现的，放 `reserved`——
//! 空 trait 摆在顶层会让「能力齐全」看起来成立，而 `find_capability` 永远返回
//! `None`。

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

/// **预留面：只有设计、零实现**（接通前不要在业务代码里引用）。
///
/// 本模块收纳 v3 §5.3/§5.10 定义但**当前仓内没有任何实现方**的 trait：
///
/// - [`IoCtl`]：厂商原始命令逃生舱（§5.10）。首个消费者出现时移出到顶层，
///   并在 `define_capabilities!` 里加一行（若确实要经注册表下发）。
/// - [`SyncOp`] / [`PollingAsyncOp`] / [`EventSource`] / [`TransactionOp`]：
///   §5.3 的三型执行模型骨架，只有关联类型、没有方法。接通方式：先让**一个**
///   真设备走通端到端（例如 ADC → `SyncOp`），再按实测形状补方法。
///
/// 纪律：放进来的前提是「文档里写了、代码里没有」；移出去的前提是
/// 「至少一个驱动实现 + 至少一条端到端调用 + 一条测试」。**两边都不许含糊**。
pub mod reserved {
    use core::ffi::c_void;

    use crate::device::{DeviceBase, DeviceError};

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
}
