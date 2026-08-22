//! GATT 服务树建模——纯数据层,nRF Connect 视图的文字版。
//!
//! **模组的诚实边界**(E104-BT5032A 手册 §5.8/§6.8):固件把 GATT 固定为
//!
//! ```text
//! Service(默认 0xFFF0,可改 16/128 位 UUID)
//!   ├─ char_from_board(默认 0xFFF1,read/notify)  ← 板→手机 数据通道(透传 notify)
//!   ├─ char_to_board  (默认 0xFFF2,read/write)   ← 手机→板 数据通道(透传 write)
//!   └─ config_channel (0xFFF3,read/write/notify) ← 空中配置通道(**不可改**;
//!        手机经它发 ASCII AT,`at+auth` 认证后生效——"手机遥控器")
//! ```
//!
//! AT 命令能改三个 UUID,**不能**新建服务/特征、不能改属性——
//! "可改 UUID、不可改结构"的边界用常量钉死(`SERVICE_COUNT`/`CHAR_COUNT`),
//! 书稿概念铺垫把这组取舍与"换 nRF52 自建协议栈"的路线对照展开。

use alloc::format;
use alloc::string::String;
use core::fmt;

/// 配置通道 UUID(0xFFF3,固件固定不可改——`custom` 恒保持它)
pub const CONFIG_CHANNEL_UUID: u16 = 0xFFF3;
/// 服务数(固件固定)
pub const SERVICE_COUNT: usize = 1;
/// 每服务特征数(2 数据通道 + 1 配置通道)
pub const CHAR_COUNT: usize = 3;

/// UUID:16 位简写或 128 位全量
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Uuid {
    U16(u16),
    U128([u8; 16]),
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Uuid::U16(v) => write!(f, "0x{v:04X}"),
            // 8-4-4-4-12 小写,nRF Connect 的习惯显示;按小端字节序组装
            // (显示序与线上序的对应=真机核对点,书稿注脚)
            Uuid::U128(b) => {
                let g = |r: core::ops::Range<usize>| -> String {
                    r.rev().map(|i| format!("{:02x}", b[i])).collect()
                };
                write!(
                    f,
                    "{}-{}-{}-{}-{}",
                    g(0..4),
                    g(4..6),
                    g(6..8),
                    g(8..10),
                    g(10..16)
                )
            }
        }
    }
}

/// GATT 服务树。字段名即方向假设(手册 §6.8 + 透传语义推出,
/// **真机核对点**:到手后订阅 FFF1/写 FFF2 各验证一次)。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Service {
    /// 服务 UUID
    pub uuid: Uuid,
    /// 板→手机 数据通道(read/notify;默认 0xFFF1)
    pub char_from_board: Uuid,
    /// 手机→板 数据通道(read/write;默认 0xFFF2)
    pub char_to_board: Uuid,
    /// 配置通道(read/write/notify;恒 0xFFF3 不可改)
    pub config_channel: Uuid,
}

impl Service {
    /// 出厂默认树:FFF0 / FFF1 / FFF2 / FFF3
    pub const fn default_tree() -> Self {
        Service {
            uuid: Uuid::U16(0xFFF0),
            char_from_board: Uuid::U16(0xFFF1),
            char_to_board: Uuid::U16(0xFFF2),
            config_channel: Uuid::U16(CONFIG_CHANNEL_UUID),
        }
    }

    /// 自定义树:三个 16 位 UUID 可改,配置通道恒 0xFFF3(**不可改**)
    pub const fn custom(svr: u16, ch1: u16, ch2: u16) -> Self {
        Service {
            uuid: Uuid::U16(svr),
            char_from_board: Uuid::U16(ch1),
            char_to_board: Uuid::U16(ch2),
            config_channel: Uuid::U16(CONFIG_CHANNEL_UUID),
        }
    }

    /// 128 位服务变体(手册只开放**服务**的 128 位 UUID;特征仍 16 位)
    pub const fn custom128(svr: [u8; 16], ch1: u16, ch2: u16) -> Self {
        Service {
            uuid: Uuid::U128(svr),
            char_from_board: Uuid::U16(ch1),
            char_to_board: Uuid::U16(ch2),
            config_channel: Uuid::U16(CONFIG_CHANNEL_UUID),
        }
    }
}

