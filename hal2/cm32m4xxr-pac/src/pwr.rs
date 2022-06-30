#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWR_CTRL1"]
    pub pwr_ctrl1: crate::Reg<pwr_ctrl1::PWR_CTRL1_SPEC>,
    #[doc = "0x04 - PWR_CTRLSTS"]
    pub pwr_ctrlsts: crate::Reg<pwr_ctrlsts::PWR_CTRLSTS_SPEC>,
    #[doc = "0x08 - PWR_CTRL2"]
    pub pwr_ctrl2: crate::Reg<pwr_ctrl2::PWR_CTRL2_SPEC>,
    #[doc = "0x0c - PWR_CTRL3"]
    pub pwr_ctrl3: crate::Reg<pwr_ctrl3::PWR_CTRL3_SPEC>,
    #[doc = "0x10 - PWR_CTRL4"]
    pub pwr_ctrl4: crate::Reg<pwr_ctrl4::PWR_CTRL4_SPEC>,
    #[doc = "0x14 - PWR_CTRL5"]
    pub pwr_ctrl5: crate::Reg<pwr_ctrl5::PWR_CTRL5_SPEC>,
    #[doc = "0x18 - PWR_CTRL6"]
    pub pwr_ctrl6: crate::Reg<pwr_ctrl6::PWR_CTRL6_SPEC>,
    #[doc = "0x1c - PWR_CTRL7"]
    pub pwr_ctrl7: crate::Reg<pwr_ctrl7::PWR_CTRL7_SPEC>,
}
#[doc = "PWR_CTRL1 register accessor: an alias for `Reg<PWR_CTRL1_SPEC>`"]
pub type PWR_CTRL1 = crate::Reg<pwr_ctrl1::PWR_CTRL1_SPEC>;
#[doc = "PWR_CTRL1"]
pub mod pwr_ctrl1;
#[doc = "PWR_CTRLSTS register accessor: an alias for `Reg<PWR_CTRLSTS_SPEC>`"]
pub type PWR_CTRLSTS = crate::Reg<pwr_ctrlsts::PWR_CTRLSTS_SPEC>;
#[doc = "PWR_CTRLSTS"]
pub mod pwr_ctrlsts;
#[doc = "PWR_CTRL2 register accessor: an alias for `Reg<PWR_CTRL2_SPEC>`"]
pub type PWR_CTRL2 = crate::Reg<pwr_ctrl2::PWR_CTRL2_SPEC>;
#[doc = "PWR_CTRL2"]
pub mod pwr_ctrl2;
#[doc = "PWR_CTRL3 register accessor: an alias for `Reg<PWR_CTRL3_SPEC>`"]
pub type PWR_CTRL3 = crate::Reg<pwr_ctrl3::PWR_CTRL3_SPEC>;
#[doc = "PWR_CTRL3"]
pub mod pwr_ctrl3;
#[doc = "PWR_CTRL4 register accessor: an alias for `Reg<PWR_CTRL4_SPEC>`"]
pub type PWR_CTRL4 = crate::Reg<pwr_ctrl4::PWR_CTRL4_SPEC>;
#[doc = "PWR_CTRL4"]
pub mod pwr_ctrl4;
#[doc = "PWR_CTRL5 register accessor: an alias for `Reg<PWR_CTRL5_SPEC>`"]
pub type PWR_CTRL5 = crate::Reg<pwr_ctrl5::PWR_CTRL5_SPEC>;
#[doc = "PWR_CTRL5"]
pub mod pwr_ctrl5;
#[doc = "PWR_CTRL6 register accessor: an alias for `Reg<PWR_CTRL6_SPEC>`"]
pub type PWR_CTRL6 = crate::Reg<pwr_ctrl6::PWR_CTRL6_SPEC>;
#[doc = "PWR_CTRL6"]
pub mod pwr_ctrl6;
#[doc = "PWR_CTRL7 register accessor: an alias for `Reg<PWR_CTRL7_SPEC>`"]
pub type PWR_CTRL7 = crate::Reg<pwr_ctrl7::PWR_CTRL7_SPEC>;
#[doc = "PWR_CTRL7"]
pub mod pwr_ctrl7;
