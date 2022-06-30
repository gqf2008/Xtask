#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OPA_CS1"]
    pub opa_cs1: crate::Reg<opa_cs1::OPA_CS1_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - OPA_CS2"]
    pub opa_cs2: crate::Reg<opa_cs2::OPA_CS2_SPEC>,
    _reserved2: [u8; 0x0c],
    #[doc = "0x20 - OPA_CS3"]
    pub opa_cs3: crate::Reg<opa_cs3::OPA_CS3_SPEC>,
    _reserved3: [u8; 0x0c],
    #[doc = "0x30 - OPA_CS4"]
    pub opa_cs4: crate::Reg<opa_cs4::OPA_CS4_SPEC>,
    _reserved4: [u8; 0x0c],
    #[doc = "0x40 - OPA_LOCK"]
    pub opa_lock: crate::Reg<opa_lock::OPA_LOCK_SPEC>,
}
#[doc = "OPA_CS1 register accessor: an alias for `Reg<OPA_CS1_SPEC>`"]
pub type OPA_CS1 = crate::Reg<opa_cs1::OPA_CS1_SPEC>;
#[doc = "OPA_CS1"]
pub mod opa_cs1;
#[doc = "OPA_CS2 register accessor: an alias for `Reg<OPA_CS2_SPEC>`"]
pub type OPA_CS2 = crate::Reg<opa_cs2::OPA_CS2_SPEC>;
#[doc = "OPA_CS2"]
pub mod opa_cs2;
#[doc = "OPA_CS3 register accessor: an alias for `Reg<OPA_CS3_SPEC>`"]
pub type OPA_CS3 = crate::Reg<opa_cs3::OPA_CS3_SPEC>;
#[doc = "OPA_CS3"]
pub mod opa_cs3;
#[doc = "OPA_CS4 register accessor: an alias for `Reg<OPA_CS4_SPEC>`"]
pub type OPA_CS4 = crate::Reg<opa_cs4::OPA_CS4_SPEC>;
#[doc = "OPA_CS4"]
pub mod opa_cs4;
#[doc = "OPA_LOCK register accessor: an alias for `Reg<OPA_LOCK_SPEC>`"]
pub type OPA_LOCK = crate::Reg<opa_lock::OPA_LOCK_SPEC>;
#[doc = "OPA_LOCK"]
pub mod opa_lock;
