#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt enable register (EXTI_INTEN)"]
    pub inten: INTEN,
    #[doc = "0x04 - Event enable register (EXTI_EVEN)"]
    pub even: EVEN,
    #[doc = "0x08 - Rising Edge Trigger Enable register (EXTI_RTEN)"]
    pub rten: RTEN,
    #[doc = "0x0c - Falling Egde Trigger Enable register (EXTI_FTEN)"]
    pub ften: FTEN,
    #[doc = "0x10 - Software interrupt event register (EXTI_SWIEV)"]
    pub swiev: SWIEV,
    #[doc = "0x14 - Pending register (EXTI_PD)"]
    pub pd: PD,
}
#[doc = "Interrupt enable register (EXTI_INTEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "Interrupt enable register (EXTI_INTEN)"]
pub mod inten;
#[doc = "Event enable register (EXTI_EVEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [even](even) module"]
pub type EVEN = crate::Reg<u32, _EVEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVEN;
#[doc = "`read()` method returns [even::R](even::R) reader structure"]
impl crate::Readable for EVEN {}
#[doc = "`write(|w| ..)` method takes [even::W](even::W) writer structure"]
impl crate::Writable for EVEN {}
#[doc = "Event enable register (EXTI_EVEN)"]
pub mod even;
#[doc = "Rising Edge Trigger Enable register (EXTI_RTEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rten](rten) module"]
pub type RTEN = crate::Reg<u32, _RTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTEN;
#[doc = "`read()` method returns [rten::R](rten::R) reader structure"]
impl crate::Readable for RTEN {}
#[doc = "`write(|w| ..)` method takes [rten::W](rten::W) writer structure"]
impl crate::Writable for RTEN {}
#[doc = "Rising Edge Trigger Enable register (EXTI_RTEN)"]
pub mod rten;
#[doc = "Falling Egde Trigger Enable register (EXTI_FTEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ften](ften) module"]
pub type FTEN = crate::Reg<u32, _FTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FTEN;
#[doc = "`read()` method returns [ften::R](ften::R) reader structure"]
impl crate::Readable for FTEN {}
#[doc = "`write(|w| ..)` method takes [ften::W](ften::W) writer structure"]
impl crate::Writable for FTEN {}
#[doc = "Falling Egde Trigger Enable register (EXTI_FTEN)"]
pub mod ften;
#[doc = "Software interrupt event register (EXTI_SWIEV)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swiev](swiev) module"]
pub type SWIEV = crate::Reg<u32, _SWIEV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWIEV;
#[doc = "`read()` method returns [swiev::R](swiev::R) reader structure"]
impl crate::Readable for SWIEV {}
#[doc = "`write(|w| ..)` method takes [swiev::W](swiev::W) writer structure"]
impl crate::Writable for SWIEV {}
#[doc = "Software interrupt event register (EXTI_SWIEV)"]
pub mod swiev;
#[doc = "Pending register (EXTI_PD)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd](pd) module"]
pub type PD = crate::Reg<u32, _PD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD;
#[doc = "`read()` method returns [pd::R](pd::R) reader structure"]
impl crate::Readable for PD {}
#[doc = "`write(|w| ..)` method takes [pd::W](pd::W) writer structure"]
impl crate::Writable for PD {}
#[doc = "Pending register (EXTI_PD)"]
pub mod pd;
