#[doc = "Register `SPI_I2SPREDIV` reader"]
pub struct R(crate::R<SPI_I2SPREDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_I2SPREDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_I2SPREDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_I2SPREDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_I2SPREDIV` writer"]
pub struct W(crate::W<SPI_I2SPREDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_I2SPREDIV_SPEC>;
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
impl From<crate::W<SPI_I2SPREDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_I2SPREDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINE` reader - LINE"]
pub type LINE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LINE` writer - LINE"]
pub type LINE_W<'a> = crate::FieldWriter<'a, u32, SPI_I2SPREDIV_SPEC, u8, u8, 8, 0>;
#[doc = "Field `ODD_EVEN_` reader - ODD_EVEN_"]
pub type ODD_EVEN__R = crate::BitReader<bool>;
#[doc = "Field `ODD_EVEN_` writer - ODD_EVEN_"]
pub type ODD_EVEN__W<'a> = crate::BitWriter<'a, u32, SPI_I2SPREDIV_SPEC, bool, 8>;
#[doc = "Field `MCLKOEN` reader - MCLKOEN"]
pub type MCLKOEN_R = crate::BitReader<bool>;
#[doc = "Field `MCLKOEN` writer - MCLKOEN"]
pub type MCLKOEN_W<'a> = crate::BitWriter<'a, u32, SPI_I2SPREDIV_SPEC, bool, 9>;
impl R {
    #[doc = "Bits 0:7 - LINE"]
    #[inline(always)]
    pub fn line(&self) -> LINE_R {
        LINE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - ODD_EVEN_"]
    #[inline(always)]
    pub fn odd_even_(&self) -> ODD_EVEN__R {
        ODD_EVEN__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MCLKOEN"]
    #[inline(always)]
    pub fn mclkoen(&self) -> MCLKOEN_R {
        MCLKOEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - LINE"]
    #[inline(always)]
    pub fn line(&mut self) -> LINE_W {
        LINE_W::new(self)
    }
    #[doc = "Bit 8 - ODD_EVEN_"]
    #[inline(always)]
    pub fn odd_even_(&mut self) -> ODD_EVEN__W {
        ODD_EVEN__W::new(self)
    }
    #[doc = "Bit 9 - MCLKOEN"]
    #[inline(always)]
    pub fn mclkoen(&mut self) -> MCLKOEN_W {
        MCLKOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_I2SPREDIV\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_i2sprediv](index.html) module"]
pub struct SPI_I2SPREDIV_SPEC;
impl crate::RegisterSpec for SPI_I2SPREDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_i2sprediv::R](R) reader structure"]
impl crate::Readable for SPI_I2SPREDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_i2sprediv::W](W) writer structure"]
impl crate::Writable for SPI_I2SPREDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_I2SPREDIV to value 0x02"]
impl crate::Resettable for SPI_I2SPREDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
