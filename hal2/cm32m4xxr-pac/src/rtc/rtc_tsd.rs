#[doc = "Register `RTC_TSD` reader"]
pub struct R(crate::R<RTC_TSD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TSD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TSD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TSD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_TSD` writer"]
pub struct W(crate::W<RTC_TSD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TSD_SPEC>;
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
impl From<crate::W<RTC_TSD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_TSD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAU` reader - DAU"]
pub type DAU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAU` writer - DAU"]
pub type DAU_W<'a> = crate::FieldWriter<'a, u32, RTC_TSD_SPEC, u8, u8, 4, 0>;
#[doc = "Field `DAT` reader - DAT"]
pub type DAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAT` writer - DAT"]
pub type DAT_W<'a> = crate::FieldWriter<'a, u32, RTC_TSD_SPEC, u8, u8, 2, 4>;
#[doc = "Field `MOU` reader - MOU"]
pub type MOU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MOU` writer - MOU"]
pub type MOU_W<'a> = crate::FieldWriter<'a, u32, RTC_TSD_SPEC, u8, u8, 4, 8>;
#[doc = "Field `MOT` reader - MOT"]
pub type MOT_R = crate::BitReader<bool>;
#[doc = "Field `MOT` writer - MOT"]
pub type MOT_W<'a> = crate::BitWriter<'a, u32, RTC_TSD_SPEC, bool, 12>;
#[doc = "Field `WDU` reader - WDU"]
pub type WDU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDU` writer - WDU"]
pub type WDU_W<'a> = crate::FieldWriter<'a, u32, RTC_TSD_SPEC, u8, u8, 3, 13>;
#[doc = "Field `YRU` reader - YRU"]
pub type YRU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YRU` writer - YRU"]
pub type YRU_W<'a> = crate::FieldWriter<'a, u32, RTC_TSD_SPEC, u8, u8, 4, 16>;
#[doc = "Field `YRT` reader - YRT"]
pub type YRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YRT` writer - YRT"]
pub type YRT_W<'a> = crate::FieldWriter<'a, u32, RTC_TSD_SPEC, u8, u8, 4, 20>;
impl R {
    #[doc = "Bits 0:3 - DAU"]
    #[inline(always)]
    pub fn dau(&self) -> DAU_R {
        DAU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - DAT"]
    #[inline(always)]
    pub fn dat(&self) -> DAT_R {
        DAT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - MOU"]
    #[inline(always)]
    pub fn mou(&self) -> MOU_R {
        MOU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - MOT"]
    #[inline(always)]
    pub fn mot(&self) -> MOT_R {
        MOT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - WDU"]
    #[inline(always)]
    pub fn wdu(&self) -> WDU_R {
        WDU_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - YRU"]
    #[inline(always)]
    pub fn yru(&self) -> YRU_R {
        YRU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - YRT"]
    #[inline(always)]
    pub fn yrt(&self) -> YRT_R {
        YRT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DAU"]
    #[inline(always)]
    pub fn dau(&mut self) -> DAU_W {
        DAU_W::new(self)
    }
    #[doc = "Bits 4:5 - DAT"]
    #[inline(always)]
    pub fn dat(&mut self) -> DAT_W {
        DAT_W::new(self)
    }
    #[doc = "Bits 8:11 - MOU"]
    #[inline(always)]
    pub fn mou(&mut self) -> MOU_W {
        MOU_W::new(self)
    }
    #[doc = "Bit 12 - MOT"]
    #[inline(always)]
    pub fn mot(&mut self) -> MOT_W {
        MOT_W::new(self)
    }
    #[doc = "Bits 13:15 - WDU"]
    #[inline(always)]
    pub fn wdu(&mut self) -> WDU_W {
        WDU_W::new(self)
    }
    #[doc = "Bits 16:19 - YRU"]
    #[inline(always)]
    pub fn yru(&mut self) -> YRU_W {
        YRU_W::new(self)
    }
    #[doc = "Bits 20:23 - YRT"]
    #[inline(always)]
    pub fn yrt(&mut self) -> YRT_W {
        YRT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_TSD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_tsd](index.html) module"]
pub struct RTC_TSD_SPEC;
impl crate::RegisterSpec for RTC_TSD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_tsd::R](R) reader structure"]
impl crate::Readable for RTC_TSD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_tsd::W](W) writer structure"]
impl crate::Writable for RTC_TSD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_TSD to value 0"]
impl crate::Resettable for RTC_TSD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
