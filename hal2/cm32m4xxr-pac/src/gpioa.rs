#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIOx_PL_CFG"]
    pub gpiox_pl_cfg: crate::Reg<gpiox_pl_cfg::GPIOX_PL_CFG_SPEC>,
    #[doc = "0x04 - GPIOx_PH_CFG"]
    pub gpiox_ph_cfg: crate::Reg<gpiox_ph_cfg::GPIOX_PH_CFG_SPEC>,
    #[doc = "0x08 - GPIOx_PID"]
    pub gpiox_pid: crate::Reg<gpiox_pid::GPIOX_PID_SPEC>,
    #[doc = "0x0c - GPIOx_POD"]
    pub gpiox_pod: crate::Reg<gpiox_pod::GPIOX_POD_SPEC>,
    #[doc = "0x10 - GPIOx_PBSC"]
    pub gpiox_pbsc: crate::Reg<gpiox_pbsc::GPIOX_PBSC_SPEC>,
    #[doc = "0x14 - GPIOx_PBC"]
    pub gpiox_pbc: crate::Reg<gpiox_pbc::GPIOX_PBC_SPEC>,
    #[doc = "0x18 - GPIOx_PLOCK_CFG"]
    pub gpiox_plock_cfg: crate::Reg<gpiox_plock_cfg::GPIOX_PLOCK_CFG_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - GPIOx_DS_CFG"]
    pub gpiox_ds_cfg: crate::Reg<gpiox_ds_cfg::GPIOX_DS_CFG_SPEC>,
    #[doc = "0x24 - GPIOx_SR_CFG"]
    pub gpiox_sr_cfg: crate::Reg<gpiox_sr_cfg::GPIOX_SR_CFG_SPEC>,
}
#[doc = "GPIOx_PL_CFG register accessor: an alias for `Reg<GPIOX_PL_CFG_SPEC>`"]
pub type GPIOX_PL_CFG = crate::Reg<gpiox_pl_cfg::GPIOX_PL_CFG_SPEC>;
#[doc = "GPIOx_PL_CFG"]
pub mod gpiox_pl_cfg;
#[doc = "GPIOx_PH_CFG register accessor: an alias for `Reg<GPIOX_PH_CFG_SPEC>`"]
pub type GPIOX_PH_CFG = crate::Reg<gpiox_ph_cfg::GPIOX_PH_CFG_SPEC>;
#[doc = "GPIOx_PH_CFG"]
pub mod gpiox_ph_cfg;
#[doc = "GPIOx_PID register accessor: an alias for `Reg<GPIOX_PID_SPEC>`"]
pub type GPIOX_PID = crate::Reg<gpiox_pid::GPIOX_PID_SPEC>;
#[doc = "GPIOx_PID"]
pub mod gpiox_pid;
#[doc = "GPIOx_POD register accessor: an alias for `Reg<GPIOX_POD_SPEC>`"]
pub type GPIOX_POD = crate::Reg<gpiox_pod::GPIOX_POD_SPEC>;
#[doc = "GPIOx_POD"]
pub mod gpiox_pod;
#[doc = "GPIOx_PBSC register accessor: an alias for `Reg<GPIOX_PBSC_SPEC>`"]
pub type GPIOX_PBSC = crate::Reg<gpiox_pbsc::GPIOX_PBSC_SPEC>;
#[doc = "GPIOx_PBSC"]
pub mod gpiox_pbsc;
#[doc = "GPIOx_PBC register accessor: an alias for `Reg<GPIOX_PBC_SPEC>`"]
pub type GPIOX_PBC = crate::Reg<gpiox_pbc::GPIOX_PBC_SPEC>;
#[doc = "GPIOx_PBC"]
pub mod gpiox_pbc;
#[doc = "GPIOx_PLOCK_CFG register accessor: an alias for `Reg<GPIOX_PLOCK_CFG_SPEC>`"]
pub type GPIOX_PLOCK_CFG = crate::Reg<gpiox_plock_cfg::GPIOX_PLOCK_CFG_SPEC>;
#[doc = "GPIOx_PLOCK_CFG"]
pub mod gpiox_plock_cfg;
#[doc = "GPIOx_DS_CFG register accessor: an alias for `Reg<GPIOX_DS_CFG_SPEC>`"]
pub type GPIOX_DS_CFG = crate::Reg<gpiox_ds_cfg::GPIOX_DS_CFG_SPEC>;
#[doc = "GPIOx_DS_CFG"]
pub mod gpiox_ds_cfg;
#[doc = "GPIOx_SR_CFG register accessor: an alias for `Reg<GPIOX_SR_CFG_SPEC>`"]
pub type GPIOX_SR_CFG = crate::Reg<gpiox_sr_cfg::GPIOX_SR_CFG_SPEC>;
#[doc = "GPIOx_SR_CFG"]
pub mod gpiox_sr_cfg;
