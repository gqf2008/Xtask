#[doc = "Register `EXTI_EMASK` reader"]
pub struct R(crate::R<EXTI_EMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_EMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_EMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_EMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_EMASK` writer"]
pub struct W(crate::W<EXTI_EMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_EMASK_SPEC>;
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
impl From<crate::W<EXTI_EMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_EMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMASK` reader - EMASK"]
pub type EMASK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EMASK` writer - EMASK"]
pub type EMASK_W<'a> = crate::FieldWriter<'a, u32, EXTI_EMASK_SPEC, u32, u32, 22, 0>;
impl R {
    #[doc = "Bits 0:21 - EMASK"]
    #[inline(always)]
    pub fn emask(&self) -> EMASK_R {
        EMASK_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - EMASK"]
    #[inline(always)]
    pub fn emask(&mut self) -> EMASK_W {
        EMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI_EMASK\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_emask](index.html) module"]
pub struct EXTI_EMASK_SPEC;
impl crate::RegisterSpec for EXTI_EMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_emask::R](R) reader structure"]
impl crate::Readable for EXTI_EMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_emask::W](W) writer structure"]
impl crate::Writable for EXTI_EMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTI_EMASK to value 0"]
impl crate::Resettable for EXTI_EMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
