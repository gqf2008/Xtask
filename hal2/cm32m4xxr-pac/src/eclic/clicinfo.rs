#[doc = "Register `clicinfo` reader"]
pub struct R(crate::R<CLICINFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLICINFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLICINFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLICINFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NUM_INTERRUPT` reader - Number of interrupt sources supported by the hardware"]
pub type NUM_INTERRUPT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VERSION` reader - Hardware implementation version number"]
pub type VERSION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLICINTCTLBITS` reader - Used to specify the effective bit-width the register clicintctl\\[i\\]"]
pub type CLICINTCTLBITS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:12 - Number of interrupt sources supported by the hardware"]
    #[inline(always)]
    pub fn num_interrupt(&self) -> NUM_INTERRUPT_R {
        NUM_INTERRUPT_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:20 - Hardware implementation version number"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bits 21:24 - Used to specify the effective bit-width the register clicintctl\\[i\\]"]
    #[inline(always)]
    pub fn clicintctlbits(&self) -> CLICINTCTLBITS_R {
        CLICINTCTLBITS_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
}
#[doc = "global info register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clicinfo](index.html) module"]
pub struct CLICINFO_SPEC;
impl crate::RegisterSpec for CLICINFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clicinfo::R](R) reader structure"]
impl crate::Readable for CLICINFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets clicinfo to value 0"]
impl crate::Resettable for CLICINFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
