#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - COMP1_CTRL"]
    pub comp1_ctrl: crate::Reg<comp1_ctrl::COMP1_CTRL_SPEC>,
    #[doc = "0x14 - COMP1_FILC"]
    pub comp1_filc: crate::Reg<comp1_filc::COMP1_FILC_SPEC>,
    #[doc = "0x18 - COMP1_FILP"]
    pub comp1_filp: crate::Reg<comp1_filp::COMP1_FILP_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x20 - COMP2_CTRL"]
    pub comp2_ctrl: crate::Reg<comp2_ctrl::COMP2_CTRL_SPEC>,
    #[doc = "0x24 - COMP2_FILC"]
    pub comp2_filc: crate::Reg<comp2_filc::COMP2_FILC_SPEC>,
    #[doc = "0x28 - COMP2_FILP"]
    pub comp2_filp: crate::Reg<comp2_filp::COMP2_FILP_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x30 - COMP3_CTRL"]
    pub comp3_ctrl: crate::Reg<comp3_ctrl::COMP3_CTRL_SPEC>,
    #[doc = "0x34 - COMP3_FILC"]
    pub comp3_filc: crate::Reg<comp3_filc::COMP3_FILC_SPEC>,
    #[doc = "0x38 - COMP3_FILP"]
    pub comp3_filp: crate::Reg<comp3_filp::COMP3_FILP_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x40 - COMP4_CTRL"]
    pub comp4_ctrl: crate::Reg<comp4_ctrl::COMP4_CTRL_SPEC>,
    #[doc = "0x44 - COMP4_FILC"]
    pub comp4_filc: crate::Reg<comp4_filc::COMP4_FILC_SPEC>,
    #[doc = "0x48 - COMP4_FILP"]
    pub comp4_filp: crate::Reg<comp4_filp::COMP4_FILP_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x50 - COMP5_CTRL"]
    pub comp5_ctrl: crate::Reg<comp5_ctrl::COMP5_CTRL_SPEC>,
    #[doc = "0x54 - COMP5_FILC"]
    pub comp5_filc: crate::Reg<comp5_filc::COMP5_FILC_SPEC>,
    #[doc = "0x58 - COMP5_FILP"]
    pub comp5_filp: crate::Reg<comp5_filp::COMP5_FILP_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x60 - COMP6_CTRL"]
    pub comp6_ctrl: crate::Reg<comp6_ctrl::COMP6_CTRL_SPEC>,
    #[doc = "0x64 - COMP6_FILC"]
    pub comp6_filc: crate::Reg<comp6_filc::COMP6_FILC_SPEC>,
    #[doc = "0x68 - COMP6_FILP"]
    pub comp6_filp: crate::Reg<comp6_filp::COMP6_FILP_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x70 - COMP7_CTRL"]
    pub comp7_ctrl: crate::Reg<comp7_ctrl::COMP7_CTRL_SPEC>,
    #[doc = "0x74 - COMP7_FILC"]
    pub comp7_filc: crate::Reg<comp7_filc::COMP7_FILC_SPEC>,
    #[doc = "0x78 - COMP7_FILP"]
    pub comp7_filp: crate::Reg<comp7_filp::COMP7_FILP_SPEC>,
    _reserved21: [u8; 0x04],
    #[doc = "0x80 - COMP_WINMODE"]
    pub comp_winmode: crate::Reg<comp_winmode::COMP_WINMODE_SPEC>,
    #[doc = "0x84 - COMP_LOCK"]
    pub comp_lock: crate::Reg<comp_lock::COMP_LOCK_SPEC>,
    _reserved23: [u8; 0x04],
    #[doc = "0x8c - COMP_INTEN"]
    pub comp_inten: crate::Reg<comp_inten::COMP_INTEN_SPEC>,
    #[doc = "0x90 - COMP_INTSTS"]
    pub comp_intsts: crate::Reg<comp_intsts::COMP_INTSTS_SPEC>,
    #[doc = "0x94 - COMP_VREFSCL"]
    pub comp_vrefscl: crate::Reg<comp_vrefscl::COMP_VREFSCL_SPEC>,
}
#[doc = "COMP1_CTRL register accessor: an alias for `Reg<COMP1_CTRL_SPEC>`"]
pub type COMP1_CTRL = crate::Reg<comp1_ctrl::COMP1_CTRL_SPEC>;
#[doc = "COMP1_CTRL"]
pub mod comp1_ctrl;
#[doc = "COMP1_FILC register accessor: an alias for `Reg<COMP1_FILC_SPEC>`"]
pub type COMP1_FILC = crate::Reg<comp1_filc::COMP1_FILC_SPEC>;
#[doc = "COMP1_FILC"]
pub mod comp1_filc;
#[doc = "COMP1_FILP register accessor: an alias for `Reg<COMP1_FILP_SPEC>`"]
pub type COMP1_FILP = crate::Reg<comp1_filp::COMP1_FILP_SPEC>;
#[doc = "COMP1_FILP"]
pub mod comp1_filp;
#[doc = "COMP2_CTRL register accessor: an alias for `Reg<COMP2_CTRL_SPEC>`"]
pub type COMP2_CTRL = crate::Reg<comp2_ctrl::COMP2_CTRL_SPEC>;
#[doc = "COMP2_CTRL"]
pub mod comp2_ctrl;
#[doc = "COMP2_FILC register accessor: an alias for `Reg<COMP2_FILC_SPEC>`"]
pub type COMP2_FILC = crate::Reg<comp2_filc::COMP2_FILC_SPEC>;
#[doc = "COMP2_FILC"]
pub mod comp2_filc;
#[doc = "COMP2_FILP register accessor: an alias for `Reg<COMP2_FILP_SPEC>`"]
pub type COMP2_FILP = crate::Reg<comp2_filp::COMP2_FILP_SPEC>;
#[doc = "COMP2_FILP"]
pub mod comp2_filp;
#[doc = "COMP3_CTRL register accessor: an alias for `Reg<COMP3_CTRL_SPEC>`"]
pub type COMP3_CTRL = crate::Reg<comp3_ctrl::COMP3_CTRL_SPEC>;
#[doc = "COMP3_CTRL"]
pub mod comp3_ctrl;
#[doc = "COMP3_FILC register accessor: an alias for `Reg<COMP3_FILC_SPEC>`"]
pub type COMP3_FILC = crate::Reg<comp3_filc::COMP3_FILC_SPEC>;
#[doc = "COMP3_FILC"]
pub mod comp3_filc;
#[doc = "COMP3_FILP register accessor: an alias for `Reg<COMP3_FILP_SPEC>`"]
pub type COMP3_FILP = crate::Reg<comp3_filp::COMP3_FILP_SPEC>;
#[doc = "COMP3_FILP"]
pub mod comp3_filp;
#[doc = "COMP4_CTRL register accessor: an alias for `Reg<COMP4_CTRL_SPEC>`"]
pub type COMP4_CTRL = crate::Reg<comp4_ctrl::COMP4_CTRL_SPEC>;
#[doc = "COMP4_CTRL"]
pub mod comp4_ctrl;
#[doc = "COMP4_FILC register accessor: an alias for `Reg<COMP4_FILC_SPEC>`"]
pub type COMP4_FILC = crate::Reg<comp4_filc::COMP4_FILC_SPEC>;
#[doc = "COMP4_FILC"]
pub mod comp4_filc;
#[doc = "COMP4_FILP register accessor: an alias for `Reg<COMP4_FILP_SPEC>`"]
pub type COMP4_FILP = crate::Reg<comp4_filp::COMP4_FILP_SPEC>;
#[doc = "COMP4_FILP"]
pub mod comp4_filp;
#[doc = "COMP5_CTRL register accessor: an alias for `Reg<COMP5_CTRL_SPEC>`"]
pub type COMP5_CTRL = crate::Reg<comp5_ctrl::COMP5_CTRL_SPEC>;
#[doc = "COMP5_CTRL"]
pub mod comp5_ctrl;
#[doc = "COMP5_FILC register accessor: an alias for `Reg<COMP5_FILC_SPEC>`"]
pub type COMP5_FILC = crate::Reg<comp5_filc::COMP5_FILC_SPEC>;
#[doc = "COMP5_FILC"]
pub mod comp5_filc;
#[doc = "COMP5_FILP register accessor: an alias for `Reg<COMP5_FILP_SPEC>`"]
pub type COMP5_FILP = crate::Reg<comp5_filp::COMP5_FILP_SPEC>;
#[doc = "COMP5_FILP"]
pub mod comp5_filp;
#[doc = "COMP6_CTRL register accessor: an alias for `Reg<COMP6_CTRL_SPEC>`"]
pub type COMP6_CTRL = crate::Reg<comp6_ctrl::COMP6_CTRL_SPEC>;
#[doc = "COMP6_CTRL"]
pub mod comp6_ctrl;
#[doc = "COMP6_FILC register accessor: an alias for `Reg<COMP6_FILC_SPEC>`"]
pub type COMP6_FILC = crate::Reg<comp6_filc::COMP6_FILC_SPEC>;
#[doc = "COMP6_FILC"]
pub mod comp6_filc;
#[doc = "COMP6_FILP register accessor: an alias for `Reg<COMP6_FILP_SPEC>`"]
pub type COMP6_FILP = crate::Reg<comp6_filp::COMP6_FILP_SPEC>;
#[doc = "COMP6_FILP"]
pub mod comp6_filp;
#[doc = "COMP7_CTRL register accessor: an alias for `Reg<COMP7_CTRL_SPEC>`"]
pub type COMP7_CTRL = crate::Reg<comp7_ctrl::COMP7_CTRL_SPEC>;
#[doc = "COMP7_CTRL"]
pub mod comp7_ctrl;
#[doc = "COMP7_FILC register accessor: an alias for `Reg<COMP7_FILC_SPEC>`"]
pub type COMP7_FILC = crate::Reg<comp7_filc::COMP7_FILC_SPEC>;
#[doc = "COMP7_FILC"]
pub mod comp7_filc;
#[doc = "COMP7_FILP register accessor: an alias for `Reg<COMP7_FILP_SPEC>`"]
pub type COMP7_FILP = crate::Reg<comp7_filp::COMP7_FILP_SPEC>;
#[doc = "COMP7_FILP"]
pub mod comp7_filp;
#[doc = "COMP_WINMODE register accessor: an alias for `Reg<COMP_WINMODE_SPEC>`"]
pub type COMP_WINMODE = crate::Reg<comp_winmode::COMP_WINMODE_SPEC>;
#[doc = "COMP_WINMODE"]
pub mod comp_winmode;
#[doc = "COMP_LOCK register accessor: an alias for `Reg<COMP_LOCK_SPEC>`"]
pub type COMP_LOCK = crate::Reg<comp_lock::COMP_LOCK_SPEC>;
#[doc = "COMP_LOCK"]
pub mod comp_lock;
#[doc = "COMP_INTEN register accessor: an alias for `Reg<COMP_INTEN_SPEC>`"]
pub type COMP_INTEN = crate::Reg<comp_inten::COMP_INTEN_SPEC>;
#[doc = "COMP_INTEN"]
pub mod comp_inten;
#[doc = "COMP_INTSTS register accessor: an alias for `Reg<COMP_INTSTS_SPEC>`"]
pub type COMP_INTSTS = crate::Reg<comp_intsts::COMP_INTSTS_SPEC>;
#[doc = "COMP_INTSTS"]
pub mod comp_intsts;
#[doc = "COMP_VREFSCL register accessor: an alias for `Reg<COMP_VREFSCL_SPEC>`"]
pub type COMP_VREFSCL = crate::Reg<comp_vrefscl::COMP_VREFSCL_SPEC>;
#[doc = "COMP_VREFSCL"]
pub mod comp_vrefscl;
