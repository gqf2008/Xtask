#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA_INTSTS"]
    pub dma_intsts: crate::Reg<dma_intsts::DMA_INTSTS_SPEC>,
    #[doc = "0x04 - DMA_INTCLR"]
    pub dma_intclr: crate::Reg<dma_intclr::DMA_INTCLR_SPEC>,
    #[doc = "0x08 - DMA_CHCFG1"]
    pub dma_chcfg1: crate::Reg<dma_chcfg1::DMA_CHCFG1_SPEC>,
    #[doc = "0x0c - DMA_TXNUM1"]
    pub dma_txnum1: crate::Reg<dma_txnum1::DMA_TXNUM1_SPEC>,
    #[doc = "0x10 - DMA_PADDR1"]
    pub dma_paddr1: crate::Reg<dma_paddr1::DMA_PADDR1_SPEC>,
    #[doc = "0x14 - DMA_MADDR1"]
    pub dma_maddr1: crate::Reg<dma_maddr1::DMA_MADDR1_SPEC>,
    #[doc = "0x18 - DMA_CHSEL1"]
    pub dma_chsel1: crate::Reg<dma_chsel1::DMA_CHSEL1_SPEC>,
    #[doc = "0x1c - DMA_CHCFG2"]
    pub dma_chcfg2: crate::Reg<dma_chcfg2::DMA_CHCFG2_SPEC>,
    #[doc = "0x20 - DMA_TXNUM2"]
    pub dma_txnum2: crate::Reg<dma_txnum2::DMA_TXNUM2_SPEC>,
    #[doc = "0x24 - DMA_PADDR2"]
    pub dma_paddr2: crate::Reg<dma_paddr2::DMA_PADDR2_SPEC>,
    #[doc = "0x28 - DMA_MADDR2"]
    pub dma_maddr2: crate::Reg<dma_maddr2::DMA_MADDR2_SPEC>,
    #[doc = "0x2c - DMA_CHSEL2"]
    pub dma_chsel2: crate::Reg<dma_chsel2::DMA_CHSEL2_SPEC>,
    #[doc = "0x30 - DMA_CHCFG3"]
    pub dma_chcfg3: crate::Reg<dma_chcfg3::DMA_CHCFG3_SPEC>,
    #[doc = "0x34 - DMA_TXNUM3"]
    pub dma_txnum3: crate::Reg<dma_txnum3::DMA_TXNUM3_SPEC>,
    #[doc = "0x38 - DMA_PADDR3"]
    pub dma_paddr3: crate::Reg<dma_paddr3::DMA_PADDR3_SPEC>,
    #[doc = "0x3c - DMA_MADDR3"]
    pub dma_maddr3: crate::Reg<dma_maddr3::DMA_MADDR3_SPEC>,
    #[doc = "0x40 - DMA_CHSEL3"]
    pub dma_chsel3: crate::Reg<dma_chsel3::DMA_CHSEL3_SPEC>,
    #[doc = "0x44 - DMA_CHCFG4"]
    pub dma_chcfg4: crate::Reg<dma_chcfg4::DMA_CHCFG4_SPEC>,
    #[doc = "0x48 - DMA_TXNUM4"]
    pub dma_txnum4: crate::Reg<dma_txnum4::DMA_TXNUM4_SPEC>,
    #[doc = "0x4c - DMA_PADDR4"]
    pub dma_paddr4: crate::Reg<dma_paddr4::DMA_PADDR4_SPEC>,
    #[doc = "0x50 - DMA_MADDR4"]
    pub dma_maddr4: crate::Reg<dma_maddr4::DMA_MADDR4_SPEC>,
    #[doc = "0x54 - DMA_CHSEL4"]
    pub dma_chsel4: crate::Reg<dma_chsel4::DMA_CHSEL4_SPEC>,
    #[doc = "0x58 - DMA_CHCFG5"]
    pub dma_chcfg5: crate::Reg<dma_chcfg5::DMA_CHCFG5_SPEC>,
    #[doc = "0x5c - DMA_TXNUM5"]
    pub dma_txnum5: crate::Reg<dma_txnum5::DMA_TXNUM5_SPEC>,
    #[doc = "0x60 - DMA_PADDR5"]
    pub dma_paddr5: crate::Reg<dma_paddr5::DMA_PADDR5_SPEC>,
    #[doc = "0x64 - DMA_MADDR5"]
    pub dma_maddr5: crate::Reg<dma_maddr5::DMA_MADDR5_SPEC>,
    #[doc = "0x68 - DMA_CHSEL5"]
    pub dma_chsel5: crate::Reg<dma_chsel5::DMA_CHSEL5_SPEC>,
    #[doc = "0x6c - DMA_CHCFG6"]
    pub dma_chcfg6: crate::Reg<dma_chcfg6::DMA_CHCFG6_SPEC>,
    #[doc = "0x70 - DMA_TXNUM6"]
    pub dma_txnum6: crate::Reg<dma_txnum6::DMA_TXNUM6_SPEC>,
    #[doc = "0x74 - DMA_PADDR6"]
    pub dma_paddr6: crate::Reg<dma_paddr6::DMA_PADDR6_SPEC>,
    #[doc = "0x78 - DMA_MADDR6"]
    pub dma_maddr6: crate::Reg<dma_maddr6::DMA_MADDR6_SPEC>,
    #[doc = "0x7c - DMA_CHSEL6"]
    pub dma_chsel6: crate::Reg<dma_chsel6::DMA_CHSEL6_SPEC>,
    #[doc = "0x80 - DMA_CHCFG7"]
    pub dma_chcfg7: crate::Reg<dma_chcfg7::DMA_CHCFG7_SPEC>,
    #[doc = "0x84 - DMA_TXNUM7"]
    pub dma_txnum7: crate::Reg<dma_txnum7::DMA_TXNUM7_SPEC>,
    #[doc = "0x88 - DMA_PADDR7"]
    pub dma_paddr7: crate::Reg<dma_paddr7::DMA_PADDR7_SPEC>,
    #[doc = "0x8c - DMA_MADDR7"]
    pub dma_maddr7: crate::Reg<dma_maddr7::DMA_MADDR7_SPEC>,
    #[doc = "0x90 - DMA_CHSEL7"]
    pub dma_chsel7: crate::Reg<dma_chsel7::DMA_CHSEL7_SPEC>,
    #[doc = "0x94 - DMA_CHCFG8"]
    pub dma_chcfg8: crate::Reg<dma_chcfg8::DMA_CHCFG8_SPEC>,
    #[doc = "0x98 - DMA_TXNUM8"]
    pub dma_txnum8: crate::Reg<dma_txnum8::DMA_TXNUM8_SPEC>,
    #[doc = "0x9c - DMA_PADDR8"]
    pub dma_paddr8: crate::Reg<dma_paddr8::DMA_PADDR8_SPEC>,
    #[doc = "0xa0 - DMA_MADDR8"]
    pub dma_maddr8: crate::Reg<dma_maddr8::DMA_MADDR8_SPEC>,
    #[doc = "0xa4 - DMA_CHSEL8"]
    pub dma_chsel8: crate::Reg<dma_chsel8::DMA_CHSEL8_SPEC>,
    #[doc = "0xa8 - DMA_CHMAPEN"]
    pub dma_chmapen: crate::Reg<dma_chmapen::DMA_CHMAPEN_SPEC>,
}
#[doc = "DMA_INTSTS register accessor: an alias for `Reg<DMA_INTSTS_SPEC>`"]
pub type DMA_INTSTS = crate::Reg<dma_intsts::DMA_INTSTS_SPEC>;
#[doc = "DMA_INTSTS"]
pub mod dma_intsts;
#[doc = "DMA_INTCLR register accessor: an alias for `Reg<DMA_INTCLR_SPEC>`"]
pub type DMA_INTCLR = crate::Reg<dma_intclr::DMA_INTCLR_SPEC>;
#[doc = "DMA_INTCLR"]
pub mod dma_intclr;
#[doc = "DMA_CHCFG1 register accessor: an alias for `Reg<DMA_CHCFG1_SPEC>`"]
pub type DMA_CHCFG1 = crate::Reg<dma_chcfg1::DMA_CHCFG1_SPEC>;
#[doc = "DMA_CHCFG1"]
pub mod dma_chcfg1;
#[doc = "DMA_TXNUM1 register accessor: an alias for `Reg<DMA_TXNUM1_SPEC>`"]
pub type DMA_TXNUM1 = crate::Reg<dma_txnum1::DMA_TXNUM1_SPEC>;
#[doc = "DMA_TXNUM1"]
pub mod dma_txnum1;
#[doc = "DMA_PADDR1 register accessor: an alias for `Reg<DMA_PADDR1_SPEC>`"]
pub type DMA_PADDR1 = crate::Reg<dma_paddr1::DMA_PADDR1_SPEC>;
#[doc = "DMA_PADDR1"]
pub mod dma_paddr1;
#[doc = "DMA_MADDR1 register accessor: an alias for `Reg<DMA_MADDR1_SPEC>`"]
pub type DMA_MADDR1 = crate::Reg<dma_maddr1::DMA_MADDR1_SPEC>;
#[doc = "DMA_MADDR1"]
pub mod dma_maddr1;
#[doc = "DMA_CHSEL1 register accessor: an alias for `Reg<DMA_CHSEL1_SPEC>`"]
pub type DMA_CHSEL1 = crate::Reg<dma_chsel1::DMA_CHSEL1_SPEC>;
#[doc = "DMA_CHSEL1"]
pub mod dma_chsel1;
#[doc = "DMA_CHCFG2 register accessor: an alias for `Reg<DMA_CHCFG2_SPEC>`"]
pub type DMA_CHCFG2 = crate::Reg<dma_chcfg2::DMA_CHCFG2_SPEC>;
#[doc = "DMA_CHCFG2"]
pub mod dma_chcfg2;
#[doc = "DMA_TXNUM2 register accessor: an alias for `Reg<DMA_TXNUM2_SPEC>`"]
pub type DMA_TXNUM2 = crate::Reg<dma_txnum2::DMA_TXNUM2_SPEC>;
#[doc = "DMA_TXNUM2"]
pub mod dma_txnum2;
#[doc = "DMA_PADDR2 register accessor: an alias for `Reg<DMA_PADDR2_SPEC>`"]
pub type DMA_PADDR2 = crate::Reg<dma_paddr2::DMA_PADDR2_SPEC>;
#[doc = "DMA_PADDR2"]
pub mod dma_paddr2;
#[doc = "DMA_MADDR2 register accessor: an alias for `Reg<DMA_MADDR2_SPEC>`"]
pub type DMA_MADDR2 = crate::Reg<dma_maddr2::DMA_MADDR2_SPEC>;
#[doc = "DMA_MADDR2"]
pub mod dma_maddr2;
#[doc = "DMA_CHSEL2 register accessor: an alias for `Reg<DMA_CHSEL2_SPEC>`"]
pub type DMA_CHSEL2 = crate::Reg<dma_chsel2::DMA_CHSEL2_SPEC>;
#[doc = "DMA_CHSEL2"]
pub mod dma_chsel2;
#[doc = "DMA_CHCFG3 register accessor: an alias for `Reg<DMA_CHCFG3_SPEC>`"]
pub type DMA_CHCFG3 = crate::Reg<dma_chcfg3::DMA_CHCFG3_SPEC>;
#[doc = "DMA_CHCFG3"]
pub mod dma_chcfg3;
#[doc = "DMA_TXNUM3 register accessor: an alias for `Reg<DMA_TXNUM3_SPEC>`"]
pub type DMA_TXNUM3 = crate::Reg<dma_txnum3::DMA_TXNUM3_SPEC>;
#[doc = "DMA_TXNUM3"]
pub mod dma_txnum3;
#[doc = "DMA_PADDR3 register accessor: an alias for `Reg<DMA_PADDR3_SPEC>`"]
pub type DMA_PADDR3 = crate::Reg<dma_paddr3::DMA_PADDR3_SPEC>;
#[doc = "DMA_PADDR3"]
pub mod dma_paddr3;
#[doc = "DMA_MADDR3 register accessor: an alias for `Reg<DMA_MADDR3_SPEC>`"]
pub type DMA_MADDR3 = crate::Reg<dma_maddr3::DMA_MADDR3_SPEC>;
#[doc = "DMA_MADDR3"]
pub mod dma_maddr3;
#[doc = "DMA_CHSEL3 register accessor: an alias for `Reg<DMA_CHSEL3_SPEC>`"]
pub type DMA_CHSEL3 = crate::Reg<dma_chsel3::DMA_CHSEL3_SPEC>;
#[doc = "DMA_CHSEL3"]
pub mod dma_chsel3;
#[doc = "DMA_CHCFG4 register accessor: an alias for `Reg<DMA_CHCFG4_SPEC>`"]
pub type DMA_CHCFG4 = crate::Reg<dma_chcfg4::DMA_CHCFG4_SPEC>;
#[doc = "DMA_CHCFG4"]
pub mod dma_chcfg4;
#[doc = "DMA_TXNUM4 register accessor: an alias for `Reg<DMA_TXNUM4_SPEC>`"]
pub type DMA_TXNUM4 = crate::Reg<dma_txnum4::DMA_TXNUM4_SPEC>;
#[doc = "DMA_TXNUM4"]
pub mod dma_txnum4;
#[doc = "DMA_PADDR4 register accessor: an alias for `Reg<DMA_PADDR4_SPEC>`"]
pub type DMA_PADDR4 = crate::Reg<dma_paddr4::DMA_PADDR4_SPEC>;
#[doc = "DMA_PADDR4"]
pub mod dma_paddr4;
#[doc = "DMA_MADDR4 register accessor: an alias for `Reg<DMA_MADDR4_SPEC>`"]
pub type DMA_MADDR4 = crate::Reg<dma_maddr4::DMA_MADDR4_SPEC>;
#[doc = "DMA_MADDR4"]
pub mod dma_maddr4;
#[doc = "DMA_CHSEL4 register accessor: an alias for `Reg<DMA_CHSEL4_SPEC>`"]
pub type DMA_CHSEL4 = crate::Reg<dma_chsel4::DMA_CHSEL4_SPEC>;
#[doc = "DMA_CHSEL4"]
pub mod dma_chsel4;
#[doc = "DMA_CHCFG5 register accessor: an alias for `Reg<DMA_CHCFG5_SPEC>`"]
pub type DMA_CHCFG5 = crate::Reg<dma_chcfg5::DMA_CHCFG5_SPEC>;
#[doc = "DMA_CHCFG5"]
pub mod dma_chcfg5;
#[doc = "DMA_TXNUM5 register accessor: an alias for `Reg<DMA_TXNUM5_SPEC>`"]
pub type DMA_TXNUM5 = crate::Reg<dma_txnum5::DMA_TXNUM5_SPEC>;
#[doc = "DMA_TXNUM5"]
pub mod dma_txnum5;
#[doc = "DMA_PADDR5 register accessor: an alias for `Reg<DMA_PADDR5_SPEC>`"]
pub type DMA_PADDR5 = crate::Reg<dma_paddr5::DMA_PADDR5_SPEC>;
#[doc = "DMA_PADDR5"]
pub mod dma_paddr5;
#[doc = "DMA_MADDR5 register accessor: an alias for `Reg<DMA_MADDR5_SPEC>`"]
pub type DMA_MADDR5 = crate::Reg<dma_maddr5::DMA_MADDR5_SPEC>;
#[doc = "DMA_MADDR5"]
pub mod dma_maddr5;
#[doc = "DMA_CHSEL5 register accessor: an alias for `Reg<DMA_CHSEL5_SPEC>`"]
pub type DMA_CHSEL5 = crate::Reg<dma_chsel5::DMA_CHSEL5_SPEC>;
#[doc = "DMA_CHSEL5"]
pub mod dma_chsel5;
#[doc = "DMA_CHCFG6 register accessor: an alias for `Reg<DMA_CHCFG6_SPEC>`"]
pub type DMA_CHCFG6 = crate::Reg<dma_chcfg6::DMA_CHCFG6_SPEC>;
#[doc = "DMA_CHCFG6"]
pub mod dma_chcfg6;
#[doc = "DMA_TXNUM6 register accessor: an alias for `Reg<DMA_TXNUM6_SPEC>`"]
pub type DMA_TXNUM6 = crate::Reg<dma_txnum6::DMA_TXNUM6_SPEC>;
#[doc = "DMA_TXNUM6"]
pub mod dma_txnum6;
#[doc = "DMA_PADDR6 register accessor: an alias for `Reg<DMA_PADDR6_SPEC>`"]
pub type DMA_PADDR6 = crate::Reg<dma_paddr6::DMA_PADDR6_SPEC>;
#[doc = "DMA_PADDR6"]
pub mod dma_paddr6;
#[doc = "DMA_MADDR6 register accessor: an alias for `Reg<DMA_MADDR6_SPEC>`"]
pub type DMA_MADDR6 = crate::Reg<dma_maddr6::DMA_MADDR6_SPEC>;
#[doc = "DMA_MADDR6"]
pub mod dma_maddr6;
#[doc = "DMA_CHSEL6 register accessor: an alias for `Reg<DMA_CHSEL6_SPEC>`"]
pub type DMA_CHSEL6 = crate::Reg<dma_chsel6::DMA_CHSEL6_SPEC>;
#[doc = "DMA_CHSEL6"]
pub mod dma_chsel6;
#[doc = "DMA_CHCFG7 register accessor: an alias for `Reg<DMA_CHCFG7_SPEC>`"]
pub type DMA_CHCFG7 = crate::Reg<dma_chcfg7::DMA_CHCFG7_SPEC>;
#[doc = "DMA_CHCFG7"]
pub mod dma_chcfg7;
#[doc = "DMA_TXNUM7 register accessor: an alias for `Reg<DMA_TXNUM7_SPEC>`"]
pub type DMA_TXNUM7 = crate::Reg<dma_txnum7::DMA_TXNUM7_SPEC>;
#[doc = "DMA_TXNUM7"]
pub mod dma_txnum7;
#[doc = "DMA_PADDR7 register accessor: an alias for `Reg<DMA_PADDR7_SPEC>`"]
pub type DMA_PADDR7 = crate::Reg<dma_paddr7::DMA_PADDR7_SPEC>;
#[doc = "DMA_PADDR7"]
pub mod dma_paddr7;
#[doc = "DMA_MADDR7 register accessor: an alias for `Reg<DMA_MADDR7_SPEC>`"]
pub type DMA_MADDR7 = crate::Reg<dma_maddr7::DMA_MADDR7_SPEC>;
#[doc = "DMA_MADDR7"]
pub mod dma_maddr7;
#[doc = "DMA_CHSEL7 register accessor: an alias for `Reg<DMA_CHSEL7_SPEC>`"]
pub type DMA_CHSEL7 = crate::Reg<dma_chsel7::DMA_CHSEL7_SPEC>;
#[doc = "DMA_CHSEL7"]
pub mod dma_chsel7;
#[doc = "DMA_CHCFG8 register accessor: an alias for `Reg<DMA_CHCFG8_SPEC>`"]
pub type DMA_CHCFG8 = crate::Reg<dma_chcfg8::DMA_CHCFG8_SPEC>;
#[doc = "DMA_CHCFG8"]
pub mod dma_chcfg8;
#[doc = "DMA_TXNUM8 register accessor: an alias for `Reg<DMA_TXNUM8_SPEC>`"]
pub type DMA_TXNUM8 = crate::Reg<dma_txnum8::DMA_TXNUM8_SPEC>;
#[doc = "DMA_TXNUM8"]
pub mod dma_txnum8;
#[doc = "DMA_PADDR8 register accessor: an alias for `Reg<DMA_PADDR8_SPEC>`"]
pub type DMA_PADDR8 = crate::Reg<dma_paddr8::DMA_PADDR8_SPEC>;
#[doc = "DMA_PADDR8"]
pub mod dma_paddr8;
#[doc = "DMA_MADDR8 register accessor: an alias for `Reg<DMA_MADDR8_SPEC>`"]
pub type DMA_MADDR8 = crate::Reg<dma_maddr8::DMA_MADDR8_SPEC>;
#[doc = "DMA_MADDR8"]
pub mod dma_maddr8;
#[doc = "DMA_CHSEL8 register accessor: an alias for `Reg<DMA_CHSEL8_SPEC>`"]
pub type DMA_CHSEL8 = crate::Reg<dma_chsel8::DMA_CHSEL8_SPEC>;
#[doc = "DMA_CHSEL8"]
pub mod dma_chsel8;
#[doc = "DMA_CHMAPEN register accessor: an alias for `Reg<DMA_CHMAPEN_SPEC>`"]
pub type DMA_CHMAPEN = crate::Reg<dma_chmapen::DMA_CHMAPEN_SPEC>;
#[doc = "DMA_CHMAPEN"]
pub mod dma_chmapen;
