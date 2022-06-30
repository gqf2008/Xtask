#[doc = "Register `RTC_ALRMASS` reader"]
pub struct R(crate::R<RTC_ALRMASS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_ALRMASS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_ALRMASS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_ALRMASS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_ALRMASS` writer"]
pub struct W(crate::W<RTC_ALRMASS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_ALRMASS_SPEC>;
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
impl From<crate::W<RTC_ALRMASS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_ALRMASS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSV` reader - SSV"]
pub type SSV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SSV` writer - SSV"]
pub type SSV_W<'a> = crate::FieldWriter<'a, u32, RTC_ALRMASS_SPEC, u16, u16, 15, 0>;
#[doc = "Field `MASKSSB` reader - MASKSSB"]
pub type MASKSSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASKSSB` writer - MASKSSB"]
pub type MASKSSB_W<'a> = crate::FieldWriter<'a, u32, RTC_ALRMASS_SPEC, u8, u8, 4, 24>;
impl R {
    #[doc = "Bits 0:14 - SSV"]
    #[inline(always)]
    pub fn ssv(&self) -> SSV_R {
        SSV_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 24:27 - MASKSSB"]
    #[inline(always)]
    pub fn maskssb(&self) -> MASKSSB_R {
        MASKSSB_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - SSV"]
    #[inline(always)]
    pub fn ssv(&mut self) -> SSV_W {
        SSV_W::new(self)
    }
    #[doc = "Bits 24:27 - MASKSSB"]
    #[inline(always)]
    pub fn maskssb(&mut self) -> MASKSSB_W {
        MASKSSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_ALRMASS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_alrmass](index.html) module"]
pub struct RTC_ALRMASS_SPEC;
impl crate::RegisterSpec for RTC_ALRMASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_alrmass::R](R) reader structure"]
impl crate::Readable for RTC_ALRMASS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_alrmass::W](W) writer structure"]
impl crate::Writable for RTC_ALRMASS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_ALRMASS to value 0"]
impl crate::Resettable for RTC_ALRMASS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
