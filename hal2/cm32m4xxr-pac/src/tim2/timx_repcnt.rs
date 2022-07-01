#[doc = "Register `TIMx_REPCNT` reader"]
pub struct R(crate::R<TIMX_REPCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_REPCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_REPCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_REPCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_REPCNT` writer"]
pub struct W(crate::W<TIMX_REPCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_REPCNT_SPEC>;
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
impl From<crate::W<TIMX_REPCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_REPCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REPCNT` reader - REPCNT"]
pub type REPCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REPCNT` writer - REPCNT"]
pub type REPCNT_W<'a> = crate::FieldWriter<'a, u32, TIMX_REPCNT_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - REPCNT"]
    #[inline(always)]
    pub fn repcnt(&self) -> REPCNT_R {
        REPCNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - REPCNT"]
    #[inline(always)]
    pub fn repcnt(&mut self) -> REPCNT_W {
        REPCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMx_REPCNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_repcnt](index.html) module"]
pub struct TIMX_REPCNT_SPEC;
impl crate::RegisterSpec for TIMX_REPCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timx_repcnt::R](R) reader structure"]
impl crate::Readable for TIMX_REPCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_repcnt::W](W) writer structure"]
impl crate::Writable for TIMX_REPCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMx_REPCNT to value 0"]
impl crate::Resettable for TIMX_REPCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
