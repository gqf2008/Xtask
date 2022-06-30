#[doc = "Register `RTC_TST` reader"]
pub struct R(crate::R<RTC_TST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_TST` writer"]
pub struct W(crate::W<RTC_TST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TST_SPEC>;
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
impl From<crate::W<RTC_TST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_TST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEU` reader - SEU"]
pub type SEU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEU` writer - SEU"]
pub type SEU_W<'a> = crate::FieldWriter<'a, u32, RTC_TST_SPEC, u8, u8, 4, 0>;
#[doc = "Field `SET` reader - SET"]
pub type SET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SET` writer - SET"]
pub type SET_W<'a> = crate::FieldWriter<'a, u32, RTC_TST_SPEC, u8, u8, 3, 4>;
#[doc = "Field `MIU` reader - MIU"]
pub type MIU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIU` writer - MIU"]
pub type MIU_W<'a> = crate::FieldWriter<'a, u32, RTC_TST_SPEC, u8, u8, 4, 8>;
#[doc = "Field `MIT` reader - MIT"]
pub type MIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIT` writer - MIT"]
pub type MIT_W<'a> = crate::FieldWriter<'a, u32, RTC_TST_SPEC, u8, u8, 3, 12>;
#[doc = "Field `HOU` reader - HOU"]
pub type HOU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOU` writer - HOU"]
pub type HOU_W<'a> = crate::FieldWriter<'a, u32, RTC_TST_SPEC, u8, u8, 4, 16>;
#[doc = "Field `HOT` reader - HOT"]
pub type HOT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOT` writer - HOT"]
pub type HOT_W<'a> = crate::FieldWriter<'a, u32, RTC_TST_SPEC, u8, u8, 2, 20>;
#[doc = "Field `APM` reader - APM"]
pub type APM_R = crate::BitReader<bool>;
#[doc = "Field `APM` writer - APM"]
pub type APM_W<'a> = crate::BitWriter<'a, u32, RTC_TST_SPEC, bool, 22>;
impl R {
    #[doc = "Bits 0:3 - SEU"]
    #[inline(always)]
    pub fn seu(&self) -> SEU_R {
        SEU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - SET"]
    #[inline(always)]
    pub fn set(&self) -> SET_R {
        SET_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - MIU"]
    #[inline(always)]
    pub fn miu(&self) -> MIU_R {
        MIU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - MIT"]
    #[inline(always)]
    pub fn mit(&self) -> MIT_R {
        MIT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - HOU"]
    #[inline(always)]
    pub fn hou(&self) -> HOU_R {
        HOU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - HOT"]
    #[inline(always)]
    pub fn hot(&self) -> HOT_R {
        HOT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - APM"]
    #[inline(always)]
    pub fn apm(&self) -> APM_R {
        APM_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - SEU"]
    #[inline(always)]
    pub fn seu(&mut self) -> SEU_W {
        SEU_W::new(self)
    }
    #[doc = "Bits 4:6 - SET"]
    #[inline(always)]
    pub fn set(&mut self) -> SET_W {
        SET_W::new(self)
    }
    #[doc = "Bits 8:11 - MIU"]
    #[inline(always)]
    pub fn miu(&mut self) -> MIU_W {
        MIU_W::new(self)
    }
    #[doc = "Bits 12:14 - MIT"]
    #[inline(always)]
    pub fn mit(&mut self) -> MIT_W {
        MIT_W::new(self)
    }
    #[doc = "Bits 16:19 - HOU"]
    #[inline(always)]
    pub fn hou(&mut self) -> HOU_W {
        HOU_W::new(self)
    }
    #[doc = "Bits 20:21 - HOT"]
    #[inline(always)]
    pub fn hot(&mut self) -> HOT_W {
        HOT_W::new(self)
    }
    #[doc = "Bit 22 - APM"]
    #[inline(always)]
    pub fn apm(&mut self) -> APM_W {
        APM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_TST\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_tst](index.html) module"]
pub struct RTC_TST_SPEC;
impl crate::RegisterSpec for RTC_TST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_tst::R](R) reader structure"]
impl crate::Readable for RTC_TST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_tst::W](W) writer structure"]
impl crate::Writable for RTC_TST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_TST to value 0"]
impl crate::Resettable for RTC_TST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
