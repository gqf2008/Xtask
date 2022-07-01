#[doc = "Register `RCC_APB2PCLKEN` reader"]
pub struct R(crate::R<RCC_APB2PCLKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB2PCLKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB2PCLKEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB2PCLKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_APB2PCLKEN` writer"]
pub struct W(crate::W<RCC_APB2PCLKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB2PCLKEN_SPEC>;
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
impl From<crate::W<RCC_APB2PCLKEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB2PCLKEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFIOEN` reader - AFIOEN"]
pub type AFIOEN_R = crate::BitReader<bool>;
#[doc = "Field `AFIOEN` writer - AFIOEN"]
pub type AFIOEN_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PCLKEN_SPEC, bool, 0>;
#[doc = "Field `IOPAEN` reader - IOPAEN"]
pub type IOPAEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPAEN` writer - IOPAEN"]
pub type IOPAEN_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PCLKEN_SPEC, bool, 2>;
#[doc = "Field `IOPBEN` reader - IOPBEN"]
pub type IOPBEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPBEN` writer - IOPBEN"]
pub type IOPBEN_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PCLKEN_SPEC, bool, 3>;
#[doc = "Field `IOPCEN` reader - IOPCEN"]
pub type IOPCEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPCEN` writer - IOPCEN"]
pub type IOPCEN_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PCLKEN_SPEC, bool, 4>;
#[doc = "Field `IOPDEN` reader - IOPDEN"]
pub type IOPDEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPDEN` writer - IOPDEN"]
pub type IOPDEN_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PCLKEN_SPEC, bool, 5>;
#[doc = "Field `IOPEEN` reader - IOPEEN"]
pub type IOPEEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPEEN` writer - IOPEEN"]
pub type IOPEEN_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PCLKEN_SPEC, bool, 6>;
#[doc = "Field `IOPFEN` reader - IOPFEN"]
pub type IOPFEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPFEN` writer - IOPFEN"]
pub type IOPFEN_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PCLKEN_SPEC, bool, 7>;
#[doc = "Field `IOPGEN` reader - IOPGEN"]
pub type IOPGEN_R = crate::BitReader<bool>;
#[doc = "Field `IOPGEN` writer - IOPGEN"]
pub type IOPGEN_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PCLKEN_SPEC, bool, 8>;
#[doc = "Field `TIM1EN` reader - TIM1EN"]
pub type TIM1EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM1EN` writer - TIM1EN"]
pub type TIM1EN_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PCLKEN_SPEC, bool, 11>;
#[doc = "Field `SPI1EN` reader - SPI1EN"]
pub type SPI1EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI1EN` writer - SPI1EN"]
pub type SPI1EN_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PCLKEN_SPEC, bool, 12>;
#[doc = "Field `TIM8EN` reader - TIM8EN"]
pub type TIM8EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM8EN` writer - TIM8EN"]
pub type TIM8EN_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PCLKEN_SPEC, bool, 13>;
#[doc = "Field `USART1EN` reader - USART1EN"]
pub type USART1EN_R = crate::BitReader<bool>;
#[doc = "Field `USART1EN` writer - USART1EN"]
pub type USART1EN_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PCLKEN_SPEC, bool, 14>;
#[doc = "Field `UART6EN` reader - UART6EN"]
pub type UART6EN_R = crate::BitReader<bool>;
#[doc = "Field `UART6EN` writer - UART6EN"]
pub type UART6EN_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PCLKEN_SPEC, bool, 17>;
#[doc = "Field `UART7EN` reader - UART7EN"]
pub type UART7EN_R = crate::BitReader<bool>;
#[doc = "Field `UART7EN` writer - UART7EN"]
pub type UART7EN_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PCLKEN_SPEC, bool, 18>;
#[doc = "Field `I2C3EN` reader - I2C3EN"]
pub type I2C3EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C3EN` writer - I2C3EN"]
pub type I2C3EN_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PCLKEN_SPEC, bool, 19>;
#[doc = "Field `I2C4EN` reader - I2C4EN"]
pub type I2C4EN_R = crate::BitReader<bool>;
#[doc = "Field `I2C4EN` writer - I2C4EN"]
pub type I2C4EN_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PCLKEN_SPEC, bool, 20>;
impl R {
    #[doc = "Bit 0 - AFIOEN"]
    #[inline(always)]
    pub fn afioen(&self) -> AFIOEN_R {
        AFIOEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - IOPAEN"]
    #[inline(always)]
    pub fn iopaen(&self) -> IOPAEN_R {
        IOPAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IOPBEN"]
    #[inline(always)]
    pub fn iopben(&self) -> IOPBEN_R {
        IOPBEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IOPCEN"]
    #[inline(always)]
    pub fn iopcen(&self) -> IOPCEN_R {
        IOPCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IOPDEN"]
    #[inline(always)]
    pub fn iopden(&self) -> IOPDEN_R {
        IOPDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IOPEEN"]
    #[inline(always)]
    pub fn iopeen(&self) -> IOPEEN_R {
        IOPEEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IOPFEN"]
    #[inline(always)]
    pub fn iopfen(&self) -> IOPFEN_R {
        IOPFEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IOPGEN"]
    #[inline(always)]
    pub fn iopgen(&self) -> IOPGEN_R {
        IOPGEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1EN"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1EN"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8EN"]
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1EN"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - UART6EN"]
    #[inline(always)]
    pub fn uart6en(&self) -> UART6EN_R {
        UART6EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - UART7EN"]
    #[inline(always)]
    pub fn uart7en(&self) -> UART7EN_R {
        UART7EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - I2C3EN"]
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I2C4EN"]
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AFIOEN"]
    #[inline(always)]
    pub fn afioen(&mut self) -> AFIOEN_W {
        AFIOEN_W::new(self)
    }
    #[doc = "Bit 2 - IOPAEN"]
    #[inline(always)]
    pub fn iopaen(&mut self) -> IOPAEN_W {
        IOPAEN_W::new(self)
    }
    #[doc = "Bit 3 - IOPBEN"]
    #[inline(always)]
    pub fn iopben(&mut self) -> IOPBEN_W {
        IOPBEN_W::new(self)
    }
    #[doc = "Bit 4 - IOPCEN"]
    #[inline(always)]
    pub fn iopcen(&mut self) -> IOPCEN_W {
        IOPCEN_W::new(self)
    }
    #[doc = "Bit 5 - IOPDEN"]
    #[inline(always)]
    pub fn iopden(&mut self) -> IOPDEN_W {
        IOPDEN_W::new(self)
    }
    #[doc = "Bit 6 - IOPEEN"]
    #[inline(always)]
    pub fn iopeen(&mut self) -> IOPEEN_W {
        IOPEEN_W::new(self)
    }
    #[doc = "Bit 7 - IOPFEN"]
    #[inline(always)]
    pub fn iopfen(&mut self) -> IOPFEN_W {
        IOPFEN_W::new(self)
    }
    #[doc = "Bit 8 - IOPGEN"]
    #[inline(always)]
    pub fn iopgen(&mut self) -> IOPGEN_W {
        IOPGEN_W::new(self)
    }
    #[doc = "Bit 11 - TIM1EN"]
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W {
        TIM1EN_W::new(self)
    }
    #[doc = "Bit 12 - SPI1EN"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W {
        SPI1EN_W::new(self)
    }
    #[doc = "Bit 13 - TIM8EN"]
    #[inline(always)]
    pub fn tim8en(&mut self) -> TIM8EN_W {
        TIM8EN_W::new(self)
    }
    #[doc = "Bit 14 - USART1EN"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W {
        USART1EN_W::new(self)
    }
    #[doc = "Bit 17 - UART6EN"]
    #[inline(always)]
    pub fn uart6en(&mut self) -> UART6EN_W {
        UART6EN_W::new(self)
    }
    #[doc = "Bit 18 - UART7EN"]
    #[inline(always)]
    pub fn uart7en(&mut self) -> UART7EN_W {
        UART7EN_W::new(self)
    }
    #[doc = "Bit 19 - I2C3EN"]
    #[inline(always)]
    pub fn i2c3en(&mut self) -> I2C3EN_W {
        I2C3EN_W::new(self)
    }
    #[doc = "Bit 20 - I2C4EN"]
    #[inline(always)]
    pub fn i2c4en(&mut self) -> I2C4EN_W {
        I2C4EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC_APB2PCLKEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb2pclken](index.html) module"]
pub struct RCC_APB2PCLKEN_SPEC;
impl crate::RegisterSpec for RCC_APB2PCLKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_apb2pclken::R](R) reader structure"]
impl crate::Readable for RCC_APB2PCLKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_apb2pclken::W](W) writer structure"]
impl crate::Writable for RCC_APB2PCLKEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_APB2PCLKEN to value 0"]
impl crate::Resettable for RCC_APB2PCLKEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
