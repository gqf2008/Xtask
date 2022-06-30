#[doc = "Register `QSPI_CTRL0` reader"]
pub struct R(crate::R<QSPI_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_CTRL0` writer"]
pub struct W(crate::W<QSPI_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_CTRL0_SPEC>;
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
impl From<crate::W<QSPI_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFS` reader - DFS"]
pub type DFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DFS` writer - DFS"]
pub type DFS_W<'a> = crate::FieldWriter<'a, u32, QSPI_CTRL0_SPEC, u8, u8, 5, 0>;
#[doc = "Field `FRF` reader - FRF"]
pub type FRF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRF` writer - FRF"]
pub type FRF_W<'a> = crate::FieldWriter<'a, u32, QSPI_CTRL0_SPEC, u8, u8, 2, 6>;
#[doc = "Field `SCPH` reader - SCPH"]
pub type SCPH_R = crate::BitReader<bool>;
#[doc = "Field `SCPH` writer - SCPH"]
pub type SCPH_W<'a> = crate::BitWriter<'a, u32, QSPI_CTRL0_SPEC, bool, 8>;
#[doc = "Field `SCPOL` reader - SCPOL"]
pub type SCPOL_R = crate::BitReader<bool>;
#[doc = "Field `SCPOL` writer - SCPOL"]
pub type SCPOL_W<'a> = crate::BitWriter<'a, u32, QSPI_CTRL0_SPEC, bool, 9>;
#[doc = "Field `TMOD` reader - TMOD"]
pub type TMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMOD` writer - TMOD"]
pub type TMOD_W<'a> = crate::FieldWriter<'a, u32, QSPI_CTRL0_SPEC, u8, u8, 2, 10>;
#[doc = "Field `SRL` reader - SRL"]
pub type SRL_R = crate::BitReader<bool>;
#[doc = "Field `SRL` writer - SRL"]
pub type SRL_W<'a> = crate::BitWriter<'a, u32, QSPI_CTRL0_SPEC, bool, 13>;
#[doc = "Field `SSTE` reader - SSTE"]
pub type SSTE_R = crate::BitReader<bool>;
#[doc = "Field `SSTE` writer - SSTE"]
pub type SSTE_W<'a> = crate::BitWriter<'a, u32, QSPI_CTRL0_SPEC, bool, 14>;
#[doc = "Field `CFS` reader - CFS"]
pub type CFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFS` writer - CFS"]
pub type CFS_W<'a> = crate::FieldWriter<'a, u32, QSPI_CTRL0_SPEC, u8, u8, 4, 16>;
#[doc = "Field `SPI_FRF` reader - SPI_FRF"]
pub type SPI_FRF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_FRF` writer - SPI_FRF"]
pub type SPI_FRF_W<'a> = crate::FieldWriter<'a, u32, QSPI_CTRL0_SPEC, u8, u8, 2, 22>;
impl R {
    #[doc = "Bits 0:4 - DFS"]
    #[inline(always)]
    pub fn dfs(&self) -> DFS_R {
        DFS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - FRF"]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - SCPH"]
    #[inline(always)]
    pub fn scph(&self) -> SCPH_R {
        SCPH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCPOL"]
    #[inline(always)]
    pub fn scpol(&self) -> SCPOL_R {
        SCPOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - TMOD"]
    #[inline(always)]
    pub fn tmod(&self) -> TMOD_R {
        TMOD_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 13 - SRL"]
    #[inline(always)]
    pub fn srl(&self) -> SRL_R {
        SRL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SSTE"]
    #[inline(always)]
    pub fn sste(&self) -> SSTE_R {
        SSTE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:19 - CFS"]
    #[inline(always)]
    pub fn cfs(&self) -> CFS_R {
        CFS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - SPI_FRF"]
    #[inline(always)]
    pub fn spi_frf(&self) -> SPI_FRF_R {
        SPI_FRF_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DFS"]
    #[inline(always)]
    pub fn dfs(&mut self) -> DFS_W {
        DFS_W::new(self)
    }
    #[doc = "Bits 6:7 - FRF"]
    #[inline(always)]
    pub fn frf(&mut self) -> FRF_W {
        FRF_W::new(self)
    }
    #[doc = "Bit 8 - SCPH"]
    #[inline(always)]
    pub fn scph(&mut self) -> SCPH_W {
        SCPH_W::new(self)
    }
    #[doc = "Bit 9 - SCPOL"]
    #[inline(always)]
    pub fn scpol(&mut self) -> SCPOL_W {
        SCPOL_W::new(self)
    }
    #[doc = "Bits 10:11 - TMOD"]
    #[inline(always)]
    pub fn tmod(&mut self) -> TMOD_W {
        TMOD_W::new(self)
    }
    #[doc = "Bit 13 - SRL"]
    #[inline(always)]
    pub fn srl(&mut self) -> SRL_W {
        SRL_W::new(self)
    }
    #[doc = "Bit 14 - SSTE"]
    #[inline(always)]
    pub fn sste(&mut self) -> SSTE_W {
        SSTE_W::new(self)
    }
    #[doc = "Bits 16:19 - CFS"]
    #[inline(always)]
    pub fn cfs(&mut self) -> CFS_W {
        CFS_W::new(self)
    }
    #[doc = "Bits 22:23 - SPI_FRF"]
    #[inline(always)]
    pub fn spi_frf(&mut self) -> SPI_FRF_W {
        SPI_FRF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI_CTRL0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_ctrl0](index.html) module"]
pub struct QSPI_CTRL0_SPEC;
impl crate::RegisterSpec for QSPI_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_ctrl0::R](R) reader structure"]
impl crate::Readable for QSPI_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_ctrl0::W](W) writer structure"]
impl crate::Writable for QSPI_CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_CTRL0 to value 0x0080_4407"]
impl crate::Resettable for QSPI_CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_4407
    }
}
