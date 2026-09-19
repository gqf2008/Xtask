//! WCH PFIC 的常量与位掩码(纯常量,host 可测;被 `ch583`/`ch572` 两口共用)。
//!
//! 为什么单独成文件:本内核的软中断(yield/抢占请求)走 QingKe 专用
//! `SWI_IRQn=14`,而 PFIC 的 **pending 与 enable 是两组分离的寄存器**——
//! `irq()` 置 pending(`IPSR[0]`)只是"举手",中断要被取走还必须在 `IENR[0]` 里
//! 使能该 IRQ。官方三套 RTOS 移植(FreeRTOS/RT-Thread/HarmonyOS)的初始化里都有
//! `PFIC_EnableIRQ(SWI_IRQn)`,本内核同样必须有:漏了它,`irq()` 会静默失效
//! (yield/抢占只剩一个 tick 兜底),而构建、门禁、host 测试全绿——正是本口要
//! 消灭的"只能上板才能发现"(2026-09-19 PR #19 独立审查实测)。
//! 把掩码抽成常量并配 host 单测,是让"漏掉 bit14"这件事在 host 上就红。

/// `IENR[0]` 偏移:使能核中断 0..31(写 1 使能;复位不使能,与 NVIC 的 ISER 同型)
pub(crate) const IENR_OFFSET: usize = 0x100;
/// `IPSR[0]` 偏移:置 pending(写 1 置位)
pub(crate) const IPSR_OFFSET: usize = 0x200;
/// `IPRR[0]` 偏移:清 pending(写 1 清位)
pub(crate) const IPRR_OFFSET: usize = 0x280;

/// SysTick 核中断号(CH583/CH572 同值)
pub(crate) const SYSTICK_IRQ: u32 = 12;
/// QingKe 专用软中断号(= PendSV 角色;官方 FreeRTOS `portmacro.h` 的 `SWI_IRQn`)
pub(crate) const SWI_IRQ: u32 = 14;

/// `IENR[0]` 需要使能的位:**SysTick + 软中断**。
///
/// 两个都必须显式使能(`PFIC_EnableIRQ` 语义),缺一即半瘫:
/// 缺 bit12 → 没有节拍;缺 bit14 → `irq()` 置的 pending 永远不被取走。
#[inline]
pub(crate) const fn ienr_enable_mask() -> u32 {
    irq_bit(SYSTICK_IRQ) | irq_bit(SWI_IRQ)
}

/// 单个 IRQ 对应的位(`IPSR`/`IPRR` 用;避免各处再写魔数)
#[inline]
pub(crate) const fn irq_bit(irq: u32) -> u32 {
    1 << irq
}

#[cfg(test)]
mod tests {
    use super::{
        ienr_enable_mask, irq_bit, IENR_OFFSET, IPRR_OFFSET, IPSR_OFFSET, SWI_IRQ, SYSTICK_IRQ,
    };

    /// 寄存器偏移是官方 `RVMSIS/core_riscv.h` 的 PFIC 布局(`PFIC->IENR[0]` /
    /// `IPSR[0]` / `IPRR[0]`),两口直址访问全靠它——钉住,改名/改位都在 host 上红。
    #[test]
    fn pfic_offsets_match_official_layout() {
        assert_eq!(IENR_OFFSET, 0x100);
        assert_eq!(IPSR_OFFSET, 0x200);
        assert_eq!(IPRR_OFFSET, 0x280);
    }

    /// 回归(PR #19 独立审查):软中断 IRQ 必须与 SysTick 并列出现在使能掩码里。
    /// 只置 pending 不使能 = 请求挂在一个永远不被取走的中断上,而全部门禁仍绿。
    /// 位号本身也一并钉住:12/14 是 WCH 定的核中断号,改号会让这条红。
    #[test]
    fn ienr_mask_enables_swi_alongside_systick() {
        let mask = ienr_enable_mask();
        assert_eq!(mask, 0x5000, "使能掩码应恰为 bit12|bit14");
        assert_ne!(mask & irq_bit(SYSTICK_IRQ), 0, "缺 SysTick 使能位:没有节拍");
        assert_ne!(
            mask & irq_bit(SWI_IRQ),
            0,
            "缺 SWI 使能位:irq() 的 pending 无人取走"
        );
    }
}
