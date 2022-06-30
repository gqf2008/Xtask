#[doc = "Register `RCC_APB1PCLKEN` reader"]
pub struct R(crate::R<RCC_APB1PCLKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB1PCLKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB1PCLKEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB1PCLKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_APB1PCLKEN` writer"]
pub struct W(crate::W<RCC_APB1PCLKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB1PCLKEN_SPEC>;
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
impl From<crate::W<RCC_APB1PCLKEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB1PCLKEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2EN` reader - TIM2EN"]
pub type TIM2EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM2EN` writer - TIM2EN"]
pub type TIM2EN_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PCLKEN_SPEC, bool, 0>;
#[doc = "Field `TIM3EN` reader - TIM3EN"]
pub type TIM3EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM3EN` writer - TIM3EN"]
pub type TIM3EN_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PCLKEN_SPEC, bool, 1>;
#[doc = "Field `TIM4EN` reader - TIM4EN"]
pub type TIM4EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM4EN` writer - TIM4EN"]
pub type TIM4EN_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PCLKEN_SPEC, bool, 2>;
#[doc = "Field `TIM5EN` reader - TIM5EN"]
pub type TIM5EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM5EN` writer - TIM5EN"]
pub type TIM5EN_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PCLKEN_SPEC, bool, 3>;
#[doc = "Field `TIM6EN` reader - TIM6EN"]
pub type TIM6EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM6EN` writer - TIM6EN"]
pub type TIM6EN_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PCLKEN_SPEC, bool, 4>;
#[doc = "Field `TIM7EN` reader - TIM7EN"]
pub type TIM7EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM7EN` writer - TIM7EN"]
pub type TIM7EN_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PCLKEN_SPEC, bool, 5>;
#[doc = "Field `COMPEN` reader - COMPEN"]
pub type COMPEN_R = crate::BitReader<bool>;
#[doc = "Field `COMPEN` writer - COMPEN"]
pub type COMPEN_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PCLKEN_SPEC, bool, 6>;
#[doc = "Field `COMPFILTEN` reader - COMPFILTEN"]
pub type COMPFILTEN_R = crate::BitReader<bool>;
#[doc = "Field `COMPFILTEN` writer - COMPFILTEN"]
pub type COMPFILTEN_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PCLKEN_SPEC, bool, 7>;
#[doc = "Field `TSCEN` reader - TSCEN"]
pub type TSCEN_R = crate::BitReader<bool>;
#[doc = "Field `TSCEN` writer - TSCEN"]
pub type TSCEN_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PCLKEN_SPEC, bool, 10>;
#[doc = "Field `WWDGEN` reader - WWDGEN"]
pub type WWDGEN_R = crate::BitReader<bool>;
#[doc = "Field `WWDGEN` writer - WWDGEN"]
pub type WWDGEN_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PCLKEN_SPEC, bool, 11>;
#[doc = "Field `SPI2EN` reader - SPI2EN"]
pub type SPI2EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI2EN` writer - SPI2EN"]
pub type SPI2EN_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PCLKEN_SPEC, bool, 14>;
#[doc = "Field `SPI3EN` reader - SPI3EN"]
pub type SPI3EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI3EN` writer - SPI3EN"]
pub type SPI3EN_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PCLKEN_SPEC, bool, 15>;
#[doc = "Field `USART2EN` reader - USART2EN"]
pub type USART2EN_R = crate::BitReader<bool>;
#[doc = "Field `USART2EN` writer - USART2EN"]
pub type USART2EN_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PCLKEN_SPEC, bool, 17>;
#[doc = "Field `USART3EN` reader - USART3EN"]
pub type USART3EN_R = crate::BitReader<bool>;
#[doc = "Field `USART3EN` writer - USART3EN"]
pub type USART3EN_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PCLKEN_SPEC, bool, 18>;
#[doc = "Field `UART4EN` reader - UART4EN"]
pub type UART4EN_R = crate::BitReader<bool>;
#[doc = "Field `UART4EN` writer - UART4EN"]
pub type UART4EN_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PCLKEN_SPEC, bool, 19>;
#[doc = "Field `UART5EN` reader - UART5EN"]
pub type UART5EN_R = crate::BitReader<bool>;
#[doc = "Field `UART5EN` writer - UART5EN"]
pub type UART5EN_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PCLKEN_SPEC, bool, 20>;
#[doc = "Field `I2C1EN` reader - I2C1EN"]
pub type I2C1EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C1EN` writer - I2C1EN"]
pub type I2C1EN_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PCLKEN_SPEC, bool, 21>;
#[doc = "Field `I2C2EN` reader - I2C2EN"]
pub type I2C2EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C2EN` writer - I2C2EN"]
pub type I2C2EN_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PCLKEN_SPEC, bool, 22>;
#[doc = "Field `CAN1EN` reader - CAN1EN"]
pub type CAN1EN_R = crate::BitReader<bool>;
#[doc = "Field `CAN1EN` writer - CAN1EN"]
pub type CAN1EN_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PCLKEN_SPEC, bool, 25>;
#[doc = "Field `CAN2EN` reader - CAN2EN"]
pub type CAN2EN_R = crate::BitReader<bool>;
#[doc = "Field `CAN2EN` writer - CAN2EN"]
pub type CAN2EN_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PCLKEN_SPEC, bool, 26>;
#[doc = "Field `BKPEN` reader - BKPEN"]
pub type BKPEN_R = crate::BitReader<bool>;
#[doc = "Field `BKPEN` writer - BKPEN"]
pub type BKPEN_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PCLKEN_SPEC, bool, 27>;
#[doc = "Field `PWREN` reader - PWREN"]
pub type PWREN_R = crate::BitReader<bool>;
#[doc = "Field `PWREN` writer - PWREN"]
pub type PWREN_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PCLKEN_SPEC, bool, 28>;
#[doc = "Field `DACEN` reader - DACEN"]
pub type DACEN_R = crate::BitReader<bool>;
#[doc = "Field `DACEN` writer - DACEN"]
pub type DACEN_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PCLKEN_SPEC, bool, 29>;
#[doc = "Field `OPAMPEN` reader - OPAMPEN"]
pub type OPAMPEN_R = crate::BitReader<bool>;
#[doc = "Field `OPAMPEN` writer - OPAMPEN"]
pub type OPAMPEN_W<'a> = crate::BitWriter<'a, u32, RCC_APB1PCLKEN_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - TIM2EN"]
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3EN"]
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4EN"]
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5EN"]
    #[inline(always)]
    pub fn tim5en(&self) -> TIM5EN_R {
        TIM5EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6EN"]
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7EN"]
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - COMPEN"]
    #[inline(always)]
    pub fn compen(&self) -> COMPEN_R {
        COMPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - COMPFILTEN"]
    #[inline(always)]
    pub fn compfilten(&self) -> COMPFILTEN_R {
        COMPFILTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - TSCEN"]
    #[inline(always)]
    pub fn tscen(&self) -> TSCEN_R {
        TSCEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDGEN"]
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2EN"]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3EN"]
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2EN"]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3EN"]
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4EN"]
    #[inline(always)]
    pub fn uart4en(&self) -> UART4EN_R {
        UART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5EN"]
    #[inline(always)]
    pub fn uart5en(&self) -> UART5EN_R {
        UART5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1EN"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2EN"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1EN"]
    #[inline(always)]
    pub fn can1en(&self) -> CAN1EN_R {
        CAN1EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN2EN"]
    #[inline(always)]
    pub fn can2en(&self) -> CAN2EN_R {
        CAN2EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - BKPEN"]
    #[inline(always)]
    pub fn bkpen(&self) -> BKPEN_R {
        BKPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PWREN"]
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DACEN"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - OPAMPEN"]
    #[inline(always)]
    pub fn opampen(&self) -> OPAMPEN_R {
        OPAMPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2EN"]
    #[inline(always)]
    pub fn tim2en(&mut self) -> TIM2EN_W {
        TIM2EN_W::new(self)
    }
    #[doc = "Bit 1 - TIM3EN"]
    #[inline(always)]
    pub fn tim3en(&mut self) -> TIM3EN_W {
        TIM3EN_W::new(self)
    }
    #[doc = "Bit 2 - TIM4EN"]
    #[inline(always)]
    pub fn tim4en(&mut self) -> TIM4EN_W {
        TIM4EN_W::new(self)
    }
    #[doc = "Bit 3 - TIM5EN"]
    #[inline(always)]
    pub fn tim5en(&mut self) -> TIM5EN_W {
        TIM5EN_W::new(self)
    }
    #[doc = "Bit 4 - TIM6EN"]
    #[inline(always)]
    pub fn tim6en(&mut self) -> TIM6EN_W {
        TIM6EN_W::new(self)
    }
    #[doc = "Bit 5 - TIM7EN"]
    #[inline(always)]
    pub fn tim7en(&mut self) -> TIM7EN_W {
        TIM7EN_W::new(self)
    }
    #[doc = "Bit 6 - COMPEN"]
    #[inline(always)]
    pub fn compen(&mut self) -> COMPEN_W {
        COMPEN_W::new(self)
    }
    #[doc = "Bit 7 - COMPFILTEN"]
    #[inline(always)]
    pub fn compfilten(&mut self) -> COMPFILTEN_W {
        COMPFILTEN_W::new(self)
    }
    #[doc = "Bit 10 - TSCEN"]
    #[inline(always)]
    pub fn tscen(&mut self) -> TSCEN_W {
        TSCEN_W::new(self)
    }
    #[doc = "Bit 11 - WWDGEN"]
    #[inline(always)]
    pub fn wwdgen(&mut self) -> WWDGEN_W {
        WWDGEN_W::new(self)
    }
    #[doc = "Bit 14 - SPI2EN"]
    #[inline(always)]
    pub fn spi2en(&mut self) -> SPI2EN_W {
        SPI2EN_W::new(self)
    }
    #[doc = "Bit 15 - SPI3EN"]
    #[inline(always)]
    pub fn spi3en(&mut self) -> SPI3EN_W {
        SPI3EN_W::new(self)
    }
    #[doc = "Bit 17 - USART2EN"]
    #[inline(always)]
    pub fn usart2en(&mut self) -> USART2EN_W {
        USART2EN_W::new(self)
    }
    #[doc = "Bit 18 - USART3EN"]
    #[inline(always)]
    pub fn usart3en(&mut self) -> USART3EN_W {
        USART3EN_W::new(self)
    }
    #[doc = "Bit 19 - UART4EN"]
    #[inline(always)]
    pub fn uart4en(&mut self) -> UART4EN_W {
        UART4EN_W::new(self)
    }
    #[doc = "Bit 20 - UART5EN"]
    #[inline(always)]
    pub fn uart5en(&mut self) -> UART5EN_W {
        UART5EN_W::new(self)
    }
    #[doc = "Bit 21 - I2C1EN"]
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W {
        I2C1EN_W::new(self)
    }
    #[doc = "Bit 22 - I2C2EN"]
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2C2EN_W {
        I2C2EN_W::new(self)
    }
    #[doc = "Bit 25 - CAN1EN"]
    #[inline(always)]
    pub fn can1en(&mut self) -> CAN1EN_W {
        CAN1EN_W::new(self)
    }
    #[doc = "Bit 26 - CAN2EN"]
    #[inline(always)]
    pub fn can2en(&mut self) -> CAN2EN_W {
        CAN2EN_W::new(self)
    }
    #[doc = "Bit 27 - BKPEN"]
    #[inline(always)]
    pub fn bkpen(&mut self) -> BKPEN_W {
        BKPEN_W::new(self)
    }
    #[doc = "Bit 28 - PWREN"]
    #[inline(always)]
    pub fn pwren(&mut self) -> PWREN_W {
        PWREN_W::new(self)
    }
    #[doc = "Bit 29 - DACEN"]
    #[inline(always)]
    pub fn dacen(&mut self) -> DACEN_W {
        DACEN_W::new(self)
    }
    #[doc = "Bit 31 - OPAMPEN"]
    #[inline(always)]
    pub fn opampen(&mut self) -> OPAMPEN_W {
        OPAMPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC_APB1PCLKEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb1pclken](index.html) module"]
pub struct RCC_APB1PCLKEN_SPEC;
impl crate::RegisterSpec for RCC_APB1PCLKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_apb1pclken::R](R) reader structure"]
impl crate::Readable for RCC_APB1PCLKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_apb1pclken::W](W) writer structure"]
impl crate::Writable for RCC_APB1PCLKEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_APB1PCLKEN to value 0"]
impl crate::Resettable for RCC_APB1PCLKEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
