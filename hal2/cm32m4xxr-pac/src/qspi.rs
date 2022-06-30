#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - QSPI_CTRL0"]
    pub qspi_ctrl0: crate::Reg<qspi_ctrl0::QSPI_CTRL0_SPEC>,
    #[doc = "0x04 - QSPI_CTRL1"]
    pub qspi_ctrl1: crate::Reg<qspi_ctrl1::QSPI_CTRL1_SPEC>,
    #[doc = "0x08 - QSPI_EN"]
    pub qspi_en: crate::Reg<qspi_en::QSPI_EN_SPEC>,
    #[doc = "0x0c - QSPI_MW_CTRL"]
    pub qspi_mw_ctrl: crate::Reg<qspi_mw_ctrl::QSPI_MW_CTRL_SPEC>,
    #[doc = "0x10 - QSPI_SLAVE_EN"]
    pub qspi_slave_en: crate::Reg<qspi_slave_en::QSPI_SLAVE_EN_SPEC>,
    #[doc = "0x14 - QSPI_BAUD"]
    pub qspi_baud: crate::Reg<qspi_baud::QSPI_BAUD_SPEC>,
    #[doc = "0x18 - QSPI_TXFT"]
    pub qspi_txft: crate::Reg<qspi_txft::QSPI_TXFT_SPEC>,
    #[doc = "0x1c - QSPI_RXFT"]
    pub qspi_rxft: crate::Reg<qspi_rxft::QSPI_RXFT_SPEC>,
    #[doc = "0x20 - QSPI_TXFN"]
    pub qspi_txfn: crate::Reg<qspi_txfn::QSPI_TXFN_SPEC>,
    #[doc = "0x24 - QSPI_RXFN"]
    pub qspi_rxfn: crate::Reg<qspi_rxfn::QSPI_RXFN_SPEC>,
    #[doc = "0x28 - QSPI_STS"]
    pub qspi_sts: crate::Reg<qspi_sts::QSPI_STS_SPEC>,
    #[doc = "0x2c - QSPI_IMASK"]
    pub qspi_imask: crate::Reg<qspi_imask::QSPI_IMASK_SPEC>,
    #[doc = "0x30 - QSPI_ISTS"]
    pub qspi_ists: crate::Reg<qspi_ists::QSPI_ISTS_SPEC>,
    #[doc = "0x34 - QSPI_RISTS"]
    pub qspi_rists: crate::Reg<qspi_rists::QSPI_RISTS_SPEC>,
    #[doc = "0x38 - QSPI_TXFOI_CLR"]
    pub qspi_txfoi_clr: crate::Reg<qspi_txfoi_clr::QSPI_TXFOI_CLR_SPEC>,
    #[doc = "0x3c - QSPI_RXFOI_CLR"]
    pub qspi_rxfoi_clr: crate::Reg<qspi_rxfoi_clr::QSPI_RXFOI_CLR_SPEC>,
    #[doc = "0x40 - QSPI_RXFUI_CLR"]
    pub qspi_rxfui_clr: crate::Reg<qspi_rxfui_clr::QSPI_RXFUI_CLR_SPEC>,
    #[doc = "0x44 - QSPI_MMC_CLR"]
    pub qspi_mmc_clr: crate::Reg<qspi_mmc_clr::QSPI_MMC_CLR_SPEC>,
    #[doc = "0x48 - QSPI_ICLR"]
    pub qspi_iclr: crate::Reg<qspi_iclr::QSPI_ICLR_SPEC>,
    #[doc = "0x4c - QSPI_DMA_CTRL"]
    pub qspi_dma_ctrl: crate::Reg<qspi_dma_ctrl::QSPI_DMA_CTRL_SPEC>,
    #[doc = "0x50 - QSPI_DMATDL_CTRL"]
    pub qspi_dmatdl_ctrl: crate::Reg<qspi_dmatdl_ctrl::QSPI_DMATDL_CTRL_SPEC>,
    #[doc = "0x54 - QSPI_DMARDL_CTRL"]
    pub qspi_dmardl_ctrl: crate::Reg<qspi_dmardl_ctrl::QSPI_DMARDL_CTRL_SPEC>,
    #[doc = "0x58 - QSPI_IDCODE"]
    pub qspi_idcode: crate::Reg<qspi_idcode::QSPI_IDCODE_SPEC>,
    _reserved23: [u8; 0x04],
    #[doc = "0x60 - QSPI_DATx"]
    pub qspi_datx: crate::Reg<qspi_datx::QSPI_DATX_SPEC>,
    _reserved24: [u8; 0x8c],
    #[doc = "0xf0 - QSPI_RS_DELAY"]
    pub qspi_rs_delay: crate::Reg<qspi_rs_delay::QSPI_RS_DELAY_SPEC>,
    #[doc = "0xf4 - QSPI_ENH_CTRL0"]
    pub qspi_enh_ctrl0: crate::Reg<qspi_enh_ctrl0::QSPI_ENH_CTRL0_SPEC>,
    #[doc = "0xf8 - QSPI_DDR_TXDE"]
    pub qspi_ddr_txde: crate::Reg<qspi_ddr_txde::QSPI_DDR_TXDE_SPEC>,
    #[doc = "0xfc - XIP_MODE"]
    pub xip_mode: crate::Reg<xip_mode::XIP_MODE_SPEC>,
    #[doc = "0x100 - XIP_INCR_TOC"]
    pub xip_incr_toc: crate::Reg<xip_incr_toc::XIP_INCR_TOC_SPEC>,
    #[doc = "0x104 - XIP_WRAP_TOC"]
    pub xip_wrap_toc: crate::Reg<xip_wrap_toc::XIP_WRAP_TOC_SPEC>,
    #[doc = "0x108 - XIP_CTRL"]
    pub xip_ctrl: crate::Reg<xip_ctrl::XIP_CTRL_SPEC>,
    #[doc = "0x10c - XIP_SLAVE_EN"]
    pub xip_slave_en: crate::Reg<xip_slave_en::XIP_SLAVE_EN_SPEC>,
    #[doc = "0x110 - XIP_RXFOI_CLR"]
    pub xip_rxfoi_clr: crate::Reg<xip_rxfoi_clr::XIP_RXFOI_CLR_SPEC>,
    #[doc = "0x114 - XIP_TOUT"]
    pub xip_tout: crate::Reg<xip_tout::XIP_TOUT_SPEC>,
}
#[doc = "QSPI_CTRL0 register accessor: an alias for `Reg<QSPI_CTRL0_SPEC>`"]
pub type QSPI_CTRL0 = crate::Reg<qspi_ctrl0::QSPI_CTRL0_SPEC>;
#[doc = "QSPI_CTRL0"]
pub mod qspi_ctrl0;
#[doc = "QSPI_CTRL1 register accessor: an alias for `Reg<QSPI_CTRL1_SPEC>`"]
pub type QSPI_CTRL1 = crate::Reg<qspi_ctrl1::QSPI_CTRL1_SPEC>;
#[doc = "QSPI_CTRL1"]
pub mod qspi_ctrl1;
#[doc = "QSPI_EN register accessor: an alias for `Reg<QSPI_EN_SPEC>`"]
pub type QSPI_EN = crate::Reg<qspi_en::QSPI_EN_SPEC>;
#[doc = "QSPI_EN"]
pub mod qspi_en;
#[doc = "QSPI_MW_CTRL register accessor: an alias for `Reg<QSPI_MW_CTRL_SPEC>`"]
pub type QSPI_MW_CTRL = crate::Reg<qspi_mw_ctrl::QSPI_MW_CTRL_SPEC>;
#[doc = "QSPI_MW_CTRL"]
pub mod qspi_mw_ctrl;
#[doc = "QSPI_SLAVE_EN register accessor: an alias for `Reg<QSPI_SLAVE_EN_SPEC>`"]
pub type QSPI_SLAVE_EN = crate::Reg<qspi_slave_en::QSPI_SLAVE_EN_SPEC>;
#[doc = "QSPI_SLAVE_EN"]
pub mod qspi_slave_en;
#[doc = "QSPI_BAUD register accessor: an alias for `Reg<QSPI_BAUD_SPEC>`"]
pub type QSPI_BAUD = crate::Reg<qspi_baud::QSPI_BAUD_SPEC>;
#[doc = "QSPI_BAUD"]
pub mod qspi_baud;
#[doc = "QSPI_TXFT register accessor: an alias for `Reg<QSPI_TXFT_SPEC>`"]
pub type QSPI_TXFT = crate::Reg<qspi_txft::QSPI_TXFT_SPEC>;
#[doc = "QSPI_TXFT"]
pub mod qspi_txft;
#[doc = "QSPI_RXFT register accessor: an alias for `Reg<QSPI_RXFT_SPEC>`"]
pub type QSPI_RXFT = crate::Reg<qspi_rxft::QSPI_RXFT_SPEC>;
#[doc = "QSPI_RXFT"]
pub mod qspi_rxft;
#[doc = "QSPI_TXFN register accessor: an alias for `Reg<QSPI_TXFN_SPEC>`"]
pub type QSPI_TXFN = crate::Reg<qspi_txfn::QSPI_TXFN_SPEC>;
#[doc = "QSPI_TXFN"]
pub mod qspi_txfn;
#[doc = "QSPI_RXFN register accessor: an alias for `Reg<QSPI_RXFN_SPEC>`"]
pub type QSPI_RXFN = crate::Reg<qspi_rxfn::QSPI_RXFN_SPEC>;
#[doc = "QSPI_RXFN"]
pub mod qspi_rxfn;
#[doc = "QSPI_STS register accessor: an alias for `Reg<QSPI_STS_SPEC>`"]
pub type QSPI_STS = crate::Reg<qspi_sts::QSPI_STS_SPEC>;
#[doc = "QSPI_STS"]
pub mod qspi_sts;
#[doc = "QSPI_IMASK register accessor: an alias for `Reg<QSPI_IMASK_SPEC>`"]
pub type QSPI_IMASK = crate::Reg<qspi_imask::QSPI_IMASK_SPEC>;
#[doc = "QSPI_IMASK"]
pub mod qspi_imask;
#[doc = "QSPI_ISTS register accessor: an alias for `Reg<QSPI_ISTS_SPEC>`"]
pub type QSPI_ISTS = crate::Reg<qspi_ists::QSPI_ISTS_SPEC>;
#[doc = "QSPI_ISTS"]
pub mod qspi_ists;
#[doc = "QSPI_RISTS register accessor: an alias for `Reg<QSPI_RISTS_SPEC>`"]
pub type QSPI_RISTS = crate::Reg<qspi_rists::QSPI_RISTS_SPEC>;
#[doc = "QSPI_RISTS"]
pub mod qspi_rists;
#[doc = "QSPI_TXFOI_CLR register accessor: an alias for `Reg<QSPI_TXFOI_CLR_SPEC>`"]
pub type QSPI_TXFOI_CLR = crate::Reg<qspi_txfoi_clr::QSPI_TXFOI_CLR_SPEC>;
#[doc = "QSPI_TXFOI_CLR"]
pub mod qspi_txfoi_clr;
#[doc = "QSPI_RXFOI_CLR register accessor: an alias for `Reg<QSPI_RXFOI_CLR_SPEC>`"]
pub type QSPI_RXFOI_CLR = crate::Reg<qspi_rxfoi_clr::QSPI_RXFOI_CLR_SPEC>;
#[doc = "QSPI_RXFOI_CLR"]
pub mod qspi_rxfoi_clr;
#[doc = "QSPI_RXFUI_CLR register accessor: an alias for `Reg<QSPI_RXFUI_CLR_SPEC>`"]
pub type QSPI_RXFUI_CLR = crate::Reg<qspi_rxfui_clr::QSPI_RXFUI_CLR_SPEC>;
#[doc = "QSPI_RXFUI_CLR"]
pub mod qspi_rxfui_clr;
#[doc = "QSPI_MMC_CLR register accessor: an alias for `Reg<QSPI_MMC_CLR_SPEC>`"]
pub type QSPI_MMC_CLR = crate::Reg<qspi_mmc_clr::QSPI_MMC_CLR_SPEC>;
#[doc = "QSPI_MMC_CLR"]
pub mod qspi_mmc_clr;
#[doc = "QSPI_ICLR register accessor: an alias for `Reg<QSPI_ICLR_SPEC>`"]
pub type QSPI_ICLR = crate::Reg<qspi_iclr::QSPI_ICLR_SPEC>;
#[doc = "QSPI_ICLR"]
pub mod qspi_iclr;
#[doc = "QSPI_DMA_CTRL register accessor: an alias for `Reg<QSPI_DMA_CTRL_SPEC>`"]
pub type QSPI_DMA_CTRL = crate::Reg<qspi_dma_ctrl::QSPI_DMA_CTRL_SPEC>;
#[doc = "QSPI_DMA_CTRL"]
pub mod qspi_dma_ctrl;
#[doc = "QSPI_DMATDL_CTRL register accessor: an alias for `Reg<QSPI_DMATDL_CTRL_SPEC>`"]
pub type QSPI_DMATDL_CTRL = crate::Reg<qspi_dmatdl_ctrl::QSPI_DMATDL_CTRL_SPEC>;
#[doc = "QSPI_DMATDL_CTRL"]
pub mod qspi_dmatdl_ctrl;
#[doc = "QSPI_DMARDL_CTRL register accessor: an alias for `Reg<QSPI_DMARDL_CTRL_SPEC>`"]
pub type QSPI_DMARDL_CTRL = crate::Reg<qspi_dmardl_ctrl::QSPI_DMARDL_CTRL_SPEC>;
#[doc = "QSPI_DMARDL_CTRL"]
pub mod qspi_dmardl_ctrl;
#[doc = "QSPI_IDCODE register accessor: an alias for `Reg<QSPI_IDCODE_SPEC>`"]
pub type QSPI_IDCODE = crate::Reg<qspi_idcode::QSPI_IDCODE_SPEC>;
#[doc = "QSPI_IDCODE"]
pub mod qspi_idcode;
#[doc = "QSPI_DATx register accessor: an alias for `Reg<QSPI_DATX_SPEC>`"]
pub type QSPI_DATX = crate::Reg<qspi_datx::QSPI_DATX_SPEC>;
#[doc = "QSPI_DATx"]
pub mod qspi_datx;
#[doc = "QSPI_RS_DELAY register accessor: an alias for `Reg<QSPI_RS_DELAY_SPEC>`"]
pub type QSPI_RS_DELAY = crate::Reg<qspi_rs_delay::QSPI_RS_DELAY_SPEC>;
#[doc = "QSPI_RS_DELAY"]
pub mod qspi_rs_delay;
#[doc = "QSPI_ENH_CTRL0 register accessor: an alias for `Reg<QSPI_ENH_CTRL0_SPEC>`"]
pub type QSPI_ENH_CTRL0 = crate::Reg<qspi_enh_ctrl0::QSPI_ENH_CTRL0_SPEC>;
#[doc = "QSPI_ENH_CTRL0"]
pub mod qspi_enh_ctrl0;
#[doc = "QSPI_DDR_TXDE register accessor: an alias for `Reg<QSPI_DDR_TXDE_SPEC>`"]
pub type QSPI_DDR_TXDE = crate::Reg<qspi_ddr_txde::QSPI_DDR_TXDE_SPEC>;
#[doc = "QSPI_DDR_TXDE"]
pub mod qspi_ddr_txde;
#[doc = "XIP_MODE register accessor: an alias for `Reg<XIP_MODE_SPEC>`"]
pub type XIP_MODE = crate::Reg<xip_mode::XIP_MODE_SPEC>;
#[doc = "XIP_MODE"]
pub mod xip_mode;
#[doc = "XIP_INCR_TOC register accessor: an alias for `Reg<XIP_INCR_TOC_SPEC>`"]
pub type XIP_INCR_TOC = crate::Reg<xip_incr_toc::XIP_INCR_TOC_SPEC>;
#[doc = "XIP_INCR_TOC"]
pub mod xip_incr_toc;
#[doc = "XIP_WRAP_TOC register accessor: an alias for `Reg<XIP_WRAP_TOC_SPEC>`"]
pub type XIP_WRAP_TOC = crate::Reg<xip_wrap_toc::XIP_WRAP_TOC_SPEC>;
#[doc = "XIP_WRAP_TOC"]
pub mod xip_wrap_toc;
#[doc = "XIP_CTRL register accessor: an alias for `Reg<XIP_CTRL_SPEC>`"]
pub type XIP_CTRL = crate::Reg<xip_ctrl::XIP_CTRL_SPEC>;
#[doc = "XIP_CTRL"]
pub mod xip_ctrl;
#[doc = "XIP_SLAVE_EN register accessor: an alias for `Reg<XIP_SLAVE_EN_SPEC>`"]
pub type XIP_SLAVE_EN = crate::Reg<xip_slave_en::XIP_SLAVE_EN_SPEC>;
#[doc = "XIP_SLAVE_EN"]
pub mod xip_slave_en;
#[doc = "XIP_RXFOI_CLR register accessor: an alias for `Reg<XIP_RXFOI_CLR_SPEC>`"]
pub type XIP_RXFOI_CLR = crate::Reg<xip_rxfoi_clr::XIP_RXFOI_CLR_SPEC>;
#[doc = "XIP_RXFOI_CLR"]
pub mod xip_rxfoi_clr;
#[doc = "XIP_TOUT register accessor: an alias for `Reg<XIP_TOUT_SPEC>`"]
pub type XIP_TOUT = crate::Reg<xip_tout::XIP_TOUT_SPEC>;
#[doc = "XIP_TOUT"]
pub mod xip_tout;