impl fmt::Display for Service {
    /// nRF Connect 视图的文字版(书稿截图配对)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Service {}", self.uuid)?;
        writeln!(f, "  ├─ {}  notify/read   板→手机(数据出)", self.char_from_board)?;
        writeln!(f, "  ├─ {}  write/read    手机→板(数据入)", self.char_to_board)?;
        write!(f, "  └─ {}  r/w/notify    空中配置通道(不可改)", self.config_channel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn default_tree_is_fff0_to_fff3() {
        let t = Service::default_tree();
        assert_eq!(t.uuid, Uuid::U16(0xFFF0));
        assert_eq!(t.char_from_board, Uuid::U16(0xFFF1));
        assert_eq!(t.char_to_board, Uuid::U16(0xFFF2));
        assert_eq!(t.config_channel, Uuid::U16(0xFFF3));
    }

    #[test]
    fn custom_keeps_config_channel_positive_control() {
        // 阳性对照:实现若误改 config_channel 立即红("不可改"边界钉进测试)
        let t = Service::custom(0x4352, 0x4353, 0x4354);
        assert_eq!(t.uuid, Uuid::U16(0x4352));
        assert_eq!(t.char_from_board, Uuid::U16(0x4353));
        assert_eq!(t.char_to_board, Uuid::U16(0x4354));
        assert_eq!(t.config_channel, Uuid::U16(CONFIG_CHANNEL_UUID));
        // 128 位变体同样保持
        let t128 = Service::custom128([0u8; 16], 1, 2);
        assert_eq!(t128.config_channel, Uuid::U16(CONFIG_CHANNEL_UUID));
    }

    #[test]
    fn custom128_service_uuid_is_u128() {
        let t = Service::custom128([0xAB; 16], 0x4353, 0x4354);
        assert_eq!(t.uuid, Uuid::U128([0xAB; 16]));
        assert_eq!(t.char_from_board, Uuid::U16(0x4353)); // 特征仍 16 位
    }

    #[test]
    fn uuid_display_u16() {
        assert_eq!(Uuid::U16(0x4352).to_string(), "0x4352");
        assert_eq!(Uuid::U16(0xFFF3).to_string(), "0xFFF3");
    }

    #[test]
    fn uuid_display_u128_sig_order() {
        // 小端组装:线上字节 [0..4] 显示为逆序组
        let mut b = [0u8; 16];
        b[0] = 0x34; b[1] = 0x12; // → 1234
        b[2] = 0x00; b[3] = 0x00;
        b[4] = 0x78; b[5] = 0x56; // → 5678
        b[6] = 0x00; b[7] = 0x00;
        b[8] = 0x21; b[9] = 0x43; // → 4321
        b[10] = 0xEF; b[11] = 0xCD; b[12] = 0xAB; b[13] = 0x89; b[14] = 0x67; b[15] = 0x45;
        assert_eq!(
            Uuid::U128(b).to_string(),
            "00001234-5678-0000-4321-456789abcdef"
        );
    }

    #[test]
    fn service_display_is_nrf_view() {
        let s = Service::default_tree().to_string();
        assert!(s.contains("FFF0"), "缺服务 UUID: {s}");
        assert!(s.contains("FFF1") && s.contains("notify"), "缺板→手机通道: {s}");
        assert!(s.contains("FFF2") && s.contains("write"), "缺手机→板通道: {s}");
        assert!(s.contains("FFF3") && s.contains("配置"), "缺配置通道: {s}");
    }

    #[test]
    fn tree_bounds_are_compile_time_truth() {
        // 边界常量自证(结构不可扩——改了常量此测试即红)
        assert_eq!(SERVICE_COUNT, 1);
        assert_eq!(CHAR_COUNT, 3);
        assert_eq!(CONFIG_CHANNEL_UUID, 0xFFF3);
    }
}
