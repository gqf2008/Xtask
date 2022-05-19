#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - power and clock gating control register (PWRCLKCTL)"]
    pub pwrclkctl: PWRCLKCTL,
}
#[doc = "power and clock gating control register (PWRCLKCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrclkctl](pwrclkctl) module"]
pub type PWRCLKCTL = crate::Reg<u32, _PWRCLKCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCLKCTL;
#[doc = "`read()` method returns [pwrclkctl::R](pwrclkctl::R) reader structure"]
impl crate::Readable for PWRCLKCTL {}
#[doc = "`write(|w| ..)` method takes [pwrclkctl::W](pwrclkctl::W) writer structure"]
impl crate::Writable for PWRCLKCTL {}
#[doc = "power and clock gating control register (PWRCLKCTL)"]
pub mod pwrclkctl;
