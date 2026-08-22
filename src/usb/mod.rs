//! 第 23 章:USB 协议栈(usb-device 生态 + CDC 虚拟串口)。
//!
//! 分层与第 21/22 章同构——"会算错的"下沉宿主测、"只能上板的"留真机:
//!
//! - [`otg`]:OTG 寄存器位域的纯解码层(RX 收包状态、全局中断位图、EP 偏移、
//!   槽位分配),宿主全量回归(对应 ch22 的 `slip`/ch21 的 `sd_proto`);
//! - [`fifo`]:FIFO 预算纯函数(单包必放得下 → 写免阻塞);
//! - 芯片实现 `Gd32UsbBus`(UsbBus trait)在 chip/gd32vf103/usb.rs,
//!   经 bsp re-export;协议(枚举/描述符/请求)全部在 usb-device 内部——
//!   我们只写"寄存器 ↔ 包"的搬运工(ch22 的 smoltcp 同哲学)。
//!
//! 无中断方案:USB 状态机由任务 1ms 轮询驱动(≪ 10ms 合规窗口),
//! 不绑定 USBFS/USBFS_WKUP 向量;中断驱动变体见书稿改造练习。

pub use usb_device::*;
pub use usbd_serial::*;

pub mod fifo;
pub mod otg;
