#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC interrupt enable register"]
    pub inten: INTEN,
    #[doc = "0x04 - control register"]
    pub ctl: CTL,
    #[doc = "0x08 - RTC prescaler high register"]
    pub psch: PSCH,
    #[doc = "0x0c - RTC prescaler low register"]
    pub pscl: PSCL,
    #[doc = "0x10 - RTC divider high register"]
    pub divh: DIVH,
    #[doc = "0x14 - RTC divider low register"]
    pub divl: DIVL,
    #[doc = "0x18 - RTC counter high register"]
    pub cnth: CNTH,
    #[doc = "0x1c - RTC counter low register"]
    pub cntl: CNTL,
    #[doc = "0x20 - Alarm high register"]
    pub alrmh: ALRMH,
    #[doc = "0x24 - RTC alarm low register"]
    pub alrml: ALRML,
}
#[doc = "RTC interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "RTC interrupt enable register"]
pub mod inten;
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "control register"]
pub mod ctl;
#[doc = "RTC prescaler high register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psch](psch) module"]
pub type PSCH = crate::Reg<u32, _PSCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSCH;
#[doc = "`write(|w| ..)` method takes [psch::W](psch::W) writer structure"]
impl crate::Writable for PSCH {}
#[doc = "RTC prescaler high register"]
pub mod psch;
#[doc = "RTC prescaler low register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pscl](pscl) module"]
pub type PSCL = crate::Reg<u32, _PSCL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSCL;
#[doc = "`write(|w| ..)` method takes [pscl::W](pscl::W) writer structure"]
impl crate::Writable for PSCL {}
#[doc = "RTC prescaler low register"]
pub mod pscl;
#[doc = "RTC divider high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divh](divh) module"]
pub type DIVH = crate::Reg<u32, _DIVH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIVH;
#[doc = "`read()` method returns [divh::R](divh::R) reader structure"]
impl crate::Readable for DIVH {}
#[doc = "RTC divider high register"]
pub mod divh;
#[doc = "RTC divider low register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divl](divl) module"]
pub type DIVL = crate::Reg<u32, _DIVL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIVL;
#[doc = "`read()` method returns [divl::R](divl::R) reader structure"]
impl crate::Readable for DIVL {}
#[doc = "RTC divider low register"]
pub mod divl;
#[doc = "RTC counter high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnth](cnth) module"]
pub type CNTH = crate::Reg<u32, _CNTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTH;
#[doc = "`read()` method returns [cnth::R](cnth::R) reader structure"]
impl crate::Readable for CNTH {}
#[doc = "`write(|w| ..)` method takes [cnth::W](cnth::W) writer structure"]
impl crate::Writable for CNTH {}
#[doc = "RTC counter high register"]
pub mod cnth;
#[doc = "RTC counter low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntl](cntl) module"]
pub type CNTL = crate::Reg<u32, _CNTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTL;
#[doc = "`read()` method returns [cntl::R](cntl::R) reader structure"]
impl crate::Readable for CNTL {}
#[doc = "`write(|w| ..)` method takes [cntl::W](cntl::W) writer structure"]
impl crate::Writable for CNTL {}
#[doc = "RTC counter low register"]
pub mod cntl;
#[doc = "Alarm high register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmh](alrmh) module"]
pub type ALRMH = crate::Reg<u32, _ALRMH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMH;
#[doc = "`write(|w| ..)` method takes [alrmh::W](alrmh::W) writer structure"]
impl crate::Writable for ALRMH {}
#[doc = "Alarm high register"]
pub mod alrmh;
#[doc = "RTC alarm low register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrml](alrml) module"]
pub type ALRML = crate::Reg<u32, _ALRML>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRML;
#[doc = "`write(|w| ..)` method takes [alrml::W](alrml::W) writer structure"]
impl crate::Writable for ALRML {}
#[doc = "RTC alarm low register"]
pub mod alrml;
