#[doc = "Register `SPI_CTRL2` reader"]
pub struct R(crate::R<SPI_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_CTRL2` writer"]
pub struct W(crate::W<SPI_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CTRL2_SPEC>;
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
impl From<crate::W<SPI_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDMAEN` reader - RDMAEN"]
pub type RDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `RDMAEN` writer - RDMAEN"]
pub type RDMAEN_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL2_SPEC, bool, 0>;
#[doc = "Field `TDMAEN` reader - TDMAEN"]
pub type TDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `TDMAEN` writer - TDMAEN"]
pub type TDMAEN_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL2_SPEC, bool, 1>;
#[doc = "Field `SSOEN_` reader - SSOEN_"]
pub type SSOEN__R = crate::BitReader<bool>;
#[doc = "Field `SSOEN_` writer - SSOEN_"]
pub type SSOEN__W<'a> = crate::BitWriter<'a, u32, SPI_CTRL2_SPEC, bool, 2>;
#[doc = "Field `ERRINTEN__` reader - ERRINTEN__"]
pub type ERRINTEN___R = crate::BitReader<bool>;
#[doc = "Field `ERRINTEN__` writer - ERRINTEN__"]
pub type ERRINTEN___W<'a> = crate::BitWriter<'a, u32, SPI_CTRL2_SPEC, bool, 5>;
#[doc = "Field `RNEINTEN` reader - RNEINTEN"]
pub type RNEINTEN_R = crate::BitReader<bool>;
#[doc = "Field `RNEINTEN` writer - RNEINTEN"]
pub type RNEINTEN_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL2_SPEC, bool, 6>;
#[doc = "Field `TEINTEN` reader - TEINTEN"]
pub type TEINTEN_R = crate::BitReader<bool>;
#[doc = "Field `TEINTEN` writer - TEINTEN"]
pub type TEINTEN_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL2_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - RDMAEN"]
    #[inline(always)]
    pub fn rdmaen(&self) -> RDMAEN_R {
        RDMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TDMAEN"]
    #[inline(always)]
    pub fn tdmaen(&self) -> TDMAEN_R {
        TDMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSOEN_"]
    #[inline(always)]
    pub fn ssoen_(&self) -> SSOEN__R {
        SSOEN__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - ERRINTEN__"]
    #[inline(always)]
    pub fn errinten__(&self) -> ERRINTEN___R {
        ERRINTEN___R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RNEINTEN"]
    #[inline(always)]
    pub fn rneinten(&self) -> RNEINTEN_R {
        RNEINTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TEINTEN"]
    #[inline(always)]
    pub fn teinten(&self) -> TEINTEN_R {
        TEINTEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RDMAEN"]
    #[inline(always)]
    pub fn rdmaen(&mut self) -> RDMAEN_W {
        RDMAEN_W::new(self)
    }
    #[doc = "Bit 1 - TDMAEN"]
    #[inline(always)]
    pub fn tdmaen(&mut self) -> TDMAEN_W {
        TDMAEN_W::new(self)
    }
    #[doc = "Bit 2 - SSOEN_"]
    #[inline(always)]
    pub fn ssoen_(&mut self) -> SSOEN__W {
        SSOEN__W::new(self)
    }
    #[doc = "Bit 5 - ERRINTEN__"]
    #[inline(always)]
    pub fn errinten__(&mut self) -> ERRINTEN___W {
        ERRINTEN___W::new(self)
    }
    #[doc = "Bit 6 - RNEINTEN"]
    #[inline(always)]
    pub fn rneinten(&mut self) -> RNEINTEN_W {
        RNEINTEN_W::new(self)
    }
    #[doc = "Bit 7 - TEINTEN"]
    #[inline(always)]
    pub fn teinten(&mut self) -> TEINTEN_W {
        TEINTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_CTRL2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ctrl2](index.html) module"]
pub struct SPI_CTRL2_SPEC;
impl crate::RegisterSpec for SPI_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_ctrl2::R](R) reader structure"]
impl crate::Readable for SPI_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_ctrl2::W](W) writer structure"]
impl crate::Writable for SPI_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_CTRL2 to value 0"]
impl crate::Resettable for SPI_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
