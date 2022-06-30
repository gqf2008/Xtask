#[doc = "Register `QSPI_TXFT` reader"]
pub struct R(crate::R<QSPI_TXFT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_TXFT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_TXFT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_TXFT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_TXFT` writer"]
pub struct W(crate::W<QSPI_TXFT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_TXFT_SPEC>;
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
impl From<crate::W<QSPI_TXFT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_TXFT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXFT_TEI` reader - TXFT_TEI"]
pub type TXFT_TEI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXFT_TEI` writer - TXFT_TEI"]
pub type TXFT_TEI_W<'a> = crate::FieldWriter<'a, u32, QSPI_TXFT_SPEC, u8, u8, 5, 0>;
#[doc = "Field `TXFT_ST` reader - TXFT_ST"]
pub type TXFT_ST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXFT_ST` writer - TXFT_ST"]
pub type TXFT_ST_W<'a> = crate::FieldWriter<'a, u32, QSPI_TXFT_SPEC, u8, u8, 5, 16>;
impl R {
    #[doc = "Bits 0:4 - TXFT_TEI"]
    #[inline(always)]
    pub fn txft_tei(&self) -> TXFT_TEI_R {
        TXFT_TEI_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - TXFT_ST"]
    #[inline(always)]
    pub fn txft_st(&self) -> TXFT_ST_R {
        TXFT_ST_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - TXFT_TEI"]
    #[inline(always)]
    pub fn txft_tei(&mut self) -> TXFT_TEI_W {
        TXFT_TEI_W::new(self)
    }
    #[doc = "Bits 16:20 - TXFT_ST"]
    #[inline(always)]
    pub fn txft_st(&mut self) -> TXFT_ST_W {
        TXFT_ST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI_TXFT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_txft](index.html) module"]
pub struct QSPI_TXFT_SPEC;
impl crate::RegisterSpec for QSPI_TXFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_txft::R](R) reader structure"]
impl crate::Readable for QSPI_TXFT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_txft::W](W) writer structure"]
impl crate::Writable for QSPI_TXFT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_TXFT to value 0"]
impl crate::Resettable for QSPI_TXFT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
