#[doc = "Register `TIMx_CCDAT6` reader"]
pub struct R(crate::R<TIMX_CCDAT6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_CCDAT6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_CCDAT6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_CCDAT6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_CCDAT6` writer"]
pub struct W(crate::W<TIMX_CCDAT6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_CCDAT6_SPEC>;
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
impl From<crate::W<TIMX_CCDAT6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_CCDAT6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCDAT6` reader - CCDAT6"]
pub type CCDAT6_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CCDAT6` writer - CCDAT6"]
pub type CCDAT6_W<'a> = crate::FieldWriter<'a, u32, TIMX_CCDAT6_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - CCDAT6"]
    #[inline(always)]
    pub fn ccdat6(&self) -> CCDAT6_R {
        CCDAT6_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CCDAT6"]
    #[inline(always)]
    pub fn ccdat6(&mut self) -> CCDAT6_W {
        CCDAT6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMx_CCDAT6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccdat6](index.html) module"]
pub struct TIMX_CCDAT6_SPEC;
impl crate::RegisterSpec for TIMX_CCDAT6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timx_ccdat6::R](R) reader structure"]
impl crate::Readable for TIMX_CCDAT6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_ccdat6::W](W) writer structure"]
impl crate::Writable for TIMX_CCDAT6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMx_CCDAT6 to value 0"]
impl crate::Resettable for TIMX_CCDAT6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
