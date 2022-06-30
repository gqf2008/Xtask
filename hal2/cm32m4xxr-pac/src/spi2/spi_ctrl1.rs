#[doc = "Register `SPI_CTRL1` reader"]
pub struct R(crate::R<SPI_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_CTRL1` writer"]
pub struct W(crate::W<SPI_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CTRL1_SPEC>;
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
impl From<crate::W<SPI_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKPHA` reader - CLKPHA"]
pub type CLKPHA_R = crate::BitReader<bool>;
#[doc = "Field `CLKPHA` writer - CLKPHA"]
pub type CLKPHA_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL1_SPEC, bool, 0>;
#[doc = "Field `CLKPOL` reader - CLKPOL"]
pub type CLKPOL_R = crate::BitReader<bool>;
#[doc = "Field `CLKPOL` writer - CLKPOL"]
pub type CLKPOL_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL1_SPEC, bool, 1>;
#[doc = "Field `MSEL_` reader - MSEL_"]
pub type MSEL__R = crate::BitReader<bool>;
#[doc = "Field `MSEL_` writer - MSEL_"]
pub type MSEL__W<'a> = crate::BitWriter<'a, u32, SPI_CTRL1_SPEC, bool, 2>;
#[doc = "Field `BR` reader - BR"]
pub type BR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BR` writer - BR"]
pub type BR_W<'a> = crate::FieldWriter<'a, u32, SPI_CTRL1_SPEC, u8, u8, 3, 3>;
#[doc = "Field `SPIEN` reader - SPIEN"]
pub type SPIEN_R = crate::BitReader<bool>;
#[doc = "Field `SPIEN` writer - SPIEN"]
pub type SPIEN_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL1_SPEC, bool, 6>;
#[doc = "Field `LSBFF_` reader - LSBFF_"]
pub type LSBFF__R = crate::BitReader<bool>;
#[doc = "Field `LSBFF_` writer - LSBFF_"]
pub type LSBFF__W<'a> = crate::BitWriter<'a, u32, SPI_CTRL1_SPEC, bool, 7>;
#[doc = "Field `SSEL` reader - SSEL"]
pub type SSEL_R = crate::BitReader<bool>;
#[doc = "Field `SSEL` writer - SSEL"]
pub type SSEL_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL1_SPEC, bool, 8>;
#[doc = "Field `SSMEN` reader - SSMEN"]
pub type SSMEN_R = crate::BitReader<bool>;
#[doc = "Field `SSMEN` writer - SSMEN"]
pub type SSMEN_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL1_SPEC, bool, 9>;
#[doc = "Field `RONLY` reader - RONLY"]
pub type RONLY_R = crate::BitReader<bool>;
#[doc = "Field `RONLY` writer - RONLY"]
pub type RONLY_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL1_SPEC, bool, 10>;
#[doc = "Field `DATFF` reader - DATFF"]
pub type DATFF_R = crate::BitReader<bool>;
#[doc = "Field `DATFF` writer - DATFF"]
pub type DATFF_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL1_SPEC, bool, 11>;
#[doc = "Field `CRCNEXT` reader - CRCNEXT"]
pub type CRCNEXT_R = crate::BitReader<bool>;
#[doc = "Field `CRCNEXT` writer - CRCNEXT"]
pub type CRCNEXT_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL1_SPEC, bool, 12>;
#[doc = "Field `CRCEN` reader - CRCEN"]
pub type CRCEN_R = crate::BitReader<bool>;
#[doc = "Field `CRCEN` writer - CRCEN"]
pub type CRCEN_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL1_SPEC, bool, 13>;
#[doc = "Field `BIDIROEN` reader - BIDIROEN"]
pub type BIDIROEN_R = crate::BitReader<bool>;
#[doc = "Field `BIDIROEN` writer - BIDIROEN"]
pub type BIDIROEN_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL1_SPEC, bool, 14>;
#[doc = "Field `BIDIRMODE` reader - BIDIRMODE"]
pub type BIDIRMODE_R = crate::BitReader<bool>;
#[doc = "Field `BIDIRMODE` writer - BIDIRMODE"]
pub type BIDIRMODE_W<'a> = crate::BitWriter<'a, u32, SPI_CTRL1_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - CLKPHA"]
    #[inline(always)]
    pub fn clkpha(&self) -> CLKPHA_R {
        CLKPHA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CLKPOL"]
    #[inline(always)]
    pub fn clkpol(&self) -> CLKPOL_R {
        CLKPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MSEL_"]
    #[inline(always)]
    pub fn msel_(&self) -> MSEL__R {
        MSEL__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - BR"]
    #[inline(always)]
    pub fn br(&self) -> BR_R {
        BR_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - SPIEN"]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LSBFF_"]
    #[inline(always)]
    pub fn lsbff_(&self) -> LSBFF__R {
        LSBFF__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SSEL"]
    #[inline(always)]
    pub fn ssel(&self) -> SSEL_R {
        SSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SSMEN"]
    #[inline(always)]
    pub fn ssmen(&self) -> SSMEN_R {
        SSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RONLY"]
    #[inline(always)]
    pub fn ronly(&self) -> RONLY_R {
        RONLY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DATFF"]
    #[inline(always)]
    pub fn datff(&self) -> DATFF_R {
        DATFF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CRCNEXT"]
    #[inline(always)]
    pub fn crcnext(&self) -> CRCNEXT_R {
        CRCNEXT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CRCEN"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - BIDIROEN"]
    #[inline(always)]
    pub fn bidiroen(&self) -> BIDIROEN_R {
        BIDIROEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - BIDIRMODE"]
    #[inline(always)]
    pub fn bidirmode(&self) -> BIDIRMODE_R {
        BIDIRMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CLKPHA"]
    #[inline(always)]
    pub fn clkpha(&mut self) -> CLKPHA_W {
        CLKPHA_W::new(self)
    }
    #[doc = "Bit 1 - CLKPOL"]
    #[inline(always)]
    pub fn clkpol(&mut self) -> CLKPOL_W {
        CLKPOL_W::new(self)
    }
    #[doc = "Bit 2 - MSEL_"]
    #[inline(always)]
    pub fn msel_(&mut self) -> MSEL__W {
        MSEL__W::new(self)
    }
    #[doc = "Bits 3:5 - BR"]
    #[inline(always)]
    pub fn br(&mut self) -> BR_W {
        BR_W::new(self)
    }
    #[doc = "Bit 6 - SPIEN"]
    #[inline(always)]
    pub fn spien(&mut self) -> SPIEN_W {
        SPIEN_W::new(self)
    }
    #[doc = "Bit 7 - LSBFF_"]
    #[inline(always)]
    pub fn lsbff_(&mut self) -> LSBFF__W {
        LSBFF__W::new(self)
    }
    #[doc = "Bit 8 - SSEL"]
    #[inline(always)]
    pub fn ssel(&mut self) -> SSEL_W {
        SSEL_W::new(self)
    }
    #[doc = "Bit 9 - SSMEN"]
    #[inline(always)]
    pub fn ssmen(&mut self) -> SSMEN_W {
        SSMEN_W::new(self)
    }
    #[doc = "Bit 10 - RONLY"]
    #[inline(always)]
    pub fn ronly(&mut self) -> RONLY_W {
        RONLY_W::new(self)
    }
    #[doc = "Bit 11 - DATFF"]
    #[inline(always)]
    pub fn datff(&mut self) -> DATFF_W {
        DATFF_W::new(self)
    }
    #[doc = "Bit 12 - CRCNEXT"]
    #[inline(always)]
    pub fn crcnext(&mut self) -> CRCNEXT_W {
        CRCNEXT_W::new(self)
    }
    #[doc = "Bit 13 - CRCEN"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 14 - BIDIROEN"]
    #[inline(always)]
    pub fn bidiroen(&mut self) -> BIDIROEN_W {
        BIDIROEN_W::new(self)
    }
    #[doc = "Bit 15 - BIDIRMODE"]
    #[inline(always)]
    pub fn bidirmode(&mut self) -> BIDIRMODE_W {
        BIDIRMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_CTRL1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ctrl1](index.html) module"]
pub struct SPI_CTRL1_SPEC;
impl crate::RegisterSpec for SPI_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_ctrl1::R](R) reader structure"]
impl crate::Readable for SPI_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_ctrl1::W](W) writer structure"]
impl crate::Writable for SPI_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_CTRL1 to value 0"]
impl crate::Resettable for SPI_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
