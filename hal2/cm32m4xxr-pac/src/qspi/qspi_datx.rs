#[doc = "Register `QSPI_DATx` reader"]
pub struct R(crate::R<QSPI_DATX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_DATX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_DATX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_DATX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_DATx` writer"]
pub struct W(crate::W<QSPI_DATX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_DATX_SPEC>;
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
impl From<crate::W<QSPI_DATX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_DATX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATx` reader - DATx"]
pub type DATX_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATx` writer - DATx"]
pub type DATX_W<'a> = crate::FieldWriter<'a, u32, QSPI_DATX_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - DATx"]
    #[inline(always)]
    pub fn datx(&self) -> DATX_R {
        DATX_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DATx"]
    #[inline(always)]
    pub fn datx(&mut self) -> DATX_W {
        DATX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI_DATx\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_datx](index.html) module"]
pub struct QSPI_DATX_SPEC;
impl crate::RegisterSpec for QSPI_DATX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_datx::R](R) reader structure"]
impl crate::Readable for QSPI_DATX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_datx::W](W) writer structure"]
impl crate::Writable for QSPI_DATX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_DATx to value 0"]
impl crate::Resettable for QSPI_DATX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
