#[doc = "Register `QSPI_MMC_CLR` reader"]
pub struct R(crate::R<QSPI_MMC_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_MMC_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_MMC_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_MMC_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_MMC_CLR` writer"]
pub struct W(crate::W<QSPI_MMC_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_MMC_CLR_SPEC>;
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
impl From<crate::W<QSPI_MMC_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_MMC_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MMCIC` reader - MMCIC"]
pub type MMCIC_R = crate::BitReader<bool>;
#[doc = "Field `MMCIC` writer - MMCIC"]
pub type MMCIC_W<'a> = crate::BitWriter<'a, u32, QSPI_MMC_CLR_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - MMCIC"]
    #[inline(always)]
    pub fn mmcic(&self) -> MMCIC_R {
        MMCIC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MMCIC"]
    #[inline(always)]
    pub fn mmcic(&mut self) -> MMCIC_W {
        MMCIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI_MMC_CLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_mmc_clr](index.html) module"]
pub struct QSPI_MMC_CLR_SPEC;
impl crate::RegisterSpec for QSPI_MMC_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_mmc_clr::R](R) reader structure"]
impl crate::Readable for QSPI_MMC_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_mmc_clr::W](W) writer structure"]
impl crate::Writable for QSPI_MMC_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_MMC_CLR to value 0"]
impl crate::Resettable for QSPI_MMC_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
