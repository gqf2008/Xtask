#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AFIO_ECTRL"]
    pub afio_ectrl: crate::Reg<afio_ectrl::AFIO_ECTRL_SPEC>,
    #[doc = "0x04 - AFIO_RMP_CFG"]
    pub afio_rmp_cfg: crate::Reg<afio_rmp_cfg::AFIO_RMP_CFG_SPEC>,
    #[doc = "0x08 - AFIO_EXTI_CFG1"]
    pub afio_exti_cfg1: crate::Reg<afio_exti_cfg1::AFIO_EXTI_CFG1_SPEC>,
    #[doc = "0x0c - AFIO_EXTI_CFG2"]
    pub afio_exti_cfg2: crate::Reg<afio_exti_cfg2::AFIO_EXTI_CFG2_SPEC>,
    #[doc = "0x10 - AFIO_EXTI_CFG3"]
    pub afio_exti_cfg3: crate::Reg<afio_exti_cfg3::AFIO_EXTI_CFG3_SPEC>,
    #[doc = "0x14 - AFIO_EXTI_CFG4"]
    pub afio_exti_cfg4: crate::Reg<afio_exti_cfg4::AFIO_EXTI_CFG4_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - AFIO_RMP_CFG3"]
    pub afio_rmp_cfg3: crate::Reg<afio_rmp_cfg3::AFIO_RMP_CFG3_SPEC>,
    #[doc = "0x24 - AFIO_RMP_CFG4"]
    pub afio_rmp_cfg4: crate::Reg<afio_rmp_cfg4::AFIO_RMP_CFG4_SPEC>,
    #[doc = "0x28 - AFIO_RMP_CFG5"]
    pub afio_rmp_cfg5: crate::Reg<afio_rmp_cfg5::AFIO_RMP_CFG5_SPEC>,
}
#[doc = "AFIO_ECTRL register accessor: an alias for `Reg<AFIO_ECTRL_SPEC>`"]
pub type AFIO_ECTRL = crate::Reg<afio_ectrl::AFIO_ECTRL_SPEC>;
#[doc = "AFIO_ECTRL"]
pub mod afio_ectrl;
#[doc = "AFIO_RMP_CFG register accessor: an alias for `Reg<AFIO_RMP_CFG_SPEC>`"]
pub type AFIO_RMP_CFG = crate::Reg<afio_rmp_cfg::AFIO_RMP_CFG_SPEC>;
#[doc = "AFIO_RMP_CFG"]
pub mod afio_rmp_cfg;
#[doc = "AFIO_EXTI_CFG1 register accessor: an alias for `Reg<AFIO_EXTI_CFG1_SPEC>`"]
pub type AFIO_EXTI_CFG1 = crate::Reg<afio_exti_cfg1::AFIO_EXTI_CFG1_SPEC>;
#[doc = "AFIO_EXTI_CFG1"]
pub mod afio_exti_cfg1;
#[doc = "AFIO_EXTI_CFG2 register accessor: an alias for `Reg<AFIO_EXTI_CFG2_SPEC>`"]
pub type AFIO_EXTI_CFG2 = crate::Reg<afio_exti_cfg2::AFIO_EXTI_CFG2_SPEC>;
#[doc = "AFIO_EXTI_CFG2"]
pub mod afio_exti_cfg2;
#[doc = "AFIO_EXTI_CFG3 register accessor: an alias for `Reg<AFIO_EXTI_CFG3_SPEC>`"]
pub type AFIO_EXTI_CFG3 = crate::Reg<afio_exti_cfg3::AFIO_EXTI_CFG3_SPEC>;
#[doc = "AFIO_EXTI_CFG3"]
pub mod afio_exti_cfg3;
#[doc = "AFIO_EXTI_CFG4 register accessor: an alias for `Reg<AFIO_EXTI_CFG4_SPEC>`"]
pub type AFIO_EXTI_CFG4 = crate::Reg<afio_exti_cfg4::AFIO_EXTI_CFG4_SPEC>;
#[doc = "AFIO_EXTI_CFG4"]
pub mod afio_exti_cfg4;
#[doc = "AFIO_RMP_CFG3 register accessor: an alias for `Reg<AFIO_RMP_CFG3_SPEC>`"]
pub type AFIO_RMP_CFG3 = crate::Reg<afio_rmp_cfg3::AFIO_RMP_CFG3_SPEC>;
#[doc = "AFIO_RMP_CFG3"]
pub mod afio_rmp_cfg3;
#[doc = "AFIO_RMP_CFG4 register accessor: an alias for `Reg<AFIO_RMP_CFG4_SPEC>`"]
pub type AFIO_RMP_CFG4 = crate::Reg<afio_rmp_cfg4::AFIO_RMP_CFG4_SPEC>;
#[doc = "AFIO_RMP_CFG4"]
pub mod afio_rmp_cfg4;
#[doc = "AFIO_RMP_CFG5 register accessor: an alias for `Reg<AFIO_RMP_CFG5_SPEC>`"]
pub type AFIO_RMP_CFG5 = crate::Reg<afio_rmp_cfg5::AFIO_RMP_CFG5_SPEC>;
#[doc = "AFIO_RMP_CFG5"]
pub mod afio_rmp_cfg5;
