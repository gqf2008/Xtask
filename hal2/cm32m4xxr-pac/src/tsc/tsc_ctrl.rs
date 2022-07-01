#[doc = "Register `TSC_CTRL` reader"]
pub struct R(crate::R<TSC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSC_CTRL` writer"]
pub struct W(crate::W<TSC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSC_CTRL_SPEC>;
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
impl From<crate::W<TSC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DET_PERIOD` reader - DET_PERIOD"]
pub type DET_PERIOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DET_PERIOD` writer - DET_PERIOD"]
pub type DET_PERIOD_W<'a> = crate::FieldWriter<'a, u32, TSC_CTRL_SPEC, u8, u8, 4, 0>;
#[doc = "Field `DET_FILTER` reader - DET_FILTER"]
pub type DET_FILTER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DET_FILTER` writer - DET_FILTER"]
pub type DET_FILTER_W<'a> = crate::FieldWriter<'a, u32, TSC_CTRL_SPEC, u8, u8, 2, 4>;
#[doc = "Field `HW_DET_MODE` reader - HW_DET_MODE"]
pub type HW_DET_MODE_R = crate::BitReader<bool>;
#[doc = "Field `HW_DET_MODE` writer - HW_DET_MODE"]
pub type HW_DET_MODE_W<'a> = crate::BitWriter<'a, u32, TSC_CTRL_SPEC, bool, 6>;
#[doc = "Field `HW_DET_ST` reader - HW_DET_ST"]
pub type HW_DET_ST_R = crate::BitReader<bool>;
#[doc = "Field `HW_DET_ST` writer - HW_DET_ST"]
pub type HW_DET_ST_W<'a> = crate::BitWriter<'a, u32, TSC_CTRL_SPEC, bool, 7>;
#[doc = "Field `LESS_DET_SEL` reader - LESS_DET_SEL"]
pub type LESS_DET_SEL_R = crate::BitReader<bool>;
#[doc = "Field `LESS_DET_SEL` writer - LESS_DET_SEL"]
pub type LESS_DET_SEL_W<'a> = crate::BitWriter<'a, u32, TSC_CTRL_SPEC, bool, 8>;
#[doc = "Field `GREAT_DET_SEL` reader - GREAT_DET_SEL"]
pub type GREAT_DET_SEL_R = crate::BitReader<bool>;
#[doc = "Field `GREAT_DET_SEL` writer - GREAT_DET_SEL"]
pub type GREAT_DET_SEL_W<'a> = crate::BitWriter<'a, u32, TSC_CTRL_SPEC, bool, 9>;
#[doc = "Field `DET_INTEN` reader - DET_INTEN"]
pub type DET_INTEN_R = crate::BitReader<bool>;
#[doc = "Field `DET_INTEN` writer - DET_INTEN"]
pub type DET_INTEN_W<'a> = crate::BitWriter<'a, u32, TSC_CTRL_SPEC, bool, 10>;
#[doc = "Field `TIM4_ETR` reader - TIM4_ETR"]
pub type TIM4_ETR_R = crate::BitReader<bool>;
#[doc = "Field `TIM4_ETR` writer - TIM4_ETR"]
pub type TIM4_ETR_W<'a> = crate::BitWriter<'a, u32, TSC_CTRL_SPEC, bool, 12>;
#[doc = "Field `TIM2_ETR_CH1` reader - TIM2_ETR_CH1"]
pub type TIM2_ETR_CH1_R = crate::BitReader<bool>;
#[doc = "Field `TIM2_ETR_CH1` writer - TIM2_ETR_CH1"]
pub type TIM2_ETR_CH1_W<'a> = crate::BitWriter<'a, u32, TSC_CTRL_SPEC, bool, 13>;
impl R {
    #[doc = "Bits 0:3 - DET_PERIOD"]
    #[inline(always)]
    pub fn det_period(&self) -> DET_PERIOD_R {
        DET_PERIOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - DET_FILTER"]
    #[inline(always)]
    pub fn det_filter(&self) -> DET_FILTER_R {
        DET_FILTER_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - HW_DET_MODE"]
    #[inline(always)]
    pub fn hw_det_mode(&self) -> HW_DET_MODE_R {
        HW_DET_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HW_DET_ST"]
    #[inline(always)]
    pub fn hw_det_st(&self) -> HW_DET_ST_R {
        HW_DET_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LESS_DET_SEL"]
    #[inline(always)]
    pub fn less_det_sel(&self) -> LESS_DET_SEL_R {
        LESS_DET_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GREAT_DET_SEL"]
    #[inline(always)]
    pub fn great_det_sel(&self) -> GREAT_DET_SEL_R {
        GREAT_DET_SEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DET_INTEN"]
    #[inline(always)]
    pub fn det_inten(&self) -> DET_INTEN_R {
        DET_INTEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - TIM4_ETR"]
    #[inline(always)]
    pub fn tim4_etr(&self) -> TIM4_ETR_R {
        TIM4_ETR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM2_ETR_CH1"]
    #[inline(always)]
    pub fn tim2_etr_ch1(&self) -> TIM2_ETR_CH1_R {
        TIM2_ETR_CH1_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - DET_PERIOD"]
    #[inline(always)]
    pub fn det_period(&mut self) -> DET_PERIOD_W {
        DET_PERIOD_W::new(self)
    }
    #[doc = "Bits 4:5 - DET_FILTER"]
    #[inline(always)]
    pub fn det_filter(&mut self) -> DET_FILTER_W {
        DET_FILTER_W::new(self)
    }
    #[doc = "Bit 6 - HW_DET_MODE"]
    #[inline(always)]
    pub fn hw_det_mode(&mut self) -> HW_DET_MODE_W {
        HW_DET_MODE_W::new(self)
    }
    #[doc = "Bit 7 - HW_DET_ST"]
    #[inline(always)]
    pub fn hw_det_st(&mut self) -> HW_DET_ST_W {
        HW_DET_ST_W::new(self)
    }
    #[doc = "Bit 8 - LESS_DET_SEL"]
    #[inline(always)]
    pub fn less_det_sel(&mut self) -> LESS_DET_SEL_W {
        LESS_DET_SEL_W::new(self)
    }
    #[doc = "Bit 9 - GREAT_DET_SEL"]
    #[inline(always)]
    pub fn great_det_sel(&mut self) -> GREAT_DET_SEL_W {
        GREAT_DET_SEL_W::new(self)
    }
    #[doc = "Bit 10 - DET_INTEN"]
    #[inline(always)]
    pub fn det_inten(&mut self) -> DET_INTEN_W {
        DET_INTEN_W::new(self)
    }
    #[doc = "Bit 12 - TIM4_ETR"]
    #[inline(always)]
    pub fn tim4_etr(&mut self) -> TIM4_ETR_W {
        TIM4_ETR_W::new(self)
    }
    #[doc = "Bit 13 - TIM2_ETR_CH1"]
    #[inline(always)]
    pub fn tim2_etr_ch1(&mut self) -> TIM2_ETR_CH1_W {
        TIM2_ETR_CH1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSC_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsc_ctrl](index.html) module"]
pub struct TSC_CTRL_SPEC;
impl crate::RegisterSpec for TSC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsc_ctrl::R](R) reader structure"]
impl crate::Readable for TSC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsc_ctrl::W](W) writer structure"]
impl crate::Writable for TSC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSC_CTRL to value 0x0703"]
impl crate::Resettable for TSC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0703
    }
}
