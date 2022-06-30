#[doc = "Register `QSPI_RXFN` reader"]
pub struct R(crate::R<QSPI_RXFN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_RXFN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_RXFN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_RXFN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_RXFN` writer"]
pub struct W(crate::W<QSPI_RXFN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_RXFN_SPEC>;
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
impl From<crate::W<QSPI_RXFN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_RXFN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFN` reader - RXFN"]
pub type RXFN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXFN` writer - RXFN"]
pub type RXFN_W<'a> = crate::FieldWriter<'a, u32, QSPI_RXFN_SPEC, u8, u8, 6, 0>;
impl R {
    #[doc = "Bits 0:5 - RXFN"]
    #[inline(always)]
    pub fn rxfn(&self) -> RXFN_R {
        RXFN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - RXFN"]
    #[inline(always)]
    pub fn rxfn(&mut self) -> RXFN_W {
        RXFN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI_RXFN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_rxfn](index.html) module"]
pub struct QSPI_RXFN_SPEC;
impl crate::RegisterSpec for QSPI_RXFN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_rxfn::R](R) reader structure"]
impl crate::Readable for QSPI_RXFN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_rxfn::W](W) writer structure"]
impl crate::Writable for QSPI_RXFN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_RXFN to value 0"]
impl crate::Resettable for QSPI_RXFN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
