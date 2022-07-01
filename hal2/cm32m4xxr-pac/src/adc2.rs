#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC_STS"]
    pub adc_sts: crate::Reg<adc_sts::ADC_STS_SPEC>,
    #[doc = "0x04 - ADC_CTRL1"]
    pub adc_ctrl1: crate::Reg<adc_ctrl1::ADC_CTRL1_SPEC>,
    #[doc = "0x08 - ADC_CTRL2"]
    pub adc_ctrl2: crate::Reg<adc_ctrl2::ADC_CTRL2_SPEC>,
    #[doc = "0x0c - ADC_SMPR1"]
    pub adc_smpr1: crate::Reg<adc_smpr1::ADC_SMPR1_SPEC>,
    #[doc = "0x10 - ADC_SMPR2"]
    pub adc_smpr2: crate::Reg<adc_smpr2::ADC_SMPR2_SPEC>,
    #[doc = "0x14 - ADC_JOFFSET1"]
    pub adc_joffset1: crate::Reg<adc_joffset1::ADC_JOFFSET1_SPEC>,
    #[doc = "0x18 - ADC_JOFFSET2"]
    pub adc_joffset2: crate::Reg<adc_joffset2::ADC_JOFFSET2_SPEC>,
    #[doc = "0x1c - ADC_JOFFSET3"]
    pub adc_joffset3: crate::Reg<adc_joffset3::ADC_JOFFSET3_SPEC>,
    #[doc = "0x20 - ADC_JOFFSET4"]
    pub adc_joffset4: crate::Reg<adc_joffset4::ADC_JOFFSET4_SPEC>,
    #[doc = "0x24 - ADC_WDGHIGH"]
    pub adc_wdghigh: crate::Reg<adc_wdghigh::ADC_WDGHIGH_SPEC>,
    #[doc = "0x28 - ADC_WDGLOW"]
    pub adc_wdglow: crate::Reg<adc_wdglow::ADC_WDGLOW_SPEC>,
    #[doc = "0x2c - ADC_RSEQ1"]
    pub adc_rseq1: crate::Reg<adc_rseq1::ADC_RSEQ1_SPEC>,
    #[doc = "0x30 - ADC_RSEQ2"]
    pub adc_rseq2: crate::Reg<adc_rseq2::ADC_RSEQ2_SPEC>,
    #[doc = "0x34 - ADC_RSEQ3"]
    pub adc_rseq3: crate::Reg<adc_rseq3::ADC_RSEQ3_SPEC>,
    #[doc = "0x38 - ADC_JSEQ"]
    pub adc_jseq: crate::Reg<adc_jseq::ADC_JSEQ_SPEC>,
    #[doc = "0x3c - ADC_JDAT1"]
    pub adc_jdat1: crate::Reg<adc_jdat1::ADC_JDAT1_SPEC>,
    #[doc = "0x40 - ADC_JDAT2"]
    pub adc_jdat2: crate::Reg<adc_jdat2::ADC_JDAT2_SPEC>,
    #[doc = "0x44 - ADC_JDAT3"]
    pub adc_jdat3: crate::Reg<adc_jdat3::ADC_JDAT3_SPEC>,
    #[doc = "0x48 - ADC_JDAT4"]
    pub adc_jdat4: crate::Reg<adc_jdat4::ADC_JDAT4_SPEC>,
    #[doc = "0x4c - ADC_JDAT"]
    pub adc_jdat: crate::Reg<adc_jdat::ADC_JDAT_SPEC>,
    #[doc = "0x50 - ADC_DIFSEL"]
    pub adc_difsel: crate::Reg<adc_difsel::ADC_DIFSEL_SPEC>,
    #[doc = "0x54 - ADC_CALFACT"]
    pub adc_calfact: crate::Reg<adc_calfact::ADC_CALFACT_SPEC>,
    #[doc = "0x58 - ADC_CTRL3"]
    pub adc_ctrl3: crate::Reg<adc_ctrl3::ADC_CTRL3_SPEC>,
    #[doc = "0x5c - ADC_SAMPT3"]
    pub adc_sampt3: crate::Reg<adc_sampt3::ADC_SAMPT3_SPEC>,
    #[doc = "0x60 - ADC_IPTST"]
    pub adc_iptst: crate::Reg<adc_iptst::ADC_IPTST_SPEC>,
}
#[doc = "ADC_STS register accessor: an alias for `Reg<ADC_STS_SPEC>`"]
pub type ADC_STS = crate::Reg<adc_sts::ADC_STS_SPEC>;
#[doc = "ADC_STS"]
pub mod adc_sts;
#[doc = "ADC_CTRL1 register accessor: an alias for `Reg<ADC_CTRL1_SPEC>`"]
pub type ADC_CTRL1 = crate::Reg<adc_ctrl1::ADC_CTRL1_SPEC>;
#[doc = "ADC_CTRL1"]
pub mod adc_ctrl1;
#[doc = "ADC_CTRL2 register accessor: an alias for `Reg<ADC_CTRL2_SPEC>`"]
pub type ADC_CTRL2 = crate::Reg<adc_ctrl2::ADC_CTRL2_SPEC>;
#[doc = "ADC_CTRL2"]
pub mod adc_ctrl2;
#[doc = "ADC_SMPR1 register accessor: an alias for `Reg<ADC_SMPR1_SPEC>`"]
pub type ADC_SMPR1 = crate::Reg<adc_smpr1::ADC_SMPR1_SPEC>;
#[doc = "ADC_SMPR1"]
pub mod adc_smpr1;
#[doc = "ADC_SMPR2 register accessor: an alias for `Reg<ADC_SMPR2_SPEC>`"]
pub type ADC_SMPR2 = crate::Reg<adc_smpr2::ADC_SMPR2_SPEC>;
#[doc = "ADC_SMPR2"]
pub mod adc_smpr2;
#[doc = "ADC_JOFFSET1 register accessor: an alias for `Reg<ADC_JOFFSET1_SPEC>`"]
pub type ADC_JOFFSET1 = crate::Reg<adc_joffset1::ADC_JOFFSET1_SPEC>;
#[doc = "ADC_JOFFSET1"]
pub mod adc_joffset1;
#[doc = "ADC_JOFFSET2 register accessor: an alias for `Reg<ADC_JOFFSET2_SPEC>`"]
pub type ADC_JOFFSET2 = crate::Reg<adc_joffset2::ADC_JOFFSET2_SPEC>;
#[doc = "ADC_JOFFSET2"]
pub mod adc_joffset2;
#[doc = "ADC_JOFFSET3 register accessor: an alias for `Reg<ADC_JOFFSET3_SPEC>`"]
pub type ADC_JOFFSET3 = crate::Reg<adc_joffset3::ADC_JOFFSET3_SPEC>;
#[doc = "ADC_JOFFSET3"]
pub mod adc_joffset3;
#[doc = "ADC_JOFFSET4 register accessor: an alias for `Reg<ADC_JOFFSET4_SPEC>`"]
pub type ADC_JOFFSET4 = crate::Reg<adc_joffset4::ADC_JOFFSET4_SPEC>;
#[doc = "ADC_JOFFSET4"]
pub mod adc_joffset4;
#[doc = "ADC_WDGHIGH register accessor: an alias for `Reg<ADC_WDGHIGH_SPEC>`"]
pub type ADC_WDGHIGH = crate::Reg<adc_wdghigh::ADC_WDGHIGH_SPEC>;
#[doc = "ADC_WDGHIGH"]
pub mod adc_wdghigh;
#[doc = "ADC_WDGLOW register accessor: an alias for `Reg<ADC_WDGLOW_SPEC>`"]
pub type ADC_WDGLOW = crate::Reg<adc_wdglow::ADC_WDGLOW_SPEC>;
#[doc = "ADC_WDGLOW"]
pub mod adc_wdglow;
#[doc = "ADC_RSEQ1 register accessor: an alias for `Reg<ADC_RSEQ1_SPEC>`"]
pub type ADC_RSEQ1 = crate::Reg<adc_rseq1::ADC_RSEQ1_SPEC>;
#[doc = "ADC_RSEQ1"]
pub mod adc_rseq1;
#[doc = "ADC_RSEQ2 register accessor: an alias for `Reg<ADC_RSEQ2_SPEC>`"]
pub type ADC_RSEQ2 = crate::Reg<adc_rseq2::ADC_RSEQ2_SPEC>;
#[doc = "ADC_RSEQ2"]
pub mod adc_rseq2;
#[doc = "ADC_RSEQ3 register accessor: an alias for `Reg<ADC_RSEQ3_SPEC>`"]
pub type ADC_RSEQ3 = crate::Reg<adc_rseq3::ADC_RSEQ3_SPEC>;
#[doc = "ADC_RSEQ3"]
pub mod adc_rseq3;
#[doc = "ADC_JSEQ register accessor: an alias for `Reg<ADC_JSEQ_SPEC>`"]
pub type ADC_JSEQ = crate::Reg<adc_jseq::ADC_JSEQ_SPEC>;
#[doc = "ADC_JSEQ"]
pub mod adc_jseq;
#[doc = "ADC_JDAT1 register accessor: an alias for `Reg<ADC_JDAT1_SPEC>`"]
pub type ADC_JDAT1 = crate::Reg<adc_jdat1::ADC_JDAT1_SPEC>;
#[doc = "ADC_JDAT1"]
pub mod adc_jdat1;
#[doc = "ADC_JDAT2 register accessor: an alias for `Reg<ADC_JDAT2_SPEC>`"]
pub type ADC_JDAT2 = crate::Reg<adc_jdat2::ADC_JDAT2_SPEC>;
#[doc = "ADC_JDAT2"]
pub mod adc_jdat2;
#[doc = "ADC_JDAT3 register accessor: an alias for `Reg<ADC_JDAT3_SPEC>`"]
pub type ADC_JDAT3 = crate::Reg<adc_jdat3::ADC_JDAT3_SPEC>;
#[doc = "ADC_JDAT3"]
pub mod adc_jdat3;
#[doc = "ADC_JDAT4 register accessor: an alias for `Reg<ADC_JDAT4_SPEC>`"]
pub type ADC_JDAT4 = crate::Reg<adc_jdat4::ADC_JDAT4_SPEC>;
#[doc = "ADC_JDAT4"]
pub mod adc_jdat4;
#[doc = "ADC_JDAT register accessor: an alias for `Reg<ADC_JDAT_SPEC>`"]
pub type ADC_JDAT = crate::Reg<adc_jdat::ADC_JDAT_SPEC>;
#[doc = "ADC_JDAT"]
pub mod adc_jdat;
#[doc = "ADC_DIFSEL register accessor: an alias for `Reg<ADC_DIFSEL_SPEC>`"]
pub type ADC_DIFSEL = crate::Reg<adc_difsel::ADC_DIFSEL_SPEC>;
#[doc = "ADC_DIFSEL"]
pub mod adc_difsel;
#[doc = "ADC_CALFACT register accessor: an alias for `Reg<ADC_CALFACT_SPEC>`"]
pub type ADC_CALFACT = crate::Reg<adc_calfact::ADC_CALFACT_SPEC>;
#[doc = "ADC_CALFACT"]
pub mod adc_calfact;
#[doc = "ADC_CTRL3 register accessor: an alias for `Reg<ADC_CTRL3_SPEC>`"]
pub type ADC_CTRL3 = crate::Reg<adc_ctrl3::ADC_CTRL3_SPEC>;
#[doc = "ADC_CTRL3"]
pub mod adc_ctrl3;
#[doc = "ADC_SAMPT3 register accessor: an alias for `Reg<ADC_SAMPT3_SPEC>`"]
pub type ADC_SAMPT3 = crate::Reg<adc_sampt3::ADC_SAMPT3_SPEC>;
#[doc = "ADC_SAMPT3"]
pub mod adc_sampt3;
#[doc = "ADC_IPTST register accessor: an alias for `Reg<ADC_IPTST_SPEC>`"]
pub type ADC_IPTST = crate::Reg<adc_iptst::ADC_IPTST_SPEC>;
#[doc = "ADC_IPTST"]
pub mod adc_iptst;
