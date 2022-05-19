#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM/NOR flash control register 0"]
    pub snctl0: SNCTL0,
    #[doc = "0x04 - SRAM/NOR flash timing configuration register 0"]
    pub sntcfg0: SNTCFG0,
    #[doc = "0x08 - SRAM/NOR flash control register 1"]
    pub snctl1: SNCTL1,
}
#[doc = "SRAM/NOR flash control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snctl0](snctl0) module"]
pub type SNCTL0 = crate::Reg<u32, _SNCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SNCTL0;
#[doc = "`read()` method returns [snctl0::R](snctl0::R) reader structure"]
impl crate::Readable for SNCTL0 {}
#[doc = "`write(|w| ..)` method takes [snctl0::W](snctl0::W) writer structure"]
impl crate::Writable for SNCTL0 {}
#[doc = "SRAM/NOR flash control register 0"]
pub mod snctl0;
#[doc = "SRAM/NOR flash timing configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sntcfg0](sntcfg0) module"]
pub type SNTCFG0 = crate::Reg<u32, _SNTCFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SNTCFG0;
#[doc = "`read()` method returns [sntcfg0::R](sntcfg0::R) reader structure"]
impl crate::Readable for SNTCFG0 {}
#[doc = "`write(|w| ..)` method takes [sntcfg0::W](sntcfg0::W) writer structure"]
impl crate::Writable for SNTCFG0 {}
#[doc = "SRAM/NOR flash timing configuration register 0"]
pub mod sntcfg0;
#[doc = "SRAM/NOR flash control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snctl1](snctl1) module"]
pub type SNCTL1 = crate::Reg<u32, _SNCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SNCTL1;
#[doc = "`read()` method returns [snctl1::R](snctl1::R) reader structure"]
impl crate::Readable for SNCTL1 {}
#[doc = "`write(|w| ..)` method takes [snctl1::W](snctl1::W) writer structure"]
impl crate::Writable for SNCTL1 {}
#[doc = "SRAM/NOR flash control register 1"]
pub mod snctl1;
