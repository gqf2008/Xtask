#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - power control register"]
    pub ctl: CTL,
    #[doc = "0x04 - power control/status register"]
    pub cs: CS,
}
#[doc = "power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "power control register"]
pub mod ctl;
#[doc = "power control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs](cs) module"]
pub type CS = crate::Reg<u32, _CS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CS;
#[doc = "`read()` method returns [cs::R](cs::R) reader structure"]
impl crate::Readable for CS {}
#[doc = "`write(|w| ..)` method takes [cs::W](cs::W) writer structure"]
impl crate::Writable for CS {}
#[doc = "power control/status register"]
pub mod cs;
