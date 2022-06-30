#[doc = "Register `RTC_SCTRL` reader"]
pub struct R(crate::R<RTC_SCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_SCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_SCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_SCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_SCTRL` writer"]
pub struct W(crate::W<RTC_SCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_SCTRL_SPEC>;
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
impl From<crate::W<RTC_SCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_SCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADFS` reader - ADFS"]
pub type ADFS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADFS` writer - ADFS"]
pub type ADFS_W<'a> = crate::FieldWriter<'a, u32, RTC_SCTRL_SPEC, u16, u16, 15, 0>;
#[doc = "Field `SUB1S` reader - SUB1S"]
pub type SUB1S_R = crate::BitReader<bool>;
#[doc = "Field `SUB1S` writer - SUB1S"]
pub type SUB1S_W<'a> = crate::BitWriter<'a, u32, RTC_SCTRL_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:14 - ADFS"]
    #[inline(always)]
    pub fn adfs(&self) -> ADFS_R {
        ADFS_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - SUB1S"]
    #[inline(always)]
    pub fn sub1s(&self) -> SUB1S_R {
        SUB1S_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - ADFS"]
    #[inline(always)]
    pub fn adfs(&mut self) -> ADFS_W {
        ADFS_W::new(self)
    }
    #[doc = "Bit 31 - SUB1S"]
    #[inline(always)]
    pub fn sub1s(&mut self) -> SUB1S_W {
        SUB1S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_SCTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_sctrl](index.html) module"]
pub struct RTC_SCTRL_SPEC;
impl crate::RegisterSpec for RTC_SCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_sctrl::R](R) reader structure"]
impl crate::Readable for RTC_SCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_sctrl::W](W) writer structure"]
impl crate::Writable for RTC_SCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_SCTRL to value 0"]
impl crate::Resettable for RTC_SCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
