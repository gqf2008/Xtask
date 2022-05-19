#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register 0"]
    pub ctl0: CTL0,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - Control register 1"]
    pub ctl1: CTL1,
    _reserved2: [u8; 2usize],
    #[doc = "0x08 - Slave address register 0"]
    pub saddr0: SADDR0,
    _reserved3: [u8; 2usize],
    #[doc = "0x0c - Slave address register 1"]
    pub saddr1: SADDR1,
    _reserved4: [u8; 2usize],
    #[doc = "0x10 - Transfer buffer register"]
    pub data: DATA,
    _reserved5: [u8; 2usize],
    #[doc = "0x14 - Transfer status register 0"]
    pub stat0: STAT0,
    _reserved6: [u8; 2usize],
    #[doc = "0x18 - Transfer status register 1"]
    pub stat1: STAT1,
    _reserved7: [u8; 2usize],
    #[doc = "0x1c - Clock configure register"]
    pub ckcfg: CKCFG,
    _reserved8: [u8; 2usize],
    #[doc = "0x20 - Rise time register"]
    pub rt: RT,
    _reserved9: [u8; 110usize],
    #[doc = "0x90 - Fast mode plus configure register"]
    pub fmpcfg: FMPCFG,
}
#[doc = "Control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](ctl0) module"]
pub type CTL0 = crate::Reg<u16, _CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL0;
#[doc = "`read()` method returns [ctl0::R](ctl0::R) reader structure"]
impl crate::Readable for CTL0 {}
#[doc = "`write(|w| ..)` method takes [ctl0::W](ctl0::W) writer structure"]
impl crate::Writable for CTL0 {}
#[doc = "Control register 0"]
pub mod ctl0;
#[doc = "Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](ctl1) module"]
pub type CTL1 = crate::Reg<u16, _CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL1;
#[doc = "`read()` method returns [ctl1::R](ctl1::R) reader structure"]
impl crate::Readable for CTL1 {}
#[doc = "`write(|w| ..)` method takes [ctl1::W](ctl1::W) writer structure"]
impl crate::Writable for CTL1 {}
#[doc = "Control register 1"]
pub mod ctl1;
#[doc = "Slave address register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saddr0](saddr0) module"]
pub type SADDR0 = crate::Reg<u16, _SADDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SADDR0;
#[doc = "`read()` method returns [saddr0::R](saddr0::R) reader structure"]
impl crate::Readable for SADDR0 {}
#[doc = "`write(|w| ..)` method takes [saddr0::W](saddr0::W) writer structure"]
impl crate::Writable for SADDR0 {}
#[doc = "Slave address register 0"]
pub mod saddr0;
#[doc = "Slave address register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saddr1](saddr1) module"]
pub type SADDR1 = crate::Reg<u16, _SADDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SADDR1;
#[doc = "`read()` method returns [saddr1::R](saddr1::R) reader structure"]
impl crate::Readable for SADDR1 {}
#[doc = "`write(|w| ..)` method takes [saddr1::W](saddr1::W) writer structure"]
impl crate::Writable for SADDR1 {}
#[doc = "Slave address register 1"]
pub mod saddr1;
#[doc = "Transfer buffer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](data) module"]
pub type DATA = crate::Reg<u16, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "Transfer buffer register"]
pub mod data;
#[doc = "Transfer status register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat0](stat0) module"]
pub type STAT0 = crate::Reg<u16, _STAT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT0;
#[doc = "`read()` method returns [stat0::R](stat0::R) reader structure"]
impl crate::Readable for STAT0 {}
#[doc = "`write(|w| ..)` method takes [stat0::W](stat0::W) writer structure"]
impl crate::Writable for STAT0 {}
#[doc = "Transfer status register 0"]
pub mod stat0;
#[doc = "Transfer status register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat1](stat1) module"]
pub type STAT1 = crate::Reg<u16, _STAT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT1;
#[doc = "`read()` method returns [stat1::R](stat1::R) reader structure"]
impl crate::Readable for STAT1 {}
#[doc = "Transfer status register 1"]
pub mod stat1;
#[doc = "Clock configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcfg](ckcfg) module"]
pub type CKCFG = crate::Reg<u16, _CKCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKCFG;
#[doc = "`read()` method returns [ckcfg::R](ckcfg::R) reader structure"]
impl crate::Readable for CKCFG {}
#[doc = "`write(|w| ..)` method takes [ckcfg::W](ckcfg::W) writer structure"]
impl crate::Writable for CKCFG {}
#[doc = "Clock configure register"]
pub mod ckcfg;
#[doc = "Rise time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rt](rt) module"]
pub type RT = crate::Reg<u16, _RT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RT;
#[doc = "`read()` method returns [rt::R](rt::R) reader structure"]
impl crate::Readable for RT {}
#[doc = "`write(|w| ..)` method takes [rt::W](rt::W) writer structure"]
impl crate::Writable for RT {}
#[doc = "Rise time register"]
pub mod rt;
#[doc = "Fast mode plus configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmpcfg](fmpcfg) module"]
pub type FMPCFG = crate::Reg<u16, _FMPCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPCFG;
#[doc = "`read()` method returns [fmpcfg::R](fmpcfg::R) reader structure"]
impl crate::Readable for FMPCFG {}
#[doc = "`write(|w| ..)` method takes [fmpcfg::W](fmpcfg::W) writer structure"]
impl crate::Writable for FMPCFG {}
#[doc = "Fast mode plus configure register"]
pub mod fmpcfg;
