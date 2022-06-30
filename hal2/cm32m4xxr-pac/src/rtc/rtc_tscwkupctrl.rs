#[doc = "Register `RTC_TSCWKUPCTRL` reader"]
pub struct R(crate::R<RTC_TSCWKUPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TSCWKUPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TSCWKUPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TSCWKUPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_TSCWKUPCTRL` writer"]
pub struct W(crate::W<RTC_TSCWKUPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TSCWKUPCTRL_SPEC>;
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
impl From<crate::W<RTC_TSCWKUPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_TSCWKUPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUPEN` reader - WKUPEN"]
pub type WKUPEN_R = crate::BitReader<bool>;
#[doc = "Field `WKUPEN` writer - WKUPEN"]
pub type WKUPEN_W<'a> = crate::BitWriter<'a, u32, RTC_TSCWKUPCTRL_SPEC, bool, 0>;
#[doc = "Field `WKUPCNF` reader - WKUPCNF"]
pub type WKUPCNF_R = crate::BitReader<bool>;
#[doc = "Field `WKUPCNF` writer - WKUPCNF"]
pub type WKUPCNF_W<'a> = crate::BitWriter<'a, u32, RTC_TSCWKUPCTRL_SPEC, bool, 2>;
#[doc = "Field `WKUPOFF` reader - WKUPOFF"]
pub type WKUPOFF_R = crate::BitReader<bool>;
#[doc = "Field `WKUPOFF` writer - WKUPOFF"]
pub type WKUPOFF_W<'a> = crate::BitWriter<'a, u32, RTC_TSCWKUPCTRL_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - WKUPEN"]
    #[inline(always)]
    pub fn wkupen(&self) -> WKUPEN_R {
        WKUPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - WKUPCNF"]
    #[inline(always)]
    pub fn wkupcnf(&self) -> WKUPCNF_R {
        WKUPCNF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WKUPOFF"]
    #[inline(always)]
    pub fn wkupoff(&self) -> WKUPOFF_R {
        WKUPOFF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WKUPEN"]
    #[inline(always)]
    pub fn wkupen(&mut self) -> WKUPEN_W {
        WKUPEN_W::new(self)
    }
    #[doc = "Bit 2 - WKUPCNF"]
    #[inline(always)]
    pub fn wkupcnf(&mut self) -> WKUPCNF_W {
        WKUPCNF_W::new(self)
    }
    #[doc = "Bit 3 - WKUPOFF"]
    #[inline(always)]
    pub fn wkupoff(&mut self) -> WKUPOFF_W {
        WKUPOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_TSCWKUPCTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_tscwkupctrl](index.html) module"]
pub struct RTC_TSCWKUPCTRL_SPEC;
impl crate::RegisterSpec for RTC_TSCWKUPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_tscwkupctrl::R](R) reader structure"]
impl crate::Readable for RTC_TSCWKUPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_tscwkupctrl::W](W) writer structure"]
impl crate::Writable for RTC_TSCWKUPCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_TSCWKUPCTRL to value 0x08"]
impl crate::Resettable for RTC_TSCWKUPCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
