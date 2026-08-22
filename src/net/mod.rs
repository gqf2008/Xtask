//! 第 22 章:TCP/IP 协议栈(smoltcp 接入)。
//!
//! 分层与第 21 章文件系统同构——"会算错"的下沉宿主测、"只能上板"的留真机:
//!
//! - [`slip`]:RFC 1055 帧编解码,纯状态机/纯函数,宿主全量回归(对应 ch21 的 `sd_proto`)。
//!
//! 设备适配(→ `device`)与 Interface 组装(→ `stack`)随各自提交补入;
//! 真机部分(UART 时序、弱符号中断向量绑定)经示例 `net_echo` 验证。

pub use smoltcp::*;

pub mod slip;
