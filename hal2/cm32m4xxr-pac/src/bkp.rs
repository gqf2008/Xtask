#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - BKP_DAT1"]
    pub bkp_dat1: crate::Reg<bkp_dat1::BKP_DAT1_SPEC>,
    #[doc = "0x08 - BKP_DAT2"]
    pub bkp_dat2: crate::Reg<bkp_dat2::BKP_DAT2_SPEC>,
    #[doc = "0x0c - BKP_DAT3"]
    pub bkp_dat3: crate::Reg<bkp_dat3::BKP_DAT3_SPEC>,
    #[doc = "0x10 - BKP_DAT4"]
    pub bkp_dat4: crate::Reg<bkp_dat4::BKP_DAT4_SPEC>,
    #[doc = "0x14 - BKP_DAT5"]
    pub bkp_dat5: crate::Reg<bkp_dat5::BKP_DAT5_SPEC>,
    #[doc = "0x18 - BKP_DAT6"]
    pub bkp_dat6: crate::Reg<bkp_dat6::BKP_DAT6_SPEC>,
    #[doc = "0x1c - BKP_DAT7"]
    pub bkp_dat7: crate::Reg<bkp_dat7::BKP_DAT7_SPEC>,
    #[doc = "0x20 - BKP_DAT8"]
    pub bkp_dat8: crate::Reg<bkp_dat8::BKP_DAT8_SPEC>,
    #[doc = "0x24 - BKP_DAT9"]
    pub bkp_dat9: crate::Reg<bkp_dat9::BKP_DAT9_SPEC>,
    #[doc = "0x28 - BKP_DAT10"]
    pub bkp_dat10: crate::Reg<bkp_dat10::BKP_DAT10_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - BKP_CTRL"]
    pub bkp_ctrl: crate::Reg<bkp_ctrl::BKP_CTRL_SPEC>,
    #[doc = "0x34 - BKP_CSTS"]
    pub bkp_csts: crate::Reg<bkp_csts::BKP_CSTS_SPEC>,
    _reserved12: [u8; 0x08],
    #[doc = "0x40 - BKP_DAT11"]
    pub bkp_dat11: crate::Reg<bkp_dat11::BKP_DAT11_SPEC>,
    #[doc = "0x44 - BKP_DAT12"]
    pub bkp_dat12: crate::Reg<bkp_dat12::BKP_DAT12_SPEC>,
    #[doc = "0x48 - BKP_DAT13"]
    pub bkp_dat13: crate::Reg<bkp_dat13::BKP_DAT13_SPEC>,
    #[doc = "0x4c - BKP_DAT14"]
    pub bkp_dat14: crate::Reg<bkp_dat14::BKP_DAT14_SPEC>,
    #[doc = "0x50 - BKP_DAT15"]
    pub bkp_dat15: crate::Reg<bkp_dat15::BKP_DAT15_SPEC>,
    #[doc = "0x54 - BKP_DAT16"]
    pub bkp_dat16: crate::Reg<bkp_dat16::BKP_DAT16_SPEC>,
    #[doc = "0x58 - BKP_DAT17"]
    pub bkp_dat17: crate::Reg<bkp_dat17::BKP_DAT17_SPEC>,
    #[doc = "0x5c - BKP_DAT18"]
    pub bkp_dat18: crate::Reg<bkp_dat18::BKP_DAT18_SPEC>,
    #[doc = "0x60 - BKP_DAT19"]
    pub bkp_dat19: crate::Reg<bkp_dat19::BKP_DAT19_SPEC>,
    #[doc = "0x64 - BKP_DAT20"]
    pub bkp_dat20: crate::Reg<bkp_dat20::BKP_DAT20_SPEC>,
    #[doc = "0x68 - BKP_DAT21"]
    pub bkp_dat21: crate::Reg<bkp_dat21::BKP_DAT21_SPEC>,
    #[doc = "0x6c - BKP_DAT22"]
    pub bkp_dat22: crate::Reg<bkp_dat22::BKP_DAT22_SPEC>,
    #[doc = "0x70 - BKP_DAT23"]
    pub bkp_dat23: crate::Reg<bkp_dat23::BKP_DAT23_SPEC>,
    #[doc = "0x74 - BKP_DAT24"]
    pub bkp_dat24: crate::Reg<bkp_dat24::BKP_DAT24_SPEC>,
    #[doc = "0x78 - BKP_DAT25"]
    pub bkp_dat25: crate::Reg<bkp_dat25::BKP_DAT25_SPEC>,
    #[doc = "0x7c - BKP_DAT26"]
    pub bkp_dat26: crate::Reg<bkp_dat26::BKP_DAT26_SPEC>,
    #[doc = "0x80 - BKP_DAT27"]
    pub bkp_dat27: crate::Reg<bkp_dat27::BKP_DAT27_SPEC>,
    #[doc = "0x84 - BKP_DAT28"]
    pub bkp_dat28: crate::Reg<bkp_dat28::BKP_DAT28_SPEC>,
    #[doc = "0x88 - BKP_DAT29"]
    pub bkp_dat29: crate::Reg<bkp_dat29::BKP_DAT29_SPEC>,
    #[doc = "0x8c - BKP_DAT30"]
    pub bkp_dat30: crate::Reg<bkp_dat30::BKP_DAT30_SPEC>,
    #[doc = "0x90 - BKP_DAT31"]
    pub bkp_dat31: crate::Reg<bkp_dat31::BKP_DAT31_SPEC>,
    #[doc = "0x94 - BKP_DAT32"]
    pub bkp_dat32: crate::Reg<bkp_dat32::BKP_DAT32_SPEC>,
    #[doc = "0x98 - BKP_DAT33"]
    pub bkp_dat33: crate::Reg<bkp_dat33::BKP_DAT33_SPEC>,
    #[doc = "0x9c - BKP_DAT34"]
    pub bkp_dat34: crate::Reg<bkp_dat34::BKP_DAT34_SPEC>,
    #[doc = "0xa0 - BKP_DAT35"]
    pub bkp_dat35: crate::Reg<bkp_dat35::BKP_DAT35_SPEC>,
    #[doc = "0xa4 - BKP_DAT36"]
    pub bkp_dat36: crate::Reg<bkp_dat36::BKP_DAT36_SPEC>,
    #[doc = "0xa8 - BKP_DAT37"]
    pub bkp_dat37: crate::Reg<bkp_dat37::BKP_DAT37_SPEC>,
    #[doc = "0xac - BKP_DAT38"]
    pub bkp_dat38: crate::Reg<bkp_dat38::BKP_DAT38_SPEC>,
    #[doc = "0xb0 - BKP_DAT39"]
    pub bkp_dat39: crate::Reg<bkp_dat39::BKP_DAT39_SPEC>,
    #[doc = "0xb4 - BKP_DAT40"]
    pub bkp_dat40: crate::Reg<bkp_dat40::BKP_DAT40_SPEC>,
    #[doc = "0xb8 - BKP_DAT41"]
    pub bkp_dat41: crate::Reg<bkp_dat41::BKP_DAT41_SPEC>,
    #[doc = "0xbc - BKP_DAT42"]
    pub bkp_dat42: crate::Reg<bkp_dat42::BKP_DAT42_SPEC>,
}
#[doc = "BKP_DAT1 register accessor: an alias for `Reg<BKP_DAT1_SPEC>`"]
pub type BKP_DAT1 = crate::Reg<bkp_dat1::BKP_DAT1_SPEC>;
#[doc = "BKP_DAT1"]
pub mod bkp_dat1;
#[doc = "BKP_DAT2 register accessor: an alias for `Reg<BKP_DAT2_SPEC>`"]
pub type BKP_DAT2 = crate::Reg<bkp_dat2::BKP_DAT2_SPEC>;
#[doc = "BKP_DAT2"]
pub mod bkp_dat2;
#[doc = "BKP_DAT3 register accessor: an alias for `Reg<BKP_DAT3_SPEC>`"]
pub type BKP_DAT3 = crate::Reg<bkp_dat3::BKP_DAT3_SPEC>;
#[doc = "BKP_DAT3"]
pub mod bkp_dat3;
#[doc = "BKP_DAT4 register accessor: an alias for `Reg<BKP_DAT4_SPEC>`"]
pub type BKP_DAT4 = crate::Reg<bkp_dat4::BKP_DAT4_SPEC>;
#[doc = "BKP_DAT4"]
pub mod bkp_dat4;
#[doc = "BKP_DAT5 register accessor: an alias for `Reg<BKP_DAT5_SPEC>`"]
pub type BKP_DAT5 = crate::Reg<bkp_dat5::BKP_DAT5_SPEC>;
#[doc = "BKP_DAT5"]
pub mod bkp_dat5;
#[doc = "BKP_DAT6 register accessor: an alias for `Reg<BKP_DAT6_SPEC>`"]
pub type BKP_DAT6 = crate::Reg<bkp_dat6::BKP_DAT6_SPEC>;
#[doc = "BKP_DAT6"]
pub mod bkp_dat6;
#[doc = "BKP_DAT7 register accessor: an alias for `Reg<BKP_DAT7_SPEC>`"]
pub type BKP_DAT7 = crate::Reg<bkp_dat7::BKP_DAT7_SPEC>;
#[doc = "BKP_DAT7"]
pub mod bkp_dat7;
#[doc = "BKP_DAT8 register accessor: an alias for `Reg<BKP_DAT8_SPEC>`"]
pub type BKP_DAT8 = crate::Reg<bkp_dat8::BKP_DAT8_SPEC>;
#[doc = "BKP_DAT8"]
pub mod bkp_dat8;
#[doc = "BKP_DAT9 register accessor: an alias for `Reg<BKP_DAT9_SPEC>`"]
pub type BKP_DAT9 = crate::Reg<bkp_dat9::BKP_DAT9_SPEC>;
#[doc = "BKP_DAT9"]
pub mod bkp_dat9;
#[doc = "BKP_DAT10 register accessor: an alias for `Reg<BKP_DAT10_SPEC>`"]
pub type BKP_DAT10 = crate::Reg<bkp_dat10::BKP_DAT10_SPEC>;
#[doc = "BKP_DAT10"]
pub mod bkp_dat10;
#[doc = "BKP_CTRL register accessor: an alias for `Reg<BKP_CTRL_SPEC>`"]
pub type BKP_CTRL = crate::Reg<bkp_ctrl::BKP_CTRL_SPEC>;
#[doc = "BKP_CTRL"]
pub mod bkp_ctrl;
#[doc = "BKP_CSTS register accessor: an alias for `Reg<BKP_CSTS_SPEC>`"]
pub type BKP_CSTS = crate::Reg<bkp_csts::BKP_CSTS_SPEC>;
#[doc = "BKP_CSTS"]
pub mod bkp_csts;
#[doc = "BKP_DAT11 register accessor: an alias for `Reg<BKP_DAT11_SPEC>`"]
pub type BKP_DAT11 = crate::Reg<bkp_dat11::BKP_DAT11_SPEC>;
#[doc = "BKP_DAT11"]
pub mod bkp_dat11;
#[doc = "BKP_DAT12 register accessor: an alias for `Reg<BKP_DAT12_SPEC>`"]
pub type BKP_DAT12 = crate::Reg<bkp_dat12::BKP_DAT12_SPEC>;
#[doc = "BKP_DAT12"]
pub mod bkp_dat12;
#[doc = "BKP_DAT13 register accessor: an alias for `Reg<BKP_DAT13_SPEC>`"]
pub type BKP_DAT13 = crate::Reg<bkp_dat13::BKP_DAT13_SPEC>;
#[doc = "BKP_DAT13"]
pub mod bkp_dat13;
#[doc = "BKP_DAT14 register accessor: an alias for `Reg<BKP_DAT14_SPEC>`"]
pub type BKP_DAT14 = crate::Reg<bkp_dat14::BKP_DAT14_SPEC>;
#[doc = "BKP_DAT14"]
pub mod bkp_dat14;
#[doc = "BKP_DAT15 register accessor: an alias for `Reg<BKP_DAT15_SPEC>`"]
pub type BKP_DAT15 = crate::Reg<bkp_dat15::BKP_DAT15_SPEC>;
#[doc = "BKP_DAT15"]
pub mod bkp_dat15;
#[doc = "BKP_DAT16 register accessor: an alias for `Reg<BKP_DAT16_SPEC>`"]
pub type BKP_DAT16 = crate::Reg<bkp_dat16::BKP_DAT16_SPEC>;
#[doc = "BKP_DAT16"]
pub mod bkp_dat16;
#[doc = "BKP_DAT17 register accessor: an alias for `Reg<BKP_DAT17_SPEC>`"]
pub type BKP_DAT17 = crate::Reg<bkp_dat17::BKP_DAT17_SPEC>;
#[doc = "BKP_DAT17"]
pub mod bkp_dat17;
#[doc = "BKP_DAT18 register accessor: an alias for `Reg<BKP_DAT18_SPEC>`"]
pub type BKP_DAT18 = crate::Reg<bkp_dat18::BKP_DAT18_SPEC>;
#[doc = "BKP_DAT18"]
pub mod bkp_dat18;
#[doc = "BKP_DAT19 register accessor: an alias for `Reg<BKP_DAT19_SPEC>`"]
pub type BKP_DAT19 = crate::Reg<bkp_dat19::BKP_DAT19_SPEC>;
#[doc = "BKP_DAT19"]
pub mod bkp_dat19;
#[doc = "BKP_DAT20 register accessor: an alias for `Reg<BKP_DAT20_SPEC>`"]
pub type BKP_DAT20 = crate::Reg<bkp_dat20::BKP_DAT20_SPEC>;
#[doc = "BKP_DAT20"]
pub mod bkp_dat20;
#[doc = "BKP_DAT21 register accessor: an alias for `Reg<BKP_DAT21_SPEC>`"]
pub type BKP_DAT21 = crate::Reg<bkp_dat21::BKP_DAT21_SPEC>;
#[doc = "BKP_DAT21"]
pub mod bkp_dat21;
#[doc = "BKP_DAT22 register accessor: an alias for `Reg<BKP_DAT22_SPEC>`"]
pub type BKP_DAT22 = crate::Reg<bkp_dat22::BKP_DAT22_SPEC>;
#[doc = "BKP_DAT22"]
pub mod bkp_dat22;
#[doc = "BKP_DAT23 register accessor: an alias for `Reg<BKP_DAT23_SPEC>`"]
pub type BKP_DAT23 = crate::Reg<bkp_dat23::BKP_DAT23_SPEC>;
#[doc = "BKP_DAT23"]
pub mod bkp_dat23;
#[doc = "BKP_DAT24 register accessor: an alias for `Reg<BKP_DAT24_SPEC>`"]
pub type BKP_DAT24 = crate::Reg<bkp_dat24::BKP_DAT24_SPEC>;
#[doc = "BKP_DAT24"]
pub mod bkp_dat24;
#[doc = "BKP_DAT25 register accessor: an alias for `Reg<BKP_DAT25_SPEC>`"]
pub type BKP_DAT25 = crate::Reg<bkp_dat25::BKP_DAT25_SPEC>;
#[doc = "BKP_DAT25"]
pub mod bkp_dat25;
#[doc = "BKP_DAT26 register accessor: an alias for `Reg<BKP_DAT26_SPEC>`"]
pub type BKP_DAT26 = crate::Reg<bkp_dat26::BKP_DAT26_SPEC>;
#[doc = "BKP_DAT26"]
pub mod bkp_dat26;
#[doc = "BKP_DAT27 register accessor: an alias for `Reg<BKP_DAT27_SPEC>`"]
pub type BKP_DAT27 = crate::Reg<bkp_dat27::BKP_DAT27_SPEC>;
#[doc = "BKP_DAT27"]
pub mod bkp_dat27;
#[doc = "BKP_DAT28 register accessor: an alias for `Reg<BKP_DAT28_SPEC>`"]
pub type BKP_DAT28 = crate::Reg<bkp_dat28::BKP_DAT28_SPEC>;
#[doc = "BKP_DAT28"]
pub mod bkp_dat28;
#[doc = "BKP_DAT29 register accessor: an alias for `Reg<BKP_DAT29_SPEC>`"]
pub type BKP_DAT29 = crate::Reg<bkp_dat29::BKP_DAT29_SPEC>;
#[doc = "BKP_DAT29"]
pub mod bkp_dat29;
#[doc = "BKP_DAT30 register accessor: an alias for `Reg<BKP_DAT30_SPEC>`"]
pub type BKP_DAT30 = crate::Reg<bkp_dat30::BKP_DAT30_SPEC>;
#[doc = "BKP_DAT30"]
pub mod bkp_dat30;
#[doc = "BKP_DAT31 register accessor: an alias for `Reg<BKP_DAT31_SPEC>`"]
pub type BKP_DAT31 = crate::Reg<bkp_dat31::BKP_DAT31_SPEC>;
#[doc = "BKP_DAT31"]
pub mod bkp_dat31;
#[doc = "BKP_DAT32 register accessor: an alias for `Reg<BKP_DAT32_SPEC>`"]
pub type BKP_DAT32 = crate::Reg<bkp_dat32::BKP_DAT32_SPEC>;
#[doc = "BKP_DAT32"]
pub mod bkp_dat32;
#[doc = "BKP_DAT33 register accessor: an alias for `Reg<BKP_DAT33_SPEC>`"]
pub type BKP_DAT33 = crate::Reg<bkp_dat33::BKP_DAT33_SPEC>;
#[doc = "BKP_DAT33"]
pub mod bkp_dat33;
#[doc = "BKP_DAT34 register accessor: an alias for `Reg<BKP_DAT34_SPEC>`"]
pub type BKP_DAT34 = crate::Reg<bkp_dat34::BKP_DAT34_SPEC>;
#[doc = "BKP_DAT34"]
pub mod bkp_dat34;
#[doc = "BKP_DAT35 register accessor: an alias for `Reg<BKP_DAT35_SPEC>`"]
pub type BKP_DAT35 = crate::Reg<bkp_dat35::BKP_DAT35_SPEC>;
#[doc = "BKP_DAT35"]
pub mod bkp_dat35;
#[doc = "BKP_DAT36 register accessor: an alias for `Reg<BKP_DAT36_SPEC>`"]
pub type BKP_DAT36 = crate::Reg<bkp_dat36::BKP_DAT36_SPEC>;
#[doc = "BKP_DAT36"]
pub mod bkp_dat36;
#[doc = "BKP_DAT37 register accessor: an alias for `Reg<BKP_DAT37_SPEC>`"]
pub type BKP_DAT37 = crate::Reg<bkp_dat37::BKP_DAT37_SPEC>;
#[doc = "BKP_DAT37"]
pub mod bkp_dat37;
#[doc = "BKP_DAT38 register accessor: an alias for `Reg<BKP_DAT38_SPEC>`"]
pub type BKP_DAT38 = crate::Reg<bkp_dat38::BKP_DAT38_SPEC>;
#[doc = "BKP_DAT38"]
pub mod bkp_dat38;
#[doc = "BKP_DAT39 register accessor: an alias for `Reg<BKP_DAT39_SPEC>`"]
pub type BKP_DAT39 = crate::Reg<bkp_dat39::BKP_DAT39_SPEC>;
#[doc = "BKP_DAT39"]
pub mod bkp_dat39;
#[doc = "BKP_DAT40 register accessor: an alias for `Reg<BKP_DAT40_SPEC>`"]
pub type BKP_DAT40 = crate::Reg<bkp_dat40::BKP_DAT40_SPEC>;
#[doc = "BKP_DAT40"]
pub mod bkp_dat40;
#[doc = "BKP_DAT41 register accessor: an alias for `Reg<BKP_DAT41_SPEC>`"]
pub type BKP_DAT41 = crate::Reg<bkp_dat41::BKP_DAT41_SPEC>;
#[doc = "BKP_DAT41"]
pub mod bkp_dat41;
#[doc = "BKP_DAT42 register accessor: an alias for `Reg<BKP_DAT42_SPEC>`"]
pub type BKP_DAT42 = crate::Reg<bkp_dat42::BKP_DAT42_SPEC>;
#[doc = "BKP_DAT42"]
pub mod bkp_dat42;
