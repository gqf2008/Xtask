#[doc = "Register `QSPI_DDR_TXDE` reader"]
pub struct R(crate::R<QSPI_DDR_TXDE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_DDR_TXDE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_DDR_TXDE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_DDR_TXDE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_DDR_TXDE` writer"]
pub struct W(crate::W<QSPI_DDR_TXDE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_DDR_TXDE_SPEC>;
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
impl From<crate::W<QSPI_DDR_TXDE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_DDR_TXDE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDE` reader - TXDE"]
pub type TXDE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXDE` writer - TXDE"]
pub type TXDE_W<'a> = crate::FieldWriter<'a, u32, QSPI_DDR_TXDE_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - TXDE"]
    #[inline(always)]
    pub fn txde(&self) -> TXDE_R {
        TXDE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TXDE"]
    #[inline(always)]
    pub fn txde(&mut self) -> TXDE_W {
        TXDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI_DDR_TXDE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_ddr_txde](index.html) module"]
pub struct QSPI_DDR_TXDE_SPEC;
impl crate::RegisterSpec for QSPI_DDR_TXDE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_ddr_txde::R](R) reader structure"]
impl crate::Readable for QSPI_DDR_TXDE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_ddr_txde::W](W) writer structure"]
impl crate::Writable for QSPI_DDR_TXDE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_DDR_TXDE to value 0"]
impl crate::Resettable for QSPI_DDR_TXDE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
