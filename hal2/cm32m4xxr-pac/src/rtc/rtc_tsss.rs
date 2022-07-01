#[doc = "Register `RTC_TSSS` reader"]
pub struct R(crate::R<RTC_TSSS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TSSS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TSSS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TSSS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_TSSS` writer"]
pub struct W(crate::W<RTC_TSSS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TSSS_SPEC>;
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
impl From<crate::W<RTC_TSSS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_TSSS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSE` reader - SSE"]
pub type SSE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SSE` writer - SSE"]
pub type SSE_W<'a> = crate::FieldWriter<'a, u32, RTC_TSSS_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - SSE"]
    #[inline(always)]
    pub fn sse(&self) -> SSE_R {
        SSE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SSE"]
    #[inline(always)]
    pub fn sse(&mut self) -> SSE_W {
        SSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_TSSS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_tsss](index.html) module"]
pub struct RTC_TSSS_SPEC;
impl crate::RegisterSpec for RTC_TSSS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_tsss::R](R) reader structure"]
impl crate::Readable for RTC_TSSS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_tsss::W](W) writer structure"]
impl crate::Writable for RTC_TSSS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_TSSS to value 0"]
impl crate::Resettable for RTC_TSSS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
