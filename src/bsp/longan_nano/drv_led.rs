//! longan_nano LED 设备：板级 `Led` → 驱动层 `LedDevice` 的桥接
//!
//! 设备模型：`DrvLed` 内部持有板级 LED（`RED`/`GREEN`/`BLUE` 之一），以
//! `RefCell` 提供 `&self` 操作。驱动层（`drv::LedDevice`）只分发共享引用，
//! 可变性由设备自持——与内核 `Queue` "自持可变状态"同款模式。
//! 注意：RefCell 借用不关中断，因此每个操作都在 `sync::free` 临界区内完成，
//! 否则任务在 `borrow_mut` 与释放借用之间被抢占时，被抢占任务再借用会
//! double-borrow panic。

use core::cell::RefCell;

use crate::drv::LedDevice;
use crate::sync;

use super::led::Led;

/// LED 设备：包装一个板级 LED，绑定到驱动层 trait
pub struct DrvLed<L: Led> {
    inner: RefCell<L>,
}

impl<L: Led> DrvLed<L> {
    /// 包装板级 LED 实例
    pub fn new(led: L) -> Self {
        Self {
            inner: RefCell::new(led),
        }
    }
}

impl<L: Led> LedDevice for DrvLed<L> {
    fn on(&self) {
        sync::free(|_| self.inner.borrow_mut().on());
    }

    fn off(&self) {
        sync::free(|_| self.inner.borrow_mut().off());
    }

    fn toggle(&self) {
        sync::free(|_| self.inner.borrow_mut().toggle());
    }
}
