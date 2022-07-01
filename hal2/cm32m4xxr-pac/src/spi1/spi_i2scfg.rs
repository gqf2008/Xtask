#[doc = "Register `SPI_I2SCFG` reader"]
pub struct R(crate::R<SPI_I2SCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_I2SCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_I2SCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_I2SCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_I2SCFG` writer"]
pub struct W(crate::W<SPI_I2SCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_I2SCFG_SPEC>;
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
impl From<crate::W<SPI_I2SCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_I2SCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHBITS` reader - CHBITS"]
pub type CHBITS_R = crate::BitReader<bool>;
#[doc = "Field `CHBITS` writer - CHBITS"]
pub type CHBITS_W<'a> = crate::BitWriter<'a, u32, SPI_I2SCFG_SPEC, bool, 0>;
#[doc = "Field `TDATLEN` reader - TDATLEN"]
pub type TDATLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TDATLEN` writer - TDATLEN"]
pub type TDATLEN_W<'a> = crate::FieldWriter<'a, u32, SPI_I2SCFG_SPEC, u8, u8, 2, 1>;
#[doc = "Field `CLKPOL` reader - CLKPOL"]
pub type CLKPOL_R = crate::BitReader<bool>;
#[doc = "Field `CLKPOL` writer - CLKPOL"]
pub type CLKPOL_W<'a> = crate::BitWriter<'a, u32, SPI_I2SCFG_SPEC, bool, 3>;
#[doc = "Field `STDSEL` reader - STDSEL"]
pub type STDSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STDSEL` writer - STDSEL"]
pub type STDSEL_W<'a> = crate::FieldWriter<'a, u32, SPI_I2SCFG_SPEC, u8, u8, 2, 4>;
#[doc = "Field `PCMFSYNC` reader - PCMFSYNC"]
pub type PCMFSYNC_R = crate::BitReader<bool>;
#[doc = "Field `PCMFSYNC` writer - PCMFSYNC"]
pub type PCMFSYNC_W<'a> = crate::BitWriter<'a, u32, SPI_I2SCFG_SPEC, bool, 7>;
#[doc = "Field `MODCFG` reader - MODCFG"]
pub type MODCFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODCFG` writer - MODCFG"]
pub type MODCFG_W<'a> = crate::FieldWriter<'a, u32, SPI_I2SCFG_SPEC, u8, u8, 2, 8>;
#[doc = "Field `I2SEN` reader - I2SEN"]
pub type I2SEN_R = crate::BitReader<bool>;
#[doc = "Field `I2SEN` writer - I2SEN"]
pub type I2SEN_W<'a> = crate::BitWriter<'a, u32, SPI_I2SCFG_SPEC, bool, 10>;
#[doc = "Field `MODSEL` reader - MODSEL"]
pub type MODSEL_R = crate::BitReader<bool>;
#[doc = "Field `MODSEL` writer - MODSEL"]
pub type MODSEL_W<'a> = crate::BitWriter<'a, u32, SPI_I2SCFG_SPEC, bool, 11>;
impl R {
    #[doc = "Bit 0 - CHBITS"]
    #[inline(always)]
    pub fn chbits(&self) -> CHBITS_R {
        CHBITS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - TDATLEN"]
    #[inline(always)]
    pub fn tdatlen(&self) -> TDATLEN_R {
        TDATLEN_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - CLKPOL"]
    #[inline(always)]
    pub fn clkpol(&self) -> CLKPOL_R {
        CLKPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - STDSEL"]
    #[inline(always)]
    pub fn stdsel(&self) -> STDSEL_R {
        STDSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - PCMFSYNC"]
    #[inline(always)]
    pub fn pcmfsync(&self) -> PCMFSYNC_R {
        PCMFSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - MODCFG"]
    #[inline(always)]
    pub fn modcfg(&self) -> MODCFG_R {
        MODCFG_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - I2SEN"]
    #[inline(always)]
    pub fn i2sen(&self) -> I2SEN_R {
        I2SEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MODSEL"]
    #[inline(always)]
    pub fn modsel(&self) -> MODSEL_R {
        MODSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CHBITS"]
    #[inline(always)]
    pub fn chbits(&mut self) -> CHBITS_W {
        CHBITS_W::new(self)
    }
    #[doc = "Bits 1:2 - TDATLEN"]
    #[inline(always)]
    pub fn tdatlen(&mut self) -> TDATLEN_W {
        TDATLEN_W::new(self)
    }
    #[doc = "Bit 3 - CLKPOL"]
    #[inline(always)]
    pub fn clkpol(&mut self) -> CLKPOL_W {
        CLKPOL_W::new(self)
    }
    #[doc = "Bits 4:5 - STDSEL"]
    #[inline(always)]
    pub fn stdsel(&mut self) -> STDSEL_W {
        STDSEL_W::new(self)
    }
    #[doc = "Bit 7 - PCMFSYNC"]
    #[inline(always)]
    pub fn pcmfsync(&mut self) -> PCMFSYNC_W {
        PCMFSYNC_W::new(self)
    }
    #[doc = "Bits 8:9 - MODCFG"]
    #[inline(always)]
    pub fn modcfg(&mut self) -> MODCFG_W {
        MODCFG_W::new(self)
    }
    #[doc = "Bit 10 - I2SEN"]
    #[inline(always)]
    pub fn i2sen(&mut self) -> I2SEN_W {
        I2SEN_W::new(self)
    }
    #[doc = "Bit 11 - MODSEL"]
    #[inline(always)]
    pub fn modsel(&mut self) -> MODSEL_W {
        MODSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_I2SCFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_i2scfg](index.html) module"]
pub struct SPI_I2SCFG_SPEC;
impl crate::RegisterSpec for SPI_I2SCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_i2scfg::R](R) reader structure"]
impl crate::Readable for SPI_I2SCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_i2scfg::W](W) writer structure"]
impl crate::Writable for SPI_I2SCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_I2SCFG to value 0"]
impl crate::Resettable for SPI_I2SCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
