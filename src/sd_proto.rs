//! SD 卡 SPI 模式的协议纯函数层（与芯片无关，宿主可测）
//!
//! 第 21 章的分层：**规范里"会算错"的部分**独立成纯函数——命令帧怎么摆、
//! CSD 容量怎么解析、SDHC 与 SDSC 的地址怎么换算，全部是 SD 规范中的位域
//! 数学，与任何芯片无关；硬件的时钟、片选、时序在 BSP 里（
//! `bsp/longan_nano/drv_sd.rs`），那里只能真机验证。把这两者切开的原因很
//! 实际：真机上看不到寄存器枚举，宿主测试却能看到字节——协议位域错一处，
//! 在真机上表现为"SD 卡初始化失败"，定位要靠逻辑分析仪；在宿主上就是一条
//! 断言红。协议驱动的第一层防御就是把这些纯计算钉死在宿主侧。

use crate::device::SECTOR_SIZE;

/// 命令号（SPI 模式）
pub const CMD0: u8 = 0; // GO_IDLE_STATE：切入 SPI 模式
pub const CMD8: u8 = 8; // SEND_IF_COND：电压/校验握手（区分 SDSC v1 与 v2 的充要命令）
pub const CMD9: u8 = 9; // SEND_CSD：读卡专用数据（容量在这里）
pub const CMD16: u8 = 16; // SET_BLOCKLEN：定块长（512B，之后所有单块命令按此传输）
pub const CMD17: u8 = 17; // READ_SINGLE_BLOCK
pub const CMD24: u8 = 24; // WRITE_BLOCK
pub const CMD41: u8 = 41; // ACMD41（需 CMD55 前缀）：上电初始化握手
pub const CMD55: u8 = 55; // APP_CMD：宣告下一条是应用类命令（ACMD 前缀）
pub const CMD58: u8 = 58; // READ_OCR：CCS 位（SDHC/SDXC 标记）在这里

/// CMD0 的 CRC7 固定值（SD 规范直接给出成品：全 0 参数 + 命令号 0）
pub const CRC7_CMD0: u8 = 0x95;
/// CMD8 的 CRC7 固定值（参数 0x000001AA + 命令号 8）
pub const CRC7_CMD8: u8 = 0x87;

/// 组装 6 字节命令帧：`01_cccccc`（起始位 + 传输位 + 6 位命令号）+ 4 字节参数大端 + CRC。
/// 注意 CRC 只有 CMD0/CMD8 有意义（SPI 模式其余命令不校验，传 0 即可），
/// 这两个成品值由规范给定、不许现场重算（CRC7 多项式是另一坨容易错的东西）。
pub fn cmd_frame(cmd: u8, arg: u32, crc: u8) -> [u8; 6] {
    [
        0x40 | (cmd & 0x3F),
        (arg >> 24) as u8,
        (arg >> 16) as u8,
        (arg >> 8) as u8,
        arg as u8,
        crc,
    ]
}

/// 单块命令（CMD17/CMD24）的参数地址换算：
/// - SDSC（CCS=0）规范规定传**字节地址** = 扇区号 << 9；
/// - SDHC/SDXC（CCS=1）规范规定直接传**扇区号**。
/// 命令参数只有 32 位——SD 规范的容量上限（2TiB）就是被它卡出来的；
/// 该断言即契约的失控测试：扇区号溢出时必须在宿主/开发期炸出来。
pub fn block_addr(ccs: bool, no: u64) -> u32 {
    let addr = if ccs { no } else { no << 9 };
    assert!(
        addr <= u32::MAX as u64,
        "扇区地址超出 32 位命令参数范围：扇区号 {} 换算后为 {}",
        no,
        addr
    );
    addr as u32
}

