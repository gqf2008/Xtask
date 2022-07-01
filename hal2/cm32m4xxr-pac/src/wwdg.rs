#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - WWDG_CTRL"]
    pub wwdg_ctrl: crate::Reg<wwdg_ctrl::WWDG_CTRL_SPEC>,
    #[doc = "0x04 - WWDG_CFG"]
    pub wwdg_cfg: crate::Reg<wwdg_cfg::WWDG_CFG_SPEC>,
    #[doc = "0x08 - WWDG_STS"]
    pub wwdg_sts: crate::Reg<wwdg_sts::WWDG_STS_SPEC>,
}
#[doc = "WWDG_CTRL register accessor: an alias for `Reg<WWDG_CTRL_SPEC>`"]
pub type WWDG_CTRL = crate::Reg<wwdg_ctrl::WWDG_CTRL_SPEC>;
#[doc = "WWDG_CTRL"]
pub mod wwdg_ctrl;
#[doc = "WWDG_CFG register accessor: an alias for `Reg<WWDG_CFG_SPEC>`"]
pub type WWDG_CFG = crate::Reg<wwdg_cfg::WWDG_CFG_SPEC>;
#[doc = "WWDG_CFG"]
pub mod wwdg_cfg;
#[doc = "WWDG_STS register accessor: an alias for `Reg<WWDG_STS_SPEC>`"]
pub type WWDG_STS = crate::Reg<wwdg_sts::WWDG_STS_SPEC>;
#[doc = "WWDG_STS"]
pub mod wwdg_sts;
