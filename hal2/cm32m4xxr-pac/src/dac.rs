#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DAC_CTRL"]
    pub dac_ctrl: crate::Reg<dac_ctrl::DAC_CTRL_SPEC>,
    #[doc = "0x04 - DAC_SOTTR"]
    pub dac_sottr: crate::Reg<dac_sottr::DAC_SOTTR_SPEC>,
    #[doc = "0x08 - DAC_DR12CH1"]
    pub dac_dr12ch1: crate::Reg<dac_dr12ch1::DAC_DR12CH1_SPEC>,
    #[doc = "0x0c - DAC_DL12CH1"]
    pub dac_dl12ch1: crate::Reg<dac_dl12ch1::DAC_DL12CH1_SPEC>,
    #[doc = "0x10 - DAC_DR8CH1"]
    pub dac_dr8ch1: crate::Reg<dac_dr8ch1::DAC_DR8CH1_SPEC>,
    #[doc = "0x14 - DAC_DR12CH2"]
    pub dac_dr12ch2: crate::Reg<dac_dr12ch2::DAC_DR12CH2_SPEC>,
    #[doc = "0x18 - DAC_DL12CH2"]
    pub dac_dl12ch2: crate::Reg<dac_dl12ch2::DAC_DL12CH2_SPEC>,
    #[doc = "0x1c - DAC_DR8CH2"]
    pub dac_dr8ch2: crate::Reg<dac_dr8ch2::DAC_DR8CH2_SPEC>,
    #[doc = "0x20 - DAC_DR12DCH"]
    pub dac_dr12dch: crate::Reg<dac_dr12dch::DAC_DR12DCH_SPEC>,
    #[doc = "0x24 - DAC_DL12DCH"]
    pub dac_dl12dch: crate::Reg<dac_dl12dch::DAC_DL12DCH_SPEC>,
    #[doc = "0x28 - DAC_DR8DCH"]
    pub dac_dr8dch: crate::Reg<dac_dr8dch::DAC_DR8DCH_SPEC>,
    #[doc = "0x2c - DAC_DATO1"]
    pub dac_dato1: crate::Reg<dac_dato1::DAC_DATO1_SPEC>,
    #[doc = "0x30 - DAC_DATO2"]
    pub dac_dato2: crate::Reg<dac_dato2::DAC_DATO2_SPEC>,
}
#[doc = "DAC_CTRL register accessor: an alias for `Reg<DAC_CTRL_SPEC>`"]
pub type DAC_CTRL = crate::Reg<dac_ctrl::DAC_CTRL_SPEC>;
#[doc = "DAC_CTRL"]
pub mod dac_ctrl;
#[doc = "DAC_SOTTR register accessor: an alias for `Reg<DAC_SOTTR_SPEC>`"]
pub type DAC_SOTTR = crate::Reg<dac_sottr::DAC_SOTTR_SPEC>;
#[doc = "DAC_SOTTR"]
pub mod dac_sottr;
#[doc = "DAC_DR12CH1 register accessor: an alias for `Reg<DAC_DR12CH1_SPEC>`"]
pub type DAC_DR12CH1 = crate::Reg<dac_dr12ch1::DAC_DR12CH1_SPEC>;
#[doc = "DAC_DR12CH1"]
pub mod dac_dr12ch1;
#[doc = "DAC_DL12CH1 register accessor: an alias for `Reg<DAC_DL12CH1_SPEC>`"]
pub type DAC_DL12CH1 = crate::Reg<dac_dl12ch1::DAC_DL12CH1_SPEC>;
#[doc = "DAC_DL12CH1"]
pub mod dac_dl12ch1;
#[doc = "DAC_DR8CH1 register accessor: an alias for `Reg<DAC_DR8CH1_SPEC>`"]
pub type DAC_DR8CH1 = crate::Reg<dac_dr8ch1::DAC_DR8CH1_SPEC>;
#[doc = "DAC_DR8CH1"]
pub mod dac_dr8ch1;
#[doc = "DAC_DR12CH2 register accessor: an alias for `Reg<DAC_DR12CH2_SPEC>`"]
pub type DAC_DR12CH2 = crate::Reg<dac_dr12ch2::DAC_DR12CH2_SPEC>;
#[doc = "DAC_DR12CH2"]
pub mod dac_dr12ch2;
#[doc = "DAC_DL12CH2 register accessor: an alias for `Reg<DAC_DL12CH2_SPEC>`"]
pub type DAC_DL12CH2 = crate::Reg<dac_dl12ch2::DAC_DL12CH2_SPEC>;
#[doc = "DAC_DL12CH2"]
pub mod dac_dl12ch2;
#[doc = "DAC_DR8CH2 register accessor: an alias for `Reg<DAC_DR8CH2_SPEC>`"]
pub type DAC_DR8CH2 = crate::Reg<dac_dr8ch2::DAC_DR8CH2_SPEC>;
#[doc = "DAC_DR8CH2"]
pub mod dac_dr8ch2;
#[doc = "DAC_DR12DCH register accessor: an alias for `Reg<DAC_DR12DCH_SPEC>`"]
pub type DAC_DR12DCH = crate::Reg<dac_dr12dch::DAC_DR12DCH_SPEC>;
#[doc = "DAC_DR12DCH"]
pub mod dac_dr12dch;
#[doc = "DAC_DL12DCH register accessor: an alias for `Reg<DAC_DL12DCH_SPEC>`"]
pub type DAC_DL12DCH = crate::Reg<dac_dl12dch::DAC_DL12DCH_SPEC>;
#[doc = "DAC_DL12DCH"]
pub mod dac_dl12dch;
#[doc = "DAC_DR8DCH register accessor: an alias for `Reg<DAC_DR8DCH_SPEC>`"]
pub type DAC_DR8DCH = crate::Reg<dac_dr8dch::DAC_DR8DCH_SPEC>;
#[doc = "DAC_DR8DCH"]
pub mod dac_dr8dch;
#[doc = "DAC_DATO1 register accessor: an alias for `Reg<DAC_DATO1_SPEC>`"]
pub type DAC_DATO1 = crate::Reg<dac_dato1::DAC_DATO1_SPEC>;
#[doc = "DAC_DATO1"]
pub mod dac_dato1;
#[doc = "DAC_DATO2 register accessor: an alias for `Reg<DAC_DATO2_SPEC>`"]
pub type DAC_DATO2 = crate::Reg<dac_dato2::DAC_DATO2_SPEC>;
#[doc = "DAC_DATO2"]
pub mod dac_dato2;
