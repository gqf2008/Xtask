#[doc = "Register `QSPI_DMARDL_CTRL` reader"]
pub struct R(crate::R<QSPI_DMARDL_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_DMARDL_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_DMARDL_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_DMARDL_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_DMARDL_CTRL` writer"]
pub struct W(crate::W<QSPI_DMARDL_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_DMARDL_CTRL_SPEC>;
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
impl From<crate::W<QSPI_DMARDL_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_DMARDL_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMARDL` reader - DMARDL"]
pub type DMARDL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMARDL` writer - DMARDL"]
pub type DMARDL_W<'a> = crate::FieldWriter<'a, u32, QSPI_DMARDL_CTRL_SPEC, u8, u8, 6, 0>;
impl R {
    #[doc = "Bits 0:5 - DMARDL"]
    #[inline(always)]
    pub fn dmardl(&self) -> DMARDL_R {
        DMARDL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - DMARDL"]
    #[inline(always)]
    pub fn dmardl(&mut self) -> DMARDL_W {
        DMARDL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI_DMARDL_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_dmardl_ctrl](index.html) module"]
pub struct QSPI_DMARDL_CTRL_SPEC;
impl crate::RegisterSpec for QSPI_DMARDL_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_dmardl_ctrl::R](R) reader structure"]
impl crate::Readable for QSPI_DMARDL_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_dmardl_ctrl::W](W) writer structure"]
impl crate::Writable for QSPI_DMARDL_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_DMARDL_CTRL to value 0"]
impl crate::Resettable for QSPI_DMARDL_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
