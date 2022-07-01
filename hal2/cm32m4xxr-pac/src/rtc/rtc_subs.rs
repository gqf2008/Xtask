#[doc = "Register `RTC_SUBS` reader"]
pub struct R(crate::R<RTC_SUBS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_SUBS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_SUBS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_SUBS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_SUBS` writer"]
pub struct W(crate::W<RTC_SUBS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_SUBS_SPEC>;
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
impl From<crate::W<RTC_SUBS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_SUBS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SS` reader - SS"]
pub type SS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SS` writer - SS"]
pub type SS_W<'a> = crate::FieldWriter<'a, u32, RTC_SUBS_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - SS"]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SS"]
    #[inline(always)]
    pub fn ss(&mut self) -> SS_W {
        SS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_SUBS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_subs](index.html) module"]
pub struct RTC_SUBS_SPEC;
impl crate::RegisterSpec for RTC_SUBS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_subs::R](R) reader structure"]
impl crate::Readable for RTC_SUBS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_subs::W](W) writer structure"]
impl crate::Writable for RTC_SUBS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_SUBS to value 0"]
impl crate::Resettable for RTC_SUBS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
