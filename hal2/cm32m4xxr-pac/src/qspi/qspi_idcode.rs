#[doc = "Register `QSPI_IDCODE` reader"]
pub struct R(crate::R<QSPI_IDCODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_IDCODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_IDCODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_IDCODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_IDCODE` writer"]
pub struct W(crate::W<QSPI_IDCODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_IDCODE_SPEC>;
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
impl From<crate::W<QSPI_IDCODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_IDCODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDCODE` reader - IDCODE"]
pub type IDCODE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IDCODE` writer - IDCODE"]
pub type IDCODE_W<'a> = crate::FieldWriter<'a, u32, QSPI_IDCODE_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - IDCODE"]
    #[inline(always)]
    pub fn idcode(&self) -> IDCODE_R {
        IDCODE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IDCODE"]
    #[inline(always)]
    pub fn idcode(&mut self) -> IDCODE_W {
        IDCODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI_IDCODE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_idcode](index.html) module"]
pub struct QSPI_IDCODE_SPEC;
impl crate::RegisterSpec for QSPI_IDCODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_idcode::R](R) reader structure"]
impl crate::Readable for QSPI_IDCODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_idcode::W](W) writer structure"]
impl crate::Writable for QSPI_IDCODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_IDCODE to value 0xffff_ffff"]
impl crate::Resettable for QSPI_IDCODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
