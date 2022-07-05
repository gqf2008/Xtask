#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Eclic global configuration register"]
    pub cliccfg: crate::Reg<cliccfg::CLICCFG_SPEC>,
    #[doc = "0x04 - global info register"]
    pub clicinfo: crate::Reg<clicinfo::CLICINFO_SPEC>,
    _reserved2: [u8; 0x03],
    #[doc = "0x0b - set the target interrupt threshold level"]
    pub mth: crate::Reg<mth::MTH_SPEC>,
    _reserved3: [u8; 0x0ff5],
    #[doc = "0x1004 - Interrupt control and state"]
    pub clicint1: crate::Reg<clicint1::CLICINT1_SPEC>,
    #[doc = "0x1008 - Interrupt control and state"]
    pub clicint2: crate::Reg<clicint2::CLICINT2_SPEC>,
    #[doc = "0x100c - Interrupt control and state"]
    pub clicint3: crate::Reg<clicint3::CLICINT3_SPEC>,
    #[doc = "0x1010 - Interrupt control and state"]
    pub clicint4: crate::Reg<clicint4::CLICINT4_SPEC>,
    #[doc = "0x1014 - Interrupt control and state"]
    pub clicint5: crate::Reg<clicint5::CLICINT5_SPEC>,
    #[doc = "0x1018 - Interrupt control and state"]
    pub clicint6: crate::Reg<clicint6::CLICINT6_SPEC>,
    #[doc = "0x101c - Interrupt control and state"]
    pub clicint7: crate::Reg<clicint7::CLICINT7_SPEC>,
    #[doc = "0x1020 - Interrupt control and state"]
    pub clicint8: crate::Reg<clicint8::CLICINT8_SPEC>,
    #[doc = "0x1024 - Interrupt control and state"]
    pub clicint9: crate::Reg<clicint9::CLICINT9_SPEC>,
    #[doc = "0x1028 - Interrupt control and state"]
    pub clicint10: crate::Reg<clicint10::CLICINT10_SPEC>,
    #[doc = "0x102c - Interrupt control and state"]
    pub clicint11: crate::Reg<clicint11::CLICINT11_SPEC>,
    #[doc = "0x1030 - Interrupt control and state"]
    pub clicint12: crate::Reg<clicint12::CLICINT12_SPEC>,
    #[doc = "0x1034 - Interrupt control and state"]
    pub clicint13: crate::Reg<clicint13::CLICINT13_SPEC>,
    #[doc = "0x1038 - Interrupt control and state"]
    pub clicint14: crate::Reg<clicint14::CLICINT14_SPEC>,
    #[doc = "0x103c - Interrupt control and state"]
    pub clicint15: crate::Reg<clicint15::CLICINT15_SPEC>,
    #[doc = "0x1040 - Interrupt control and state"]
    pub clicint16: crate::Reg<clicint16::CLICINT16_SPEC>,
    #[doc = "0x1044 - Interrupt control and state"]
    pub clicint17: crate::Reg<clicint17::CLICINT17_SPEC>,
    #[doc = "0x1048 - Interrupt control and state"]
    pub clicint18: crate::Reg<clicint18::CLICINT18_SPEC>,
    #[doc = "0x104c - Interrupt control and state"]
    pub clicint19: crate::Reg<clicint19::CLICINT19_SPEC>,
    #[doc = "0x1050 - Interrupt control and state"]
    pub clicint20: crate::Reg<clicint20::CLICINT20_SPEC>,
    #[doc = "0x1054 - Interrupt control and state"]
    pub clicint21: crate::Reg<clicint21::CLICINT21_SPEC>,
    #[doc = "0x1058 - Interrupt control and state"]
    pub clicint22: crate::Reg<clicint22::CLICINT22_SPEC>,
    #[doc = "0x105c - Interrupt control and state"]
    pub clicint23: crate::Reg<clicint23::CLICINT23_SPEC>,
    #[doc = "0x1060 - Interrupt control and state"]
    pub clicint24: crate::Reg<clicint24::CLICINT24_SPEC>,
    #[doc = "0x1064 - Interrupt control and state"]
    pub clicint25: crate::Reg<clicint25::CLICINT25_SPEC>,
    #[doc = "0x1068 - Interrupt control and state"]
    pub clicint26: crate::Reg<clicint26::CLICINT26_SPEC>,
    #[doc = "0x106c - Interrupt control and state"]
    pub clicint27: crate::Reg<clicint27::CLICINT27_SPEC>,
    #[doc = "0x1070 - Interrupt control and state"]
    pub clicint28: crate::Reg<clicint28::CLICINT28_SPEC>,
    #[doc = "0x1074 - Interrupt control and state"]
    pub clicint29: crate::Reg<clicint29::CLICINT29_SPEC>,
    #[doc = "0x1078 - Interrupt control and state"]
    pub clicint30: crate::Reg<clicint30::CLICINT30_SPEC>,
    #[doc = "0x107c - Interrupt control and state"]
    pub clicint31: crate::Reg<clicint31::CLICINT31_SPEC>,
    #[doc = "0x1080 - Interrupt control and state"]
    pub clicint32: crate::Reg<clicint32::CLICINT32_SPEC>,
    #[doc = "0x1084 - Interrupt control and state"]
    pub clicint33: crate::Reg<clicint33::CLICINT33_SPEC>,
    #[doc = "0x1088 - Interrupt control and state"]
    pub clicint34: crate::Reg<clicint34::CLICINT34_SPEC>,
    #[doc = "0x108c - Interrupt control and state"]
    pub clicint35: crate::Reg<clicint35::CLICINT35_SPEC>,
    #[doc = "0x1090 - Interrupt control and state"]
    pub clicint36: crate::Reg<clicint36::CLICINT36_SPEC>,
    #[doc = "0x1094 - Interrupt control and state"]
    pub clicint37: crate::Reg<clicint37::CLICINT37_SPEC>,
    #[doc = "0x1098 - Interrupt control and state"]
    pub clicint38: crate::Reg<clicint38::CLICINT38_SPEC>,
    #[doc = "0x109c - Interrupt control and state"]
    pub clicint39: crate::Reg<clicint39::CLICINT39_SPEC>,
    #[doc = "0x10a0 - Interrupt control and state"]
    pub clicint40: crate::Reg<clicint40::CLICINT40_SPEC>,
    #[doc = "0x10a4 - Interrupt control and state"]
    pub clicint41: crate::Reg<clicint41::CLICINT41_SPEC>,
    #[doc = "0x10a8 - Interrupt control and state"]
    pub clicint42: crate::Reg<clicint42::CLICINT42_SPEC>,
    #[doc = "0x10ac - Interrupt control and state"]
    pub clicint43: crate::Reg<clicint43::CLICINT43_SPEC>,
    #[doc = "0x10b0 - Interrupt control and state"]
    pub clicint44: crate::Reg<clicint44::CLICINT44_SPEC>,
    #[doc = "0x10b4 - Interrupt control and state"]
    pub clicint45: crate::Reg<clicint45::CLICINT45_SPEC>,
    #[doc = "0x10b8 - Interrupt control and state"]
    pub clicint46: crate::Reg<clicint46::CLICINT46_SPEC>,
    #[doc = "0x10bc - Interrupt control and state"]
    pub clicint47: crate::Reg<clicint47::CLICINT47_SPEC>,
    #[doc = "0x10c0 - Interrupt control and state"]
    pub clicint48: crate::Reg<clicint48::CLICINT48_SPEC>,
    #[doc = "0x10c4 - Interrupt control and state"]
    pub clicint49: crate::Reg<clicint49::CLICINT49_SPEC>,
    #[doc = "0x10c8 - Interrupt control and state"]
    pub clicint50: crate::Reg<clicint50::CLICINT50_SPEC>,
    #[doc = "0x10cc - Interrupt control and state"]
    pub clicint51: crate::Reg<clicint51::CLICINT51_SPEC>,
    #[doc = "0x10d0 - Interrupt control and state"]
    pub clicint52: crate::Reg<clicint52::CLICINT52_SPEC>,
    #[doc = "0x10d4 - Interrupt control and state"]
    pub clicint53: crate::Reg<clicint53::CLICINT53_SPEC>,
    #[doc = "0x10d8 - Interrupt control and state"]
    pub clicint54: crate::Reg<clicint54::CLICINT54_SPEC>,
    #[doc = "0x10dc - Interrupt control and state"]
    pub clicint55: crate::Reg<clicint55::CLICINT55_SPEC>,
    #[doc = "0x10e0 - Interrupt control and state"]
    pub clicint56: crate::Reg<clicint56::CLICINT56_SPEC>,
    #[doc = "0x10e4 - Interrupt control and state"]
    pub clicint57: crate::Reg<clicint57::CLICINT57_SPEC>,
    #[doc = "0x10e8 - Interrupt control and state"]
    pub clicint58: crate::Reg<clicint58::CLICINT58_SPEC>,
    #[doc = "0x10ec - Interrupt control and state"]
    pub clicint59: crate::Reg<clicint59::CLICINT59_SPEC>,
    #[doc = "0x10f0 - Interrupt control and state"]
    pub clicint60: crate::Reg<clicint60::CLICINT60_SPEC>,
    #[doc = "0x10f4 - Interrupt control and state"]
    pub clicint61: crate::Reg<clicint61::CLICINT61_SPEC>,
    #[doc = "0x10f8 - Interrupt control and state"]
    pub clicint62: crate::Reg<clicint62::CLICINT62_SPEC>,
    #[doc = "0x10fc - Interrupt control and state"]
    pub clicint63: crate::Reg<clicint63::CLICINT63_SPEC>,
    #[doc = "0x1100 - Interrupt control and state"]
    pub clicint64: crate::Reg<clicint64::CLICINT64_SPEC>,
    #[doc = "0x1104 - Interrupt control and state"]
    pub clicint65: crate::Reg<clicint65::CLICINT65_SPEC>,
    #[doc = "0x1108 - Interrupt control and state"]
    pub clicint66: crate::Reg<clicint66::CLICINT66_SPEC>,
    #[doc = "0x110c - Interrupt control and state"]
    pub clicint67: crate::Reg<clicint67::CLICINT67_SPEC>,
    #[doc = "0x1110 - Interrupt control and state"]
    pub clicint68: crate::Reg<clicint68::CLICINT68_SPEC>,
    #[doc = "0x1114 - Interrupt control and state"]
    pub clicint69: crate::Reg<clicint69::CLICINT69_SPEC>,
    #[doc = "0x1118 - Interrupt control and state"]
    pub clicint70: crate::Reg<clicint70::CLICINT70_SPEC>,
    #[doc = "0x111c - Interrupt control and state"]
    pub clicint71: crate::Reg<clicint71::CLICINT71_SPEC>,
    #[doc = "0x1120 - Interrupt control and state"]
    pub clicint72: crate::Reg<clicint72::CLICINT72_SPEC>,
    #[doc = "0x1124 - Interrupt control and state"]
    pub clicint73: crate::Reg<clicint73::CLICINT73_SPEC>,
    #[doc = "0x1128 - Interrupt control and state"]
    pub clicint74: crate::Reg<clicint74::CLICINT74_SPEC>,
    #[doc = "0x112c - Interrupt control and state"]
    pub clicint75: crate::Reg<clicint75::CLICINT75_SPEC>,
    #[doc = "0x1130 - Interrupt control and state"]
    pub clicint76: crate::Reg<clicint76::CLICINT76_SPEC>,
    #[doc = "0x1134 - Interrupt control and state"]
    pub clicint77: crate::Reg<clicint77::CLICINT77_SPEC>,
    #[doc = "0x1138 - Interrupt control and state"]
    pub clicint78: crate::Reg<clicint78::CLICINT78_SPEC>,
    #[doc = "0x113c - Interrupt control and state"]
    pub clicint79: crate::Reg<clicint79::CLICINT79_SPEC>,
    #[doc = "0x1140 - Interrupt control and state"]
    pub clicint80: crate::Reg<clicint80::CLICINT80_SPEC>,
    #[doc = "0x1144 - Interrupt control and state"]
    pub clicint81: crate::Reg<clicint81::CLICINT81_SPEC>,
    #[doc = "0x1148 - Interrupt control and state"]
    pub clicint82: crate::Reg<clicint82::CLICINT82_SPEC>,
    #[doc = "0x114c - Interrupt control and state"]
    pub clicint83: crate::Reg<clicint83::CLICINT83_SPEC>,
    #[doc = "0x1150 - Interrupt control and state"]
    pub clicint84: crate::Reg<clicint84::CLICINT84_SPEC>,
    #[doc = "0x1154 - Interrupt control and state"]
    pub clicint85: crate::Reg<clicint85::CLICINT85_SPEC>,
    #[doc = "0x1158 - Interrupt control and state"]
    pub clicint86: crate::Reg<clicint86::CLICINT86_SPEC>,
    #[doc = "0x115c - Interrupt control and state"]
    pub clicint87: crate::Reg<clicint87::CLICINT87_SPEC>,
    #[doc = "0x1160 - Interrupt control and state"]
    pub clicint88: crate::Reg<clicint88::CLICINT88_SPEC>,
    #[doc = "0x1164 - Interrupt control and state"]
    pub clicint89: crate::Reg<clicint89::CLICINT89_SPEC>,
    #[doc = "0x1168 - Interrupt control and state"]
    pub clicint90: crate::Reg<clicint90::CLICINT90_SPEC>,
    #[doc = "0x116c - Interrupt control and state"]
    pub clicint91: crate::Reg<clicint91::CLICINT91_SPEC>,
    #[doc = "0x1170 - Interrupt control and state"]
    pub clicint92: crate::Reg<clicint92::CLICINT92_SPEC>,
    #[doc = "0x1174 - Interrupt control and state"]
    pub clicint93: crate::Reg<clicint93::CLICINT93_SPEC>,
    #[doc = "0x1178 - Interrupt control and state"]
    pub clicint94: crate::Reg<clicint94::CLICINT94_SPEC>,
    #[doc = "0x117c - Interrupt control and state"]
    pub clicint95: crate::Reg<clicint95::CLICINT95_SPEC>,
    #[doc = "0x1180 - Interrupt control and state"]
    pub clicint96: crate::Reg<clicint96::CLICINT96_SPEC>,
    #[doc = "0x1184 - Interrupt control and state"]
    pub clicint97: crate::Reg<clicint97::CLICINT97_SPEC>,
    #[doc = "0x1188 - Interrupt control and state"]
    pub clicint98: crate::Reg<clicint98::CLICINT98_SPEC>,
    #[doc = "0x118c - Interrupt control and state"]
    pub clicint99: crate::Reg<clicint99::CLICINT99_SPEC>,
    #[doc = "0x1190 - Interrupt control and state"]
    pub clicint100: crate::Reg<clicint100::CLICINT100_SPEC>,
    #[doc = "0x1194 - Interrupt control and state"]
    pub clicint101: crate::Reg<clicint101::CLICINT101_SPEC>,
    #[doc = "0x1198 - Interrupt control and state"]
    pub clicint102: crate::Reg<clicint102::CLICINT102_SPEC>,
    #[doc = "0x119c - Interrupt control and state"]
    pub clicint103: crate::Reg<clicint103::CLICINT103_SPEC>,
    #[doc = "0x11a0 - Interrupt control and state"]
    pub clicint104: crate::Reg<clicint104::CLICINT104_SPEC>,
}
#[doc = "cliccfg register accessor: an alias for `Reg<CLICCFG_SPEC>`"]
pub type CLICCFG = crate::Reg<cliccfg::CLICCFG_SPEC>;
#[doc = "Eclic global configuration register"]
pub mod cliccfg;
#[doc = "clicinfo register accessor: an alias for `Reg<CLICINFO_SPEC>`"]
pub type CLICINFO = crate::Reg<clicinfo::CLICINFO_SPEC>;
#[doc = "global info register"]
pub mod clicinfo;
#[doc = "mth register accessor: an alias for `Reg<MTH_SPEC>`"]
pub type MTH = crate::Reg<mth::MTH_SPEC>;
#[doc = "set the target interrupt threshold level"]
pub mod mth;
#[doc = "clicint1 register accessor: an alias for `Reg<CLICINT1_SPEC>`"]
pub type CLICINT1 = crate::Reg<clicint1::CLICINT1_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint1;
#[doc = "clicint2 register accessor: an alias for `Reg<CLICINT2_SPEC>`"]
pub type CLICINT2 = crate::Reg<clicint2::CLICINT2_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint2;
#[doc = "clicint3 register accessor: an alias for `Reg<CLICINT3_SPEC>`"]
pub type CLICINT3 = crate::Reg<clicint3::CLICINT3_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint3;
#[doc = "clicint4 register accessor: an alias for `Reg<CLICINT4_SPEC>`"]
pub type CLICINT4 = crate::Reg<clicint4::CLICINT4_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint4;
#[doc = "clicint5 register accessor: an alias for `Reg<CLICINT5_SPEC>`"]
pub type CLICINT5 = crate::Reg<clicint5::CLICINT5_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint5;
#[doc = "clicint6 register accessor: an alias for `Reg<CLICINT6_SPEC>`"]
pub type CLICINT6 = crate::Reg<clicint6::CLICINT6_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint6;
#[doc = "clicint7 register accessor: an alias for `Reg<CLICINT7_SPEC>`"]
pub type CLICINT7 = crate::Reg<clicint7::CLICINT7_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint7;
#[doc = "clicint8 register accessor: an alias for `Reg<CLICINT8_SPEC>`"]
pub type CLICINT8 = crate::Reg<clicint8::CLICINT8_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint8;
#[doc = "clicint9 register accessor: an alias for `Reg<CLICINT9_SPEC>`"]
pub type CLICINT9 = crate::Reg<clicint9::CLICINT9_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint9;
#[doc = "clicint10 register accessor: an alias for `Reg<CLICINT10_SPEC>`"]
pub type CLICINT10 = crate::Reg<clicint10::CLICINT10_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint10;
#[doc = "clicint11 register accessor: an alias for `Reg<CLICINT11_SPEC>`"]
pub type CLICINT11 = crate::Reg<clicint11::CLICINT11_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint11;
#[doc = "clicint12 register accessor: an alias for `Reg<CLICINT12_SPEC>`"]
pub type CLICINT12 = crate::Reg<clicint12::CLICINT12_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint12;
#[doc = "clicint13 register accessor: an alias for `Reg<CLICINT13_SPEC>`"]
pub type CLICINT13 = crate::Reg<clicint13::CLICINT13_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint13;
#[doc = "clicint14 register accessor: an alias for `Reg<CLICINT14_SPEC>`"]
pub type CLICINT14 = crate::Reg<clicint14::CLICINT14_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint14;
#[doc = "clicint15 register accessor: an alias for `Reg<CLICINT15_SPEC>`"]
pub type CLICINT15 = crate::Reg<clicint15::CLICINT15_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint15;
#[doc = "clicint16 register accessor: an alias for `Reg<CLICINT16_SPEC>`"]
pub type CLICINT16 = crate::Reg<clicint16::CLICINT16_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint16;
#[doc = "clicint17 register accessor: an alias for `Reg<CLICINT17_SPEC>`"]
pub type CLICINT17 = crate::Reg<clicint17::CLICINT17_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint17;
#[doc = "clicint18 register accessor: an alias for `Reg<CLICINT18_SPEC>`"]
pub type CLICINT18 = crate::Reg<clicint18::CLICINT18_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint18;
#[doc = "clicint19 register accessor: an alias for `Reg<CLICINT19_SPEC>`"]
pub type CLICINT19 = crate::Reg<clicint19::CLICINT19_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint19;
#[doc = "clicint20 register accessor: an alias for `Reg<CLICINT20_SPEC>`"]
pub type CLICINT20 = crate::Reg<clicint20::CLICINT20_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint20;
#[doc = "clicint21 register accessor: an alias for `Reg<CLICINT21_SPEC>`"]
pub type CLICINT21 = crate::Reg<clicint21::CLICINT21_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint21;
#[doc = "clicint22 register accessor: an alias for `Reg<CLICINT22_SPEC>`"]
pub type CLICINT22 = crate::Reg<clicint22::CLICINT22_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint22;
#[doc = "clicint23 register accessor: an alias for `Reg<CLICINT23_SPEC>`"]
pub type CLICINT23 = crate::Reg<clicint23::CLICINT23_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint23;
#[doc = "clicint24 register accessor: an alias for `Reg<CLICINT24_SPEC>`"]
pub type CLICINT24 = crate::Reg<clicint24::CLICINT24_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint24;
#[doc = "clicint25 register accessor: an alias for `Reg<CLICINT25_SPEC>`"]
pub type CLICINT25 = crate::Reg<clicint25::CLICINT25_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint25;
#[doc = "clicint26 register accessor: an alias for `Reg<CLICINT26_SPEC>`"]
pub type CLICINT26 = crate::Reg<clicint26::CLICINT26_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint26;
#[doc = "clicint27 register accessor: an alias for `Reg<CLICINT27_SPEC>`"]
pub type CLICINT27 = crate::Reg<clicint27::CLICINT27_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint27;
#[doc = "clicint28 register accessor: an alias for `Reg<CLICINT28_SPEC>`"]
pub type CLICINT28 = crate::Reg<clicint28::CLICINT28_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint28;
#[doc = "clicint29 register accessor: an alias for `Reg<CLICINT29_SPEC>`"]
pub type CLICINT29 = crate::Reg<clicint29::CLICINT29_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint29;
#[doc = "clicint30 register accessor: an alias for `Reg<CLICINT30_SPEC>`"]
pub type CLICINT30 = crate::Reg<clicint30::CLICINT30_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint30;
#[doc = "clicint31 register accessor: an alias for `Reg<CLICINT31_SPEC>`"]
pub type CLICINT31 = crate::Reg<clicint31::CLICINT31_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint31;
#[doc = "clicint32 register accessor: an alias for `Reg<CLICINT32_SPEC>`"]
pub type CLICINT32 = crate::Reg<clicint32::CLICINT32_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint32;
#[doc = "clicint33 register accessor: an alias for `Reg<CLICINT33_SPEC>`"]
pub type CLICINT33 = crate::Reg<clicint33::CLICINT33_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint33;
#[doc = "clicint34 register accessor: an alias for `Reg<CLICINT34_SPEC>`"]
pub type CLICINT34 = crate::Reg<clicint34::CLICINT34_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint34;
#[doc = "clicint35 register accessor: an alias for `Reg<CLICINT35_SPEC>`"]
pub type CLICINT35 = crate::Reg<clicint35::CLICINT35_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint35;
#[doc = "clicint36 register accessor: an alias for `Reg<CLICINT36_SPEC>`"]
pub type CLICINT36 = crate::Reg<clicint36::CLICINT36_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint36;
#[doc = "clicint37 register accessor: an alias for `Reg<CLICINT37_SPEC>`"]
pub type CLICINT37 = crate::Reg<clicint37::CLICINT37_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint37;
#[doc = "clicint38 register accessor: an alias for `Reg<CLICINT38_SPEC>`"]
pub type CLICINT38 = crate::Reg<clicint38::CLICINT38_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint38;
#[doc = "clicint39 register accessor: an alias for `Reg<CLICINT39_SPEC>`"]
pub type CLICINT39 = crate::Reg<clicint39::CLICINT39_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint39;
#[doc = "clicint40 register accessor: an alias for `Reg<CLICINT40_SPEC>`"]
pub type CLICINT40 = crate::Reg<clicint40::CLICINT40_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint40;
#[doc = "clicint41 register accessor: an alias for `Reg<CLICINT41_SPEC>`"]
pub type CLICINT41 = crate::Reg<clicint41::CLICINT41_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint41;
#[doc = "clicint42 register accessor: an alias for `Reg<CLICINT42_SPEC>`"]
pub type CLICINT42 = crate::Reg<clicint42::CLICINT42_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint42;
#[doc = "clicint43 register accessor: an alias for `Reg<CLICINT43_SPEC>`"]
pub type CLICINT43 = crate::Reg<clicint43::CLICINT43_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint43;
#[doc = "clicint44 register accessor: an alias for `Reg<CLICINT44_SPEC>`"]
pub type CLICINT44 = crate::Reg<clicint44::CLICINT44_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint44;
#[doc = "clicint45 register accessor: an alias for `Reg<CLICINT45_SPEC>`"]
pub type CLICINT45 = crate::Reg<clicint45::CLICINT45_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint45;
#[doc = "clicint46 register accessor: an alias for `Reg<CLICINT46_SPEC>`"]
pub type CLICINT46 = crate::Reg<clicint46::CLICINT46_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint46;
#[doc = "clicint47 register accessor: an alias for `Reg<CLICINT47_SPEC>`"]
pub type CLICINT47 = crate::Reg<clicint47::CLICINT47_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint47;
#[doc = "clicint48 register accessor: an alias for `Reg<CLICINT48_SPEC>`"]
pub type CLICINT48 = crate::Reg<clicint48::CLICINT48_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint48;
#[doc = "clicint49 register accessor: an alias for `Reg<CLICINT49_SPEC>`"]
pub type CLICINT49 = crate::Reg<clicint49::CLICINT49_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint49;
#[doc = "clicint50 register accessor: an alias for `Reg<CLICINT50_SPEC>`"]
pub type CLICINT50 = crate::Reg<clicint50::CLICINT50_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint50;
#[doc = "clicint51 register accessor: an alias for `Reg<CLICINT51_SPEC>`"]
pub type CLICINT51 = crate::Reg<clicint51::CLICINT51_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint51;
#[doc = "clicint52 register accessor: an alias for `Reg<CLICINT52_SPEC>`"]
pub type CLICINT52 = crate::Reg<clicint52::CLICINT52_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint52;
#[doc = "clicint53 register accessor: an alias for `Reg<CLICINT53_SPEC>`"]
pub type CLICINT53 = crate::Reg<clicint53::CLICINT53_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint53;
#[doc = "clicint54 register accessor: an alias for `Reg<CLICINT54_SPEC>`"]
pub type CLICINT54 = crate::Reg<clicint54::CLICINT54_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint54;
#[doc = "clicint55 register accessor: an alias for `Reg<CLICINT55_SPEC>`"]
pub type CLICINT55 = crate::Reg<clicint55::CLICINT55_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint55;
#[doc = "clicint56 register accessor: an alias for `Reg<CLICINT56_SPEC>`"]
pub type CLICINT56 = crate::Reg<clicint56::CLICINT56_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint56;
#[doc = "clicint57 register accessor: an alias for `Reg<CLICINT57_SPEC>`"]
pub type CLICINT57 = crate::Reg<clicint57::CLICINT57_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint57;
#[doc = "clicint58 register accessor: an alias for `Reg<CLICINT58_SPEC>`"]
pub type CLICINT58 = crate::Reg<clicint58::CLICINT58_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint58;
#[doc = "clicint59 register accessor: an alias for `Reg<CLICINT59_SPEC>`"]
pub type CLICINT59 = crate::Reg<clicint59::CLICINT59_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint59;
#[doc = "clicint60 register accessor: an alias for `Reg<CLICINT60_SPEC>`"]
pub type CLICINT60 = crate::Reg<clicint60::CLICINT60_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint60;
#[doc = "clicint61 register accessor: an alias for `Reg<CLICINT61_SPEC>`"]
pub type CLICINT61 = crate::Reg<clicint61::CLICINT61_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint61;
#[doc = "clicint62 register accessor: an alias for `Reg<CLICINT62_SPEC>`"]
pub type CLICINT62 = crate::Reg<clicint62::CLICINT62_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint62;
#[doc = "clicint63 register accessor: an alias for `Reg<CLICINT63_SPEC>`"]
pub type CLICINT63 = crate::Reg<clicint63::CLICINT63_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint63;
#[doc = "clicint64 register accessor: an alias for `Reg<CLICINT64_SPEC>`"]
pub type CLICINT64 = crate::Reg<clicint64::CLICINT64_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint64;
#[doc = "clicint65 register accessor: an alias for `Reg<CLICINT65_SPEC>`"]
pub type CLICINT65 = crate::Reg<clicint65::CLICINT65_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint65;
#[doc = "clicint66 register accessor: an alias for `Reg<CLICINT66_SPEC>`"]
pub type CLICINT66 = crate::Reg<clicint66::CLICINT66_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint66;
#[doc = "clicint67 register accessor: an alias for `Reg<CLICINT67_SPEC>`"]
pub type CLICINT67 = crate::Reg<clicint67::CLICINT67_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint67;
#[doc = "clicint68 register accessor: an alias for `Reg<CLICINT68_SPEC>`"]
pub type CLICINT68 = crate::Reg<clicint68::CLICINT68_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint68;
#[doc = "clicint69 register accessor: an alias for `Reg<CLICINT69_SPEC>`"]
pub type CLICINT69 = crate::Reg<clicint69::CLICINT69_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint69;
#[doc = "clicint70 register accessor: an alias for `Reg<CLICINT70_SPEC>`"]
pub type CLICINT70 = crate::Reg<clicint70::CLICINT70_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint70;
#[doc = "clicint71 register accessor: an alias for `Reg<CLICINT71_SPEC>`"]
pub type CLICINT71 = crate::Reg<clicint71::CLICINT71_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint71;
#[doc = "clicint72 register accessor: an alias for `Reg<CLICINT72_SPEC>`"]
pub type CLICINT72 = crate::Reg<clicint72::CLICINT72_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint72;
#[doc = "clicint73 register accessor: an alias for `Reg<CLICINT73_SPEC>`"]
pub type CLICINT73 = crate::Reg<clicint73::CLICINT73_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint73;
#[doc = "clicint74 register accessor: an alias for `Reg<CLICINT74_SPEC>`"]
pub type CLICINT74 = crate::Reg<clicint74::CLICINT74_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint74;
#[doc = "clicint75 register accessor: an alias for `Reg<CLICINT75_SPEC>`"]
pub type CLICINT75 = crate::Reg<clicint75::CLICINT75_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint75;
#[doc = "clicint76 register accessor: an alias for `Reg<CLICINT76_SPEC>`"]
pub type CLICINT76 = crate::Reg<clicint76::CLICINT76_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint76;
#[doc = "clicint77 register accessor: an alias for `Reg<CLICINT77_SPEC>`"]
pub type CLICINT77 = crate::Reg<clicint77::CLICINT77_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint77;
#[doc = "clicint78 register accessor: an alias for `Reg<CLICINT78_SPEC>`"]
pub type CLICINT78 = crate::Reg<clicint78::CLICINT78_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint78;
#[doc = "clicint79 register accessor: an alias for `Reg<CLICINT79_SPEC>`"]
pub type CLICINT79 = crate::Reg<clicint79::CLICINT79_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint79;
#[doc = "clicint80 register accessor: an alias for `Reg<CLICINT80_SPEC>`"]
pub type CLICINT80 = crate::Reg<clicint80::CLICINT80_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint80;
#[doc = "clicint81 register accessor: an alias for `Reg<CLICINT81_SPEC>`"]
pub type CLICINT81 = crate::Reg<clicint81::CLICINT81_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint81;
#[doc = "clicint82 register accessor: an alias for `Reg<CLICINT82_SPEC>`"]
pub type CLICINT82 = crate::Reg<clicint82::CLICINT82_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint82;
#[doc = "clicint83 register accessor: an alias for `Reg<CLICINT83_SPEC>`"]
pub type CLICINT83 = crate::Reg<clicint83::CLICINT83_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint83;
#[doc = "clicint84 register accessor: an alias for `Reg<CLICINT84_SPEC>`"]
pub type CLICINT84 = crate::Reg<clicint84::CLICINT84_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint84;
#[doc = "clicint85 register accessor: an alias for `Reg<CLICINT85_SPEC>`"]
pub type CLICINT85 = crate::Reg<clicint85::CLICINT85_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint85;
#[doc = "clicint86 register accessor: an alias for `Reg<CLICINT86_SPEC>`"]
pub type CLICINT86 = crate::Reg<clicint86::CLICINT86_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint86;
#[doc = "clicint87 register accessor: an alias for `Reg<CLICINT87_SPEC>`"]
pub type CLICINT87 = crate::Reg<clicint87::CLICINT87_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint87;
#[doc = "clicint88 register accessor: an alias for `Reg<CLICINT88_SPEC>`"]
pub type CLICINT88 = crate::Reg<clicint88::CLICINT88_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint88;
#[doc = "clicint89 register accessor: an alias for `Reg<CLICINT89_SPEC>`"]
pub type CLICINT89 = crate::Reg<clicint89::CLICINT89_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint89;
#[doc = "clicint90 register accessor: an alias for `Reg<CLICINT90_SPEC>`"]
pub type CLICINT90 = crate::Reg<clicint90::CLICINT90_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint90;
#[doc = "clicint91 register accessor: an alias for `Reg<CLICINT91_SPEC>`"]
pub type CLICINT91 = crate::Reg<clicint91::CLICINT91_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint91;
#[doc = "clicint92 register accessor: an alias for `Reg<CLICINT92_SPEC>`"]
pub type CLICINT92 = crate::Reg<clicint92::CLICINT92_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint92;
#[doc = "clicint93 register accessor: an alias for `Reg<CLICINT93_SPEC>`"]
pub type CLICINT93 = crate::Reg<clicint93::CLICINT93_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint93;
#[doc = "clicint94 register accessor: an alias for `Reg<CLICINT94_SPEC>`"]
pub type CLICINT94 = crate::Reg<clicint94::CLICINT94_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint94;
#[doc = "clicint95 register accessor: an alias for `Reg<CLICINT95_SPEC>`"]
pub type CLICINT95 = crate::Reg<clicint95::CLICINT95_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint95;
#[doc = "clicint96 register accessor: an alias for `Reg<CLICINT96_SPEC>`"]
pub type CLICINT96 = crate::Reg<clicint96::CLICINT96_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint96;
#[doc = "clicint97 register accessor: an alias for `Reg<CLICINT97_SPEC>`"]
pub type CLICINT97 = crate::Reg<clicint97::CLICINT97_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint97;
#[doc = "clicint98 register accessor: an alias for `Reg<CLICINT98_SPEC>`"]
pub type CLICINT98 = crate::Reg<clicint98::CLICINT98_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint98;
#[doc = "clicint99 register accessor: an alias for `Reg<CLICINT99_SPEC>`"]
pub type CLICINT99 = crate::Reg<clicint99::CLICINT99_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint99;
#[doc = "clicint100 register accessor: an alias for `Reg<CLICINT100_SPEC>`"]
pub type CLICINT100 = crate::Reg<clicint100::CLICINT100_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint100;
#[doc = "clicint101 register accessor: an alias for `Reg<CLICINT101_SPEC>`"]
pub type CLICINT101 = crate::Reg<clicint101::CLICINT101_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint101;
#[doc = "clicint102 register accessor: an alias for `Reg<CLICINT102_SPEC>`"]
pub type CLICINT102 = crate::Reg<clicint102::CLICINT102_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint102;
#[doc = "clicint103 register accessor: an alias for `Reg<CLICINT103_SPEC>`"]
pub type CLICINT103 = crate::Reg<clicint103::CLICINT103_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint103;
#[doc = "clicint104 register accessor: an alias for `Reg<CLICINT104_SPEC>`"]
pub type CLICINT104 = crate::Reg<clicint104::CLICINT104_SPEC>;
#[doc = "Interrupt control and state"]
pub mod clicint104;
