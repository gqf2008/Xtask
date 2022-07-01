#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IWDG_KEY"]
    pub iwdg_key: crate::Reg<iwdg_key::IWDG_KEY_SPEC>,
    #[doc = "0x04 - IWDG_PREDIV"]
    pub iwdg_prediv: crate::Reg<iwdg_prediv::IWDG_PREDIV_SPEC>,
    #[doc = "0x08 - IWDG_RELV"]
    pub iwdg_relv: crate::Reg<iwdg_relv::IWDG_RELV_SPEC>,
    #[doc = "0x0c - IWDG_STS"]
    pub iwdg_sts: crate::Reg<iwdg_sts::IWDG_STS_SPEC>,
}
#[doc = "IWDG_KEY register accessor: an alias for `Reg<IWDG_KEY_SPEC>`"]
pub type IWDG_KEY = crate::Reg<iwdg_key::IWDG_KEY_SPEC>;
#[doc = "IWDG_KEY"]
pub mod iwdg_key;
#[doc = "IWDG_PREDIV register accessor: an alias for `Reg<IWDG_PREDIV_SPEC>`"]
pub type IWDG_PREDIV = crate::Reg<iwdg_prediv::IWDG_PREDIV_SPEC>;
#[doc = "IWDG_PREDIV"]
pub mod iwdg_prediv;
#[doc = "IWDG_RELV register accessor: an alias for `Reg<IWDG_RELV_SPEC>`"]
pub type IWDG_RELV = crate::Reg<iwdg_relv::IWDG_RELV_SPEC>;
#[doc = "IWDG_RELV"]
pub mod iwdg_relv;
#[doc = "IWDG_STS register accessor: an alias for `Reg<IWDG_STS_SPEC>`"]
pub type IWDG_STS = crate::Reg<iwdg_sts::IWDG_STS_SPEC>;
#[doc = "IWDG_STS"]
pub mod iwdg_sts;
