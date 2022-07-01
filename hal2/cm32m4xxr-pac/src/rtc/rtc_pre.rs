#[doc = "Register `RTC_PRE` reader"]
pub struct R(crate::R<RTC_PRE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_PRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_PRE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_PRE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_PRE` writer"]
pub struct W(crate::W<RTC_PRE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_PRE_SPEC>;
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
impl From<crate::W<RTC_PRE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_PRE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVS` reader - DIVS"]
pub type DIVS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIVS` writer - DIVS"]
pub type DIVS_W<'a> = crate::FieldWriter<'a, u32, RTC_PRE_SPEC, u16, u16, 15, 0>;
#[doc = "Field `DIVA` reader - DIVA"]
pub type DIVA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVA` writer - DIVA"]
pub type DIVA_W<'a> = crate::FieldWriter<'a, u32, RTC_PRE_SPEC, u8, u8, 7, 16>;
impl R {
    #[doc = "Bits 0:14 - DIVS"]
    #[inline(always)]
    pub fn divs(&self) -> DIVS_R {
        DIVS_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:22 - DIVA"]
    #[inline(always)]
    pub fn diva(&self) -> DIVA_R {
        DIVA_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - DIVS"]
    #[inline(always)]
    pub fn divs(&mut self) -> DIVS_W {
        DIVS_W::new(self)
    }
    #[doc = "Bits 16:22 - DIVA"]
    #[inline(always)]
    pub fn diva(&mut self) -> DIVA_W {
        DIVA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_PRE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_pre](index.html) module"]
pub struct RTC_PRE_SPEC;
impl crate::RegisterSpec for RTC_PRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_pre::R](R) reader structure"]
impl crate::Readable for RTC_PRE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_pre::W](W) writer structure"]
impl crate::Writable for RTC_PRE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_PRE to value 0x007f_00ff"]
impl crate::Resettable for RTC_PRE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x007f_00ff
    }
}
