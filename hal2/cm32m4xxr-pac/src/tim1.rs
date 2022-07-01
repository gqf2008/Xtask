#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIMx_CTRL1"]
    pub timx_ctrl1: crate::Reg<timx_ctrl1::TIMX_CTRL1_SPEC>,
    #[doc = "0x04 - TIMx_CTRL2"]
    pub timx_ctrl2: crate::Reg<timx_ctrl2::TIMX_CTRL2_SPEC>,
    #[doc = "0x08 - TIMx_SMCTRL"]
    pub timx_smctrl: crate::Reg<timx_smctrl::TIMX_SMCTRL_SPEC>,
    #[doc = "0x0c - TIMx_DINTEN"]
    pub timx_dinten: crate::Reg<timx_dinten::TIMX_DINTEN_SPEC>,
    #[doc = "0x10 - TIMx_STS"]
    pub timx_sts: crate::Reg<timx_sts::TIMX_STS_SPEC>,
    #[doc = "0x14 - TIMx_EVTGEN"]
    pub timx_evtgen: crate::Reg<timx_evtgen::TIMX_EVTGEN_SPEC>,
    #[doc = "0x18 - TIMx_CCMOD1"]
    pub timx_ccmod1: crate::Reg<timx_ccmod1::TIMX_CCMOD1_SPEC>,
    #[doc = "0x1c - TIMx_CCMOD2"]
    pub timx_ccmod2: crate::Reg<timx_ccmod2::TIMX_CCMOD2_SPEC>,
    #[doc = "0x20 - TIMx_CCEN"]
    pub timx_ccen: crate::Reg<timx_ccen::TIMX_CCEN_SPEC>,
    #[doc = "0x24 - TIMx_CNT"]
    pub timx_cnt: crate::Reg<timx_cnt::TIMX_CNT_SPEC>,
    #[doc = "0x28 - TIMx_PSC"]
    pub timx_psc: crate::Reg<timx_psc::TIMX_PSC_SPEC>,
    #[doc = "0x2c - TIMx_AR"]
    pub timx_ar: crate::Reg<timx_ar::TIMX_AR_SPEC>,
    #[doc = "0x30 - TIMx_REPCNT"]
    pub timx_repcnt: crate::Reg<timx_repcnt::TIMX_REPCNT_SPEC>,
    #[doc = "0x34 - TIMx_CCDAT1"]
    pub timx_ccdat1: crate::Reg<timx_ccdat1::TIMX_CCDAT1_SPEC>,
    #[doc = "0x38 - TIMx_CCDAT2"]
    pub timx_ccdat2: crate::Reg<timx_ccdat2::TIMX_CCDAT2_SPEC>,
    #[doc = "0x3c - TIMx_CCDAT3"]
    pub timx_ccdat3: crate::Reg<timx_ccdat3::TIMX_CCDAT3_SPEC>,
    #[doc = "0x40 - TIMx_CCDAT4"]
    pub timx_ccdat4: crate::Reg<timx_ccdat4::TIMX_CCDAT4_SPEC>,
    #[doc = "0x44 - TIMx_BKDT"]
    pub timx_bkdt: crate::Reg<timx_bkdt::TIMX_BKDT_SPEC>,
    #[doc = "0x48 - TIMx_DCTRL"]
    pub timx_dctrl: crate::Reg<timx_dctrl::TIMX_DCTRL_SPEC>,
    #[doc = "0x4c - TIMx_DADDR"]
    pub timx_daddr: crate::Reg<timx_daddr::TIMX_DADDR_SPEC>,
    #[doc = "0x50 - TIMx_CCMOD3"]
    pub timx_ccmod3: crate::Reg<timx_ccmod3::TIMX_CCMOD3_SPEC>,
    _reserved21: [u8; 0x04],
    #[doc = "0x58 - TIMx_CCDAT6"]
    pub timx_ccdat6: crate::Reg<timx_ccdat6::TIMX_CCDAT6_SPEC>,
}
#[doc = "TIMx_CTRL1 register accessor: an alias for `Reg<TIMX_CTRL1_SPEC>`"]
pub type TIMX_CTRL1 = crate::Reg<timx_ctrl1::TIMX_CTRL1_SPEC>;
#[doc = "TIMx_CTRL1"]
pub mod timx_ctrl1;
#[doc = "TIMx_CTRL2 register accessor: an alias for `Reg<TIMX_CTRL2_SPEC>`"]
pub type TIMX_CTRL2 = crate::Reg<timx_ctrl2::TIMX_CTRL2_SPEC>;
#[doc = "TIMx_CTRL2"]
pub mod timx_ctrl2;
#[doc = "TIMx_SMCTRL register accessor: an alias for `Reg<TIMX_SMCTRL_SPEC>`"]
pub type TIMX_SMCTRL = crate::Reg<timx_smctrl::TIMX_SMCTRL_SPEC>;
#[doc = "TIMx_SMCTRL"]
pub mod timx_smctrl;
#[doc = "TIMx_DINTEN register accessor: an alias for `Reg<TIMX_DINTEN_SPEC>`"]
pub type TIMX_DINTEN = crate::Reg<timx_dinten::TIMX_DINTEN_SPEC>;
#[doc = "TIMx_DINTEN"]
pub mod timx_dinten;
#[doc = "TIMx_STS register accessor: an alias for `Reg<TIMX_STS_SPEC>`"]
pub type TIMX_STS = crate::Reg<timx_sts::TIMX_STS_SPEC>;
#[doc = "TIMx_STS"]
pub mod timx_sts;
#[doc = "TIMx_EVTGEN register accessor: an alias for `Reg<TIMX_EVTGEN_SPEC>`"]
pub type TIMX_EVTGEN = crate::Reg<timx_evtgen::TIMX_EVTGEN_SPEC>;
#[doc = "TIMx_EVTGEN"]
pub mod timx_evtgen;
#[doc = "TIMx_CCMOD1 register accessor: an alias for `Reg<TIMX_CCMOD1_SPEC>`"]
pub type TIMX_CCMOD1 = crate::Reg<timx_ccmod1::TIMX_CCMOD1_SPEC>;
#[doc = "TIMx_CCMOD1"]
pub mod timx_ccmod1;
#[doc = "TIMx_CCMOD2 register accessor: an alias for `Reg<TIMX_CCMOD2_SPEC>`"]
pub type TIMX_CCMOD2 = crate::Reg<timx_ccmod2::TIMX_CCMOD2_SPEC>;
#[doc = "TIMx_CCMOD2"]
pub mod timx_ccmod2;
#[doc = "TIMx_CCEN register accessor: an alias for `Reg<TIMX_CCEN_SPEC>`"]
pub type TIMX_CCEN = crate::Reg<timx_ccen::TIMX_CCEN_SPEC>;
#[doc = "TIMx_CCEN"]
pub mod timx_ccen;
#[doc = "TIMx_CNT register accessor: an alias for `Reg<TIMX_CNT_SPEC>`"]
pub type TIMX_CNT = crate::Reg<timx_cnt::TIMX_CNT_SPEC>;
#[doc = "TIMx_CNT"]
pub mod timx_cnt;
#[doc = "TIMx_PSC register accessor: an alias for `Reg<TIMX_PSC_SPEC>`"]
pub type TIMX_PSC = crate::Reg<timx_psc::TIMX_PSC_SPEC>;
#[doc = "TIMx_PSC"]
pub mod timx_psc;
#[doc = "TIMx_AR register accessor: an alias for `Reg<TIMX_AR_SPEC>`"]
pub type TIMX_AR = crate::Reg<timx_ar::TIMX_AR_SPEC>;
#[doc = "TIMx_AR"]
pub mod timx_ar;
#[doc = "TIMx_REPCNT register accessor: an alias for `Reg<TIMX_REPCNT_SPEC>`"]
pub type TIMX_REPCNT = crate::Reg<timx_repcnt::TIMX_REPCNT_SPEC>;
#[doc = "TIMx_REPCNT"]
pub mod timx_repcnt;
#[doc = "TIMx_CCDAT1 register accessor: an alias for `Reg<TIMX_CCDAT1_SPEC>`"]
pub type TIMX_CCDAT1 = crate::Reg<timx_ccdat1::TIMX_CCDAT1_SPEC>;
#[doc = "TIMx_CCDAT1"]
pub mod timx_ccdat1;
#[doc = "TIMx_CCDAT2 register accessor: an alias for `Reg<TIMX_CCDAT2_SPEC>`"]
pub type TIMX_CCDAT2 = crate::Reg<timx_ccdat2::TIMX_CCDAT2_SPEC>;
#[doc = "TIMx_CCDAT2"]
pub mod timx_ccdat2;
#[doc = "TIMx_CCDAT3 register accessor: an alias for `Reg<TIMX_CCDAT3_SPEC>`"]
pub type TIMX_CCDAT3 = crate::Reg<timx_ccdat3::TIMX_CCDAT3_SPEC>;
#[doc = "TIMx_CCDAT3"]
pub mod timx_ccdat3;
#[doc = "TIMx_CCDAT4 register accessor: an alias for `Reg<TIMX_CCDAT4_SPEC>`"]
pub type TIMX_CCDAT4 = crate::Reg<timx_ccdat4::TIMX_CCDAT4_SPEC>;
#[doc = "TIMx_CCDAT4"]
pub mod timx_ccdat4;
#[doc = "TIMx_BKDT register accessor: an alias for `Reg<TIMX_BKDT_SPEC>`"]
pub type TIMX_BKDT = crate::Reg<timx_bkdt::TIMX_BKDT_SPEC>;
#[doc = "TIMx_BKDT"]
pub mod timx_bkdt;
#[doc = "TIMx_DCTRL register accessor: an alias for `Reg<TIMX_DCTRL_SPEC>`"]
pub type TIMX_DCTRL = crate::Reg<timx_dctrl::TIMX_DCTRL_SPEC>;
#[doc = "TIMx_DCTRL"]
pub mod timx_dctrl;
#[doc = "TIMx_DADDR register accessor: an alias for `Reg<TIMX_DADDR_SPEC>`"]
pub type TIMX_DADDR = crate::Reg<timx_daddr::TIMX_DADDR_SPEC>;
#[doc = "TIMx_DADDR"]
pub mod timx_daddr;
#[doc = "TIMx_CCMOD3 register accessor: an alias for `Reg<TIMX_CCMOD3_SPEC>`"]
pub type TIMX_CCMOD3 = crate::Reg<timx_ccmod3::TIMX_CCMOD3_SPEC>;
#[doc = "TIMx_CCMOD3"]
pub mod timx_ccmod3;
#[doc = "TIMx_CCDAT6 register accessor: an alias for `Reg<TIMX_CCDAT6_SPEC>`"]
pub type TIMX_CCDAT6 = crate::Reg<timx_ccdat6::TIMX_CCDAT6_SPEC>;
#[doc = "TIMx_CCDAT6"]
pub mod timx_ccdat6;
