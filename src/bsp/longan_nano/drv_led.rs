//! longan_nano LED 设备：板级 `Led` → 驱动层控制面（`Control`）的桥接
//!
//! 设备模型：`DrvLed` 内部持有板级 LED（`RED`/`GREEN`/`BLUE` 之一），以
//! `RefCell` 提供 `&self` 操作。驱动层（`device::Control`）只分发共享引用，
//! 可变性由设备自持——与内核 `Queue` "自持可变状态"同款模式。
//! 注意：RefCell 借用不关中断，因此每个操作都在 `sync::free` 临界区内完成，
//! 否则任务在 `borrow_mut` 与释放借用之间被抢占时，被抢占任务再借用会
//! double-borrow panic。
//!
//! 纯控制设备没有数据面：`on/off/toggle` 编码为 `Control::control` 的三个
//! `op`（族 magic `'LE'`，低 16 位命令号；`arg` 不用，传 0）。

use core::cell::RefCell;

use crate::device::{Control, Device, DeviceError, DeviceKind};
use crate::sync;

use super::led::Led;

/// LED 控制命令（`Control::control` 的 op；族 magic 'LE' = 0x4C45 高 16 位）
pub const LED_ON: u32 = 0x4C45_0001;
/// LED 熄灭命令
pub const LED_OFF: u32 = 0x4C45_0002;
/// LED 翻转命令
pub const LED_TOGGLE: u32 = 0x4C45_0003;

/// LED 设备：包装一个板级 LED，绑定到驱动层控制面 trait
pub struct DrvLed<L: Led> {
    inner: RefCell<L>,
}

// SAFETY: `Device: Sync` 上界要求实现自带 Sync；`inner` 是 RefCell，
// 但全部访问都在 `sync::free` 临界区内（单核关中断），ISR 与任务不可能
// 并发借用——与 `Queue`/bus.rs 的临界区纪律同构。
unsafe impl<L: Led> Sync for DrvLed<L> {}

impl<L: Led> DrvLed<L> {
    /// 包装板级 LED 实例
    pub fn new(led: L) -> Self {
        Self {
            inner: RefCell::new(led),
        }
    }
}

impl<L: Led> Device for DrvLed<L> {
    fn kind(&self) -> DeviceKind {
        DeviceKind::Control
    }

    fn as_control(&self) -> Option<&dyn Control> {
        Some(self)
    }
}

impl<L: Led> Control for DrvLed<L> {
    /// 命令分派：LED_ON/OFF/TOGGLE；`arg` 不用（标量语义，调用侧传 0）。
    fn control(&self, op: u32, _arg: usize) -> Result<usize, DeviceError> {
        match op {
            LED_ON => {
                sync::free(|_| self.inner.borrow_mut().on());
                Ok(0)
            }
            LED_OFF => {
                sync::free(|_| self.inner.borrow_mut().off());
                Ok(0)
            }
            LED_TOGGLE => {
                sync::free(|_| self.inner.borrow_mut().toggle());
                Ok(0)
            }
            _ => Err(DeviceError::InvalidInput),
        }
    }
}
