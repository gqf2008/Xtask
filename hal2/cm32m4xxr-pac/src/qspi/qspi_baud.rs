#[doc = "Register `QSPI_BAUD` reader"]
pub struct R(crate::R<QSPI_BAUD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_BAUD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_BAUD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_BAUD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_BAUD` writer"]
pub struct W(crate::W<QSPI_BAUD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_BAUD_SPEC>;
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
impl From<crate::W<QSPI_BAUD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_BAUD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_DIV` reader - CLK_DIV"]
pub type CLK_DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLK_DIV` writer - CLK_DIV"]
pub type CLK_DIV_W<'a> = crate::FieldWriter<'a, u32, QSPI_BAUD_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - CLK_DIV"]
    #[inline(always)]
    pub fn clk_div(&self) -> CLK_DIV_R {
        CLK_DIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CLK_DIV"]
    #[inline(always)]
    pub fn clk_div(&mut self) -> CLK_DIV_W {
        CLK_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI_BAUD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_baud](index.html) module"]
pub struct QSPI_BAUD_SPEC;
impl crate::RegisterSpec for QSPI_BAUD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_baud::R](R) reader structure"]
impl crate::Readable for QSPI_BAUD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_baud::W](W) writer structure"]
impl crate::Writable for QSPI_BAUD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_BAUD to value 0"]
impl crate::Resettable for QSPI_BAUD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
