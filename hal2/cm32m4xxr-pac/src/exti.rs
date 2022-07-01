#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EXTI_IMASK"]
    pub exti_imask: crate::Reg<exti_imask::EXTI_IMASK_SPEC>,
    #[doc = "0x04 - EXTI_EMASK"]
    pub exti_emask: crate::Reg<exti_emask::EXTI_EMASK_SPEC>,
    #[doc = "0x08 - EXTI_RT_CFG"]
    pub exti_rt_cfg: crate::Reg<exti_rt_cfg::EXTI_RT_CFG_SPEC>,
    #[doc = "0x0c - EXTI_FT_CFG"]
    pub exti_ft_cfg: crate::Reg<exti_ft_cfg::EXTI_FT_CFG_SPEC>,
    #[doc = "0x10 - EXTI_SWIE"]
    pub exti_swie: crate::Reg<exti_swie::EXTI_SWIE_SPEC>,
    #[doc = "0x14 - EXTI_PEND"]
    pub exti_pend: crate::Reg<exti_pend::EXTI_PEND_SPEC>,
    #[doc = "0x18 - EXTI_TS_SEL"]
    pub exti_ts_sel: crate::Reg<exti_ts_sel::EXTI_TS_SEL_SPEC>,
}
#[doc = "EXTI_IMASK register accessor: an alias for `Reg<EXTI_IMASK_SPEC>`"]
pub type EXTI_IMASK = crate::Reg<exti_imask::EXTI_IMASK_SPEC>;
#[doc = "EXTI_IMASK"]
pub mod exti_imask;
#[doc = "EXTI_EMASK register accessor: an alias for `Reg<EXTI_EMASK_SPEC>`"]
pub type EXTI_EMASK = crate::Reg<exti_emask::EXTI_EMASK_SPEC>;
#[doc = "EXTI_EMASK"]
pub mod exti_emask;
#[doc = "EXTI_RT_CFG register accessor: an alias for `Reg<EXTI_RT_CFG_SPEC>`"]
pub type EXTI_RT_CFG = crate::Reg<exti_rt_cfg::EXTI_RT_CFG_SPEC>;
#[doc = "EXTI_RT_CFG"]
pub mod exti_rt_cfg;
#[doc = "EXTI_FT_CFG register accessor: an alias for `Reg<EXTI_FT_CFG_SPEC>`"]
pub type EXTI_FT_CFG = crate::Reg<exti_ft_cfg::EXTI_FT_CFG_SPEC>;
#[doc = "EXTI_FT_CFG"]
pub mod exti_ft_cfg;
#[doc = "EXTI_SWIE register accessor: an alias for `Reg<EXTI_SWIE_SPEC>`"]
pub type EXTI_SWIE = crate::Reg<exti_swie::EXTI_SWIE_SPEC>;
#[doc = "EXTI_SWIE"]
pub mod exti_swie;
#[doc = "EXTI_PEND register accessor: an alias for `Reg<EXTI_PEND_SPEC>`"]
pub type EXTI_PEND = crate::Reg<exti_pend::EXTI_PEND_SPEC>;
#[doc = "EXTI_PEND"]
pub mod exti_pend;
#[doc = "EXTI_TS_SEL register accessor: an alias for `Reg<EXTI_TS_SEL_SPEC>`"]
pub type EXTI_TS_SEL = crate::Reg<exti_ts_sel::EXTI_TS_SEL_SPEC>;
#[doc = "EXTI_TS_SEL"]
pub mod exti_ts_sel;
