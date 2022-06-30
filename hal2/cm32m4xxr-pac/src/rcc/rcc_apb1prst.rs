#[doc = "Register `RCC_APB1PRST` reader"]
pub struct R(crate::R<RCC_APB1PRST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB1PRST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB1PRST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB1PRST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_APB1PRST` writer"]
pub struct W(crate::W<RCC_APB1PRST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB1PRST_SPEC>;
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
impl From<crate::W<RCC_APB1PRST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB1PRST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2RST` reader - TIM2RST"]
pub type TIM2RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM2RST` writer - TIM2RST"]
pub type TIM2RST_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PRST_SPEC, bool, 0>;
#[doc = "Field `TIM3RST` reader - TIM3RST"]
pub type TIM3RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM3RST` writer - TIM3RST"]
pub type TIM3RST_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PRST_SPEC, bool, 1>;
#[doc = "Field `TIM4RST` reader - TIM4RST"]
pub type TIM4RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM4RST` writer - TIM4RST"]
pub type TIM4RST_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PRST_SPEC, bool, 2>;
#[doc = "Field `TIM5RST` reader - TIM5RST"]
pub type TIM5RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM5RST` writer - TIM5RST"]
pub type TIM5RST_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PRST_SPEC, bool, 3>;
#[doc = "Field `TIM6RST` reader - TIM6RST"]
pub type TIM6RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM6RST` writer - TIM6RST"]
pub type TIM6RST_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PRST_SPEC, bool, 4>;
#[doc = "Field `TIM7RST` reader - TIM7RST"]
pub type TIM7RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM7RST` writer - TIM7RST"]
pub type TIM7RST_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PRST_SPEC, bool, 5>;
#[doc = "Field `TSCRST` reader - TSCRST"]
pub type TSCRST_R = crate::BitReader<bool>;
#[doc = "Field `TSCRST` writer - TSCRST"]
pub type TSCRST_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PRST_SPEC, bool, 10>;
#[doc = "Field `WWDGRST` reader - WWDGRST"]
pub type WWDGRST_R = crate::BitReader<bool>;
#[doc = "Field `WWDGRST` writer - WWDGRST"]
pub type WWDGRST_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PRST_SPEC, bool, 11>;
#[doc = "Field `SPI2RST` reader - SPI2RST"]
pub type SPI2RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI2RST` writer - SPI2RST"]
pub type SPI2RST_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PRST_SPEC, bool, 14>;
#[doc = "Field `SPI3RST` reader - SPI3RST"]
pub type SPI3RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI3RST` writer - SPI3RST"]
pub type SPI3RST_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PRST_SPEC, bool, 15>;
#[doc = "Field `USART2RST` reader - USART2RST"]
pub type USART2RST_R = crate::BitReader<bool>;
#[doc = "Field `USART2RST` writer - USART2RST"]
pub type USART2RST_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PRST_SPEC, bool, 17>;
#[doc = "Field `USART3RST` reader - USART3RST"]
pub type USART3RST_R = crate::BitReader<bool>;
#[doc = "Field `USART3RST` writer - USART3RST"]
pub type USART3RST_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PRST_SPEC, bool, 18>;
#[doc = "Field `UART4RST` reader - UART4RST"]
pub type UART4RST_R = crate::BitReader<bool>;
#[doc = "Field `UART4RST` writer - UART4RST"]
pub type UART4RST_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PRST_SPEC, bool, 19>;
#[doc = "Field `UART5RST` reader - UART5RST"]
pub type UART5RST_R = crate::BitReader<bool>;
#[doc = "Field `UART5RST` writer - UART5RST"]
pub type UART5RST_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PRST_SPEC, bool, 20>;
#[doc = "Field `I2C1RST` reader - I2C1RST"]
pub type I2C1RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C1RST` writer - I2C1RST"]
pub type I2C1RST_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PRST_SPEC, bool, 21>;
#[doc = "Field `I2C2RST` reader - I2C2RST"]
pub type I2C2RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C2RST` writer - I2C2RST"]
pub type I2C2RST_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PRST_SPEC, bool, 22>;
#[doc = "Field `USBRST` reader - USBRST"]
pub type USBRST_R = crate::BitReader<bool>;
#[doc = "Field `USBRST` writer - USBRST"]
pub type USBRST_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PRST_SPEC, bool, 23>;
#[doc = "Field `CAN1RST` reader - CAN1RST"]
pub type CAN1RST_R = crate::BitReader<bool>;
#[doc = "Field `CAN1RST` writer - CAN1RST"]
pub type CAN1RST_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PRST_SPEC, bool, 25>;
#[doc = "Field `CAN2RST` reader - CAN2RST"]
pub type CAN2RST_R = crate::BitReader<bool>;
#[doc = "Field `CAN2RST` writer - CAN2RST"]
pub type CAN2RST_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PRST_SPEC, bool, 26>;
#[doc = "Field `BKPRST` reader - BKPRST"]
pub type BKPRST_R = crate::BitReader<bool>;
#[doc = "Field `BKPRST` writer - BKPRST"]
pub type BKPRST_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PRST_SPEC, bool, 27>;
#[doc = "Field `PWRRST` reader - PWRRST"]
pub type PWRRST_R = crate::BitReader<bool>;
#[doc = "Field `PWRRST` writer - PWRRST"]
pub type PWRRST_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PRST_SPEC, bool, 28>;
#[doc = "Field `DACRST` reader - DACRST"]
pub type DACRST_R = crate::BitReader<bool>;
#[doc = "Field `DACRST` writer - DACRST"]
pub type DACRST_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PRST_SPEC, bool, 29>;
impl R {
    #[doc = "Bit 0 - TIM2RST"]
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3RST"]
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4RST"]
    #[inline(always)]
    pub fn tim4rst(&self) -> TIM4RST_R {
        TIM4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5RST"]
    #[inline(always)]
    pub fn tim5rst(&self) -> TIM5RST_R {
        TIM5RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6RST"]
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7RST"]
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - TSCRST"]
    #[inline(always)]
    pub fn tscrst(&self) -> TSCRST_R {
        TSCRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDGRST"]
    #[inline(always)]
    pub fn wwdgrst(&self) -> WWDGRST_R {
        WWDGRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2RST"]
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3RST"]
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2RST"]
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3RST"]
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4RST"]
    #[inline(always)]
    pub fn uart4rst(&self) -> UART4RST_R {
        UART4RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5RST"]
    #[inline(always)]
    pub fn uart5rst(&self) -> UART5RST_R {
        UART5RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1RST"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2RST"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USBRST"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1RST"]
    #[inline(always)]
    pub fn can1rst(&self) -> CAN1RST_R {
        CAN1RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN2RST"]
    #[inline(always)]
    pub fn can2rst(&self) -> CAN2RST_R {
        CAN2RST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - BKPRST"]
    #[inline(always)]
    pub fn bkprst(&self) -> BKPRST_R {
        BKPRST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PWRRST"]
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DACRST"]
    #[inline(always)]
    pub fn dacrst(&self) -> DACRST_R {
        DACRST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2RST"]
    #[inline(always)]
    pub fn tim2rst(&mut self) -> TIM2RST_W {
        TIM2RST_W::new(self)
    }
    #[doc = "Bit 1 - TIM3RST"]
    #[inline(always)]
    pub fn tim3rst(&mut self) -> TIM3RST_W {
        TIM3RST_W::new(self)
    }
    #[doc = "Bit 2 - TIM4RST"]
    #[inline(always)]
    pub fn tim4rst(&mut self) -> TIM4RST_W {
        TIM4RST_W::new(self)
    }
    #[doc = "Bit 3 - TIM5RST"]
    #[inline(always)]
    pub fn tim5rst(&mut self) -> TIM5RST_W {
        TIM5RST_W::new(self)
    }
    #[doc = "Bit 4 - TIM6RST"]
    #[inline(always)]
    pub fn tim6rst(&mut self) -> TIM6RST_W {
        TIM6RST_W::new(self)
    }
    #[doc = "Bit 5 - TIM7RST"]
    #[inline(always)]
    pub fn tim7rst(&mut self) -> TIM7RST_W {
        TIM7RST_W::new(self)
    }
    #[doc = "Bit 10 - TSCRST"]
    #[inline(always)]
    pub fn tscrst(&mut self) -> TSCRST_W {
        TSCRST_W::new(self)
    }
    #[doc = "Bit 11 - WWDGRST"]
    #[inline(always)]
    pub fn wwdgrst(&mut self) -> WWDGRST_W {
        WWDGRST_W::new(self)
    }
    #[doc = "Bit 14 - SPI2RST"]
    #[inline(always)]
    pub fn spi2rst(&mut self) -> SPI2RST_W {
        SPI2RST_W::new(self)
    }
    #[doc = "Bit 15 - SPI3RST"]
    #[inline(always)]
    pub fn spi3rst(&mut self) -> SPI3RST_W {
        SPI3RST_W::new(self)
    }
    #[doc = "Bit 17 - USART2RST"]
    #[inline(always)]
    pub fn usart2rst(&mut self) -> USART2RST_W {
        USART2RST_W::new(self)
    }
    #[doc = "Bit 18 - USART3RST"]
    #[inline(always)]
    pub fn usart3rst(&mut self) -> USART3RST_W {
        USART3RST_W::new(self)
    }
    #[doc = "Bit 19 - UART4RST"]
    #[inline(always)]
    pub fn uart4rst(&mut self) -> UART4RST_W {
        UART4RST_W::new(self)
    }
    #[doc = "Bit 20 - UART5RST"]
    #[inline(always)]
    pub fn uart5rst(&mut self) -> UART5RST_W {
        UART5RST_W::new(self)
    }
    #[doc = "Bit 21 - I2C1RST"]
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W {
        I2C1RST_W::new(self)
    }
    #[doc = "Bit 22 - I2C2RST"]
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2C2RST_W {
        I2C2RST_W::new(self)
    }
    #[doc = "Bit 23 - USBRST"]
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W {
        USBRST_W::new(self)
    }
    #[doc = "Bit 25 - CAN1RST"]
    #[inline(always)]
    pub fn can1rst(&mut self) -> CAN1RST_W {
        CAN1RST_W::new(self)
    }
    #[doc = "Bit 26 - CAN2RST"]
    #[inline(always)]
    pub fn can2rst(&mut self) -> CAN2RST_W {
        CAN2RST_W::new(self)
    }
    #[doc = "Bit 27 - BKPRST"]
    #[inline(always)]
    pub fn bkprst(&mut self) -> BKPRST_W {
        BKPRST_W::new(self)
    }
    #[doc = "Bit 28 - PWRRST"]
    #[inline(always)]
    pub fn pwrrst(&mut self) -> PWRRST_W {
        PWRRST_W::new(self)
    }
    #[doc = "Bit 29 - DACRST"]
    #[inline(always)]
    pub fn dacrst(&mut self) -> DACRST_W {
        DACRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC_APB1PRST\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb1prst](index.html) module"]
pub struct RCC_APB1PRST_SPEC;
impl crate::RegisterSpec for RCC_APB1PRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_apb1prst::R](R) reader structure"]
impl crate::Readable for RCC_APB1PRST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_apb1prst::W](W) writer structure"]
impl crate::Writable for RCC_APB1PRST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_APB1PRST to value 0"]
impl crate::Resettable for RCC_APB1PRST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
