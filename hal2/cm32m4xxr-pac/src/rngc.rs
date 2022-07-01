#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RNG_CON"]
    pub rng_con: crate::Reg<rng_con::RNG_CON_SPEC>,
    #[doc = "0x04 - RNG_CON_WR_READY"]
    pub rng_con_wr_ready: crate::Reg<rng_con_wr_ready::RNG_CON_WR_READY_SPEC>,
    #[doc = "0x08 - RNG_DATA"]
    pub rng_data: crate::Reg<rng_data::RNG_DATA_SPEC>,
    #[doc = "0x0c - RNG_READY"]
    pub rng_ready: crate::Reg<rng_ready::RNG_READY_SPEC>,
}
#[doc = "RNG_CON register accessor: an alias for `Reg<RNG_CON_SPEC>`"]
pub type RNG_CON = crate::Reg<rng_con::RNG_CON_SPEC>;
#[doc = "RNG_CON"]
pub mod rng_con;
#[doc = "RNG_CON_WR_READY register accessor: an alias for `Reg<RNG_CON_WR_READY_SPEC>`"]
pub type RNG_CON_WR_READY = crate::Reg<rng_con_wr_ready::RNG_CON_WR_READY_SPEC>;
#[doc = "RNG_CON_WR_READY"]
pub mod rng_con_wr_ready;
#[doc = "RNG_DATA register accessor: an alias for `Reg<RNG_DATA_SPEC>`"]
pub type RNG_DATA = crate::Reg<rng_data::RNG_DATA_SPEC>;
#[doc = "RNG_DATA"]
pub mod rng_data;
#[doc = "RNG_READY register accessor: an alias for `Reg<RNG_READY_SPEC>`"]
pub type RNG_READY = crate::Reg<rng_ready::RNG_READY_SPEC>;
#[doc = "RNG_READY"]
pub mod rng_ready;
