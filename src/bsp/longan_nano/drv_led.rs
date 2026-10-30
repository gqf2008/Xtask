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
//!
//! 生命周期（v3 §5.6）：`DrvLed` 默认 **closed**——构造后必须先 `open()`
//! 才能做业务（未 open 的 `Switchable` 方法返回 `InvalidInput`）。常驻设备
//! 没有可失败的硬件初始化，`open` 只做"逻辑使能 + 确定初态 off"；`reset`
//! 结束后状态 = `close` 后状态。全仓唯一消费者是 `examples/driver.rs`
//! （填槽前先 `open`）。

use core::cell::RefCell;

use crate::device::caps::{OpenState, Openable, Recoverable};
use crate::device::{Device, DeviceBase, DeviceError, DeviceKind, Switchable};
use crate::sync;

use super::led::Led;

/// LED 设备：包装一个板级 LED，绑定到驱动层开关量能力
pub struct DrvLed<L: Led> {
    inner: RefCell<L>,
    state: OpenState,
}

// SAFETY: `Device: Sync` 上界要求实现自带 Sync；`inner` 是 RefCell、`state`
// 是 Cell，但两者的全部访问都在 `sync::free` 临界区内（单核关中断），ISR 与
// 任务不可能并发借用——与 `Queue`/bus.rs 的临界区纪律同构。
unsafe impl<L: Led> Sync for DrvLed<L> {}

impl<L: Led> DrvLed<L> {
    /// 包装板级 LED 实例（初态 closed，业务前须 `open`）
    pub fn new(led: L) -> Self {
        Self {
            inner: RefCell::new(led),
            state: OpenState::new(), // 初态 closed（§5.6）
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

impl<L: Led> Openable for DrvLed<L> {
    fn open(&self) -> Result<(), DeviceError> {
        sync::assert_c2("DrvLed::open");
        sync::free(|_| {
            self.state.open_with(|| {
                // LED 常驻、无硬件 open：逻辑使能 + 确定初态 off。
                // 若将来有可失败硬件序列，失败分支必须在闭包内撤销已做部分。
                self.inner.borrow_mut().off();
                Ok(())
            })
        })
    }

    fn close(&self) {
        sync::assert_c2("DrvLed::close");
        sync::free(|_| {
            self.inner.borrow_mut().off(); // 关灯是安全终态，无失败路径
            self.state.mark_closed(); // 幂等
        });
    }

    fn is_open(&self) -> bool {
        // C1：允许单段 sync::free（§4.3）；只读状态，不触硬件。
        sync::free(|_| self.state.is_open())
    }
}

impl<L: Led> Recoverable for DrvLed<L> {
    fn reset(&self) -> Result<(), DeviceError> {
        sync::assert_c2("DrvLed::reset");
        sync::free(|_| {
            // LED 无 owner 队列（同步直写）：§5.6 的 Cancelled 规则无在途路径。
            self.inner.borrow_mut().off();
            self.state.mark_closed();
            Ok(())
        })
    }
}

impl<L: Led> Switchable for DrvLed<L> {
    fn on(&self) -> Result<(), DeviceError> {
        sync::assert_c2("DrvLed::on");
        sync::free(|_| {
            // 状态检查与硬件写必须在同一次临界区（§4.7 R5，不跨两次 free）。
            self.state.ensure_open()?;
            self.inner.borrow_mut().on();
            Ok(())
        })
    }

    fn off(&self) -> Result<(), DeviceError> {
        sync::assert_c2("DrvLed::off");
        sync::free(|_| {
            self.state.ensure_open()?;
            self.inner.borrow_mut().off();
            Ok(())
        })
    }

    fn toggle(&self) -> Result<(), DeviceError> {
        sync::assert_c2("DrvLed::toggle");
        sync::free(|_| {
            self.state.ensure_open()?;
            self.inner.borrow_mut().toggle();
            Ok(())
        })
    }
}
