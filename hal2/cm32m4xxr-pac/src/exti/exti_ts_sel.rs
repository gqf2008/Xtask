#[doc = "Register `EXTI_TS_SEL` reader"]
pub struct R(crate::R<EXTI_TS_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_TS_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_TS_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_TS_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_TS_SEL` writer"]
pub struct W(crate::W<EXTI_TS_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_TS_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EXTI_TS_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_TS_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSSEL` reader - TSSEL"]
pub type TSSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSSEL` writer - TSSEL"]
pub type TSSEL_W<'a> = crate::FieldWriter<'a, u32, EXTI_TS_SEL_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 0:3 - TSSEL"]
    #[inline(always)]
    pub fn tssel(&self) -> TSSEL_R {
        TSSEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TSSEL"]
    #[inline(always)]
    pub fn tssel(&mut self) -> TSSEL_W {
        TSSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI_TS_SEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_ts_sel](index.html) module"]
pub struct EXTI_TS_SEL_SPEC;
impl crate::RegisterSpec for EXTI_TS_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_ts_sel::R](R) reader structure"]
impl crate::Readable for EXTI_TS_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_ts_sel::W](W) writer structure"]
impl crate::Writable for EXTI_TS_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTI_TS_SEL to value 0"]
impl crate::Resettable for EXTI_TS_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
