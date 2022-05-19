#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - host configuration register (HCTL)"]
    pub hctl: HCTL,
    #[doc = "0x04 - Host frame interval register"]
    pub hft: HFT,
    #[doc = "0x08 - FS host frame number/frame time remaining register (HFINFR)"]
    pub hfinfr: HFINFR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Host periodic transmit FIFO/queue status register (HPTFQSTAT)"]
    pub hptfqstat: HPTFQSTAT,
    #[doc = "0x14 - Host all channels interrupt register"]
    pub hachint: HACHINT,
    #[doc = "0x18 - host all channels interrupt mask register"]
    pub hachinten: HACHINTEN,
    _reserved6: [u8; 36usize],
    #[doc = "0x40 - Host port control and status register (USBFS_HPCS)"]
    pub hpcs: HPCS,
    _reserved7: [u8; 188usize],
    #[doc = "0x100 - host channel-0 characteristics register (HCH0CTL)"]
    pub hch0ctl: HCH0CTL,
    _reserved8: [u8; 4usize],
    #[doc = "0x108 - host channel-0 interrupt register (USBFS_HCHxINTF)"]
    pub hch0intf: HCH0INTF,
    #[doc = "0x10c - host channel-0 interrupt enable register (HCH0INTEN)"]
    pub hch0inten: HCH0INTEN,
    #[doc = "0x110 - host channel-0 transfer length register"]
    pub hch0len: HCH0LEN,
    _reserved11: [u8; 12usize],
    #[doc = "0x120 - host channel-1 characteristics register (HCH1CTL)"]
    pub hch1ctl: HCH1CTL,
    _reserved12: [u8; 4usize],
    #[doc = "0x128 - host channel-1 interrupt register (HCH1INTF)"]
    pub hch1intf: HCH1INTF,
    #[doc = "0x12c - host channel-1 interrupt enable register (HCH1INTEN)"]
    pub hch1inten: HCH1INTEN,
    #[doc = "0x130 - host channel-1 transfer length register"]
    pub hch1len: HCH1LEN,
    _reserved15: [u8; 12usize],
    #[doc = "0x140 - host channel-2 characteristics register (HCH2CTL)"]
    pub hch2ctl: HCH2CTL,
    _reserved16: [u8; 4usize],
    #[doc = "0x148 - host channel-2 interrupt register (HCH2INTF)"]
    pub hch2intf: HCH2INTF,
    #[doc = "0x14c - host channel-2 interrupt enable register (HCH2INTEN)"]
    pub hch2inten: HCH2INTEN,
    #[doc = "0x150 - host channel-2 transfer length register"]
    pub hch2len: HCH2LEN,
    _reserved19: [u8; 12usize],
    #[doc = "0x160 - host channel-3 characteristics register (HCH3CTL)"]
    pub hch3ctl: HCH3CTL,
    _reserved20: [u8; 4usize],
    #[doc = "0x168 - host channel-3 interrupt register (HCH3INTF)"]
    pub hch3intf: HCH3INTF,
    #[doc = "0x16c - host channel-3 interrupt enable register (HCH3INTEN)"]
    pub hch3inten: HCH3INTEN,
    #[doc = "0x170 - host channel-3 transfer length register"]
    pub hch3len: HCH3LEN,
    _reserved23: [u8; 12usize],
    #[doc = "0x180 - host channel-4 characteristics register (HCH4CTL)"]
    pub hch4ctl: HCH4CTL,
    _reserved24: [u8; 4usize],
    #[doc = "0x188 - host channel-4 interrupt register (HCH4INTF)"]
    pub hch4intf: HCH4INTF,
    #[doc = "0x18c - host channel-4 interrupt enable register (HCH4INTEN)"]
    pub hch4inten: HCH4INTEN,
    #[doc = "0x190 - host channel-4 transfer length register"]
    pub hch4len: HCH4LEN,
    _reserved27: [u8; 12usize],
    #[doc = "0x1a0 - host channel-5 characteristics register (HCH5CTL)"]
    pub hch5ctl: HCH5CTL,
    _reserved28: [u8; 4usize],
    #[doc = "0x1a8 - host channel-5 interrupt register (HCH5INTF)"]
    pub hch5intf: HCH5INTF,
    #[doc = "0x1ac - host channel-5 interrupt enable register (HCH5INTEN)"]
    pub hch5inten: HCH5INTEN,
    #[doc = "0x1b0 - host channel-5 transfer length register"]
    pub hch5len: HCH5LEN,
    _reserved31: [u8; 12usize],
    #[doc = "0x1c0 - host channel-6 characteristics register (HCH6CTL)"]
    pub hch6ctl: HCH6CTL,
    _reserved32: [u8; 4usize],
    #[doc = "0x1c8 - host channel-6 interrupt register (HCH6INTF)"]
    pub hch6intf: HCH6INTF,
    #[doc = "0x1cc - host channel-6 interrupt enable register (HCH6INTEN)"]
    pub hch6inten: HCH6INTEN,
    #[doc = "0x1d0 - host channel-6 transfer length register"]
    pub hch6len: HCH6LEN,
    _reserved35: [u8; 12usize],
    #[doc = "0x1e0 - host channel-7 characteristics register (HCH7CTL)"]
    pub hch7ctl: HCH7CTL,
    _reserved36: [u8; 4usize],
    #[doc = "0x1e8 - host channel-7 interrupt register (HCH7INTF)"]
    pub hch7intf: HCH7INTF,
    #[doc = "0x1ec - host channel-7 interrupt enable register (HCH7INTEN)"]
    pub hch7inten: HCH7INTEN,
    #[doc = "0x1f0 - host channel-7 transfer length register"]
    pub hch7len: HCH7LEN,
}
#[doc = "host configuration register (HCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctl](hctl) module"]
pub type HCTL = crate::Reg<u32, _HCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTL;
#[doc = "`read()` method returns [hctl::R](hctl::R) reader structure"]
impl crate::Readable for HCTL {}
#[doc = "`write(|w| ..)` method takes [hctl::W](hctl::W) writer structure"]
impl crate::Writable for HCTL {}
#[doc = "host configuration register (HCTL)"]
pub mod hctl;
#[doc = "Host frame interval register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hft](hft) module"]
pub type HFT = crate::Reg<u32, _HFT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFT;
#[doc = "`read()` method returns [hft::R](hft::R) reader structure"]
impl crate::Readable for HFT {}
#[doc = "`write(|w| ..)` method takes [hft::W](hft::W) writer structure"]
impl crate::Writable for HFT {}
#[doc = "Host frame interval register"]
pub mod hft;
#[doc = "FS host frame number/frame time remaining register (HFINFR)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfinfr](hfinfr) module"]
pub type HFINFR = crate::Reg<u32, _HFINFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFINFR;
#[doc = "`read()` method returns [hfinfr::R](hfinfr::R) reader structure"]
impl crate::Readable for HFINFR {}
#[doc = "FS host frame number/frame time remaining register (HFINFR)"]
pub mod hfinfr;
#[doc = "Host periodic transmit FIFO/queue status register (HPTFQSTAT)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hptfqstat](hptfqstat) module"]
pub type HPTFQSTAT = crate::Reg<u32, _HPTFQSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPTFQSTAT;
#[doc = "`read()` method returns [hptfqstat::R](hptfqstat::R) reader structure"]
impl crate::Readable for HPTFQSTAT {}
#[doc = "Host periodic transmit FIFO/queue status register (HPTFQSTAT)"]
pub mod hptfqstat;
#[doc = "Host all channels interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hachint](hachint) module"]
pub type HACHINT = crate::Reg<u32, _HACHINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HACHINT;
#[doc = "`read()` method returns [hachint::R](hachint::R) reader structure"]
impl crate::Readable for HACHINT {}
#[doc = "Host all channels interrupt register"]
pub mod hachint;
#[doc = "host all channels interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hachinten](hachinten) module"]
pub type HACHINTEN = crate::Reg<u32, _HACHINTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HACHINTEN;
#[doc = "`read()` method returns [hachinten::R](hachinten::R) reader structure"]
impl crate::Readable for HACHINTEN {}
#[doc = "`write(|w| ..)` method takes [hachinten::W](hachinten::W) writer structure"]
impl crate::Writable for HACHINTEN {}
#[doc = "host all channels interrupt mask register"]
pub mod hachinten;
#[doc = "Host port control and status register (USBFS_HPCS)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpcs](hpcs) module"]
pub type HPCS = crate::Reg<u32, _HPCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPCS;
#[doc = "`read()` method returns [hpcs::R](hpcs::R) reader structure"]
impl crate::Readable for HPCS {}
#[doc = "`write(|w| ..)` method takes [hpcs::W](hpcs::W) writer structure"]
impl crate::Writable for HPCS {}
#[doc = "Host port control and status register (USBFS_HPCS)"]
pub mod hpcs;
#[doc = "host channel-0 characteristics register (HCH0CTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch0ctl](hch0ctl) module"]
pub type HCH0CTL = crate::Reg<u32, _HCH0CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH0CTL;
#[doc = "`read()` method returns [hch0ctl::R](hch0ctl::R) reader structure"]
impl crate::Readable for HCH0CTL {}
#[doc = "`write(|w| ..)` method takes [hch0ctl::W](hch0ctl::W) writer structure"]
impl crate::Writable for HCH0CTL {}
#[doc = "host channel-0 characteristics register (HCH0CTL)"]
pub mod hch0ctl;
#[doc = "host channel-1 characteristics register (HCH1CTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch1ctl](hch1ctl) module"]
pub type HCH1CTL = crate::Reg<u32, _HCH1CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH1CTL;
#[doc = "`read()` method returns [hch1ctl::R](hch1ctl::R) reader structure"]
impl crate::Readable for HCH1CTL {}
#[doc = "`write(|w| ..)` method takes [hch1ctl::W](hch1ctl::W) writer structure"]
impl crate::Writable for HCH1CTL {}
#[doc = "host channel-1 characteristics register (HCH1CTL)"]
pub mod hch1ctl;
#[doc = "host channel-2 characteristics register (HCH2CTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch2ctl](hch2ctl) module"]
pub type HCH2CTL = crate::Reg<u32, _HCH2CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH2CTL;
#[doc = "`read()` method returns [hch2ctl::R](hch2ctl::R) reader structure"]
impl crate::Readable for HCH2CTL {}
#[doc = "`write(|w| ..)` method takes [hch2ctl::W](hch2ctl::W) writer structure"]
impl crate::Writable for HCH2CTL {}
#[doc = "host channel-2 characteristics register (HCH2CTL)"]
pub mod hch2ctl;
#[doc = "host channel-3 characteristics register (HCH3CTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch3ctl](hch3ctl) module"]
pub type HCH3CTL = crate::Reg<u32, _HCH3CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH3CTL;
#[doc = "`read()` method returns [hch3ctl::R](hch3ctl::R) reader structure"]
impl crate::Readable for HCH3CTL {}
#[doc = "`write(|w| ..)` method takes [hch3ctl::W](hch3ctl::W) writer structure"]
impl crate::Writable for HCH3CTL {}
#[doc = "host channel-3 characteristics register (HCH3CTL)"]
pub mod hch3ctl;
#[doc = "host channel-4 characteristics register (HCH4CTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch4ctl](hch4ctl) module"]
pub type HCH4CTL = crate::Reg<u32, _HCH4CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH4CTL;
#[doc = "`read()` method returns [hch4ctl::R](hch4ctl::R) reader structure"]
impl crate::Readable for HCH4CTL {}
#[doc = "`write(|w| ..)` method takes [hch4ctl::W](hch4ctl::W) writer structure"]
impl crate::Writable for HCH4CTL {}
#[doc = "host channel-4 characteristics register (HCH4CTL)"]
pub mod hch4ctl;
#[doc = "host channel-5 characteristics register (HCH5CTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch5ctl](hch5ctl) module"]
pub type HCH5CTL = crate::Reg<u32, _HCH5CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH5CTL;
#[doc = "`read()` method returns [hch5ctl::R](hch5ctl::R) reader structure"]
impl crate::Readable for HCH5CTL {}
#[doc = "`write(|w| ..)` method takes [hch5ctl::W](hch5ctl::W) writer structure"]
impl crate::Writable for HCH5CTL {}
#[doc = "host channel-5 characteristics register (HCH5CTL)"]
pub mod hch5ctl;
#[doc = "host channel-6 characteristics register (HCH6CTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch6ctl](hch6ctl) module"]
pub type HCH6CTL = crate::Reg<u32, _HCH6CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH6CTL;
#[doc = "`read()` method returns [hch6ctl::R](hch6ctl::R) reader structure"]
impl crate::Readable for HCH6CTL {}
#[doc = "`write(|w| ..)` method takes [hch6ctl::W](hch6ctl::W) writer structure"]
impl crate::Writable for HCH6CTL {}
#[doc = "host channel-6 characteristics register (HCH6CTL)"]
pub mod hch6ctl;
#[doc = "host channel-7 characteristics register (HCH7CTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch7ctl](hch7ctl) module"]
pub type HCH7CTL = crate::Reg<u32, _HCH7CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH7CTL;
#[doc = "`read()` method returns [hch7ctl::R](hch7ctl::R) reader structure"]
impl crate::Readable for HCH7CTL {}
#[doc = "`write(|w| ..)` method takes [hch7ctl::W](hch7ctl::W) writer structure"]
impl crate::Writable for HCH7CTL {}
#[doc = "host channel-7 characteristics register (HCH7CTL)"]
pub mod hch7ctl;
#[doc = "host channel-0 interrupt register (USBFS_HCHxINTF)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch0intf](hch0intf) module"]
pub type HCH0INTF = crate::Reg<u32, _HCH0INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH0INTF;
#[doc = "`read()` method returns [hch0intf::R](hch0intf::R) reader structure"]
impl crate::Readable for HCH0INTF {}
#[doc = "`write(|w| ..)` method takes [hch0intf::W](hch0intf::W) writer structure"]
impl crate::Writable for HCH0INTF {}
#[doc = "host channel-0 interrupt register (USBFS_HCHxINTF)"]
pub mod hch0intf;
#[doc = "host channel-1 interrupt register (HCH1INTF)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch1intf](hch1intf) module"]
pub type HCH1INTF = crate::Reg<u32, _HCH1INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH1INTF;
#[doc = "`read()` method returns [hch1intf::R](hch1intf::R) reader structure"]
impl crate::Readable for HCH1INTF {}
#[doc = "`write(|w| ..)` method takes [hch1intf::W](hch1intf::W) writer structure"]
impl crate::Writable for HCH1INTF {}
#[doc = "host channel-1 interrupt register (HCH1INTF)"]
pub mod hch1intf;
#[doc = "host channel-2 interrupt register (HCH2INTF)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch2intf](hch2intf) module"]
pub type HCH2INTF = crate::Reg<u32, _HCH2INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH2INTF;
#[doc = "`read()` method returns [hch2intf::R](hch2intf::R) reader structure"]
impl crate::Readable for HCH2INTF {}
#[doc = "`write(|w| ..)` method takes [hch2intf::W](hch2intf::W) writer structure"]
impl crate::Writable for HCH2INTF {}
#[doc = "host channel-2 interrupt register (HCH2INTF)"]
pub mod hch2intf;
#[doc = "host channel-3 interrupt register (HCH3INTF)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch3intf](hch3intf) module"]
pub type HCH3INTF = crate::Reg<u32, _HCH3INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH3INTF;
#[doc = "`read()` method returns [hch3intf::R](hch3intf::R) reader structure"]
impl crate::Readable for HCH3INTF {}
#[doc = "`write(|w| ..)` method takes [hch3intf::W](hch3intf::W) writer structure"]
impl crate::Writable for HCH3INTF {}
#[doc = "host channel-3 interrupt register (HCH3INTF)"]
pub mod hch3intf;
#[doc = "host channel-4 interrupt register (HCH4INTF)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch4intf](hch4intf) module"]
pub type HCH4INTF = crate::Reg<u32, _HCH4INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH4INTF;
#[doc = "`read()` method returns [hch4intf::R](hch4intf::R) reader structure"]
impl crate::Readable for HCH4INTF {}
#[doc = "`write(|w| ..)` method takes [hch4intf::W](hch4intf::W) writer structure"]
impl crate::Writable for HCH4INTF {}
#[doc = "host channel-4 interrupt register (HCH4INTF)"]
pub mod hch4intf;
#[doc = "host channel-5 interrupt register (HCH5INTF)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch5intf](hch5intf) module"]
pub type HCH5INTF = crate::Reg<u32, _HCH5INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH5INTF;
#[doc = "`read()` method returns [hch5intf::R](hch5intf::R) reader structure"]
impl crate::Readable for HCH5INTF {}
#[doc = "`write(|w| ..)` method takes [hch5intf::W](hch5intf::W) writer structure"]
impl crate::Writable for HCH5INTF {}
#[doc = "host channel-5 interrupt register (HCH5INTF)"]
pub mod hch5intf;
#[doc = "host channel-6 interrupt register (HCH6INTF)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch6intf](hch6intf) module"]
pub type HCH6INTF = crate::Reg<u32, _HCH6INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH6INTF;
#[doc = "`read()` method returns [hch6intf::R](hch6intf::R) reader structure"]
impl crate::Readable for HCH6INTF {}
#[doc = "`write(|w| ..)` method takes [hch6intf::W](hch6intf::W) writer structure"]
impl crate::Writable for HCH6INTF {}
#[doc = "host channel-6 interrupt register (HCH6INTF)"]
pub mod hch6intf;
#[doc = "host channel-7 interrupt register (HCH7INTF)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch7intf](hch7intf) module"]
pub type HCH7INTF = crate::Reg<u32, _HCH7INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH7INTF;
#[doc = "`read()` method returns [hch7intf::R](hch7intf::R) reader structure"]
impl crate::Readable for HCH7INTF {}
#[doc = "`write(|w| ..)` method takes [hch7intf::W](hch7intf::W) writer structure"]
impl crate::Writable for HCH7INTF {}
#[doc = "host channel-7 interrupt register (HCH7INTF)"]
pub mod hch7intf;
#[doc = "host channel-0 interrupt enable register (HCH0INTEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch0inten](hch0inten) module"]
pub type HCH0INTEN = crate::Reg<u32, _HCH0INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH0INTEN;
#[doc = "`read()` method returns [hch0inten::R](hch0inten::R) reader structure"]
impl crate::Readable for HCH0INTEN {}
#[doc = "`write(|w| ..)` method takes [hch0inten::W](hch0inten::W) writer structure"]
impl crate::Writable for HCH0INTEN {}
#[doc = "host channel-0 interrupt enable register (HCH0INTEN)"]
pub mod hch0inten;
#[doc = "host channel-1 interrupt enable register (HCH1INTEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch1inten](hch1inten) module"]
pub type HCH1INTEN = crate::Reg<u32, _HCH1INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH1INTEN;
#[doc = "`read()` method returns [hch1inten::R](hch1inten::R) reader structure"]
impl crate::Readable for HCH1INTEN {}
#[doc = "`write(|w| ..)` method takes [hch1inten::W](hch1inten::W) writer structure"]
impl crate::Writable for HCH1INTEN {}
#[doc = "host channel-1 interrupt enable register (HCH1INTEN)"]
pub mod hch1inten;
#[doc = "host channel-2 interrupt enable register (HCH2INTEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch2inten](hch2inten) module"]
pub type HCH2INTEN = crate::Reg<u32, _HCH2INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH2INTEN;
#[doc = "`read()` method returns [hch2inten::R](hch2inten::R) reader structure"]
impl crate::Readable for HCH2INTEN {}
#[doc = "`write(|w| ..)` method takes [hch2inten::W](hch2inten::W) writer structure"]
impl crate::Writable for HCH2INTEN {}
#[doc = "host channel-2 interrupt enable register (HCH2INTEN)"]
pub mod hch2inten;
#[doc = "host channel-3 interrupt enable register (HCH3INTEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch3inten](hch3inten) module"]
pub type HCH3INTEN = crate::Reg<u32, _HCH3INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH3INTEN;
#[doc = "`read()` method returns [hch3inten::R](hch3inten::R) reader structure"]
impl crate::Readable for HCH3INTEN {}
#[doc = "`write(|w| ..)` method takes [hch3inten::W](hch3inten::W) writer structure"]
impl crate::Writable for HCH3INTEN {}
#[doc = "host channel-3 interrupt enable register (HCH3INTEN)"]
pub mod hch3inten;
#[doc = "host channel-4 interrupt enable register (HCH4INTEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch4inten](hch4inten) module"]
pub type HCH4INTEN = crate::Reg<u32, _HCH4INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH4INTEN;
#[doc = "`read()` method returns [hch4inten::R](hch4inten::R) reader structure"]
impl crate::Readable for HCH4INTEN {}
#[doc = "`write(|w| ..)` method takes [hch4inten::W](hch4inten::W) writer structure"]
impl crate::Writable for HCH4INTEN {}
#[doc = "host channel-4 interrupt enable register (HCH4INTEN)"]
pub mod hch4inten;
#[doc = "host channel-5 interrupt enable register (HCH5INTEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch5inten](hch5inten) module"]
pub type HCH5INTEN = crate::Reg<u32, _HCH5INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH5INTEN;
#[doc = "`read()` method returns [hch5inten::R](hch5inten::R) reader structure"]
impl crate::Readable for HCH5INTEN {}
#[doc = "`write(|w| ..)` method takes [hch5inten::W](hch5inten::W) writer structure"]
impl crate::Writable for HCH5INTEN {}
#[doc = "host channel-5 interrupt enable register (HCH5INTEN)"]
pub mod hch5inten;
#[doc = "host channel-6 interrupt enable register (HCH6INTEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch6inten](hch6inten) module"]
pub type HCH6INTEN = crate::Reg<u32, _HCH6INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH6INTEN;
#[doc = "`read()` method returns [hch6inten::R](hch6inten::R) reader structure"]
impl crate::Readable for HCH6INTEN {}
#[doc = "`write(|w| ..)` method takes [hch6inten::W](hch6inten::W) writer structure"]
impl crate::Writable for HCH6INTEN {}
#[doc = "host channel-6 interrupt enable register (HCH6INTEN)"]
pub mod hch6inten;
#[doc = "host channel-7 interrupt enable register (HCH7INTEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch7inten](hch7inten) module"]
pub type HCH7INTEN = crate::Reg<u32, _HCH7INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH7INTEN;
#[doc = "`read()` method returns [hch7inten::R](hch7inten::R) reader structure"]
impl crate::Readable for HCH7INTEN {}
#[doc = "`write(|w| ..)` method takes [hch7inten::W](hch7inten::W) writer structure"]
impl crate::Writable for HCH7INTEN {}
#[doc = "host channel-7 interrupt enable register (HCH7INTEN)"]
pub mod hch7inten;
#[doc = "host channel-0 transfer length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch0len](hch0len) module"]
pub type HCH0LEN = crate::Reg<u32, _HCH0LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH0LEN;
#[doc = "`read()` method returns [hch0len::R](hch0len::R) reader structure"]
impl crate::Readable for HCH0LEN {}
#[doc = "`write(|w| ..)` method takes [hch0len::W](hch0len::W) writer structure"]
impl crate::Writable for HCH0LEN {}
#[doc = "host channel-0 transfer length register"]
pub mod hch0len;
#[doc = "host channel-1 transfer length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch1len](hch1len) module"]
pub type HCH1LEN = crate::Reg<u32, _HCH1LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH1LEN;
#[doc = "`read()` method returns [hch1len::R](hch1len::R) reader structure"]
impl crate::Readable for HCH1LEN {}
#[doc = "`write(|w| ..)` method takes [hch1len::W](hch1len::W) writer structure"]
impl crate::Writable for HCH1LEN {}
#[doc = "host channel-1 transfer length register"]
pub mod hch1len;
#[doc = "host channel-2 transfer length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch2len](hch2len) module"]
pub type HCH2LEN = crate::Reg<u32, _HCH2LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH2LEN;
#[doc = "`read()` method returns [hch2len::R](hch2len::R) reader structure"]
impl crate::Readable for HCH2LEN {}
#[doc = "`write(|w| ..)` method takes [hch2len::W](hch2len::W) writer structure"]
impl crate::Writable for HCH2LEN {}
#[doc = "host channel-2 transfer length register"]
pub mod hch2len;
#[doc = "host channel-3 transfer length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch3len](hch3len) module"]
pub type HCH3LEN = crate::Reg<u32, _HCH3LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH3LEN;
#[doc = "`read()` method returns [hch3len::R](hch3len::R) reader structure"]
impl crate::Readable for HCH3LEN {}
#[doc = "`write(|w| ..)` method takes [hch3len::W](hch3len::W) writer structure"]
impl crate::Writable for HCH3LEN {}
#[doc = "host channel-3 transfer length register"]
pub mod hch3len;
#[doc = "host channel-4 transfer length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch4len](hch4len) module"]
pub type HCH4LEN = crate::Reg<u32, _HCH4LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH4LEN;
#[doc = "`read()` method returns [hch4len::R](hch4len::R) reader structure"]
impl crate::Readable for HCH4LEN {}
#[doc = "`write(|w| ..)` method takes [hch4len::W](hch4len::W) writer structure"]
impl crate::Writable for HCH4LEN {}
#[doc = "host channel-4 transfer length register"]
pub mod hch4len;
#[doc = "host channel-5 transfer length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch5len](hch5len) module"]
pub type HCH5LEN = crate::Reg<u32, _HCH5LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH5LEN;
#[doc = "`read()` method returns [hch5len::R](hch5len::R) reader structure"]
impl crate::Readable for HCH5LEN {}
#[doc = "`write(|w| ..)` method takes [hch5len::W](hch5len::W) writer structure"]
impl crate::Writable for HCH5LEN {}
#[doc = "host channel-5 transfer length register"]
pub mod hch5len;
#[doc = "host channel-6 transfer length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch6len](hch6len) module"]
pub type HCH6LEN = crate::Reg<u32, _HCH6LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH6LEN;
#[doc = "`read()` method returns [hch6len::R](hch6len::R) reader structure"]
impl crate::Readable for HCH6LEN {}
#[doc = "`write(|w| ..)` method takes [hch6len::W](hch6len::W) writer structure"]
impl crate::Writable for HCH6LEN {}
#[doc = "host channel-6 transfer length register"]
pub mod hch6len;
#[doc = "host channel-7 transfer length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch7len](hch7len) module"]
pub type HCH7LEN = crate::Reg<u32, _HCH7LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCH7LEN;
#[doc = "`read()` method returns [hch7len::R](hch7len::R) reader structure"]
impl crate::Readable for HCH7LEN {}
#[doc = "`write(|w| ..)` method takes [hch7len::W](hch7len::W) writer structure"]
impl crate::Writable for HCH7LEN {}
#[doc = "host channel-7 transfer length register"]
pub mod hch7len;
