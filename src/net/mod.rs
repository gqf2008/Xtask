//! 第 22 章:TCP/IP 协议栈(smoltcp 接入)。
//!
//! 分层与第 21 章文件系统同构——"会算错"的下沉宿主测、"只能上板"的留真机:
//!
//! - [`slip`]:RFC 1055 帧编解码,纯状态机/纯函数,宿主全量回归(对应 ch21 的 `sd_proto`);
//! - [`device`]:smoltcp `phy::Device` 适配(字节流 → 帧的搬运工,字段级拆借的 token 模型);
//! - [`stack`]:Interface 组装 helper(示例与宿主 e2e 共用)。
//!
//! 真机部分(UART 时序、弱符号中断向量绑定)经示例 `net_echo` 验证。

pub use smoltcp::*;

pub mod device;
pub mod slip;
pub mod stack;
