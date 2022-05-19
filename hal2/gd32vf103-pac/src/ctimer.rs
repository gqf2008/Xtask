#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer value (lower half)"]
    pub mtime_lo: MTIME_LO,
    #[doc = "0x04 - Timer value (upper half)"]
    pub mtime_hi: MTIME_HI,
    #[doc = "0x08 - Timer comparison value (lower half)"]
    pub mtimecmp_lo: MTIMECMP_LO,
    #[doc = "0x0c - Timer comparison value (upper half)"]
    pub mtimecmp_hi: MTIMECMP_HI,
    _reserved4: [u8; 4072usize],
    #[doc = "0xff8 - Timer control register"]
    pub mstop: MSTOP,
    #[doc = "0xffc - Software interrupt register"]
    pub msip: MSIP,
}
#[doc = "Timer value (lower half)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtime_lo](mtime_lo) module"]
pub type MTIME_LO = crate::Reg<u32, _MTIME_LO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTIME_LO;
#[doc = "`read()` method returns [mtime_lo::R](mtime_lo::R) reader structure"]
impl crate::Readable for MTIME_LO {}
#[doc = "`write(|w| ..)` method takes [mtime_lo::W](mtime_lo::W) writer structure"]
impl crate::Writable for MTIME_LO {}
#[doc = "Timer value (lower half)"]
pub mod mtime_lo;
#[doc = "Timer value (upper half)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtime_hi](mtime_hi) module"]
pub type MTIME_HI = crate::Reg<u32, _MTIME_HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTIME_HI;
#[doc = "`read()` method returns [mtime_hi::R](mtime_hi::R) reader structure"]
impl crate::Readable for MTIME_HI {}
#[doc = "`write(|w| ..)` method takes [mtime_hi::W](mtime_hi::W) writer structure"]
impl crate::Writable for MTIME_HI {}
#[doc = "Timer value (upper half)"]
pub mod mtime_hi;
#[doc = "Timer comparison value (lower half)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtimecmp_lo](mtimecmp_lo) module"]
pub type MTIMECMP_LO = crate::Reg<u32, _MTIMECMP_LO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTIMECMP_LO;
#[doc = "`read()` method returns [mtimecmp_lo::R](mtimecmp_lo::R) reader structure"]
impl crate::Readable for MTIMECMP_LO {}
#[doc = "`write(|w| ..)` method takes [mtimecmp_lo::W](mtimecmp_lo::W) writer structure"]
impl crate::Writable for MTIMECMP_LO {}
#[doc = "Timer comparison value (lower half)"]
pub mod mtimecmp_lo;
#[doc = "Timer comparison value (upper half)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtimecmp_hi](mtimecmp_hi) module"]
pub type MTIMECMP_HI = crate::Reg<u32, _MTIMECMP_HI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTIMECMP_HI;
#[doc = "`read()` method returns [mtimecmp_hi::R](mtimecmp_hi::R) reader structure"]
impl crate::Readable for MTIMECMP_HI {}
#[doc = "`write(|w| ..)` method takes [mtimecmp_hi::W](mtimecmp_hi::W) writer structure"]
impl crate::Writable for MTIMECMP_HI {}
#[doc = "Timer comparison value (upper half)"]
pub mod mtimecmp_hi;
#[doc = "Timer control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mstop](mstop) module"]
pub type MSTOP = crate::Reg<u32, _MSTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSTOP;
#[doc = "`read()` method returns [mstop::R](mstop::R) reader structure"]
impl crate::Readable for MSTOP {}
#[doc = "`write(|w| ..)` method takes [mstop::W](mstop::W) writer structure"]
impl crate::Writable for MSTOP {}
#[doc = "Timer control register"]
pub mod mstop;
#[doc = "Software interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msip](msip) module"]
pub type MSIP = crate::Reg<u32, _MSIP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSIP;
#[doc = "`read()` method returns [msip::R](msip::R) reader structure"]
impl crate::Readable for MSIP {}
#[doc = "`write(|w| ..)` method takes [msip::W](msip::W) writer structure"]
impl crate::Writable for MSIP {}
#[doc = "Software interrupt register"]
pub mod msip;
