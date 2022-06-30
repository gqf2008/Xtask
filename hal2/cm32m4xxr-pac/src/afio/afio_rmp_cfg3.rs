#[doc = "Register `AFIO_RMP_CFG3` reader"]
pub struct R(crate::R<AFIO_RMP_CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIO_RMP_CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIO_RMP_CFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIO_RMP_CFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFIO_RMP_CFG3` writer"]
pub struct W(crate::W<AFIO_RMP_CFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFIO_RMP_CFG3_SPEC>;
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
impl From<crate::W<AFIO_RMP_CFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFIO_RMP_CFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN2_RMP` reader - CAN2_RMP"]
pub type CAN2_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAN2_RMP` writer - CAN2_RMP"]
pub type CAN2_RMP_W<'a> = crate::FieldWriter<'a, u32, AFIO_RMP_CFG3_SPEC, u8, u8, 2, 1>;
#[doc = "Field `QSPI_RMP` reader - QSPI_RMP"]
pub type QSPI_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QSPI_RMP` writer - QSPI_RMP"]
pub type QSPI_RMP_W<'a> = crate::FieldWriter<'a, u32, AFIO_RMP_CFG3_SPEC, u8, u8, 2, 4>;
#[doc = "Field `I2C2_RMP` reader - I2C2_RMP"]
pub type I2C2_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2C2_RMP` writer - I2C2_RMP"]
pub type I2C2_RMP_W<'a> = crate::FieldWriter<'a, u32, AFIO_RMP_CFG3_SPEC, u8, u8, 2, 6>;
#[doc = "Field `I2C3_RMP` reader - I2C3_RMP"]
pub type I2C3_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2C3_RMP` writer - I2C3_RMP"]
pub type I2C3_RMP_W<'a> = crate::FieldWriter<'a, u32, AFIO_RMP_CFG3_SPEC, u8, u8, 2, 8>;
#[doc = "Field `I2C4_RMP` reader - I2C4_RMP"]
pub type I2C4_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2C4_RMP` writer - I2C4_RMP"]
pub type I2C4_RMP_W<'a> = crate::FieldWriter<'a, u32, AFIO_RMP_CFG3_SPEC, u8, u8, 2, 10>;
#[doc = "Field `SPI2_RMP` reader - SPI2_RMP"]
pub type SPI2_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI2_RMP` writer - SPI2_RMP"]
pub type SPI2_RMP_W<'a> = crate::FieldWriter<'a, u32, AFIO_RMP_CFG3_SPEC, u8, u8, 2, 12>;
#[doc = "Field `SPI3_RMP` reader - SPI3_RMP"]
pub type SPI3_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI3_RMP` writer - SPI3_RMP"]
pub type SPI3_RMP_W<'a> = crate::FieldWriter<'a, u32, AFIO_RMP_CFG3_SPEC, u8, u8, 2, 14>;
#[doc = "Field `SPI1_RMP_1` reader - SPI1_RMP_1"]
pub type SPI1_RMP_1_R = crate::BitReader<bool>;
#[doc = "Field `SPI1_RMP_1` writer - SPI1_RMP_1"]
pub type SPI1_RMP_1_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG3_SPEC, bool, 18>;
#[doc = "Field `USART2_RMP_1` reader - USART2_RMP_1"]
pub type USART2_RMP_1_R = crate::BitReader<bool>;
#[doc = "Field `USART2_RMP_1` writer - USART2_RMP_1"]
pub type USART2_RMP_1_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG3_SPEC, bool, 19>;
#[doc = "Field `UART4_RMP` reader - UART4_RMP"]
pub type UART4_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UART4_RMP` writer - UART4_RMP"]
pub type UART4_RMP_W<'a> = crate::FieldWriter<'a, u32, AFIO_RMP_CFG3_SPEC, u8, u8, 2, 20>;
#[doc = "Field `UART5_RMP` reader - UART5_RMP"]
pub type UART5_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UART5_RMP` writer - UART5_RMP"]
pub type UART5_RMP_W<'a> = crate::FieldWriter<'a, u32, AFIO_RMP_CFG3_SPEC, u8, u8, 2, 22>;
#[doc = "Field `UART6_RMP` reader - UART6_RMP"]
pub type UART6_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UART6_RMP` writer - UART6_RMP"]
pub type UART6_RMP_W<'a> = crate::FieldWriter<'a, u32, AFIO_RMP_CFG3_SPEC, u8, u8, 2, 24>;
#[doc = "Field `UART7_RMP` reader - UART7_RMP"]
pub type UART7_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UART7_RMP` writer - UART7_RMP"]
pub type UART7_RMP_W<'a> = crate::FieldWriter<'a, u32, AFIO_RMP_CFG3_SPEC, u8, u8, 2, 26>;
#[doc = "Field `TIM8_RMP` reader - TIM8_RMP"]
pub type TIM8_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIM8_RMP` writer - TIM8_RMP"]
pub type TIM8_RMP_W<'a> = crate::FieldWriter<'a, u32, AFIO_RMP_CFG3_SPEC, u8, u8, 2, 30>;
impl R {
    #[doc = "Bits 1:2 - CAN2_RMP"]
    #[inline(always)]
    pub fn can2_rmp(&self) -> CAN2_RMP_R {
        CAN2_RMP_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:5 - QSPI_RMP"]
    #[inline(always)]
    pub fn qspi_rmp(&self) -> QSPI_RMP_R {
        QSPI_RMP_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - I2C2_RMP"]
    #[inline(always)]
    pub fn i2c2_rmp(&self) -> I2C2_RMP_R {
        I2C2_RMP_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - I2C3_RMP"]
    #[inline(always)]
    pub fn i2c3_rmp(&self) -> I2C3_RMP_R {
        I2C3_RMP_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - I2C4_RMP"]
    #[inline(always)]
    pub fn i2c4_rmp(&self) -> I2C4_RMP_R {
        I2C4_RMP_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - SPI2_RMP"]
    #[inline(always)]
    pub fn spi2_rmp(&self) -> SPI2_RMP_R {
        SPI2_RMP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - SPI3_RMP"]
    #[inline(always)]
    pub fn spi3_rmp(&self) -> SPI3_RMP_R {
        SPI3_RMP_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 18 - SPI1_RMP_1"]
    #[inline(always)]
    pub fn spi1_rmp_1(&self) -> SPI1_RMP_1_R {
        SPI1_RMP_1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART2_RMP_1"]
    #[inline(always)]
    pub fn usart2_rmp_1(&self) -> USART2_RMP_1_R {
        USART2_RMP_1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - UART4_RMP"]
    #[inline(always)]
    pub fn uart4_rmp(&self) -> UART4_RMP_R {
        UART4_RMP_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - UART5_RMP"]
    #[inline(always)]
    pub fn uart5_rmp(&self) -> UART5_RMP_R {
        UART5_RMP_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - UART6_RMP"]
    #[inline(always)]
    pub fn uart6_rmp(&self) -> UART6_RMP_R {
        UART6_RMP_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - UART7_RMP"]
    #[inline(always)]
    pub fn uart7_rmp(&self) -> UART7_RMP_R {
        UART7_RMP_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 30:31 - TIM8_RMP"]
    #[inline(always)]
    pub fn tim8_rmp(&self) -> TIM8_RMP_R {
        TIM8_RMP_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 1:2 - CAN2_RMP"]
    #[inline(always)]
    pub fn can2_rmp(&mut self) -> CAN2_RMP_W {
        CAN2_RMP_W::new(self)
    }
    #[doc = "Bits 4:5 - QSPI_RMP"]
    #[inline(always)]
    pub fn qspi_rmp(&mut self) -> QSPI_RMP_W {
        QSPI_RMP_W::new(self)
    }
    #[doc = "Bits 6:7 - I2C2_RMP"]
    #[inline(always)]
    pub fn i2c2_rmp(&mut self) -> I2C2_RMP_W {
        I2C2_RMP_W::new(self)
    }
    #[doc = "Bits 8:9 - I2C3_RMP"]
    #[inline(always)]
    pub fn i2c3_rmp(&mut self) -> I2C3_RMP_W {
        I2C3_RMP_W::new(self)
    }
    #[doc = "Bits 10:11 - I2C4_RMP"]
    #[inline(always)]
    pub fn i2c4_rmp(&mut self) -> I2C4_RMP_W {
        I2C4_RMP_W::new(self)
    }
    #[doc = "Bits 12:13 - SPI2_RMP"]
    #[inline(always)]
    pub fn spi2_rmp(&mut self) -> SPI2_RMP_W {
        SPI2_RMP_W::new(self)
    }
    #[doc = "Bits 14:15 - SPI3_RMP"]
    #[inline(always)]
    pub fn spi3_rmp(&mut self) -> SPI3_RMP_W {
        SPI3_RMP_W::new(self)
    }
    #[doc = "Bit 18 - SPI1_RMP_1"]
    #[inline(always)]
    pub fn spi1_rmp_1(&mut self) -> SPI1_RMP_1_W {
        SPI1_RMP_1_W::new(self)
    }
    #[doc = "Bit 19 - USART2_RMP_1"]
    #[inline(always)]
    pub fn usart2_rmp_1(&mut self) -> USART2_RMP_1_W {
        USART2_RMP_1_W::new(self)
    }
    #[doc = "Bits 20:21 - UART4_RMP"]
    #[inline(always)]
    pub fn uart4_rmp(&mut self) -> UART4_RMP_W {
        UART4_RMP_W::new(self)
    }
    #[doc = "Bits 22:23 - UART5_RMP"]
    #[inline(always)]
    pub fn uart5_rmp(&mut self) -> UART5_RMP_W {
        UART5_RMP_W::new(self)
    }
    #[doc = "Bits 24:25 - UART6_RMP"]
    #[inline(always)]
    pub fn uart6_rmp(&mut self) -> UART6_RMP_W {
        UART6_RMP_W::new(self)
    }
    #[doc = "Bits 26:27 - UART7_RMP"]
    #[inline(always)]
    pub fn uart7_rmp(&mut self) -> UART7_RMP_W {
        UART7_RMP_W::new(self)
    }
    #[doc = "Bits 30:31 - TIM8_RMP"]
    #[inline(always)]
    pub fn tim8_rmp(&mut self) -> TIM8_RMP_W {
        TIM8_RMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO_RMP_CFG3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_rmp_cfg3](index.html) module"]
pub struct AFIO_RMP_CFG3_SPEC;
impl crate::RegisterSpec for AFIO_RMP_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afio_rmp_cfg3::R](R) reader structure"]
impl crate::Readable for AFIO_RMP_CFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afio_rmp_cfg3::W](W) writer structure"]
impl crate::Writable for AFIO_RMP_CFG3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFIO_RMP_CFG3 to value 0"]
impl crate::Resettable for AFIO_RMP_CFG3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
