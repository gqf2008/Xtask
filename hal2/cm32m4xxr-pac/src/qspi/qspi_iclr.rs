#[doc = "Register `QSPI_ICLR` reader"]
pub struct R(crate::R<QSPI_ICLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_ICLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_ICLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_ICLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_ICLR` writer"]
pub struct W(crate::W<QSPI_ICLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_ICLR_SPEC>;
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
impl From<crate::W<QSPI_ICLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_ICLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTC` reader - INTC"]
pub type INTC_R = crate::BitReader<bool>;
#[doc = "Field `INTC` writer - INTC"]
pub type INTC_W<'a> = crate::BitWriter<'a, u32, QSPI_ICLR_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - INTC"]
    #[inline(always)]
    pub fn intc(&self) -> INTC_R {
        INTC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INTC"]
    #[inline(always)]
    pub fn intc(&mut self) -> INTC_W {
        INTC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI_ICLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_iclr](index.html) module"]
pub struct QSPI_ICLR_SPEC;
impl crate::RegisterSpec for QSPI_ICLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_iclr::R](R) reader structure"]
impl crate::Readable for QSPI_ICLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_iclr::W](W) writer structure"]
impl crate::Writable for QSPI_ICLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_ICLR to value 0"]
impl crate::Resettable for QSPI_ICLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
