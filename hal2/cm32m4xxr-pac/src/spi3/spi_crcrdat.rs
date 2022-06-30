#[doc = "Register `SPI_CRCRDAT` reader"]
pub struct R(crate::R<SPI_CRCRDAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CRCRDAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CRCRDAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CRCRDAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_CRCRDAT` writer"]
pub struct W(crate::W<SPI_CRCRDAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CRCRDAT_SPEC>;
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
impl From<crate::W<SPI_CRCRDAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CRCRDAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCRDAT` reader - CRCRDAT"]
pub type CRCRDAT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CRCRDAT` writer - CRCRDAT"]
pub type CRCRDAT_W<'a> = crate::FieldWriter<'a, u32, SPI_CRCRDAT_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - CRCRDAT"]
    #[inline(always)]
    pub fn crcrdat(&self) -> CRCRDAT_R {
        CRCRDAT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRCRDAT"]
    #[inline(always)]
    pub fn crcrdat(&mut self) -> CRCRDAT_W {
        CRCRDAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_CRCRDAT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_crcrdat](index.html) module"]
pub struct SPI_CRCRDAT_SPEC;
impl crate::RegisterSpec for SPI_CRCRDAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_crcrdat::R](R) reader structure"]
impl crate::Readable for SPI_CRCRDAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_crcrdat::W](W) writer structure"]
impl crate::Writable for SPI_CRCRDAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_CRCRDAT to value 0"]
impl crate::Resettable for SPI_CRCRDAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
