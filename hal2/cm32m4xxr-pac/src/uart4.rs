#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USART_STS"]
    pub usart_sts: crate::Reg<usart_sts::USART_STS_SPEC>,
    #[doc = "0x04 - USART_DAT"]
    pub usart_dat: crate::Reg<usart_dat::USART_DAT_SPEC>,
    #[doc = "0x08 - USART_BRCF"]
    pub usart_brcf: crate::Reg<usart_brcf::USART_BRCF_SPEC>,
    #[doc = "0x0c - USART_CTRL1"]
    pub usart_ctrl1: crate::Reg<usart_ctrl1::USART_CTRL1_SPEC>,
    #[doc = "0x10 - USART_CTRL2"]
    pub usart_ctrl2: crate::Reg<usart_ctrl2::USART_CTRL2_SPEC>,
    #[doc = "0x14 - USART_CTRL3"]
    pub usart_ctrl3: crate::Reg<usart_ctrl3::USART_CTRL3_SPEC>,
    #[doc = "0x18 - USART_GTP"]
    pub usart_gtp: crate::Reg<usart_gtp::USART_GTP_SPEC>,
}
#[doc = "USART_STS register accessor: an alias for `Reg<USART_STS_SPEC>`"]
pub type USART_STS = crate::Reg<usart_sts::USART_STS_SPEC>;
#[doc = "USART_STS"]
pub mod usart_sts;
#[doc = "USART_DAT register accessor: an alias for `Reg<USART_DAT_SPEC>`"]
pub type USART_DAT = crate::Reg<usart_dat::USART_DAT_SPEC>;
#[doc = "USART_DAT"]
pub mod usart_dat;
#[doc = "USART_BRCF register accessor: an alias for `Reg<USART_BRCF_SPEC>`"]
pub type USART_BRCF = crate::Reg<usart_brcf::USART_BRCF_SPEC>;
#[doc = "USART_BRCF"]
pub mod usart_brcf;
#[doc = "USART_CTRL1 register accessor: an alias for `Reg<USART_CTRL1_SPEC>`"]
pub type USART_CTRL1 = crate::Reg<usart_ctrl1::USART_CTRL1_SPEC>;
#[doc = "USART_CTRL1"]
pub mod usart_ctrl1;
#[doc = "USART_CTRL2 register accessor: an alias for `Reg<USART_CTRL2_SPEC>`"]
pub type USART_CTRL2 = crate::Reg<usart_ctrl2::USART_CTRL2_SPEC>;
#[doc = "USART_CTRL2"]
pub mod usart_ctrl2;
#[doc = "USART_CTRL3 register accessor: an alias for `Reg<USART_CTRL3_SPEC>`"]
pub type USART_CTRL3 = crate::Reg<usart_ctrl3::USART_CTRL3_SPEC>;
#[doc = "USART_CTRL3"]
pub mod usart_ctrl3;
#[doc = "USART_GTP register accessor: an alias for `Reg<USART_GTP_SPEC>`"]
pub type USART_GTP = crate::Reg<usart_gtp::USART_GTP_SPEC>;
#[doc = "USART_GTP"]
pub mod usart_gtp;
