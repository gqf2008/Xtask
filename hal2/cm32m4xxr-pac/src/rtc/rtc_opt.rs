#[doc = "Register `RTC_OPT` reader"]
pub struct R(crate::R<RTC_OPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_OPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_OPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_OPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_OPT` writer"]
pub struct W(crate::W<RTC_OPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_OPT_SPEC>;
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
impl From<crate::W<RTC_OPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_OPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TYPE` reader - TYPE"]
pub type TYPE_R = crate::BitReader<bool>;
#[doc = "Field `TYPE` writer - TYPE"]
pub type TYPE_W<'a> = crate::BitWriter<'a, u32, RTC_OPT_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - TYPE"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TYPE"]
    #[inline(always)]
    pub fn type_(&mut self) -> TYPE_W {
        TYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_OPT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_opt](index.html) module"]
pub struct RTC_OPT_SPEC;
impl crate::RegisterSpec for RTC_OPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_opt::R](R) reader structure"]
impl crate::Readable for RTC_OPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_opt::W](W) writer structure"]
impl crate::Writable for RTC_OPT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_OPT to value 0"]
impl crate::Resettable for RTC_OPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
