#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI_CTRL1"]
    pub spi_ctrl1: crate::Reg<spi_ctrl1::SPI_CTRL1_SPEC>,
    #[doc = "0x04 - SPI_CTRL2"]
    pub spi_ctrl2: crate::Reg<spi_ctrl2::SPI_CTRL2_SPEC>,
    #[doc = "0x08 - SPI_STS"]
    pub spi_sts: crate::Reg<spi_sts::SPI_STS_SPEC>,
    #[doc = "0x0c - SPI_DAT"]
    pub spi_dat: crate::Reg<spi_dat::SPI_DAT_SPEC>,
    #[doc = "0x10 - SPI_CRCPOLY"]
    pub spi_crcpoly: crate::Reg<spi_crcpoly::SPI_CRCPOLY_SPEC>,
    #[doc = "0x14 - SPI_CRCRDAT"]
    pub spi_crcrdat: crate::Reg<spi_crcrdat::SPI_CRCRDAT_SPEC>,
    #[doc = "0x18 - SPI_CRCTDAT"]
    pub spi_crctdat: crate::Reg<spi_crctdat::SPI_CRCTDAT_SPEC>,
    #[doc = "0x1c - SPI_I2SCFG"]
    pub spi_i2scfg: crate::Reg<spi_i2scfg::SPI_I2SCFG_SPEC>,
    #[doc = "0x20 - SPI_I2SPREDIV"]
    pub spi_i2sprediv: crate::Reg<spi_i2sprediv::SPI_I2SPREDIV_SPEC>,
}
#[doc = "SPI_CTRL1 register accessor: an alias for `Reg<SPI_CTRL1_SPEC>`"]
pub type SPI_CTRL1 = crate::Reg<spi_ctrl1::SPI_CTRL1_SPEC>;
#[doc = "SPI_CTRL1"]
pub mod spi_ctrl1;
#[doc = "SPI_CTRL2 register accessor: an alias for `Reg<SPI_CTRL2_SPEC>`"]
pub type SPI_CTRL2 = crate::Reg<spi_ctrl2::SPI_CTRL2_SPEC>;
#[doc = "SPI_CTRL2"]
pub mod spi_ctrl2;
#[doc = "SPI_STS register accessor: an alias for `Reg<SPI_STS_SPEC>`"]
pub type SPI_STS = crate::Reg<spi_sts::SPI_STS_SPEC>;
#[doc = "SPI_STS"]
pub mod spi_sts;
#[doc = "SPI_DAT register accessor: an alias for `Reg<SPI_DAT_SPEC>`"]
pub type SPI_DAT = crate::Reg<spi_dat::SPI_DAT_SPEC>;
#[doc = "SPI_DAT"]
pub mod spi_dat;
#[doc = "SPI_CRCPOLY register accessor: an alias for `Reg<SPI_CRCPOLY_SPEC>`"]
pub type SPI_CRCPOLY = crate::Reg<spi_crcpoly::SPI_CRCPOLY_SPEC>;
#[doc = "SPI_CRCPOLY"]
pub mod spi_crcpoly;
#[doc = "SPI_CRCRDAT register accessor: an alias for `Reg<SPI_CRCRDAT_SPEC>`"]
pub type SPI_CRCRDAT = crate::Reg<spi_crcrdat::SPI_CRCRDAT_SPEC>;
#[doc = "SPI_CRCRDAT"]
pub mod spi_crcrdat;
#[doc = "SPI_CRCTDAT register accessor: an alias for `Reg<SPI_CRCTDAT_SPEC>`"]
pub type SPI_CRCTDAT = crate::Reg<spi_crctdat::SPI_CRCTDAT_SPEC>;
#[doc = "SPI_CRCTDAT"]
pub mod spi_crctdat;
#[doc = "SPI_I2SCFG register accessor: an alias for `Reg<SPI_I2SCFG_SPEC>`"]
pub type SPI_I2SCFG = crate::Reg<spi_i2scfg::SPI_I2SCFG_SPEC>;
#[doc = "SPI_I2SCFG"]
pub mod spi_i2scfg;
#[doc = "SPI_I2SPREDIV register accessor: an alias for `Reg<SPI_I2SPREDIV_SPEC>`"]
pub type SPI_I2SPREDIV = crate::Reg<spi_i2sprediv::SPI_I2SPREDIV_SPEC>;
#[doc = "SPI_I2SPREDIV"]
pub mod spi_i2sprediv;
