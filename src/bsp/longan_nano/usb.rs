//! USBFS(USBD)总线驱动 re-export——实现下沉到芯片层(chip/gd32vf103/usb.rs,
//! stdout 先例),板级只负责"这板子有 USB 口"这个事实。

#[cfg(feature = "usb")]
pub use crate::chip::gd32vf103::usb::Gd32UsbBus;
