#[doc = "Register `RTC_CALIB` reader"]
pub struct R(crate::R<RTC_CALIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CALIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CALIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CALIB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CALIB` writer"]
pub struct W(crate::W<RTC_CALIB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CALIB_SPEC>;
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
impl From<crate::W<RTC_CALIB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CALIB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CM` reader - CM"]
pub type CM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CM` writer - CM"]
pub type CM_W<'a> = crate::FieldWriter<'a, u32, RTC_CALIB_SPEC, u16, u16, 9, 0>;
#[doc = "Field `CW16` reader - CW16"]
pub type CW16_R = crate::BitReader<bool>;
#[doc = "Field `CW16` writer - CW16"]
pub type CW16_W<'a> = crate::BitWriter<'a, u32, RTC_CALIB_SPEC, bool, 13>;
#[doc = "Field `CW8` reader - CW8"]
pub type CW8_R = crate::BitReader<bool>;
#[doc = "Field `CW8` writer - CW8"]
pub type CW8_W<'a> = crate::BitWriter<'a, u32, RTC_CALIB_SPEC, bool, 14>;
#[doc = "Field `CP` reader - CP"]
pub type CP_R = crate::BitReader<bool>;
#[doc = "Field `CP` writer - CP"]
pub type CP_W<'a> = crate::BitWriter<'a, u32, RTC_CALIB_SPEC, bool, 15>;
impl R {
    #[doc = "Bits 0:8 - CM"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 13 - CW16"]
    #[inline(always)]
    pub fn cw16(&self) -> CW16_R {
        CW16_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CW8"]
    #[inline(always)]
    pub fn cw8(&self) -> CW8_R {
        CW8_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CP"]
    #[inline(always)]
    pub fn cp(&self) -> CP_R {
        CP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - CM"]
    #[inline(always)]
    pub fn cm(&mut self) -> CM_W {
        CM_W::new(self)
    }
    #[doc = "Bit 13 - CW16"]
    #[inline(always)]
    pub fn cw16(&mut self) -> CW16_W {
        CW16_W::new(self)
    }
    #[doc = "Bit 14 - CW8"]
    #[inline(always)]
    pub fn cw8(&mut self) -> CW8_W {
        CW8_W::new(self)
    }
    #[doc = "Bit 15 - CP"]
    #[inline(always)]
    pub fn cp(&mut self) -> CP_W {
        CP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_CALIB\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_calib](index.html) module"]
pub struct RTC_CALIB_SPEC;
impl crate::RegisterSpec for RTC_CALIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_calib::R](R) reader structure"]
impl crate::Readable for RTC_CALIB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_calib::W](W) writer structure"]
impl crate::Writable for RTC_CALIB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CALIB to value 0"]
impl crate::Resettable for RTC_CALIB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
