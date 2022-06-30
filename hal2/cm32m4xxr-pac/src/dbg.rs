#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DBG_ID"]
    pub dbg_id: crate::Reg<dbg_id::DBG_ID_SPEC>,
    #[doc = "0x04 - DBG_CTRL"]
    pub dbg_ctrl: crate::Reg<dbg_ctrl::DBG_CTRL_SPEC>,
}
#[doc = "DBG_ID register accessor: an alias for `Reg<DBG_ID_SPEC>`"]
pub type DBG_ID = crate::Reg<dbg_id::DBG_ID_SPEC>;
#[doc = "DBG_ID"]
pub mod dbg_id;
#[doc = "DBG_CTRL register accessor: an alias for `Reg<DBG_CTRL_SPEC>`"]
pub type DBG_CTRL = crate::Reg<dbg_ctrl::DBG_CTRL_SPEC>;
#[doc = "DBG_CTRL"]
pub mod dbg_ctrl;
