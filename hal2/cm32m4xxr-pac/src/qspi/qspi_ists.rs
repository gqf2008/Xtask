#[doc = "Register `QSPI_ISTS` reader"]
pub struct R(crate::R<QSPI_ISTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_ISTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_ISTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_ISTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_ISTS` writer"]
pub struct W(crate::W<QSPI_ISTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_ISTS_SPEC>;
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
impl From<crate::W<QSPI_ISTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_ISTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXFEIS` reader - TXFEIS"]
pub type TXFEIS_R = crate::BitReader<bool>;
#[doc = "Field `TXFEIS` writer - TXFEIS"]
pub type TXFEIS_W<'a> = crate::BitWriter<'a, u32, QSPI_ISTS_SPEC, bool, 0>;
#[doc = "Field `TXFOIS` reader - TXFOIS"]
pub type TXFOIS_R = crate::BitReader<bool>;
#[doc = "Field `TXFOIS` writer - TXFOIS"]
pub type TXFOIS_W<'a> = crate::BitWriter<'a, u32, QSPI_ISTS_SPEC, bool, 1>;
#[doc = "Field `RXFUIS` reader - RXFUIS"]
pub type RXFUIS_R = crate::BitReader<bool>;
#[doc = "Field `RXFUIS` writer - RXFUIS"]
pub type RXFUIS_W<'a> = crate::BitWriter<'a, u32, QSPI_ISTS_SPEC, bool, 2>;
#[doc = "Field `RXFOIS` reader - RXFOIS"]
pub type RXFOIS_R = crate::BitReader<bool>;
#[doc = "Field `RXFOIS` writer - RXFOIS"]
pub type RXFOIS_W<'a> = crate::BitWriter<'a, u32, QSPI_ISTS_SPEC, bool, 3>;
#[doc = "Field `RXFFIS` reader - RXFFIS"]
pub type RXFFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXFFIS` writer - RXFFIS"]
pub type RXFFIS_W<'a> = crate::BitWriter<'a, u32, QSPI_ISTS_SPEC, bool, 4>;
#[doc = "Field `MMCIS` reader - MMCIS"]
pub type MMCIS_R = crate::BitReader<bool>;
#[doc = "Field `MMCIS` writer - MMCIS"]
pub type MMCIS_W<'a> = crate::BitWriter<'a, u32, QSPI_ISTS_SPEC, bool, 5>;
#[doc = "Field `XRXOIS` reader - XRXOIS"]
pub type XRXOIS_R = crate::BitReader<bool>;
#[doc = "Field `XRXOIS` writer - XRXOIS"]
pub type XRXOIS_W<'a> = crate::BitWriter<'a, u32, QSPI_ISTS_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 0 - TXFEIS"]
    #[inline(always)]
    pub fn txfeis(&self) -> TXFEIS_R {
        TXFEIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXFOIS"]
    #[inline(always)]
    pub fn txfois(&self) -> TXFOIS_R {
        TXFOIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXFUIS"]
    #[inline(always)]
    pub fn rxfuis(&self) -> RXFUIS_R {
        RXFUIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RXFOIS"]
    #[inline(always)]
    pub fn rxfois(&self) -> RXFOIS_R {
        RXFOIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXFFIS"]
    #[inline(always)]
    pub fn rxffis(&self) -> RXFFIS_R {
        RXFFIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMCIS"]
    #[inline(always)]
    pub fn mmcis(&self) -> MMCIS_R {
        MMCIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - XRXOIS"]
    #[inline(always)]
    pub fn xrxois(&self) -> XRXOIS_R {
        XRXOIS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXFEIS"]
    #[inline(always)]
    pub fn txfeis(&mut self) -> TXFEIS_W {
        TXFEIS_W::new(self)
    }
    #[doc = "Bit 1 - TXFOIS"]
    #[inline(always)]
    pub fn txfois(&mut self) -> TXFOIS_W {
        TXFOIS_W::new(self)
    }
    #[doc = "Bit 2 - RXFUIS"]
    #[inline(always)]
    pub fn rxfuis(&mut self) -> RXFUIS_W {
        RXFUIS_W::new(self)
    }
    #[doc = "Bit 3 - RXFOIS"]
    #[inline(always)]
    pub fn rxfois(&mut self) -> RXFOIS_W {
        RXFOIS_W::new(self)
    }
    #[doc = "Bit 4 - RXFFIS"]
    #[inline(always)]
    pub fn rxffis(&mut self) -> RXFFIS_W {
        RXFFIS_W::new(self)
    }
    #[doc = "Bit 5 - MMCIS"]
    #[inline(always)]
    pub fn mmcis(&mut self) -> MMCIS_W {
        MMCIS_W::new(self)
    }
    #[doc = "Bit 6 - XRXOIS"]
    #[inline(always)]
    pub fn xrxois(&mut self) -> XRXOIS_W {
        XRXOIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI_ISTS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_ists](index.html) module"]
pub struct QSPI_ISTS_SPEC;
impl crate::RegisterSpec for QSPI_ISTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_ists::R](R) reader structure"]
impl crate::Readable for QSPI_ISTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_ists::W](W) writer structure"]
impl crate::Writable for QSPI_ISTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_ISTS to value 0"]
impl crate::Resettable for QSPI_ISTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
