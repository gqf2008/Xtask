#[doc = "Register `TIMx_CCDAT4` reader"]
pub struct R(crate::R<TIMX_CCDAT4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_CCDAT4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_CCDAT4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_CCDAT4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_CCDAT4` writer"]
pub struct W(crate::W<TIMX_CCDAT4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_CCDAT4_SPEC>;
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
impl From<crate::W<TIMX_CCDAT4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_CCDAT4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCDAT4` reader - CCDAT4"]
pub type CCDAT4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CCDAT4` writer - CCDAT4"]
pub type CCDAT4_W<'a> = crate::FieldWriter<'a, u32, TIMX_CCDAT4_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - CCDAT4"]
    #[inline(always)]
    pub fn ccdat4(&self) -> CCDAT4_R {
        CCDAT4_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CCDAT4"]
    #[inline(always)]
    pub fn ccdat4(&mut self) -> CCDAT4_W {
        CCDAT4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMx_CCDAT4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccdat4](index.html) module"]
pub struct TIMX_CCDAT4_SPEC;
impl crate::RegisterSpec for TIMX_CCDAT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timx_ccdat4::R](R) reader structure"]
impl crate::Readable for TIMX_CCDAT4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_ccdat4::W](W) writer structure"]
impl crate::Writable for TIMX_CCDAT4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMx_CCDAT4 to value 0"]
impl crate::Resettable for TIMX_CCDAT4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
