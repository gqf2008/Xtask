//! longan_nano LED 设备：板级 `Led` → 驱动层开关量能力（`Switchable`）的桥接
//!
//! 设备模型：`DrvLed` 内部持有板级 LED（`RED`/`GREEN`/`BLUE` 之一），以
//! `RefCell` 提供 `&self` 操作。驱动层只分发共享引用，可变性由设备自持——
//! 与内核 `Queue` "自持可变状态"同款模式。
//! 注意：RefCell 借用不关中断，因此每个操作都在 `sync::free` 临界区内完成，
//! 否则任务在 `borrow_mut` 与释放借用之间被抢占时，被抢占任务再借用会
//! double-borrow panic。
//!
//! v3 起 LED 不再编码成 `Control(op, arg)` 魔数：板级 `on/off/toggle`
//! 直接映射为 `Switchable` 的同名方法。

use core::cell::RefCell;

use crate::device::{Device, DeviceBase, DeviceError, DeviceKind, Switchable};
use crate::sync;

use super::led::Led;

/// LED 设备：包装一个板级 LED，绑定到驱动层开关量能力
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

impl<L: Led> DeviceBase for DrvLed<L> {
    fn kind(&self) -> DeviceKind {
        DeviceKind::Control
    }
}

impl<L: Led> Device for DrvLed<L> {
    fn as_switchable(&self) -> Option<&(dyn Switchable + Sync)> {
        Some(self)
    }
}

impl<L: Led> Switchable for DrvLed<L> {
    fn on(&self) -> Result<(), DeviceError> {
        sync::free(|_| self.inner.borrow_mut().on());
        Ok(())
    }

    fn off(&self) -> Result<(), DeviceError> {
        sync::free(|_| self.inner.borrow_mut().off());
        Ok(())
    }

    fn toggle(&self) -> Result<(), DeviceError> {
        sync::free(|_| self.inner.borrow_mut().toggle());
        Ok(())
    }
}
