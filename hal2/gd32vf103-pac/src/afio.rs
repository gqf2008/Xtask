#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event control register"]
    pub ec: EC,
    #[doc = "0x04 - AFIO port configuration register 0"]
    pub pcf0: PCF0,
    #[doc = "0x08 - EXTI sources selection register 0"]
    pub extiss0: EXTISS0,
    #[doc = "0x0c - EXTI sources selection register 1"]
    pub extiss1: EXTISS1,
    #[doc = "0x10 - EXTI sources selection register 2"]
    pub extiss2: EXTISS2,
    #[doc = "0x14 - EXTI sources selection register 3"]
    pub extiss3: EXTISS3,
    _reserved6: [u8; 4usize],
    #[doc = "0x1c - AFIO port configuration register 1"]
    pub pcf1: PCF1,
}
#[doc = "Event control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ec](ec) module"]
pub type EC = crate::Reg<u32, _EC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EC;
#[doc = "`read()` method returns [ec::R](ec::R) reader structure"]
impl crate::Readable for EC {}
#[doc = "`write(|w| ..)` method takes [ec::W](ec::W) writer structure"]
impl crate::Writable for EC {}
#[doc = "Event control register"]
pub mod ec;
#[doc = "AFIO port configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcf0](pcf0) module"]
pub type PCF0 = crate::Reg<u32, _PCF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCF0;
#[doc = "`read()` method returns [pcf0::R](pcf0::R) reader structure"]
impl crate::Readable for PCF0 {}
#[doc = "`write(|w| ..)` method takes [pcf0::W](pcf0::W) writer structure"]
impl crate::Writable for PCF0 {}
#[doc = "AFIO port configuration register 0"]
pub mod pcf0;
#[doc = "EXTI sources selection register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extiss0](extiss0) module"]
pub type EXTISS0 = crate::Reg<u32, _EXTISS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTISS0;
#[doc = "`read()` method returns [extiss0::R](extiss0::R) reader structure"]
impl crate::Readable for EXTISS0 {}
#[doc = "`write(|w| ..)` method takes [extiss0::W](extiss0::W) writer structure"]
impl crate::Writable for EXTISS0 {}
#[doc = "EXTI sources selection register 0"]
pub mod extiss0;
#[doc = "EXTI sources selection register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extiss1](extiss1) module"]
pub type EXTISS1 = crate::Reg<u32, _EXTISS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTISS1;
#[doc = "`read()` method returns [extiss1::R](extiss1::R) reader structure"]
impl crate::Readable for EXTISS1 {}
#[doc = "`write(|w| ..)` method takes [extiss1::W](extiss1::W) writer structure"]
impl crate::Writable for EXTISS1 {}
#[doc = "EXTI sources selection register 1"]
pub mod extiss1;
#[doc = "EXTI sources selection register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extiss2](extiss2) module"]
pub type EXTISS2 = crate::Reg<u32, _EXTISS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTISS2;
#[doc = "`read()` method returns [extiss2::R](extiss2::R) reader structure"]
impl crate::Readable for EXTISS2 {}
#[doc = "`write(|w| ..)` method takes [extiss2::W](extiss2::W) writer structure"]
impl crate::Writable for EXTISS2 {}
#[doc = "EXTI sources selection register 2"]
pub mod extiss2;
#[doc = "EXTI sources selection register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extiss3](extiss3) module"]
pub type EXTISS3 = crate::Reg<u32, _EXTISS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTISS3;
#[doc = "`read()` method returns [extiss3::R](extiss3::R) reader structure"]
impl crate::Readable for EXTISS3 {}
#[doc = "`write(|w| ..)` method takes [extiss3::W](extiss3::W) writer structure"]
impl crate::Writable for EXTISS3 {}
#[doc = "EXTI sources selection register 3"]
pub mod extiss3;
#[doc = "AFIO port configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcf1](pcf1) module"]
pub type PCF1 = crate::Reg<u32, _PCF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCF1;
#[doc = "`read()` method returns [pcf1::R](pcf1::R) reader structure"]
impl crate::Readable for PCF1 {}
#[doc = "`write(|w| ..)` method takes [pcf1::W](pcf1::W) writer structure"]
impl crate::Writable for PCF1 {}
#[doc = "AFIO port configuration register 1"]
pub mod pcf1;
