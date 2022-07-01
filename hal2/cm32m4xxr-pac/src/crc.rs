#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC32DAT"]
    pub crc32dat: crate::Reg<crc32dat::CRC32DAT_SPEC>,
    #[doc = "0x04 - CRC32IDAT"]
    pub crc32idat: crate::Reg<crc32idat::CRC32IDAT_SPEC>,
    #[doc = "0x08 - CRC32CTRL"]
    pub crc32ctrl: crate::Reg<crc32ctrl::CRC32CTRL_SPEC>,
    #[doc = "0x0c - CRC16CTRL"]
    pub crc16ctrl: crate::Reg<crc16ctrl::CRC16CTRL_SPEC>,
    #[doc = "0x10 - CRC16DAT"]
    pub crc16dat: crate::Reg<crc16dat::CRC16DAT_SPEC>,
    #[doc = "0x14 - CRC16D"]
    pub crc16d: crate::Reg<crc16d::CRC16D_SPEC>,
    #[doc = "0x18 - LRC"]
    pub lrc: crate::Reg<lrc::LRC_SPEC>,
}
#[doc = "CRC32DAT register accessor: an alias for `Reg<CRC32DAT_SPEC>`"]
pub type CRC32DAT = crate::Reg<crc32dat::CRC32DAT_SPEC>;
#[doc = "CRC32DAT"]
pub mod crc32dat;
#[doc = "CRC32IDAT register accessor: an alias for `Reg<CRC32IDAT_SPEC>`"]
pub type CRC32IDAT = crate::Reg<crc32idat::CRC32IDAT_SPEC>;
#[doc = "CRC32IDAT"]
pub mod crc32idat;
#[doc = "CRC32CTRL register accessor: an alias for `Reg<CRC32CTRL_SPEC>`"]
pub type CRC32CTRL = crate::Reg<crc32ctrl::CRC32CTRL_SPEC>;
#[doc = "CRC32CTRL"]
pub mod crc32ctrl;
#[doc = "CRC16CTRL register accessor: an alias for `Reg<CRC16CTRL_SPEC>`"]
pub type CRC16CTRL = crate::Reg<crc16ctrl::CRC16CTRL_SPEC>;
#[doc = "CRC16CTRL"]
pub mod crc16ctrl;
#[doc = "CRC16DAT register accessor: an alias for `Reg<CRC16DAT_SPEC>`"]
pub type CRC16DAT = crate::Reg<crc16dat::CRC16DAT_SPEC>;
#[doc = "CRC16DAT"]
pub mod crc16dat;
#[doc = "CRC16D register accessor: an alias for `Reg<CRC16D_SPEC>`"]
pub type CRC16D = crate::Reg<crc16d::CRC16D_SPEC>;
#[doc = "CRC16D"]
pub mod crc16d;
#[doc = "LRC register accessor: an alias for `Reg<LRC_SPEC>`"]
pub type LRC = crate::Reg<lrc::LRC_SPEC>;
#[doc = "LRC"]
pub mod lrc;
