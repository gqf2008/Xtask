#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RCC_CTRL"]
    pub rcc_ctrl: crate::Reg<rcc_ctrl::RCC_CTRL_SPEC>,
    #[doc = "0x04 - RCC_CFG"]
    pub rcc_cfg: crate::Reg<rcc_cfg::RCC_CFG_SPEC>,
    #[doc = "0x08 - RCC_CLKINT"]
    pub rcc_clkint: crate::Reg<rcc_clkint::RCC_CLKINT_SPEC>,
    #[doc = "0x0c - RCC_APB2PRST"]
    pub rcc_apb2prst: crate::Reg<rcc_apb2prst::RCC_APB2PRST_SPEC>,
    #[doc = "0x10 - RCC_APB1PRST"]
    pub rcc_apb1prst: crate::Reg<rcc_apb1prst::RCC_APB1PRST_SPEC>,
    #[doc = "0x14 - RCC_AHBPCLKEN"]
    pub rcc_ahbpclken: crate::Reg<rcc_ahbpclken::RCC_AHBPCLKEN_SPEC>,
    #[doc = "0x18 - RCC_APB2PCLKEN"]
    pub rcc_apb2pclken: crate::Reg<rcc_apb2pclken::RCC_APB2PCLKEN_SPEC>,
    #[doc = "0x1c - RCC_APB1PCLKEN"]
    pub rcc_apb1pclken: crate::Reg<rcc_apb1pclken::RCC_APB1PCLKEN_SPEC>,
    #[doc = "0x20 - RCC_BDCTRL"]
    pub rcc_bdctrl: crate::Reg<rcc_bdctrl::RCC_BDCTRL_SPEC>,
    #[doc = "0x24 - RCC_CTRLSTS"]
    pub rcc_ctrlsts: crate::Reg<rcc_ctrlsts::RCC_CTRLSTS_SPEC>,
    #[doc = "0x28 - RCC_AHBPRST"]
    pub rcc_ahbprst: crate::Reg<rcc_ahbprst::RCC_AHBPRST_SPEC>,
    #[doc = "0x2c - RCC_CFG2"]
    pub rcc_cfg2: crate::Reg<rcc_cfg2::RCC_CFG2_SPEC>,
    #[doc = "0x30 - RCC_CFG3"]
    pub rcc_cfg3: crate::Reg<rcc_cfg3::RCC_CFG3_SPEC>,
}
#[doc = "RCC_CTRL register accessor: an alias for `Reg<RCC_CTRL_SPEC>`"]
pub type RCC_CTRL = crate::Reg<rcc_ctrl::RCC_CTRL_SPEC>;
#[doc = "RCC_CTRL"]
pub mod rcc_ctrl;
#[doc = "RCC_CFG register accessor: an alias for `Reg<RCC_CFG_SPEC>`"]
pub type RCC_CFG = crate::Reg<rcc_cfg::RCC_CFG_SPEC>;
#[doc = "RCC_CFG"]
pub mod rcc_cfg;
#[doc = "RCC_CLKINT register accessor: an alias for `Reg<RCC_CLKINT_SPEC>`"]
pub type RCC_CLKINT = crate::Reg<rcc_clkint::RCC_CLKINT_SPEC>;
#[doc = "RCC_CLKINT"]
pub mod rcc_clkint;
#[doc = "RCC_APB2PRST register accessor: an alias for `Reg<RCC_APB2PRST_SPEC>`"]
pub type RCC_APB2PRST = crate::Reg<rcc_apb2prst::RCC_APB2PRST_SPEC>;
#[doc = "RCC_APB2PRST"]
pub mod rcc_apb2prst;
#[doc = "RCC_APB1PRST register accessor: an alias for `Reg<RCC_APB1PRST_SPEC>`"]
pub type RCC_APB1PRST = crate::Reg<rcc_apb1prst::RCC_APB1PRST_SPEC>;
#[doc = "RCC_APB1PRST"]
pub mod rcc_apb1prst;
#[doc = "RCC_AHBPCLKEN register accessor: an alias for `Reg<RCC_AHBPCLKEN_SPEC>`"]
pub type RCC_AHBPCLKEN = crate::Reg<rcc_ahbpclken::RCC_AHBPCLKEN_SPEC>;
#[doc = "RCC_AHBPCLKEN"]
pub mod rcc_ahbpclken;
#[doc = "RCC_APB2PCLKEN register accessor: an alias for `Reg<RCC_APB2PCLKEN_SPEC>`"]
pub type RCC_APB2PCLKEN = crate::Reg<rcc_apb2pclken::RCC_APB2PCLKEN_SPEC>;
#[doc = "RCC_APB2PCLKEN"]
pub mod rcc_apb2pclken;
#[doc = "RCC_APB1PCLKEN register accessor: an alias for `Reg<RCC_APB1PCLKEN_SPEC>`"]
pub type RCC_APB1PCLKEN = crate::Reg<rcc_apb1pclken::RCC_APB1PCLKEN_SPEC>;
#[doc = "RCC_APB1PCLKEN"]
pub mod rcc_apb1pclken;
#[doc = "RCC_BDCTRL register accessor: an alias for `Reg<RCC_BDCTRL_SPEC>`"]
pub type RCC_BDCTRL = crate::Reg<rcc_bdctrl::RCC_BDCTRL_SPEC>;
#[doc = "RCC_BDCTRL"]
pub mod rcc_bdctrl;
#[doc = "RCC_CTRLSTS register accessor: an alias for `Reg<RCC_CTRLSTS_SPEC>`"]
pub type RCC_CTRLSTS = crate::Reg<rcc_ctrlsts::RCC_CTRLSTS_SPEC>;
#[doc = "RCC_CTRLSTS"]
pub mod rcc_ctrlsts;
#[doc = "RCC_AHBPRST register accessor: an alias for `Reg<RCC_AHBPRST_SPEC>`"]
pub type RCC_AHBPRST = crate::Reg<rcc_ahbprst::RCC_AHBPRST_SPEC>;
#[doc = "RCC_AHBPRST"]
pub mod rcc_ahbprst;
#[doc = "RCC_CFG2 register accessor: an alias for `Reg<RCC_CFG2_SPEC>`"]
pub type RCC_CFG2 = crate::Reg<rcc_cfg2::RCC_CFG2_SPEC>;
#[doc = "RCC_CFG2"]
pub mod rcc_cfg2;
#[doc = "RCC_CFG3 register accessor: an alias for `Reg<RCC_CFG3_SPEC>`"]
pub type RCC_CFG3 = crate::Reg<rcc_cfg3::RCC_CFG3_SPEC>;
#[doc = "RCC_CFG3"]
pub mod rcc_cfg3;
