#[doc = "Register `TIMx_AR` reader"]
pub struct R(crate::R<TIMX_AR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_AR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_AR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_AR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_AR` writer"]
pub struct W(crate::W<TIMX_AR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_AR_SPEC>;
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
impl From<crate::W<TIMX_AR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_AR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AR` reader - AR"]
pub type AR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `AR` writer - AR"]
pub type AR_W<'a> = crate::FieldWriter<'a, u32, TIMX_AR_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - AR"]
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - AR"]
    #[inline(always)]
    pub fn ar(&mut self) -> AR_W {
        AR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMx_AR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ar](index.html) module"]
pub struct TIMX_AR_SPEC;
impl crate::RegisterSpec for TIMX_AR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timx_ar::R](R) reader structure"]
impl crate::Readable for TIMX_AR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_ar::W](W) writer structure"]
impl crate::Writable for TIMX_AR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMx_AR to value 0"]
impl crate::Resettable for TIMX_AR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
