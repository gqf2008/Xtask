#[doc = "Register `RTC_WRP` reader"]
pub struct R(crate::R<RTC_WRP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_WRP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_WRP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_WRP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_WRP` writer"]
pub struct W(crate::W<RTC_WRP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_WRP_SPEC>;
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
impl From<crate::W<RTC_WRP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_WRP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PKEY` reader - PKEY"]
pub type PKEY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PKEY` writer - PKEY"]
pub type PKEY_W<'a> = crate::FieldWriter<'a, u32, RTC_WRP_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - PKEY"]
    #[inline(always)]
    pub fn pkey(&self) -> PKEY_R {
        PKEY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PKEY"]
    #[inline(always)]
    pub fn pkey(&mut self) -> PKEY_W {
        PKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_WRP\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_wrp](index.html) module"]
pub struct RTC_WRP_SPEC;
impl crate::RegisterSpec for RTC_WRP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_wrp::R](R) reader structure"]
impl crate::Readable for RTC_WRP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_wrp::W](W) writer structure"]
impl crate::Writable for RTC_WRP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_WRP to value 0"]
impl crate::Resettable for RTC_WRP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
