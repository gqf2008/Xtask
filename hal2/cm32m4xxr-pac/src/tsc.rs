#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TSC_CTRL"]
    pub tsc_ctrl: crate::Reg<tsc_ctrl::TSC_CTRL_SPEC>,
    #[doc = "0x04 - TSC_CHNEN"]
    pub tsc_chnen: crate::Reg<tsc_chnen::TSC_CHNEN_SPEC>,
    #[doc = "0x08 - TSC_STS"]
    pub tsc_sts: crate::Reg<tsc_sts::TSC_STS_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - TSC_ANA_CTRL"]
    pub tsc_ana_ctrl: crate::Reg<tsc_ana_ctrl::TSC_ANA_CTRL_SPEC>,
    #[doc = "0x14 - TSC_ANA_SEL"]
    pub tsc_ana_sel: crate::Reg<tsc_ana_sel::TSC_ANA_SEL_SPEC>,
    #[doc = "0x18 - TSC_RESR0"]
    pub tsc_resr0: crate::Reg<tsc_resr0::TSC_RESR0_SPEC>,
    #[doc = "0x1c - TSC_RESR1"]
    pub tsc_resr1: crate::Reg<tsc_resr1::TSC_RESR1_SPEC>,
    #[doc = "0x20 - TSC_RESR2"]
    pub tsc_resr2: crate::Reg<tsc_resr2::TSC_RESR2_SPEC>,
    #[doc = "0x24 - TSC_THRHD0"]
    pub tsc_thrhd0: crate::Reg<tsc_thrhd0::TSC_THRHD0_SPEC>,
    #[doc = "0x28 - TSC_THRHD1"]
    pub tsc_thrhd1: crate::Reg<tsc_thrhd1::TSC_THRHD1_SPEC>,
    #[doc = "0x2c - TSC_THRHD2"]
    pub tsc_thrhd2: crate::Reg<tsc_thrhd2::TSC_THRHD2_SPEC>,
    #[doc = "0x30 - TSC_THRHD3"]
    pub tsc_thrhd3: crate::Reg<tsc_thrhd3::TSC_THRHD3_SPEC>,
    #[doc = "0x34 - TSC_THRHD4"]
    pub tsc_thrhd4: crate::Reg<tsc_thrhd4::TSC_THRHD4_SPEC>,
    #[doc = "0x38 - TSC_THRHD5"]
    pub tsc_thrhd5: crate::Reg<tsc_thrhd5::TSC_THRHD5_SPEC>,
    #[doc = "0x3c - TSC_THRHD6"]
    pub tsc_thrhd6: crate::Reg<tsc_thrhd6::TSC_THRHD6_SPEC>,
    #[doc = "0x40 - TSC_THRHD7"]
    pub tsc_thrhd7: crate::Reg<tsc_thrhd7::TSC_THRHD7_SPEC>,
    #[doc = "0x44 - TSC_THRHD8"]
    pub tsc_thrhd8: crate::Reg<tsc_thrhd8::TSC_THRHD8_SPEC>,
    #[doc = "0x48 - TSC_THRHD9"]
    pub tsc_thrhd9: crate::Reg<tsc_thrhd9::TSC_THRHD9_SPEC>,
    #[doc = "0x4c - TSC_THRHD10"]
    pub tsc_thrhd10: crate::Reg<tsc_thrhd10::TSC_THRHD10_SPEC>,
    #[doc = "0x50 - TSC_THRHD11"]
    pub tsc_thrhd11: crate::Reg<tsc_thrhd11::TSC_THRHD11_SPEC>,
    #[doc = "0x54 - TSC_THRHD12"]
    pub tsc_thrhd12: crate::Reg<tsc_thrhd12::TSC_THRHD12_SPEC>,
    #[doc = "0x58 - TSC_THRHD13"]
    pub tsc_thrhd13: crate::Reg<tsc_thrhd13::TSC_THRHD13_SPEC>,
    #[doc = "0x5c - TSC_THRHD14"]
    pub tsc_thrhd14: crate::Reg<tsc_thrhd14::TSC_THRHD14_SPEC>,
    #[doc = "0x60 - TSC_THRHD15"]
    pub tsc_thrhd15: crate::Reg<tsc_thrhd15::TSC_THRHD15_SPEC>,
    #[doc = "0x64 - TSC_THRHD16"]
    pub tsc_thrhd16: crate::Reg<tsc_thrhd16::TSC_THRHD16_SPEC>,
    #[doc = "0x68 - TSC_THRHD17"]
    pub tsc_thrhd17: crate::Reg<tsc_thrhd17::TSC_THRHD17_SPEC>,
    #[doc = "0x6c - TSC_THRHD18"]
    pub tsc_thrhd18: crate::Reg<tsc_thrhd18::TSC_THRHD18_SPEC>,
    #[doc = "0x70 - TSC_THRHD19"]
    pub tsc_thrhd19: crate::Reg<tsc_thrhd19::TSC_THRHD19_SPEC>,
    #[doc = "0x74 - TSC_THRHD20"]
    pub tsc_thrhd20: crate::Reg<tsc_thrhd20::TSC_THRHD20_SPEC>,
    #[doc = "0x78 - TSC_THRHD21"]
    pub tsc_thrhd21: crate::Reg<tsc_thrhd21::TSC_THRHD21_SPEC>,
    #[doc = "0x7c - TSC_THRHD22"]
    pub tsc_thrhd22: crate::Reg<tsc_thrhd22::TSC_THRHD22_SPEC>,
    #[doc = "0x80 - TSC_THRHD23"]
    pub tsc_thrhd23: crate::Reg<tsc_thrhd23::TSC_THRHD23_SPEC>,
}
#[doc = "TSC_CTRL register accessor: an alias for `Reg<TSC_CTRL_SPEC>`"]
pub type TSC_CTRL = crate::Reg<tsc_ctrl::TSC_CTRL_SPEC>;
#[doc = "TSC_CTRL"]
pub mod tsc_ctrl;
#[doc = "TSC_CHNEN register accessor: an alias for `Reg<TSC_CHNEN_SPEC>`"]
pub type TSC_CHNEN = crate::Reg<tsc_chnen::TSC_CHNEN_SPEC>;
#[doc = "TSC_CHNEN"]
pub mod tsc_chnen;
#[doc = "TSC_STS register accessor: an alias for `Reg<TSC_STS_SPEC>`"]
pub type TSC_STS = crate::Reg<tsc_sts::TSC_STS_SPEC>;
#[doc = "TSC_STS"]
pub mod tsc_sts;
#[doc = "TSC_ANA_CTRL register accessor: an alias for `Reg<TSC_ANA_CTRL_SPEC>`"]
pub type TSC_ANA_CTRL = crate::Reg<tsc_ana_ctrl::TSC_ANA_CTRL_SPEC>;
#[doc = "TSC_ANA_CTRL"]
pub mod tsc_ana_ctrl;
#[doc = "TSC_ANA_SEL register accessor: an alias for `Reg<TSC_ANA_SEL_SPEC>`"]
pub type TSC_ANA_SEL = crate::Reg<tsc_ana_sel::TSC_ANA_SEL_SPEC>;
#[doc = "TSC_ANA_SEL"]
pub mod tsc_ana_sel;
#[doc = "TSC_RESR0 register accessor: an alias for `Reg<TSC_RESR0_SPEC>`"]
pub type TSC_RESR0 = crate::Reg<tsc_resr0::TSC_RESR0_SPEC>;
#[doc = "TSC_RESR0"]
pub mod tsc_resr0;
#[doc = "TSC_RESR1 register accessor: an alias for `Reg<TSC_RESR1_SPEC>`"]
pub type TSC_RESR1 = crate::Reg<tsc_resr1::TSC_RESR1_SPEC>;
#[doc = "TSC_RESR1"]
pub mod tsc_resr1;
#[doc = "TSC_RESR2 register accessor: an alias for `Reg<TSC_RESR2_SPEC>`"]
pub type TSC_RESR2 = crate::Reg<tsc_resr2::TSC_RESR2_SPEC>;
#[doc = "TSC_RESR2"]
pub mod tsc_resr2;
#[doc = "TSC_THRHD0 register accessor: an alias for `Reg<TSC_THRHD0_SPEC>`"]
pub type TSC_THRHD0 = crate::Reg<tsc_thrhd0::TSC_THRHD0_SPEC>;
#[doc = "TSC_THRHD0"]
pub mod tsc_thrhd0;
#[doc = "TSC_THRHD1 register accessor: an alias for `Reg<TSC_THRHD1_SPEC>`"]
pub type TSC_THRHD1 = crate::Reg<tsc_thrhd1::TSC_THRHD1_SPEC>;
#[doc = "TSC_THRHD1"]
pub mod tsc_thrhd1;
#[doc = "TSC_THRHD2 register accessor: an alias for `Reg<TSC_THRHD2_SPEC>`"]
pub type TSC_THRHD2 = crate::Reg<tsc_thrhd2::TSC_THRHD2_SPEC>;
#[doc = "TSC_THRHD2"]
pub mod tsc_thrhd2;
#[doc = "TSC_THRHD3 register accessor: an alias for `Reg<TSC_THRHD3_SPEC>`"]
pub type TSC_THRHD3 = crate::Reg<tsc_thrhd3::TSC_THRHD3_SPEC>;
#[doc = "TSC_THRHD3"]
pub mod tsc_thrhd3;
#[doc = "TSC_THRHD4 register accessor: an alias for `Reg<TSC_THRHD4_SPEC>`"]
pub type TSC_THRHD4 = crate::Reg<tsc_thrhd4::TSC_THRHD4_SPEC>;
#[doc = "TSC_THRHD4"]
pub mod tsc_thrhd4;
#[doc = "TSC_THRHD5 register accessor: an alias for `Reg<TSC_THRHD5_SPEC>`"]
pub type TSC_THRHD5 = crate::Reg<tsc_thrhd5::TSC_THRHD5_SPEC>;
#[doc = "TSC_THRHD5"]
pub mod tsc_thrhd5;
#[doc = "TSC_THRHD6 register accessor: an alias for `Reg<TSC_THRHD6_SPEC>`"]
pub type TSC_THRHD6 = crate::Reg<tsc_thrhd6::TSC_THRHD6_SPEC>;
#[doc = "TSC_THRHD6"]
pub mod tsc_thrhd6;
#[doc = "TSC_THRHD7 register accessor: an alias for `Reg<TSC_THRHD7_SPEC>`"]
pub type TSC_THRHD7 = crate::Reg<tsc_thrhd7::TSC_THRHD7_SPEC>;
#[doc = "TSC_THRHD7"]
pub mod tsc_thrhd7;
#[doc = "TSC_THRHD8 register accessor: an alias for `Reg<TSC_THRHD8_SPEC>`"]
pub type TSC_THRHD8 = crate::Reg<tsc_thrhd8::TSC_THRHD8_SPEC>;
#[doc = "TSC_THRHD8"]
pub mod tsc_thrhd8;
#[doc = "TSC_THRHD9 register accessor: an alias for `Reg<TSC_THRHD9_SPEC>`"]
pub type TSC_THRHD9 = crate::Reg<tsc_thrhd9::TSC_THRHD9_SPEC>;
#[doc = "TSC_THRHD9"]
pub mod tsc_thrhd9;
#[doc = "TSC_THRHD10 register accessor: an alias for `Reg<TSC_THRHD10_SPEC>`"]
pub type TSC_THRHD10 = crate::Reg<tsc_thrhd10::TSC_THRHD10_SPEC>;
#[doc = "TSC_THRHD10"]
pub mod tsc_thrhd10;
#[doc = "TSC_THRHD11 register accessor: an alias for `Reg<TSC_THRHD11_SPEC>`"]
pub type TSC_THRHD11 = crate::Reg<tsc_thrhd11::TSC_THRHD11_SPEC>;
#[doc = "TSC_THRHD11"]
pub mod tsc_thrhd11;
#[doc = "TSC_THRHD12 register accessor: an alias for `Reg<TSC_THRHD12_SPEC>`"]
pub type TSC_THRHD12 = crate::Reg<tsc_thrhd12::TSC_THRHD12_SPEC>;
#[doc = "TSC_THRHD12"]
pub mod tsc_thrhd12;
#[doc = "TSC_THRHD13 register accessor: an alias for `Reg<TSC_THRHD13_SPEC>`"]
pub type TSC_THRHD13 = crate::Reg<tsc_thrhd13::TSC_THRHD13_SPEC>;
#[doc = "TSC_THRHD13"]
pub mod tsc_thrhd13;
#[doc = "TSC_THRHD14 register accessor: an alias for `Reg<TSC_THRHD14_SPEC>`"]
pub type TSC_THRHD14 = crate::Reg<tsc_thrhd14::TSC_THRHD14_SPEC>;
#[doc = "TSC_THRHD14"]
pub mod tsc_thrhd14;
#[doc = "TSC_THRHD15 register accessor: an alias for `Reg<TSC_THRHD15_SPEC>`"]
pub type TSC_THRHD15 = crate::Reg<tsc_thrhd15::TSC_THRHD15_SPEC>;
#[doc = "TSC_THRHD15"]
pub mod tsc_thrhd15;
#[doc = "TSC_THRHD16 register accessor: an alias for `Reg<TSC_THRHD16_SPEC>`"]
pub type TSC_THRHD16 = crate::Reg<tsc_thrhd16::TSC_THRHD16_SPEC>;
#[doc = "TSC_THRHD16"]
pub mod tsc_thrhd16;
#[doc = "TSC_THRHD17 register accessor: an alias for `Reg<TSC_THRHD17_SPEC>`"]
pub type TSC_THRHD17 = crate::Reg<tsc_thrhd17::TSC_THRHD17_SPEC>;
#[doc = "TSC_THRHD17"]
pub mod tsc_thrhd17;
#[doc = "TSC_THRHD18 register accessor: an alias for `Reg<TSC_THRHD18_SPEC>`"]
pub type TSC_THRHD18 = crate::Reg<tsc_thrhd18::TSC_THRHD18_SPEC>;
#[doc = "TSC_THRHD18"]
pub mod tsc_thrhd18;
#[doc = "TSC_THRHD19 register accessor: an alias for `Reg<TSC_THRHD19_SPEC>`"]
pub type TSC_THRHD19 = crate::Reg<tsc_thrhd19::TSC_THRHD19_SPEC>;
#[doc = "TSC_THRHD19"]
pub mod tsc_thrhd19;
#[doc = "TSC_THRHD20 register accessor: an alias for `Reg<TSC_THRHD20_SPEC>`"]
pub type TSC_THRHD20 = crate::Reg<tsc_thrhd20::TSC_THRHD20_SPEC>;
#[doc = "TSC_THRHD20"]
pub mod tsc_thrhd20;
#[doc = "TSC_THRHD21 register accessor: an alias for `Reg<TSC_THRHD21_SPEC>`"]
pub type TSC_THRHD21 = crate::Reg<tsc_thrhd21::TSC_THRHD21_SPEC>;
#[doc = "TSC_THRHD21"]
pub mod tsc_thrhd21;
#[doc = "TSC_THRHD22 register accessor: an alias for `Reg<TSC_THRHD22_SPEC>`"]
pub type TSC_THRHD22 = crate::Reg<tsc_thrhd22::TSC_THRHD22_SPEC>;
#[doc = "TSC_THRHD22"]
pub mod tsc_thrhd22;
#[doc = "TSC_THRHD23 register accessor: an alias for `Reg<TSC_THRHD23_SPEC>`"]
pub type TSC_THRHD23 = crate::Reg<tsc_thrhd23::TSC_THRHD23_SPEC>;
#[doc = "TSC_THRHD23"]
pub mod tsc_thrhd23;
