#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAN_MCTRL"]
    pub can_mctrl: crate::Reg<can_mctrl::CAN_MCTRL_SPEC>,
    #[doc = "0x04 - CAN_MSTS"]
    pub can_msts: crate::Reg<can_msts::CAN_MSTS_SPEC>,
    #[doc = "0x08 - CAN_TSTS"]
    pub can_tsts: crate::Reg<can_tsts::CAN_TSTS_SPEC>,
    #[doc = "0x0c - CAN_RFF0"]
    pub can_rff0: crate::Reg<can_rff0::CAN_RFF0_SPEC>,
    #[doc = "0x10 - CAN_RFF1"]
    pub can_rff1: crate::Reg<can_rff1::CAN_RFF1_SPEC>,
    #[doc = "0x14 - CAN_INTE"]
    pub can_inte: crate::Reg<can_inte::CAN_INTE_SPEC>,
    #[doc = "0x18 - CAN_ESTS"]
    pub can_ests: crate::Reg<can_ests::CAN_ESTS_SPEC>,
    #[doc = "0x1c - CAN_BTIM"]
    pub can_btim: crate::Reg<can_btim::CAN_BTIM_SPEC>,
    _reserved8: [u8; 0x0160],
    #[doc = "0x180 - CAN_TMI0"]
    pub can_tmi0: crate::Reg<can_tmi0::CAN_TMI0_SPEC>,
    #[doc = "0x184 - CAN_TMDT0"]
    pub can_tmdt0: crate::Reg<can_tmdt0::CAN_TMDT0_SPEC>,
    #[doc = "0x188 - CAN_TMDL0"]
    pub can_tmdl0: crate::Reg<can_tmdl0::CAN_TMDL0_SPEC>,
    #[doc = "0x18c - CAN_TMDH0"]
    pub can_tmdh0: crate::Reg<can_tmdh0::CAN_TMDH0_SPEC>,
    #[doc = "0x190 - CAN_TMI1"]
    pub can_tmi1: crate::Reg<can_tmi1::CAN_TMI1_SPEC>,
    #[doc = "0x194 - CAN_TMDT1"]
    pub can_tmdt1: crate::Reg<can_tmdt1::CAN_TMDT1_SPEC>,
    #[doc = "0x198 - CAN_TMDL1"]
    pub can_tmdl1: crate::Reg<can_tmdl1::CAN_TMDL1_SPEC>,
    #[doc = "0x19c - CAN_TMDH1"]
    pub can_tmdh1: crate::Reg<can_tmdh1::CAN_TMDH1_SPEC>,
    #[doc = "0x1a0 - CAN_TMI2"]
    pub can_tmi2: crate::Reg<can_tmi2::CAN_TMI2_SPEC>,
    #[doc = "0x1a4 - CAN_TMDT2"]
    pub can_tmdt2: crate::Reg<can_tmdt2::CAN_TMDT2_SPEC>,
    #[doc = "0x1a8 - CAN_TMDL2"]
    pub can_tmdl2: crate::Reg<can_tmdl2::CAN_TMDL2_SPEC>,
    #[doc = "0x1ac - CAN_TMDH2"]
    pub can_tmdh2: crate::Reg<can_tmdh2::CAN_TMDH2_SPEC>,
    #[doc = "0x1b0 - CAN_RMI0"]
    pub can_rmi0: crate::Reg<can_rmi0::CAN_RMI0_SPEC>,
    #[doc = "0x1b4 - CAN_RMDT0"]
    pub can_rmdt0: crate::Reg<can_rmdt0::CAN_RMDT0_SPEC>,
    #[doc = "0x1b8 - CAN_RMDL0"]
    pub can_rmdl0: crate::Reg<can_rmdl0::CAN_RMDL0_SPEC>,
    #[doc = "0x1bc - CAN_RMDH0"]
    pub can_rmdh0: crate::Reg<can_rmdh0::CAN_RMDH0_SPEC>,
    #[doc = "0x1c0 - CAN_RMI1"]
    pub can_rmi1: crate::Reg<can_rmi1::CAN_RMI1_SPEC>,
    #[doc = "0x1c4 - CAN_RMDT1"]
    pub can_rmdt1: crate::Reg<can_rmdt1::CAN_RMDT1_SPEC>,
    #[doc = "0x1c8 - CAN_RMDL1"]
    pub can_rmdl1: crate::Reg<can_rmdl1::CAN_RMDL1_SPEC>,
    #[doc = "0x1cc - CAN_RMDH1"]
    pub can_rmdh1: crate::Reg<can_rmdh1::CAN_RMDH1_SPEC>,
    _reserved28: [u8; 0x30],
    #[doc = "0x200 - CAN_FMC"]
    pub can_fmc: crate::Reg<can_fmc::CAN_FMC_SPEC>,
    #[doc = "0x204 - CAN_FM1"]
    pub can_fm1: crate::Reg<can_fm1::CAN_FM1_SPEC>,
    _reserved30: [u8; 0x04],
    #[doc = "0x20c - CAN_FS1"]
    pub can_fs1: crate::Reg<can_fs1::CAN_FS1_SPEC>,
    _reserved31: [u8; 0x04],
    #[doc = "0x214 - CAN_FFA1"]
    pub can_ffa1: crate::Reg<can_ffa1::CAN_FFA1_SPEC>,
    _reserved32: [u8; 0x04],
    #[doc = "0x21c - CAN_FA1"]
    pub can_fa1: crate::Reg<can_fa1::CAN_FA1_SPEC>,
    _reserved33: [u8; 0x20],
    #[doc = "0x240 - CAN_F0B1"]
    pub can_f0b1: crate::Reg<can_f0b1::CAN_F0B1_SPEC>,
    #[doc = "0x244 - CAN_F0B2"]
    pub can_f0b2: crate::Reg<can_f0b2::CAN_F0B2_SPEC>,
    #[doc = "0x248 - CAN_F1B1"]
    pub can_f1b1: crate::Reg<can_f1b1::CAN_F1B1_SPEC>,
    #[doc = "0x24c - CAN_F1B2"]
    pub can_f1b2: crate::Reg<can_f1b2::CAN_F1B2_SPEC>,
    _reserved37: [u8; 0x58],
    #[doc = "0x2a8 - CAN_F13B1"]
    pub can_f13b1: crate::Reg<can_f13b1::CAN_F13B1_SPEC>,
    #[doc = "0x2ac - CAN_F13B2"]
    pub can_f13b2: crate::Reg<can_f13b2::CAN_F13B2_SPEC>,
}
#[doc = "CAN_MCTRL register accessor: an alias for `Reg<CAN_MCTRL_SPEC>`"]
pub type CAN_MCTRL = crate::Reg<can_mctrl::CAN_MCTRL_SPEC>;
#[doc = "CAN_MCTRL"]
pub mod can_mctrl;
#[doc = "CAN_MSTS register accessor: an alias for `Reg<CAN_MSTS_SPEC>`"]
pub type CAN_MSTS = crate::Reg<can_msts::CAN_MSTS_SPEC>;
#[doc = "CAN_MSTS"]
pub mod can_msts;
#[doc = "CAN_TSTS register accessor: an alias for `Reg<CAN_TSTS_SPEC>`"]
pub type CAN_TSTS = crate::Reg<can_tsts::CAN_TSTS_SPEC>;
#[doc = "CAN_TSTS"]
pub mod can_tsts;
#[doc = "CAN_RFF0 register accessor: an alias for `Reg<CAN_RFF0_SPEC>`"]
pub type CAN_RFF0 = crate::Reg<can_rff0::CAN_RFF0_SPEC>;
#[doc = "CAN_RFF0"]
pub mod can_rff0;
#[doc = "CAN_RFF1 register accessor: an alias for `Reg<CAN_RFF1_SPEC>`"]
pub type CAN_RFF1 = crate::Reg<can_rff1::CAN_RFF1_SPEC>;
#[doc = "CAN_RFF1"]
pub mod can_rff1;
#[doc = "CAN_INTE register accessor: an alias for `Reg<CAN_INTE_SPEC>`"]
pub type CAN_INTE = crate::Reg<can_inte::CAN_INTE_SPEC>;
#[doc = "CAN_INTE"]
pub mod can_inte;
#[doc = "CAN_ESTS register accessor: an alias for `Reg<CAN_ESTS_SPEC>`"]
pub type CAN_ESTS = crate::Reg<can_ests::CAN_ESTS_SPEC>;
#[doc = "CAN_ESTS"]
pub mod can_ests;
#[doc = "CAN_BTIM register accessor: an alias for `Reg<CAN_BTIM_SPEC>`"]
pub type CAN_BTIM = crate::Reg<can_btim::CAN_BTIM_SPEC>;
#[doc = "CAN_BTIM"]
pub mod can_btim;
#[doc = "CAN_TMI0 register accessor: an alias for `Reg<CAN_TMI0_SPEC>`"]
pub type CAN_TMI0 = crate::Reg<can_tmi0::CAN_TMI0_SPEC>;
#[doc = "CAN_TMI0"]
pub mod can_tmi0;
#[doc = "CAN_TMDT0 register accessor: an alias for `Reg<CAN_TMDT0_SPEC>`"]
pub type CAN_TMDT0 = crate::Reg<can_tmdt0::CAN_TMDT0_SPEC>;
#[doc = "CAN_TMDT0"]
pub mod can_tmdt0;
#[doc = "CAN_TMDL0 register accessor: an alias for `Reg<CAN_TMDL0_SPEC>`"]
pub type CAN_TMDL0 = crate::Reg<can_tmdl0::CAN_TMDL0_SPEC>;
#[doc = "CAN_TMDL0"]
pub mod can_tmdl0;
#[doc = "CAN_TMDH0 register accessor: an alias for `Reg<CAN_TMDH0_SPEC>`"]
pub type CAN_TMDH0 = crate::Reg<can_tmdh0::CAN_TMDH0_SPEC>;
#[doc = "CAN_TMDH0"]
pub mod can_tmdh0;
#[doc = "CAN_TMI1 register accessor: an alias for `Reg<CAN_TMI1_SPEC>`"]
pub type CAN_TMI1 = crate::Reg<can_tmi1::CAN_TMI1_SPEC>;
#[doc = "CAN_TMI1"]
pub mod can_tmi1;
#[doc = "CAN_TMDT1 register accessor: an alias for `Reg<CAN_TMDT1_SPEC>`"]
pub type CAN_TMDT1 = crate::Reg<can_tmdt1::CAN_TMDT1_SPEC>;
#[doc = "CAN_TMDT1"]
pub mod can_tmdt1;
#[doc = "CAN_TMDL1 register accessor: an alias for `Reg<CAN_TMDL1_SPEC>`"]
pub type CAN_TMDL1 = crate::Reg<can_tmdl1::CAN_TMDL1_SPEC>;
#[doc = "CAN_TMDL1"]
pub mod can_tmdl1;
#[doc = "CAN_TMDH1 register accessor: an alias for `Reg<CAN_TMDH1_SPEC>`"]
pub type CAN_TMDH1 = crate::Reg<can_tmdh1::CAN_TMDH1_SPEC>;
#[doc = "CAN_TMDH1"]
pub mod can_tmdh1;
#[doc = "CAN_TMI2 register accessor: an alias for `Reg<CAN_TMI2_SPEC>`"]
pub type CAN_TMI2 = crate::Reg<can_tmi2::CAN_TMI2_SPEC>;
#[doc = "CAN_TMI2"]
pub mod can_tmi2;
#[doc = "CAN_TMDT2 register accessor: an alias for `Reg<CAN_TMDT2_SPEC>`"]
pub type CAN_TMDT2 = crate::Reg<can_tmdt2::CAN_TMDT2_SPEC>;
#[doc = "CAN_TMDT2"]
pub mod can_tmdt2;
#[doc = "CAN_TMDL2 register accessor: an alias for `Reg<CAN_TMDL2_SPEC>`"]
pub type CAN_TMDL2 = crate::Reg<can_tmdl2::CAN_TMDL2_SPEC>;
#[doc = "CAN_TMDL2"]
pub mod can_tmdl2;
#[doc = "CAN_TMDH2 register accessor: an alias for `Reg<CAN_TMDH2_SPEC>`"]
pub type CAN_TMDH2 = crate::Reg<can_tmdh2::CAN_TMDH2_SPEC>;
#[doc = "CAN_TMDH2"]
pub mod can_tmdh2;
#[doc = "CAN_RMI0 register accessor: an alias for `Reg<CAN_RMI0_SPEC>`"]
pub type CAN_RMI0 = crate::Reg<can_rmi0::CAN_RMI0_SPEC>;
#[doc = "CAN_RMI0"]
pub mod can_rmi0;
#[doc = "CAN_RMDT0 register accessor: an alias for `Reg<CAN_RMDT0_SPEC>`"]
pub type CAN_RMDT0 = crate::Reg<can_rmdt0::CAN_RMDT0_SPEC>;
#[doc = "CAN_RMDT0"]
pub mod can_rmdt0;
#[doc = "CAN_RMDL0 register accessor: an alias for `Reg<CAN_RMDL0_SPEC>`"]
pub type CAN_RMDL0 = crate::Reg<can_rmdl0::CAN_RMDL0_SPEC>;
#[doc = "CAN_RMDL0"]
pub mod can_rmdl0;
#[doc = "CAN_RMDH0 register accessor: an alias for `Reg<CAN_RMDH0_SPEC>`"]
pub type CAN_RMDH0 = crate::Reg<can_rmdh0::CAN_RMDH0_SPEC>;
#[doc = "CAN_RMDH0"]
pub mod can_rmdh0;
#[doc = "CAN_RMI1 register accessor: an alias for `Reg<CAN_RMI1_SPEC>`"]
pub type CAN_RMI1 = crate::Reg<can_rmi1::CAN_RMI1_SPEC>;
#[doc = "CAN_RMI1"]
pub mod can_rmi1;
#[doc = "CAN_RMDT1 register accessor: an alias for `Reg<CAN_RMDT1_SPEC>`"]
pub type CAN_RMDT1 = crate::Reg<can_rmdt1::CAN_RMDT1_SPEC>;
#[doc = "CAN_RMDT1"]
pub mod can_rmdt1;
#[doc = "CAN_RMDL1 register accessor: an alias for `Reg<CAN_RMDL1_SPEC>`"]
pub type CAN_RMDL1 = crate::Reg<can_rmdl1::CAN_RMDL1_SPEC>;
#[doc = "CAN_RMDL1"]
pub mod can_rmdl1;
#[doc = "CAN_RMDH1 register accessor: an alias for `Reg<CAN_RMDH1_SPEC>`"]
pub type CAN_RMDH1 = crate::Reg<can_rmdh1::CAN_RMDH1_SPEC>;
#[doc = "CAN_RMDH1"]
pub mod can_rmdh1;
#[doc = "CAN_FMC register accessor: an alias for `Reg<CAN_FMC_SPEC>`"]
pub type CAN_FMC = crate::Reg<can_fmc::CAN_FMC_SPEC>;
#[doc = "CAN_FMC"]
pub mod can_fmc;
#[doc = "CAN_FM1 register accessor: an alias for `Reg<CAN_FM1_SPEC>`"]
pub type CAN_FM1 = crate::Reg<can_fm1::CAN_FM1_SPEC>;
#[doc = "CAN_FM1"]
pub mod can_fm1;
#[doc = "CAN_FS1 register accessor: an alias for `Reg<CAN_FS1_SPEC>`"]
pub type CAN_FS1 = crate::Reg<can_fs1::CAN_FS1_SPEC>;
#[doc = "CAN_FS1"]
pub mod can_fs1;
#[doc = "CAN_FFA1 register accessor: an alias for `Reg<CAN_FFA1_SPEC>`"]
pub type CAN_FFA1 = crate::Reg<can_ffa1::CAN_FFA1_SPEC>;
#[doc = "CAN_FFA1"]
pub mod can_ffa1;
#[doc = "CAN_FA1 register accessor: an alias for `Reg<CAN_FA1_SPEC>`"]
pub type CAN_FA1 = crate::Reg<can_fa1::CAN_FA1_SPEC>;
#[doc = "CAN_FA1"]
pub mod can_fa1;
#[doc = "CAN_F0B1 register accessor: an alias for `Reg<CAN_F0B1_SPEC>`"]
pub type CAN_F0B1 = crate::Reg<can_f0b1::CAN_F0B1_SPEC>;
#[doc = "CAN_F0B1"]
pub mod can_f0b1;
#[doc = "CAN_F0B2 register accessor: an alias for `Reg<CAN_F0B2_SPEC>`"]
pub type CAN_F0B2 = crate::Reg<can_f0b2::CAN_F0B2_SPEC>;
#[doc = "CAN_F0B2"]
pub mod can_f0b2;
#[doc = "CAN_F1B1 register accessor: an alias for `Reg<CAN_F1B1_SPEC>`"]
pub type CAN_F1B1 = crate::Reg<can_f1b1::CAN_F1B1_SPEC>;
#[doc = "CAN_F1B1"]
pub mod can_f1b1;
#[doc = "CAN_F1B2 register accessor: an alias for `Reg<CAN_F1B2_SPEC>`"]
pub type CAN_F1B2 = crate::Reg<can_f1b2::CAN_F1B2_SPEC>;
#[doc = "CAN_F1B2"]
pub mod can_f1b2;
#[doc = "CAN_F13B1 register accessor: an alias for `Reg<CAN_F13B1_SPEC>`"]
pub type CAN_F13B1 = crate::Reg<can_f13b1::CAN_F13B1_SPEC>;
#[doc = "CAN_F13B1"]
pub mod can_f13b1;
#[doc = "CAN_F13B2 register accessor: an alias for `Reg<CAN_F13B2_SPEC>`"]
pub type CAN_F13B2 = crate::Reg<can_f13b2::CAN_F13B2_SPEC>;
#[doc = "CAN_F13B2"]
pub mod can_f13b2;
