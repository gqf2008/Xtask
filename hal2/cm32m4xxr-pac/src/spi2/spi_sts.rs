#[doc = "Register `SPI_STS` reader"]
pub struct R(crate::R<SPI_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_STS` writer"]
pub struct W(crate::W<SPI_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_STS_SPEC>;
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
impl From<crate::W<SPI_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RNE` reader - RNE"]
pub type RNE_R = crate::BitReader<bool>;
#[doc = "Field `RNE` writer - RNE"]
pub type RNE_W<'a> = crate::BitWriter<'a, u32, SPI_STS_SPEC, bool, 0>;
#[doc = "Field `TE` reader - TE"]
pub type TE_R = crate::BitReader<bool>;
#[doc = "Field `TE` writer - TE"]
pub type TE_W<'a> = crate::BitWriter<'a, u32, SPI_STS_SPEC, bool, 1>;
#[doc = "Field `CHSIDE` reader - CHSIDE"]
pub type CHSIDE_R = crate::BitReader<bool>;
#[doc = "Field `CHSIDE` writer - CHSIDE"]
pub type CHSIDE_W<'a> = crate::BitWriter<'a, u32, SPI_STS_SPEC, bool, 2>;
#[doc = "Field `UNDER` reader - UNDER"]
pub type UNDER_R = crate::BitReader<bool>;
#[doc = "Field `UNDER` writer - UNDER"]
pub type UNDER_W<'a> = crate::BitWriter<'a, u32, SPI_STS_SPEC, bool, 3>;
#[doc = "Field `CRCERR` reader - CRCERR"]
pub type CRCERR_R = crate::BitReader<bool>;
#[doc = "Field `CRCERR` writer - CRCERR"]
pub type CRCERR_W<'a> = crate::BitWriter<'a, u32, SPI_STS_SPEC, bool, 4>;
#[doc = "Field `MODERR` reader - MODERR"]
pub type MODERR_R = crate::BitReader<bool>;
#[doc = "Field `MODERR` writer - MODERR"]
pub type MODERR_W<'a> = crate::BitWriter<'a, u32, SPI_STS_SPEC, bool, 5>;
#[doc = "Field `OVER` reader - OVER"]
pub type OVER_R = crate::BitReader<bool>;
#[doc = "Field `OVER` writer - OVER"]
pub type OVER_W<'a> = crate::BitWriter<'a, u32, SPI_STS_SPEC, bool, 6>;
#[doc = "Field `BUSY_` reader - BUSY_"]
pub type BUSY__R = crate::BitReader<bool>;
#[doc = "Field `BUSY_` writer - BUSY_"]
pub type BUSY__W<'a> = crate::BitWriter<'a, u32, SPI_STS_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - RNE"]
    #[inline(always)]
    pub fn rne(&self) -> RNE_R {
        RNE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TE"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CHSIDE"]
    #[inline(always)]
    pub fn chside(&self) -> CHSIDE_R {
        CHSIDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UNDER"]
    #[inline(always)]
    pub fn under(&self) -> UNDER_R {
        UNDER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRCERR"]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MODERR"]
    #[inline(always)]
    pub fn moderr(&self) -> MODERR_R {
        MODERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OVER"]
    #[inline(always)]
    pub fn over(&self) -> OVER_R {
        OVER_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BUSY_"]
    #[inline(always)]
    pub fn busy_(&self) -> BUSY__R {
        BUSY__R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RNE"]
    #[inline(always)]
    pub fn rne(&mut self) -> RNE_W {
        RNE_W::new(self)
    }
    #[doc = "Bit 1 - TE"]
    #[inline(always)]
    pub fn te(&mut self) -> TE_W {
        TE_W::new(self)
    }
    #[doc = "Bit 2 - CHSIDE"]
    #[inline(always)]
    pub fn chside(&mut self) -> CHSIDE_W {
        CHSIDE_W::new(self)
    }
    #[doc = "Bit 3 - UNDER"]
    #[inline(always)]
    pub fn under(&mut self) -> UNDER_W {
        UNDER_W::new(self)
    }
    #[doc = "Bit 4 - CRCERR"]
    #[inline(always)]
    pub fn crcerr(&mut self) -> CRCERR_W {
        CRCERR_W::new(self)
    }
    #[doc = "Bit 5 - MODERR"]
    #[inline(always)]
    pub fn moderr(&mut self) -> MODERR_W {
        MODERR_W::new(self)
    }
    #[doc = "Bit 6 - OVER"]
    #[inline(always)]
    pub fn over(&mut self) -> OVER_W {
        OVER_W::new(self)
    }
    #[doc = "Bit 7 - BUSY_"]
    #[inline(always)]
    pub fn busy_(&mut self) -> BUSY__W {
        BUSY__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_STS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_sts](index.html) module"]
pub struct SPI_STS_SPEC;
impl crate::RegisterSpec for SPI_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_sts::R](R) reader structure"]
impl crate::Readable for SPI_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_sts::W](W) writer structure"]
impl crate::Writable for SPI_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_STS to value 0x02"]
impl crate::Resettable for SPI_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
