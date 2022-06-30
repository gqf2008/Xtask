#[doc = "Register `RCC_APB2PRST` reader"]
pub struct R(crate::R<RCC_APB2PRST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB2PRST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB2PRST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB2PRST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_APB2PRST` writer"]
pub struct W(crate::W<RCC_APB2PRST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB2PRST_SPEC>;
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
impl From<crate::W<RCC_APB2PRST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB2PRST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFIORST` reader - AFIORST"]
pub type AFIORST_R = crate::BitReader<bool>;
#[doc = "Field `AFIORST` writer - AFIORST"]
pub type AFIORST_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PRST_SPEC, bool, 0>;
#[doc = "Field `IOPARST` reader - IOPARST"]
pub type IOPARST_R = crate::BitReader<bool>;
#[doc = "Field `IOPARST` writer - IOPARST"]
pub type IOPARST_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PRST_SPEC, bool, 2>;
#[doc = "Field `IOPBRST` reader - IOPBRST"]
pub type IOPBRST_R = crate::BitReader<bool>;
#[doc = "Field `IOPBRST` writer - IOPBRST"]
pub type IOPBRST_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PRST_SPEC, bool, 3>;
#[doc = "Field `IOPCRST` reader - IOPCRST"]
pub type IOPCRST_R = crate::BitReader<bool>;
#[doc = "Field `IOPCRST` writer - IOPCRST"]
pub type IOPCRST_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PRST_SPEC, bool, 4>;
#[doc = "Field `IOPDRST` reader - IOPDRST"]
pub type IOPDRST_R = crate::BitReader<bool>;
#[doc = "Field `IOPDRST` writer - IOPDRST"]
pub type IOPDRST_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PRST_SPEC, bool, 5>;
#[doc = "Field `IOPERST` reader - IOPERST"]
pub type IOPERST_R = crate::BitReader<bool>;
#[doc = "Field `IOPERST` writer - IOPERST"]
pub type IOPERST_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PRST_SPEC, bool, 6>;
#[doc = "Field `IOPFRST` reader - IOPFRST"]
pub type IOPFRST_R = crate::BitReader<bool>;
#[doc = "Field `IOPFRST` writer - IOPFRST"]
pub type IOPFRST_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PRST_SPEC, bool, 7>;
#[doc = "Field `IOPGRST` reader - IOPGRST"]
pub type IOPGRST_R = crate::BitReader<bool>;
#[doc = "Field `IOPGRST` writer - IOPGRST"]
pub type IOPGRST_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PRST_SPEC, bool, 8>;
#[doc = "Field `TIM1RST` reader - TIM1RST"]
pub type TIM1RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM1RST` writer - TIM1RST"]
pub type TIM1RST_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PRST_SPEC, bool, 11>;
#[doc = "Field `SPI1RST` reader - SPI1RST"]
pub type SPI1RST_R = crate::BitReader<bool>;
#[doc = "Field `SPI1RST` writer - SPI1RST"]
pub type SPI1RST_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PRST_SPEC, bool, 12>;
#[doc = "Field `TIM8RST` reader - TIM8RST"]
pub type TIM8RST_R = crate::BitReader<bool>;
#[doc = "Field `TIM8RST` writer - TIM8RST"]
pub type TIM8RST_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PRST_SPEC, bool, 13>;
#[doc = "Field `USART1RST` reader - USART1RST"]
pub type USART1RST_R = crate::BitReader<bool>;
#[doc = "Field `USART1RST` writer - USART1RST"]
pub type USART1RST_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PRST_SPEC, bool, 14>;
#[doc = "Field `UART6RST` reader - UART6RST"]
pub type UART6RST_R = crate::BitReader<bool>;
#[doc = "Field `UART6RST` writer - UART6RST"]
pub type UART6RST_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PRST_SPEC, bool, 17>;
#[doc = "Field `UART7RST` reader - UART7RST"]
pub type UART7RST_R = crate::BitReader<bool>;
#[doc = "Field `UART7RST` writer - UART7RST"]
pub type UART7RST_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PRST_SPEC, bool, 18>;
#[doc = "Field `I2C3RST` reader - I2C3RST"]
pub type I2C3RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C3RST` writer - I2C3RST"]
pub type I2C3RST_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PRST_SPEC, bool, 19>;
#[doc = "Field `I2C4RST` reader - I2C4RST"]
pub type I2C4RST_R = crate::BitReader<bool>;
#[doc = "Field `I2C4RST` writer - I2C4RST"]
pub type I2C4RST_W<'a> = crate::BitWriter<'a, u32, RCC_APB2PRST_SPEC, bool, 20>;
impl R {
    #[doc = "Bit 0 - AFIORST"]
    #[inline(always)]
    pub fn afiorst(&self) -> AFIORST_R {
        AFIORST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - IOPARST"]
    #[inline(always)]
    pub fn ioparst(&self) -> IOPARST_R {
        IOPARST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IOPBRST"]
    #[inline(always)]
    pub fn iopbrst(&self) -> IOPBRST_R {
        IOPBRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IOPCRST"]
    #[inline(always)]
    pub fn iopcrst(&self) -> IOPCRST_R {
        IOPCRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IOPDRST"]
    #[inline(always)]
    pub fn iopdrst(&self) -> IOPDRST_R {
        IOPDRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IOPERST"]
    #[inline(always)]
    pub fn ioperst(&self) -> IOPERST_R {
        IOPERST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IOPFRST"]
    #[inline(always)]
    pub fn iopfrst(&self) -> IOPFRST_R {
        IOPFRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - IOPGRST"]
    #[inline(always)]
    pub fn iopgrst(&self) -> IOPGRST_R {
        IOPGRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1RST"]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1RST"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8RST"]
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1RST"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - UART6RST"]
    #[inline(always)]
    pub fn uart6rst(&self) -> UART6RST_R {
        UART6RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - UART7RST"]
    #[inline(always)]
    pub fn uart7rst(&self) -> UART7RST_R {
        UART7RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - I2C3RST"]
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I2C4RST"]
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2C4RST_R {
        I2C4RST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AFIORST"]
    #[inline(always)]
    pub fn afiorst(&mut self) -> AFIORST_W {
        AFIORST_W::new(self)
    }
    #[doc = "Bit 2 - IOPARST"]
    #[inline(always)]
    pub fn ioparst(&mut self) -> IOPARST_W {
        IOPARST_W::new(self)
    }
    #[doc = "Bit 3 - IOPBRST"]
    #[inline(always)]
    pub fn iopbrst(&mut self) -> IOPBRST_W {
        IOPBRST_W::new(self)
    }
    #[doc = "Bit 4 - IOPCRST"]
    #[inline(always)]
    pub fn iopcrst(&mut self) -> IOPCRST_W {
        IOPCRST_W::new(self)
    }
    #[doc = "Bit 5 - IOPDRST"]
    #[inline(always)]
    pub fn iopdrst(&mut self) -> IOPDRST_W {
        IOPDRST_W::new(self)
    }
    #[doc = "Bit 6 - IOPERST"]
    #[inline(always)]
    pub fn ioperst(&mut self) -> IOPERST_W {
        IOPERST_W::new(self)
    }
    #[doc = "Bit 7 - IOPFRST"]
    #[inline(always)]
    pub fn iopfrst(&mut self) -> IOPFRST_W {
        IOPFRST_W::new(self)
    }
    #[doc = "Bit 8 - IOPGRST"]
    #[inline(always)]
    pub fn iopgrst(&mut self) -> IOPGRST_W {
        IOPGRST_W::new(self)
    }
    #[doc = "Bit 11 - TIM1RST"]
    #[inline(always)]
    pub fn tim1rst(&mut self) -> TIM1RST_W {
        TIM1RST_W::new(self)
    }
    #[doc = "Bit 12 - SPI1RST"]
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W {
        SPI1RST_W::new(self)
    }
    #[doc = "Bit 13 - TIM8RST"]
    #[inline(always)]
    pub fn tim8rst(&mut self) -> TIM8RST_W {
        TIM8RST_W::new(self)
    }
    #[doc = "Bit 14 - USART1RST"]
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W {
        USART1RST_W::new(self)
    }
    #[doc = "Bit 17 - UART6RST"]
    #[inline(always)]
    pub fn uart6rst(&mut self) -> UART6RST_W {
        UART6RST_W::new(self)
    }
    #[doc = "Bit 18 - UART7RST"]
    #[inline(always)]
    pub fn uart7rst(&mut self) -> UART7RST_W {
        UART7RST_W::new(self)
    }
    #[doc = "Bit 19 - I2C3RST"]
    #[inline(always)]
    pub fn i2c3rst(&mut self) -> I2C3RST_W {
        I2C3RST_W::new(self)
    }
    #[doc = "Bit 20 - I2C4RST"]
    #[inline(always)]
    pub fn i2c4rst(&mut self) -> I2C4RST_W {
        I2C4RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC_APB2PRST\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_apb2prst](index.html) module"]
pub struct RCC_APB2PRST_SPEC;
impl crate::RegisterSpec for RCC_APB2PRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_apb2prst::R](R) reader structure"]
impl crate::Readable for RCC_APB2PRST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_apb2prst::W](W) writer structure"]
impl crate::Writable for RCC_APB2PRST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_APB2PRST to value 0"]
impl crate::Resettable for RCC_APB2PRST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