/// 从 16 字节 CSD 解析扇区总数（SPI 模式下 CSD 按 MSB 顺序传成 16 个字节，
/// 与 SD 规范里 128 位的位序一致，不用翻转）。
///
/// 两种结构版本，公式都来自规范（来源核对：SD Physical Layer Spec 4.1，
/// Table 4-2 / 4-3）：
/// - **v1.x（SDSC）**：`C_SIZE` 12 位、`C_SIZE_MULT` 3 位、`READ_BL_LEN` 4 位，
///   容量 = (C_SIZE+1) × 2^(C_SIZE_MULT+2) × 2^READ_BL_LEN 字节；
/// - **v2.x（SDHC/SDXC）**：`C_SIZE` 22 位，容量 = (C_SIZE+1) × 512KiB，
///   即扇区数 = (C_SIZE+1) × 2048。
///
/// 位域位置（字节下标按传输序）：v1 的 `C_SIZE` 横跨 csd[6] 低 2 位 + csd[7]
/// 全部 + csd[8] 高 2 位；`C_SIZE_MULT` 在 csd[9] 低 2 位 + csd[10] 高 1 位；
/// `READ_BL_LEN` 在 csd[5] 低 4 位。v2 的 `C_SIZE` 在 csd[7] 低 6 位 +
/// csd[8] + csd[9]。结构版本号在 csd[0] 高 2 位。
pub fn csd_capacity(csd: &[u8; 16]) -> u64 {
    let version = csd[0] >> 6;
    if version == 2 {
        // v2：C_SIZE 22 位，直接给扇区数（结果恒为 2048 的整数倍）
        let c_size = (((csd[7] & 0x3F) as u32) << 16) | ((csd[8] as u32) << 8) | (csd[9] as u32);
        ((c_size as u64) + 1) << 11
    } else {
        // v1（含 0.9/1.0/1.1 的其余结构：这里只区分 v2 与"老公式"）
        let c_size = (((csd[6] & 0x03) as u32) << 10)
            | ((csd[7] as u32) << 2)
            | ((csd[8] >> 6) as u32);
        let c_size_mult = (((csd[9] & 0x03) as u32) << 1) | ((csd[10] >> 7) as u32);
        let block_len = 1u64 << (csd[5] & 0x0F);
        ((c_size as u64) + 1) * (1u64 << (c_size_mult + 2)) * block_len / SECTOR_SIZE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 回归：CMD0/CMD8 的成品帧（CRC7 是规范常数，不许重算）。
    /// 帧错一位真机就是"握手永远不返回"——先钉在宿主。
    #[test]
    fn cmd_frame_crc_constants() {
        assert_eq!(cmd_frame(CMD0, 0, CRC7_CMD0), [0x40, 0x00, 0x00, 0x00, 0x00, 0x95]);
        assert_eq!(cmd_frame(CMD8, 0x0000_01AA, CRC7_CMD8), [0x48, 0x00, 0x00, 0x01, 0xAA, 0x87]);
    }

    /// 回归：普通命令帧（CRC 不校验，位域拼装仍然要验）——
    /// CMD17 读扇区 3（字节地址 3<<9 = 0x600）。
    #[test]
    fn cmd_frame_generic() {
        assert_eq!(cmd_frame(CMD17, 0x600, 0), [0x51, 0x00, 0x00, 0x06, 0x00, 0x00]);
    }

    /// 回归：v1（SDSC）CSD 解析。合成字段：C_SIZE=3、C_SIZE_MULT=7、
    /// READ_BL_LEN=9(512B) → (3+1)×2^9×512B = 1MiB = 2048 扇区。
    /// 位域放置方式与 csd_capacity 的提取一一对应——错一个移位，断言即红。
    #[test]
    fn csd_capacity_parse_v1() {
        let mut csd = [0u8; 16];
        csd[0] = 0x00; // 结构版本 0（v1.0）
        csd[5] = 0x09; // READ_BL_LEN = 9
        // C_SIZE = 3：csd[6] 低 2 位 = 0，csd[7] = 0，csd[8] 高 2 位 = 3
        csd[8] = 0xC0;
        // C_SIZE_MULT = 7：csd[9] 低 2 位 = 3，csd[10] 高 1 位 = 1
        csd[9] = 0x03;
        csd[10] = 0x80;
        assert_eq!(csd_capacity(&csd), 2048);
    }

    /// 回归：v2（SDHC/SDXC）CSD 解析。C_SIZE=1023 →
    /// (1023+1)×2048 = 2097152 扇区 = 1GiB。
    #[test]
    fn csd_capacity_parse_v2() {
        let mut csd = [0u8; 16];
        csd[0] = 0x80; // 结构版本字段 = 0b10（v2.0；注意不是 0x40——那是 0b01 = v1.1）
        csd[7] = 0x00;
        csd[8] = 0x03;
        csd[9] = 0xFF;
        assert_eq!(csd_capacity(&csd), 2048 * 1024);
    }

    /// 回归：地址换算——SDHC 直传扇区号，SDSC 传字节地址（扇区号 << 9）。
    #[test]
    fn block_addr_translation() {
        assert_eq!(block_addr(true, 5), 5);
        assert_eq!(block_addr(false, 5), 5 << 9);
        // 32 位参数上限：SDHC 下扇区号最多 2^32-1（对应 2TiB）
        assert!(std::panic::catch_unwind(|| block_addr(false, 1 << 23)).is_err(), "SDSC 地址溢出必须 panic");
        assert!(std::panic::catch_unwind(|| block_addr(true, u32::MAX as u64 + 1)).is_err(), "SDHC 扇区号溢出必须 panic");
    }
}
