#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - wait state counter register"]
    pub ws: WS,
    #[doc = "0x04 - Unlock key register 0"]
    pub key0: KEY0,
    #[doc = "0x08 - Option byte unlock key register"]
    pub obkey: OBKEY,
    #[doc = "0x0c - Status register 0"]
    pub stat0: STAT0,
    #[doc = "0x10 - Control register 0"]
    pub ctl0: CTL0,
    #[doc = "0x14 - Address register 0"]
    pub addr0: ADDR0,
    _reserved6: [u8; 4usize],
    #[doc = "0x1c - Option byte status register"]
    pub obstat: OBSTAT,
    #[doc = "0x20 - Erase/Program Protection register"]
    pub wp: WP,
    _reserved8: [u8; 220usize],
    #[doc = "0x100 - Product ID register"]
    pub pid: PID,
}
#[doc = "wait state counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ws](ws) module"]
pub type WS = crate::Reg<u32, _WS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WS;
#[doc = "`read()` method returns [ws::R](ws::R) reader structure"]
impl crate::Readable for WS {}
#[doc = "`write(|w| ..)` method takes [ws::W](ws::W) writer structure"]
impl crate::Writable for WS {}
#[doc = "wait state counter register"]
pub mod ws;
#[doc = "Unlock key register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key0](key0) module"]
pub type KEY0 = crate::Reg<u32, _KEY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY0;
#[doc = "`write(|w| ..)` method takes [key0::W](key0::W) writer structure"]
impl crate::Writable for KEY0 {}
#[doc = "Unlock key register 0"]
pub mod key0;
#[doc = "Option byte unlock key register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [obkey](obkey) module"]
pub type OBKEY = crate::Reg<u32, _OBKEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OBKEY;
#[doc = "`write(|w| ..)` method takes [obkey::W](obkey::W) writer structure"]
impl crate::Writable for OBKEY {}
#[doc = "Option byte unlock key register"]
pub mod obkey;
#[doc = "Status register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat0](stat0) module"]
pub type STAT0 = crate::Reg<u32, _STAT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT0;
#[doc = "`read()` method returns [stat0::R](stat0::R) reader structure"]
impl crate::Readable for STAT0 {}
#[doc = "`write(|w| ..)` method takes [stat0::W](stat0::W) writer structure"]
impl crate::Writable for STAT0 {}
#[doc = "Status register 0"]
pub mod stat0;
#[doc = "Control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](ctl0) module"]
pub type CTL0 = crate::Reg<u32, _CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL0;
#[doc = "`read()` method returns [ctl0::R](ctl0::R) reader structure"]
impl crate::Readable for CTL0 {}
#[doc = "`write(|w| ..)` method takes [ctl0::W](ctl0::W) writer structure"]
impl crate::Writable for CTL0 {}
#[doc = "Control register 0"]
pub mod ctl0;
#[doc = "Address register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr0](addr0) module"]
pub type ADDR0 = crate::Reg<u32, _ADDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDR0;
#[doc = "`write(|w| ..)` method takes [addr0::W](addr0::W) writer structure"]
impl crate::Writable for ADDR0 {}
#[doc = "Address register 0"]
pub mod addr0;
#[doc = "Option byte status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [obstat](obstat) module"]
pub type OBSTAT = crate::Reg<u32, _OBSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OBSTAT;
#[doc = "`read()` method returns [obstat::R](obstat::R) reader structure"]
impl crate::Readable for OBSTAT {}
#[doc = "Option byte status register"]
pub mod obstat;
#[doc = "Erase/Program Protection register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wp](wp) module"]
pub type WP = crate::Reg<u32, _WP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WP;
#[doc = "`read()` method returns [wp::R](wp::R) reader structure"]
impl crate::Readable for WP {}
#[doc = "Erase/Program Protection register"]
pub mod wp;
#[doc = "Product ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid](pid) module"]
pub type PID = crate::Reg<u32, _PID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID;
#[doc = "`read()` method returns [pid::R](pid::R) reader structure"]
impl crate::Readable for PID {}
#[doc = "Product ID register"]
pub mod pid;
