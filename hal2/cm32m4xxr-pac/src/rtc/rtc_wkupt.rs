#[doc = "Register `RTC_WKUPT` reader"]
pub struct R(crate::R<RTC_WKUPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_WKUPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_WKUPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_WKUPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_WKUPT` writer"]
pub struct W(crate::W<RTC_WKUPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_WKUPT_SPEC>;
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
impl From<crate::W<RTC_WKUPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_WKUPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUPT` reader - WKUPT"]
pub type WKUPT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WKUPT` writer - WKUPT"]
pub type WKUPT_W<'a> = crate::FieldWriter<'a, u32, RTC_WKUPT_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - WKUPT"]
    #[inline(always)]
    pub fn wkupt(&self) -> WKUPT_R {
        WKUPT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WKUPT"]
    #[inline(always)]
    pub fn wkupt(&mut self) -> WKUPT_W {
        WKUPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_WKUPT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_wkupt](index.html) module"]
pub struct RTC_WKUPT_SPEC;
impl crate::RegisterSpec for RTC_WKUPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_wkupt::R](R) reader structure"]
impl crate::Readable for RTC_WKUPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_wkupt::W](W) writer structure"]
impl crate::Writable for RTC_WKUPT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_WKUPT to value 0xffff"]
impl crate::Resettable for RTC_WKUPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
