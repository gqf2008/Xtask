#[doc = "Register `RTC_TSH` reader"]
pub struct R(crate::R<RTC_TSH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TSH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TSH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TSH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_TSH` writer"]
pub struct W(crate::W<RTC_TSH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TSH_SPEC>;
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
impl From<crate::W<RTC_TSH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_TSH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCU` reader - SCU"]
pub type SCU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCU` writer - SCU"]
pub type SCU_W<'a> = crate::FieldWriter<'a, u32, RTC_TSH_SPEC, u8, u8, 4, 0>;
#[doc = "Field `SCT` reader - SCT"]
pub type SCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCT` writer - SCT"]
pub type SCT_W<'a> = crate::FieldWriter<'a, u32, RTC_TSH_SPEC, u8, u8, 3, 4>;
#[doc = "Field `MIU` reader - MIU"]
pub type MIU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIU` writer - MIU"]
pub type MIU_W<'a> = crate::FieldWriter<'a, u32, RTC_TSH_SPEC, u8, u8, 4, 8>;
#[doc = "Field `MIT` reader - MIT"]
pub type MIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIT` writer - MIT"]
pub type MIT_W<'a> = crate::FieldWriter<'a, u32, RTC_TSH_SPEC, u8, u8, 3, 12>;
#[doc = "Field `HOU` reader - HOU"]
pub type HOU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOU` writer - HOU"]
pub type HOU_W<'a> = crate::FieldWriter<'a, u32, RTC_TSH_SPEC, u8, u8, 4, 16>;
#[doc = "Field `HOT` reader - HOT"]
pub type HOT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOT` writer - HOT"]
pub type HOT_W<'a> = crate::FieldWriter<'a, u32, RTC_TSH_SPEC, u8, u8, 2, 20>;
#[doc = "Field `APM` reader - APM"]
pub type APM_R = crate::BitReader<bool>;
#[doc = "Field `APM` writer - APM"]
pub type APM_W<'a> = crate::BitWriter<'a, u32, RTC_TSH_SPEC, bool, 22>;
impl R {
    #[doc = "Bits 0:3 - SCU"]
    #[inline(always)]
    pub fn scu(&self) -> SCU_R {
        SCU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - SCT"]
    #[inline(always)]
    pub fn sct(&self) -> SCT_R {
        SCT_R::new(((self.bits >> 4) & 7) as u8)
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
    #[doc = "Bits 0:3 - SCU"]
    #[inline(always)]
    pub fn scu(&mut self) -> SCU_W {
        SCU_W::new(self)
    }
    #[doc = "Bits 4:6 - SCT"]
    #[inline(always)]
    pub fn sct(&mut self) -> SCT_W {
        SCT_W::new(self)
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
#[doc = "RTC_TSH\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_tsh](index.html) module"]
pub struct RTC_TSH_SPEC;
impl crate::RegisterSpec for RTC_TSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_tsh::R](R) reader structure"]
impl crate::Readable for RTC_TSH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_tsh::W](W) writer structure"]
impl crate::Writable for RTC_TSH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_TSH to value 0"]
impl crate::Resettable for RTC_TSH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
