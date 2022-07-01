#[doc = "Register `AFIO_RMP_CFG` reader"]
pub struct R(crate::R<AFIO_RMP_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIO_RMP_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIO_RMP_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIO_RMP_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFIO_RMP_CFG` writer"]
pub struct W(crate::W<AFIO_RMP_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFIO_RMP_CFG_SPEC>;
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
impl From<crate::W<AFIO_RMP_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFIO_RMP_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI1_RMP_0` reader - SPI1_RMP_0"]
pub type SPI1_RMP_0_R = crate::BitReader<bool>;
#[doc = "Field `SPI1_RMP_0` writer - SPI1_RMP_0"]
pub type SPI1_RMP_0_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG_SPEC, bool, 0>;
#[doc = "Field `I2C1_RMP` reader - I2C1_RMP"]
pub type I2C1_RMP_R = crate::BitReader<bool>;
#[doc = "Field `I2C1_RMP` writer - I2C1_RMP"]
pub type I2C1_RMP_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG_SPEC, bool, 1>;
#[doc = "Field `USART1_RMP` reader - USART1_RMP"]
pub type USART1_RMP_R = crate::BitReader<bool>;
#[doc = "Field `USART1_RMP` writer - USART1_RMP"]
pub type USART1_RMP_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG_SPEC, bool, 2>;
#[doc = "Field `USART2_RMP_0` reader - USART2_RMP_0"]
pub type USART2_RMP_0_R = crate::BitReader<bool>;
#[doc = "Field `USART2_RMP_0` writer - USART2_RMP_0"]
pub type USART2_RMP_0_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG_SPEC, bool, 3>;
#[doc = "Field `USART3_RMP` reader - USART3_RMP"]
pub type USART3_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USART3_RMP` writer - USART3_RMP"]
pub type USART3_RMP_W<'a> = crate::FieldWriter<'a, u32, AFIO_RMP_CFG_SPEC, u8, u8, 2, 4>;
#[doc = "Field `TIM1_RMP` reader - TIM1_RMP"]
pub type TIM1_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM1_RMP` writer - TIM1_RMP"]
pub type TIM1_RMP_W<'a> = crate::FieldWriter<'a, u32, AFIO_RMP_CFG_SPEC, u8, u8, 2, 6>;
#[doc = "Field `TIM2_RMP` reader - TIM2_RMP"]
pub type TIM2_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM2_RMP` writer - TIM2_RMP"]
pub type TIM2_RMP_W<'a> = crate::FieldWriter<'a, u32, AFIO_RMP_CFG_SPEC, u8, u8, 2, 8>;
#[doc = "Field `TIM3_RMP` reader - TIM3_RMP"]
pub type TIM3_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM3_RMP` writer - TIM3_RMP"]
pub type TIM3_RMP_W<'a> = crate::FieldWriter<'a, u32, AFIO_RMP_CFG_SPEC, u8, u8, 2, 10>;
#[doc = "Field `TIM4_RMP` reader - TIM4_RMP"]
pub type TIM4_RMP_R = crate::BitReader<bool>;
#[doc = "Field `TIM4_RMP` writer - TIM4_RMP"]
pub type TIM4_RMP_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG_SPEC, bool, 12>;
#[doc = "Field `CAN1_RMP` reader - CAN1_RMP"]
pub type CAN1_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAN1_RMP` writer - CAN1_RMP"]
pub type CAN1_RMP_W<'a> = crate::FieldWriter<'a, u32, AFIO_RMP_CFG_SPEC, u8, u8, 2, 13>;
#[doc = "Field `PD01_RMP` reader - PD01_RMP"]
pub type PD01_RMP_R = crate::BitReader<bool>;
#[doc = "Field `PD01_RMP` writer - PD01_RMP"]
pub type PD01_RMP_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG_SPEC, bool, 15>;
#[doc = "Field `TIM5CH4_RMP` reader - TIM5CH4_RMP"]
pub type TIM5CH4_RMP_R = crate::BitReader<bool>;
#[doc = "Field `TIM5CH4_RMP` writer - TIM5CH4_RMP"]
pub type TIM5CH4_RMP_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG_SPEC, bool, 16>;
#[doc = "Field `ADC1_ETRI` reader - ADC1_ETRI"]
pub type ADC1_ETRI_R = crate::BitReader<bool>;
#[doc = "Field `ADC1_ETRI` writer - ADC1_ETRI"]
pub type ADC1_ETRI_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG_SPEC, bool, 17>;
#[doc = "Field `ADC1_ETRR` reader - ADC1_ETRR"]
pub type ADC1_ETRR_R = crate::BitReader<bool>;
#[doc = "Field `ADC1_ETRR` writer - ADC1_ETRR"]
pub type ADC1_ETRR_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG_SPEC, bool, 18>;
#[doc = "Field `ADC2_ETRI` reader - ADC2_ETRI"]
pub type ADC2_ETRI_R = crate::BitReader<bool>;
#[doc = "Field `ADC2_ETRI` writer - ADC2_ETRI"]
pub type ADC2_ETRI_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG_SPEC, bool, 19>;
#[doc = "Field `ADC2_ETRR` reader - ADC2_ETRR"]
pub type ADC2_ETRR_R = crate::BitReader<bool>;
#[doc = "Field `ADC2_ETRR` writer - ADC2_ETRR"]
pub type ADC2_ETRR_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG_SPEC, bool, 20>;
#[doc = "Field `SW_JTAG_CFG` reader - SW_JTAG_CFG"]
pub type SW_JTAG_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW_JTAG_CFG` writer - SW_JTAG_CFG"]
pub type SW_JTAG_CFG_W<'a> = crate::FieldWriter<'a, u32, AFIO_RMP_CFG_SPEC, u8, u8, 3, 24>;
impl R {
    #[doc = "Bit 0 - SPI1_RMP_0"]
    #[inline(always)]
    pub fn spi1_rmp_0(&self) -> SPI1_RMP_0_R {
        SPI1_RMP_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C1_RMP"]
    #[inline(always)]
    pub fn i2c1_rmp(&self) -> I2C1_RMP_R {
        I2C1_RMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USART1_RMP"]
    #[inline(always)]
    pub fn usart1_rmp(&self) -> USART1_RMP_R {
        USART1_RMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USART2_RMP_0"]
    #[inline(always)]
    pub fn usart2_rmp_0(&self) -> USART2_RMP_0_R {
        USART2_RMP_0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - USART3_RMP"]
    #[inline(always)]
    pub fn usart3_rmp(&self) -> USART3_RMP_R {
        USART3_RMP_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - TIM1_RMP"]
    #[inline(always)]
    pub fn tim1_rmp(&self) -> TIM1_RMP_R {
        TIM1_RMP_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TIM2_RMP"]
    #[inline(always)]
    pub fn tim2_rmp(&self) -> TIM2_RMP_R {
        TIM2_RMP_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - TIM3_RMP"]
    #[inline(always)]
    pub fn tim3_rmp(&self) -> TIM3_RMP_R {
        TIM3_RMP_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - TIM4_RMP"]
    #[inline(always)]
    pub fn tim4_rmp(&self) -> TIM4_RMP_R {
        TIM4_RMP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - CAN1_RMP"]
    #[inline(always)]
    pub fn can1_rmp(&self) -> CAN1_RMP_R {
        CAN1_RMP_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - PD01_RMP"]
    #[inline(always)]
    pub fn pd01_rmp(&self) -> PD01_RMP_R {
        PD01_RMP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM5CH4_RMP"]
    #[inline(always)]
    pub fn tim5ch4_rmp(&self) -> TIM5CH4_RMP_R {
        TIM5CH4_RMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC1_ETRI"]
    #[inline(always)]
    pub fn adc1_etri(&self) -> ADC1_ETRI_R {
        ADC1_ETRI_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC1_ETRR"]
    #[inline(always)]
    pub fn adc1_etrr(&self) -> ADC1_ETRR_R {
        ADC1_ETRR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADC2_ETRI"]
    #[inline(always)]
    pub fn adc2_etri(&self) -> ADC2_ETRI_R {
        ADC2_ETRI_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC2_ETRR"]
    #[inline(always)]
    pub fn adc2_etrr(&self) -> ADC2_ETRR_R {
        ADC2_ETRR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SW_JTAG_CFG"]
    #[inline(always)]
    pub fn sw_jtag_cfg(&self) -> SW_JTAG_CFG_R {
        SW_JTAG_CFG_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SPI1_RMP_0"]
    #[inline(always)]
    pub fn spi1_rmp_0(&mut self) -> SPI1_RMP_0_W {
        SPI1_RMP_0_W::new(self)
    }
    #[doc = "Bit 1 - I2C1_RMP"]
    #[inline(always)]
    pub fn i2c1_rmp(&mut self) -> I2C1_RMP_W {
        I2C1_RMP_W::new(self)
    }
    #[doc = "Bit 2 - USART1_RMP"]
    #[inline(always)]
    pub fn usart1_rmp(&mut self) -> USART1_RMP_W {
        USART1_RMP_W::new(self)
    }
    #[doc = "Bit 3 - USART2_RMP_0"]
    #[inline(always)]
    pub fn usart2_rmp_0(&mut self) -> USART2_RMP_0_W {
        USART2_RMP_0_W::new(self)
    }
    #[doc = "Bits 4:5 - USART3_RMP"]
    #[inline(always)]
    pub fn usart3_rmp(&mut self) -> USART3_RMP_W {
        USART3_RMP_W::new(self)
    }
    #[doc = "Bits 6:7 - TIM1_RMP"]
    #[inline(always)]
    pub fn tim1_rmp(&mut self) -> TIM1_RMP_W {
        TIM1_RMP_W::new(self)
    }
    #[doc = "Bits 8:9 - TIM2_RMP"]
    #[inline(always)]
    pub fn tim2_rmp(&mut self) -> TIM2_RMP_W {
        TIM2_RMP_W::new(self)
    }
    #[doc = "Bits 10:11 - TIM3_RMP"]
    #[inline(always)]
    pub fn tim3_rmp(&mut self) -> TIM3_RMP_W {
        TIM3_RMP_W::new(self)
    }
    #[doc = "Bit 12 - TIM4_RMP"]
    #[inline(always)]
    pub fn tim4_rmp(&mut self) -> TIM4_RMP_W {
        TIM4_RMP_W::new(self)
    }
    #[doc = "Bits 13:14 - CAN1_RMP"]
    #[inline(always)]
    pub fn can1_rmp(&mut self) -> CAN1_RMP_W {
        CAN1_RMP_W::new(self)
    }
    #[doc = "Bit 15 - PD01_RMP"]
    #[inline(always)]
    pub fn pd01_rmp(&mut self) -> PD01_RMP_W {
        PD01_RMP_W::new(self)
    }
    #[doc = "Bit 16 - TIM5CH4_RMP"]
    #[inline(always)]
    pub fn tim5ch4_rmp(&mut self) -> TIM5CH4_RMP_W {
        TIM5CH4_RMP_W::new(self)
    }
    #[doc = "Bit 17 - ADC1_ETRI"]
    #[inline(always)]
    pub fn adc1_etri(&mut self) -> ADC1_ETRI_W {
        ADC1_ETRI_W::new(self)
    }
    #[doc = "Bit 18 - ADC1_ETRR"]
    #[inline(always)]
    pub fn adc1_etrr(&mut self) -> ADC1_ETRR_W {
        ADC1_ETRR_W::new(self)
    }
    #[doc = "Bit 19 - ADC2_ETRI"]
    #[inline(always)]
    pub fn adc2_etri(&mut self) -> ADC2_ETRI_W {
        ADC2_ETRI_W::new(self)
    }
    #[doc = "Bit 20 - ADC2_ETRR"]
    #[inline(always)]
    pub fn adc2_etrr(&mut self) -> ADC2_ETRR_W {
        ADC2_ETRR_W::new(self)
    }
    #[doc = "Bits 24:26 - SW_JTAG_CFG"]
    #[inline(always)]
    pub fn sw_jtag_cfg(&mut self) -> SW_JTAG_CFG_W {
        SW_JTAG_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO_RMP_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_rmp_cfg](index.html) module"]
pub struct AFIO_RMP_CFG_SPEC;
impl crate::RegisterSpec for AFIO_RMP_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afio_rmp_cfg::R](R) reader structure"]
impl crate::Readable for AFIO_RMP_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afio_rmp_cfg::W](W) writer structure"]
impl crate::Writable for AFIO_RMP_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFIO_RMP_CFG to value 0"]
impl crate::Resettable for AFIO_RMP_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
