#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FLASH_AC"]
    pub flash_ac: crate::Reg<flash_ac::FLASH_AC_SPEC>,
    #[doc = "0x04 - FLASH_KEY"]
    pub flash_key: crate::Reg<flash_key::FLASH_KEY_SPEC>,
    #[doc = "0x08 - FLASH_OPTKEY"]
    pub flash_optkey: crate::Reg<flash_optkey::FLASH_OPTKEY_SPEC>,
    #[doc = "0x0c - FLASH_STS"]
    pub flash_sts: crate::Reg<flash_sts::FLASH_STS_SPEC>,
    #[doc = "0x10 - FLASH_CTRL"]
    pub flash_ctrl: crate::Reg<flash_ctrl::FLASH_CTRL_SPEC>,
    #[doc = "0x14 - FLASH_ADD"]
    pub flash_add: crate::Reg<flash_add::FLASH_ADD_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - FLASH_OB"]
    pub flash_ob: crate::Reg<flash_ob::FLASH_OB_SPEC>,
    #[doc = "0x20 - FLASH_WRP"]
    pub flash_wrp: crate::Reg<flash_wrp::FLASH_WRP_SPEC>,
    #[doc = "0x24 - FLASH_ECCR"]
    pub flash_eccr: crate::Reg<flash_eccr::FLASH_ECCR_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x2c - FLASH_RDN"]
    pub flash_rdn: crate::Reg<flash_rdn::FLASH_RDN_SPEC>,
    #[doc = "0x30 - FLASH_CAHR"]
    pub flash_cahr: crate::Reg<flash_cahr::FLASH_CAHR_SPEC>,
}
#[doc = "FLASH_AC register accessor: an alias for `Reg<FLASH_AC_SPEC>`"]
pub type FLASH_AC = crate::Reg<flash_ac::FLASH_AC_SPEC>;
#[doc = "FLASH_AC"]
pub mod flash_ac;
#[doc = "FLASH_KEY register accessor: an alias for `Reg<FLASH_KEY_SPEC>`"]
pub type FLASH_KEY = crate::Reg<flash_key::FLASH_KEY_SPEC>;
#[doc = "FLASH_KEY"]
pub mod flash_key;
#[doc = "FLASH_OPTKEY register accessor: an alias for `Reg<FLASH_OPTKEY_SPEC>`"]
pub type FLASH_OPTKEY = crate::Reg<flash_optkey::FLASH_OPTKEY_SPEC>;
#[doc = "FLASH_OPTKEY"]
pub mod flash_optkey;
#[doc = "FLASH_STS register accessor: an alias for `Reg<FLASH_STS_SPEC>`"]
pub type FLASH_STS = crate::Reg<flash_sts::FLASH_STS_SPEC>;
#[doc = "FLASH_STS"]
pub mod flash_sts;
#[doc = "FLASH_CTRL register accessor: an alias for `Reg<FLASH_CTRL_SPEC>`"]
pub type FLASH_CTRL = crate::Reg<flash_ctrl::FLASH_CTRL_SPEC>;
#[doc = "FLASH_CTRL"]
pub mod flash_ctrl;
#[doc = "FLASH_ADD register accessor: an alias for `Reg<FLASH_ADD_SPEC>`"]
pub type FLASH_ADD = crate::Reg<flash_add::FLASH_ADD_SPEC>;
#[doc = "FLASH_ADD"]
pub mod flash_add;
#[doc = "FLASH_OB register accessor: an alias for `Reg<FLASH_OB_SPEC>`"]
pub type FLASH_OB = crate::Reg<flash_ob::FLASH_OB_SPEC>;
#[doc = "FLASH_OB"]
pub mod flash_ob;
#[doc = "FLASH_WRP register accessor: an alias for `Reg<FLASH_WRP_SPEC>`"]
pub type FLASH_WRP = crate::Reg<flash_wrp::FLASH_WRP_SPEC>;
#[doc = "FLASH_WRP"]
pub mod flash_wrp;
#[doc = "FLASH_ECCR register accessor: an alias for `Reg<FLASH_ECCR_SPEC>`"]
pub type FLASH_ECCR = crate::Reg<flash_eccr::FLASH_ECCR_SPEC>;
#[doc = "FLASH_ECCR"]
pub mod flash_eccr;
#[doc = "FLASH_RDN register accessor: an alias for `Reg<FLASH_RDN_SPEC>`"]
pub type FLASH_RDN = crate::Reg<flash_rdn::FLASH_RDN_SPEC>;
#[doc = "FLASH_RDN"]
pub mod flash_rdn;
#[doc = "FLASH_CAHR register accessor: an alias for `Reg<FLASH_CAHR_SPEC>`"]
pub type FLASH_CAHR = crate::Reg<flash_cahr::FLASH_CAHR_SPEC>;
#[doc = "FLASH_CAHR"]
pub mod flash_cahr;
