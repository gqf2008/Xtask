//! USB EP FIFO 预算——纯函数层。
//!
//! OTG_FS 的包缓冲是内建 FIFO,按 **32 位字**配深度:
//! `GRFLEN.RXFD`(公共收)+ 每个 IN EP 的 `DIEPnTFLEN`(独立发)。
//! 预算的唯一目标是:**每个 IN EP 的 TX FIFO ≥ 单包字节数/4**——
//! 这样 `UsbBus::write` 无需等待 FIFO 空间即可整包写入(书稿教学点:
//! "预算表就是为'写不阻塞'这句话服务的")。
//!
//! ⚠️ USBFS 内部 SRAM 总深度本地未核实(GD32VF103 手册规格表),
//! 以下取保守值;真机若 IN 丢包/写超时,先查这里。

/// 公共收 FIFO 深度(字)。保守预算:64 字 = 256B(低带宽 CDC 足够)。
pub const RX_WORDS: u16 = 64;

/// 4 字节/字
pub const WORD_BYTES: u16 = 4;

/// FIFO 预算:收 + 各 IN EP 发
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FifoPlan {
    pub rx_words: u16,
    /// 每 IN EP 的 TX FIFO 深度(字),索引 = EP 号(0 为 EP0 控制)
    pub tx_words: [u16; 4],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FifoError {
    /// 总深度超过 USBFS 内建 SRAM(未核实,保守上限由调用方传参)
    BudgetExceeded,
}

/// 按各 IN EP 的 MPS 预算 TX FIFO(每 EP 至少装得下单包,向上取整)。
/// `mps[0]` 是 EP0(控制)的 MPS(8)。
pub fn plan_for(mps: &[u16; 4]) -> FifoPlan {
    let mut tx_words = [0u16; 4];
    for i in 0..4 {
        tx_words[i] = mps[i].div_ceil(WORD_BYTES).max(4);
    }
    FifoPlan { rx_words: RX_WORDS, tx_words }
}

/// 校验预算不超总深度。
pub fn validate(plan: &FifoPlan, total_words: u32) -> Result<(), FifoError> {
    let sum: u32 = plan.rx_words as u32 + plan.tx_words.iter().map(|&w| w as u32).sum::<u32>();
    if sum > total_words {
        return Err(FifoError::BudgetExceeded);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fifo_plan_single_packet_guarantee() {
        // 任意 MPS 下 tx_words ≥ ceil(mps/4):单包必放得下
        for mps in [8u16, 16, 24, 32, 40, 47, 48, 63, 64, 128, 200] {
            let plan = plan_for(&[8, mps, 8, 64]);
            let need = mps.div_ceil(WORD_BYTES);
            assert!(
                plan.tx_words[1] >= need,
                "mps={mps}: tx={} need={need}",
                plan.tx_words[1]
            );
        }
    }

    #[test]
    fn fifo_plan_ep0_min_four_words() {
        // EP0 MPS=8 → ceil(8/4)=2,但保留最小 4 字(控制传输余量)
        let plan = plan_for(&[8, 0, 0, 0]);
        assert_eq!(plan.tx_words[0], 4);
    }

    #[test]
    fn fifo_budget_over_total_rejected() {
        let plan = plan_for(&[8, 64, 8, 64]);
        // 收 64 + 最小 4*4 = 80 字;给 79 字总深度必拒
        assert_eq!(validate(&plan, 103), Err(FifoError::BudgetExceeded));
        assert!(validate(&plan, 104).is_ok());
        // 实际芯片预算(保守 4K 字=16KB)绰绰有余
        assert!(validate(&plan, 4096).is_ok());
    }
}
