#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global OTG control and status register (USBFS_GOTGCS)"]
    pub gotgcs: GOTGCS,
    #[doc = "0x04 - Global OTG interrupt flag register (USBFS_GOTGINTF)"]
    pub gotgintf: GOTGINTF,
    #[doc = "0x08 - Global AHB control and status register (USBFS_GAHBCS)"]
    pub gahbcs: GAHBCS,
    #[doc = "0x0c - Global USB control and status register (USBFS_GUSBCSR)"]
    pub gusbcs: GUSBCS,
    #[doc = "0x10 - Global reset control register (USBFS_GRSTCTL)"]
    pub grstctl: GRSTCTL,
    #[doc = "0x14 - Global interrupt flag register (USBFS_GINTF)"]
    pub gintf: GINTF,
    #[doc = "0x18 - Global interrupt enable register (USBFS_GINTEN)"]
    pub ginten: GINTEN,
    _reserved_7_grstatr: [u8; 4usize],
    _reserved_8_grstatp: [u8; 4usize],
    #[doc = "0x24 - Global Receive FIFO size register (USBFS_GRFLEN)"]
    pub grflen: GRFLEN,
    _reserved_10_hnptflen: [u8; 4usize],
    #[doc = "0x2c - Host non-periodic transmit FIFO/queue status register (HNPTFQSTAT)"]
    pub hnptfqstat: HNPTFQSTAT,
    _reserved12: [u8; 8usize],
    #[doc = "0x38 - Global core configuration register (USBFS_GCCFG)"]
    pub gccfg: GCCFG,
    #[doc = "0x3c - core ID register"]
    pub cid: CID,
    _reserved14: [u8; 192usize],
    #[doc = "0x100 - Host periodic transmit FIFO length register (HPTFLEN)"]
    pub hptflen: HPTFLEN,
    #[doc = "0x104 - device IN endpoint transmit FIFO size register (DIEP1TFLEN)"]
    pub diep1tflen: DIEP1TFLEN,
    #[doc = "0x108 - device IN endpoint transmit FIFO size register (DIEP2TFLEN)"]
    pub diep2tflen: DIEP2TFLEN,
    #[doc = "0x10c - device IN endpoint transmit FIFO size register (FS_DIEP3TXFLEN)"]
    pub diep3tflen: DIEP3TFLEN,
}
impl RegisterBlock {
    #[doc = "0x1c - Global Receive status read(Host mode)"]
    #[inline(always)]
    pub fn grstatr_host(&self) -> &GRSTATR_HOST {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const GRSTATR_HOST) }
    }
    #[doc = "0x1c - Global Receive status read(Host mode)"]
    #[inline(always)]
    pub fn grstatr_host_mut(&self) -> &mut GRSTATR_HOST {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut GRSTATR_HOST) }
    }
    #[doc = "0x1c - Global Receive status read(Device mode)"]
    #[inline(always)]
    pub fn grstatr_device(&self) -> &GRSTATR_DEVICE {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const GRSTATR_DEVICE) }
    }
    #[doc = "0x1c - Global Receive status read(Device mode)"]
    #[inline(always)]
    pub fn grstatr_device_mut(&self) -> &mut GRSTATR_DEVICE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut GRSTATR_DEVICE) }
    }
    #[doc = "0x20 - Global Receive status pop(Host mode)"]
    #[inline(always)]
    pub fn grstatp_host(&self) -> &GRSTATP_HOST {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const GRSTATP_HOST) }
    }
    #[doc = "0x20 - Global Receive status pop(Host mode)"]
    #[inline(always)]
    pub fn grstatp_host_mut(&self) -> &mut GRSTATP_HOST {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut GRSTATP_HOST) }
    }
    #[doc = "0x20 - Global Receive status pop(Device mode)"]
    #[inline(always)]
    pub fn grstatp_device(&self) -> &GRSTATP_DEVICE {
        unsafe { &*(((self as *const Self) as *const u8).add(32usize) as *const GRSTATP_DEVICE) }
    }
    #[doc = "0x20 - Global Receive status pop(Device mode)"]
    #[inline(always)]
    pub fn grstatp_device_mut(&self) -> &mut GRSTATP_DEVICE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(32usize) as *mut GRSTATP_DEVICE) }
    }
    #[doc = "0x28 - Device IN endpoint 0 transmit FIFO length (Device mode)"]
    #[inline(always)]
    pub fn diep0tflen(&self) -> &DIEP0TFLEN {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const DIEP0TFLEN) }
    }
    #[doc = "0x28 - Device IN endpoint 0 transmit FIFO length (Device mode)"]
    #[inline(always)]
    pub fn diep0tflen_mut(&self) -> &mut DIEP0TFLEN {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut DIEP0TFLEN) }
    }
    #[doc = "0x28 - Host non-periodic transmit FIFO length register (Host mode)"]
    #[inline(always)]
    pub fn hnptflen(&self) -> &HNPTFLEN {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const HNPTFLEN) }
    }
    #[doc = "0x28 - Host non-periodic transmit FIFO length register (Host mode)"]
    #[inline(always)]
    pub fn hnptflen_mut(&self) -> &mut HNPTFLEN {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut HNPTFLEN) }
    }
}
#[doc = "Global OTG control and status register (USBFS_GOTGCS)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gotgcs](gotgcs) module"]
pub type GOTGCS = crate::Reg<u32, _GOTGCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GOTGCS;
#[doc = "`read()` method returns [gotgcs::R](gotgcs::R) reader structure"]
impl crate::Readable for GOTGCS {}
#[doc = "`write(|w| ..)` method takes [gotgcs::W](gotgcs::W) writer structure"]
impl crate::Writable for GOTGCS {}
#[doc = "Global OTG control and status register (USBFS_GOTGCS)"]
pub mod gotgcs;
#[doc = "Global OTG interrupt flag register (USBFS_GOTGINTF)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gotgintf](gotgintf) module"]
pub type GOTGINTF = crate::Reg<u32, _GOTGINTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GOTGINTF;
#[doc = "`read()` method returns [gotgintf::R](gotgintf::R) reader structure"]
impl crate::Readable for GOTGINTF {}
#[doc = "`write(|w| ..)` method takes [gotgintf::W](gotgintf::W) writer structure"]
impl crate::Writable for GOTGINTF {}
#[doc = "Global OTG interrupt flag register (USBFS_GOTGINTF)"]
pub mod gotgintf;
#[doc = "Global AHB control and status register (USBFS_GAHBCS)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gahbcs](gahbcs) module"]
pub type GAHBCS = crate::Reg<u32, _GAHBCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GAHBCS;
#[doc = "`read()` method returns [gahbcs::R](gahbcs::R) reader structure"]
impl crate::Readable for GAHBCS {}
#[doc = "`write(|w| ..)` method takes [gahbcs::W](gahbcs::W) writer structure"]
impl crate::Writable for GAHBCS {}
#[doc = "Global AHB control and status register (USBFS_GAHBCS)"]
pub mod gahbcs;
#[doc = "Global USB control and status register (USBFS_GUSBCSR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gusbcs](gusbcs) module"]
pub type GUSBCS = crate::Reg<u32, _GUSBCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GUSBCS;
#[doc = "`read()` method returns [gusbcs::R](gusbcs::R) reader structure"]
impl crate::Readable for GUSBCS {}
#[doc = "`write(|w| ..)` method takes [gusbcs::W](gusbcs::W) writer structure"]
impl crate::Writable for GUSBCS {}
#[doc = "Global USB control and status register (USBFS_GUSBCSR)"]
pub mod gusbcs;
#[doc = "Global reset control register (USBFS_GRSTCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grstctl](grstctl) module"]
pub type GRSTCTL = crate::Reg<u32, _GRSTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRSTCTL;
#[doc = "`read()` method returns [grstctl::R](grstctl::R) reader structure"]
impl crate::Readable for GRSTCTL {}
#[doc = "`write(|w| ..)` method takes [grstctl::W](grstctl::W) writer structure"]
impl crate::Writable for GRSTCTL {}
#[doc = "Global reset control register (USBFS_GRSTCTL)"]
pub mod grstctl;
#[doc = "Global interrupt flag register (USBFS_GINTF)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gintf](gintf) module"]
pub type GINTF = crate::Reg<u32, _GINTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GINTF;
#[doc = "`read()` method returns [gintf::R](gintf::R) reader structure"]
impl crate::Readable for GINTF {}
#[doc = "`write(|w| ..)` method takes [gintf::W](gintf::W) writer structure"]
impl crate::Writable for GINTF {}
#[doc = "Global interrupt flag register (USBFS_GINTF)"]
pub mod gintf;
#[doc = "Global interrupt enable register (USBFS_GINTEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ginten](ginten) module"]
pub type GINTEN = crate::Reg<u32, _GINTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GINTEN;
#[doc = "`read()` method returns [ginten::R](ginten::R) reader structure"]
impl crate::Readable for GINTEN {}
#[doc = "`write(|w| ..)` method takes [ginten::W](ginten::W) writer structure"]
impl crate::Writable for GINTEN {}
#[doc = "Global interrupt enable register (USBFS_GINTEN)"]
pub mod ginten;
#[doc = "Global Receive status read(Device mode)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grstatr_device](grstatr_device) module"]
pub type GRSTATR_DEVICE = crate::Reg<u32, _GRSTATR_DEVICE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRSTATR_DEVICE;
#[doc = "`read()` method returns [grstatr_device::R](grstatr_device::R) reader structure"]
impl crate::Readable for GRSTATR_DEVICE {}
#[doc = "Global Receive status read(Device mode)"]
pub mod grstatr_device;
#[doc = "Global Receive status read(Host mode)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grstatr_host](grstatr_host) module"]
pub type GRSTATR_HOST = crate::Reg<u32, _GRSTATR_HOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRSTATR_HOST;
#[doc = "`read()` method returns [grstatr_host::R](grstatr_host::R) reader structure"]
impl crate::Readable for GRSTATR_HOST {}
#[doc = "Global Receive status read(Host mode)"]
pub mod grstatr_host;
#[doc = "Global Receive status pop(Device mode)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grstatp_device](grstatp_device) module"]
pub type GRSTATP_DEVICE = crate::Reg<u32, _GRSTATP_DEVICE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRSTATP_DEVICE;
#[doc = "`read()` method returns [grstatp_device::R](grstatp_device::R) reader structure"]
impl crate::Readable for GRSTATP_DEVICE {}
#[doc = "Global Receive status pop(Device mode)"]
pub mod grstatp_device;
#[doc = "Global Receive status pop(Host mode)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grstatp_host](grstatp_host) module"]
pub type GRSTATP_HOST = crate::Reg<u32, _GRSTATP_HOST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRSTATP_HOST;
#[doc = "`read()` method returns [grstatp_host::R](grstatp_host::R) reader structure"]
impl crate::Readable for GRSTATP_HOST {}
#[doc = "Global Receive status pop(Host mode)"]
pub mod grstatp_host;
#[doc = "Global Receive FIFO size register (USBFS_GRFLEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grflen](grflen) module"]
pub type GRFLEN = crate::Reg<u32, _GRFLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRFLEN;
#[doc = "`read()` method returns [grflen::R](grflen::R) reader structure"]
impl crate::Readable for GRFLEN {}
#[doc = "`write(|w| ..)` method takes [grflen::W](grflen::W) writer structure"]
impl crate::Writable for GRFLEN {}
#[doc = "Global Receive FIFO size register (USBFS_GRFLEN)"]
pub mod grflen;
#[doc = "Host non-periodic transmit FIFO length register (Host mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hnptflen](hnptflen) module"]
pub type HNPTFLEN = crate::Reg<u32, _HNPTFLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HNPTFLEN;
#[doc = "`read()` method returns [hnptflen::R](hnptflen::R) reader structure"]
impl crate::Readable for HNPTFLEN {}
#[doc = "`write(|w| ..)` method takes [hnptflen::W](hnptflen::W) writer structure"]
impl crate::Writable for HNPTFLEN {}
#[doc = "Host non-periodic transmit FIFO length register (Host mode)"]
pub mod hnptflen;
#[doc = "Device IN endpoint 0 transmit FIFO length (Device mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep0tflen](diep0tflen) module"]
pub type DIEP0TFLEN = crate::Reg<u32, _DIEP0TFLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0TFLEN;
#[doc = "`read()` method returns [diep0tflen::R](diep0tflen::R) reader structure"]
impl crate::Readable for DIEP0TFLEN {}
#[doc = "`write(|w| ..)` method takes [diep0tflen::W](diep0tflen::W) writer structure"]
impl crate::Writable for DIEP0TFLEN {}
#[doc = "Device IN endpoint 0 transmit FIFO length (Device mode)"]
pub mod diep0tflen;
#[doc = "Host non-periodic transmit FIFO/queue status register (HNPTFQSTAT)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hnptfqstat](hnptfqstat) module"]
pub type HNPTFQSTAT = crate::Reg<u32, _HNPTFQSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HNPTFQSTAT;
#[doc = "`read()` method returns [hnptfqstat::R](hnptfqstat::R) reader structure"]
impl crate::Readable for HNPTFQSTAT {}
#[doc = "Host non-periodic transmit FIFO/queue status register (HNPTFQSTAT)"]
pub mod hnptfqstat;
#[doc = "Global core configuration register (USBFS_GCCFG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gccfg](gccfg) module"]
pub type GCCFG = crate::Reg<u32, _GCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCCFG;
#[doc = "`read()` method returns [gccfg::R](gccfg::R) reader structure"]
impl crate::Readable for GCCFG {}
#[doc = "`write(|w| ..)` method takes [gccfg::W](gccfg::W) writer structure"]
impl crate::Writable for GCCFG {}
#[doc = "Global core configuration register (USBFS_GCCFG)"]
pub mod gccfg;
#[doc = "core ID register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid](cid) module"]
pub type CID = crate::Reg<u32, _CID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID;
#[doc = "`read()` method returns [cid::R](cid::R) reader structure"]
impl crate::Readable for CID {}
#[doc = "`write(|w| ..)` method takes [cid::W](cid::W) writer structure"]
impl crate::Writable for CID {}
#[doc = "core ID register"]
pub mod cid;
#[doc = "Host periodic transmit FIFO length register (HPTFLEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hptflen](hptflen) module"]
pub type HPTFLEN = crate::Reg<u32, _HPTFLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPTFLEN;
#[doc = "`read()` method returns [hptflen::R](hptflen::R) reader structure"]
impl crate::Readable for HPTFLEN {}
#[doc = "`write(|w| ..)` method takes [hptflen::W](hptflen::W) writer structure"]
impl crate::Writable for HPTFLEN {}
#[doc = "Host periodic transmit FIFO length register (HPTFLEN)"]
pub mod hptflen;
#[doc = "device IN endpoint transmit FIFO size register (DIEP1TFLEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep1tflen](diep1tflen) module"]
pub type DIEP1TFLEN = crate::Reg<u32, _DIEP1TFLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP1TFLEN;
#[doc = "`read()` method returns [diep1tflen::R](diep1tflen::R) reader structure"]
impl crate::Readable for DIEP1TFLEN {}
#[doc = "`write(|w| ..)` method takes [diep1tflen::W](diep1tflen::W) writer structure"]
impl crate::Writable for DIEP1TFLEN {}
#[doc = "device IN endpoint transmit FIFO size register (DIEP1TFLEN)"]
pub mod diep1tflen;
#[doc = "device IN endpoint transmit FIFO size register (DIEP2TFLEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep2tflen](diep2tflen) module"]
pub type DIEP2TFLEN = crate::Reg<u32, _DIEP2TFLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP2TFLEN;
#[doc = "`read()` method returns [diep2tflen::R](diep2tflen::R) reader structure"]
impl crate::Readable for DIEP2TFLEN {}
#[doc = "`write(|w| ..)` method takes [diep2tflen::W](diep2tflen::W) writer structure"]
impl crate::Writable for DIEP2TFLEN {}
#[doc = "device IN endpoint transmit FIFO size register (DIEP2TFLEN)"]
pub mod diep2tflen;
#[doc = "device IN endpoint transmit FIFO size register (FS_DIEP3TXFLEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep3tflen](diep3tflen) module"]
pub type DIEP3TFLEN = crate::Reg<u32, _DIEP3TFLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP3TFLEN;
#[doc = "`read()` method returns [diep3tflen::R](diep3tflen::R) reader structure"]
impl crate::Readable for DIEP3TFLEN {}
#[doc = "`write(|w| ..)` method takes [diep3tflen::W](diep3tflen::W) writer structure"]
impl crate::Writable for DIEP3TFLEN {}
#[doc = "device IN endpoint transmit FIFO size register (FS_DIEP3TXFLEN)"]
pub mod diep3tflen;
