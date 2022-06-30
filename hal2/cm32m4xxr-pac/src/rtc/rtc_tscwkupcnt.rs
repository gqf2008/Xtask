#[doc = "Register `RTC_TSCWKUPCNT` reader"]
pub struct R(crate::R<RTC_TSCWKUPCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TSCWKUPCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TSCWKUPCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TSCWKUPCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_TSCWKUPCNT` writer"]
pub struct W(crate::W<RTC_TSCWKUPCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TSCWKUPCNT_SPEC>;
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
impl From<crate::W<RTC_TSCWKUPCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_TSCWKUPCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - CNT"]
pub type CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CNT` writer - CNT"]
pub type CNT_W<'a> = crate::FieldWriter<'a, u32, RTC_TSCWKUPCNT_SPEC, u16, u16, 14, 0>;
impl R {
    #[doc = "Bits 0:13 - CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - CNT"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W {
        CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_TSCWKUPCNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_tscwkupcnt](index.html) module"]
pub struct RTC_TSCWKUPCNT_SPEC;
impl crate::RegisterSpec for RTC_TSCWKUPCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_tscwkupcnt::R](R) reader structure"]
impl crate::Readable for RTC_TSCWKUPCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_tscwkupcnt::W](W) writer structure"]
impl crate::Writable for RTC_TSCWKUPCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_TSCWKUPCNT to value 0x02fe"]
impl crate::Resettable for RTC_TSCWKUPCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02fe
    }
}
