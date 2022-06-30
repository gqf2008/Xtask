#[doc = "Register `RTC_ALARMA` reader"]
pub struct R(crate::R<RTC_ALARMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_ALARMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_ALARMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_ALARMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_ALARMA` writer"]
pub struct W(crate::W<RTC_ALARMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_ALARMA_SPEC>;
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
impl From<crate::W<RTC_ALARMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_ALARMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEU` reader - SEU"]
pub type SEU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEU` writer - SEU"]
pub type SEU_W<'a> = crate::FieldWriter<'a, u32, RTC_ALARMA_SPEC, u8, u8, 4, 0>;
#[doc = "Field `SET` reader - SET"]
pub type SET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SET` writer - SET"]
pub type SET_W<'a> = crate::FieldWriter<'a, u32, RTC_ALARMA_SPEC, u8, u8, 3, 4>;
#[doc = "Field `MASK1` reader - MASK1"]
pub type MASK1_R = crate::BitReader<bool>;
#[doc = "Field `MASK1` writer - MASK1"]
pub type MASK1_W<'a> = crate::BitWriter<'a, u32, RTC_ALARMA_SPEC, bool, 7>;
#[doc = "Field `MIU` reader - MIU"]
pub type MIU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIU` writer - MIU"]
pub type MIU_W<'a> = crate::FieldWriter<'a, u32, RTC_ALARMA_SPEC, u8, u8, 4, 8>;
#[doc = "Field `MIT` reader - MIT"]
pub type MIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIT` writer - MIT"]
pub type MIT_W<'a> = crate::FieldWriter<'a, u32, RTC_ALARMA_SPEC, u8, u8, 3, 12>;
#[doc = "Field `MASK2` reader - MASK2"]
pub type MASK2_R = crate::BitReader<bool>;
#[doc = "Field `MASK2` writer - MASK2"]
pub type MASK2_W<'a> = crate::BitWriter<'a, u32, RTC_ALARMA_SPEC, bool, 15>;
#[doc = "Field `HOU` reader - HOU"]
pub type HOU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOU` writer - HOU"]
pub type HOU_W<'a> = crate::FieldWriter<'a, u32, RTC_ALARMA_SPEC, u8, u8, 4, 16>;
#[doc = "Field `HOT` reader - HOT"]
pub type HOT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOT` writer - HOT"]
pub type HOT_W<'a> = crate::FieldWriter<'a, u32, RTC_ALARMA_SPEC, u8, u8, 2, 20>;
#[doc = "Field `APM` reader - APM"]
pub type APM_R = crate::BitReader<bool>;
#[doc = "Field `APM` writer - APM"]
pub type APM_W<'a> = crate::BitWriter<'a, u32, RTC_ALARMA_SPEC, bool, 22>;
#[doc = "Field `MASK3` reader - MASK3"]
pub type MASK3_R = crate::BitReader<bool>;
#[doc = "Field `MASK3` writer - MASK3"]
pub type MASK3_W<'a> = crate::BitWriter<'a, u32, RTC_ALARMA_SPEC, bool, 23>;
#[doc = "Field `DTU` reader - DTU"]
pub type DTU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTU` writer - DTU"]
pub type DTU_W<'a> = crate::FieldWriter<'a, u32, RTC_ALARMA_SPEC, u8, u8, 4, 24>;
#[doc = "Field `DTT` reader - DTT"]
pub type DTT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTT` writer - DTT"]
pub type DTT_W<'a> = crate::FieldWriter<'a, u32, RTC_ALARMA_SPEC, u8, u8, 2, 28>;
#[doc = "Field `WKDSEL` reader - WKDSEL"]
pub type WKDSEL_R = crate::BitReader<bool>;
#[doc = "Field `WKDSEL` writer - WKDSEL"]
pub type WKDSEL_W<'a> = crate::BitWriter<'a, u32, RTC_ALARMA_SPEC, bool, 30>;
#[doc = "Field `MASK4` reader - MASK4"]
pub type MASK4_R = crate::BitReader<bool>;
#[doc = "Field `MASK4` writer - MASK4"]
pub type MASK4_W<'a> = crate::BitWriter<'a, u32, RTC_ALARMA_SPEC, bool, 31>;
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
    #[doc = "Bit 7 - MASK1"]
    #[inline(always)]
    pub fn mask1(&self) -> MASK1_R {
        MASK1_R::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 15 - MASK2"]
    #[inline(always)]
    pub fn mask2(&self) -> MASK2_R {
        MASK2_R::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bit 23 - MASK3"]
    #[inline(always)]
    pub fn mask3(&self) -> MASK3_R {
        MASK3_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - DTU"]
    #[inline(always)]
    pub fn dtu(&self) -> DTU_R {
        DTU_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - DTT"]
    #[inline(always)]
    pub fn dtt(&self) -> DTT_R {
        DTT_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - WKDSEL"]
    #[inline(always)]
    pub fn wkdsel(&self) -> WKDSEL_R {
        WKDSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - MASK4"]
    #[inline(always)]
    pub fn mask4(&self) -> MASK4_R {
        MASK4_R::new(((self.bits >> 31) & 1) != 0)
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
    #[doc = "Bit 7 - MASK1"]
    #[inline(always)]
    pub fn mask1(&mut self) -> MASK1_W {
        MASK1_W::new(self)
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
    #[doc = "Bit 15 - MASK2"]
    #[inline(always)]
    pub fn mask2(&mut self) -> MASK2_W {
        MASK2_W::new(self)
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
    #[doc = "Bit 23 - MASK3"]
    #[inline(always)]
    pub fn mask3(&mut self) -> MASK3_W {
        MASK3_W::new(self)
    }
    #[doc = "Bits 24:27 - DTU"]
    #[inline(always)]
    pub fn dtu(&mut self) -> DTU_W {
        DTU_W::new(self)
    }
    #[doc = "Bits 28:29 - DTT"]
    #[inline(always)]
    pub fn dtt(&mut self) -> DTT_W {
        DTT_W::new(self)
    }
    #[doc = "Bit 30 - WKDSEL"]
    #[inline(always)]
    pub fn wkdsel(&mut self) -> WKDSEL_W {
        WKDSEL_W::new(self)
    }
    #[doc = "Bit 31 - MASK4"]
    #[inline(always)]
    pub fn mask4(&mut self) -> MASK4_W {
        MASK4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_ALARMA\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_alarma](index.html) module"]
pub struct RTC_ALARMA_SPEC;
impl crate::RegisterSpec for RTC_ALARMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_alarma::R](R) reader structure"]
impl crate::Readable for RTC_ALARMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_alarma::W](W) writer structure"]
impl crate::Writable for RTC_ALARMA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_ALARMA to value 0"]
impl crate::Resettable for RTC_ALARMA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
