#[doc = "Register `QSPI_RS_DELAY` reader"]
pub struct R(crate::R<QSPI_RS_DELAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_RS_DELAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_RS_DELAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_RS_DELAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_RS_DELAY` writer"]
pub struct W(crate::W<QSPI_RS_DELAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_RS_DELAY_SPEC>;
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
impl From<crate::W<QSPI_RS_DELAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_RS_DELAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDCN` reader - SDCN"]
pub type SDCN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDCN` writer - SDCN"]
pub type SDCN_W<'a> = crate::FieldWriter<'a, u32, QSPI_RS_DELAY_SPEC, u8, u8, 8, 0>;
#[doc = "Field `SES` reader - SES"]
pub type SES_R = crate::BitReader<bool>;
#[doc = "Field `SES` writer - SES"]
pub type SES_W<'a> = crate::BitWriter<'a, u32, QSPI_RS_DELAY_SPEC, bool, 16>;
impl R {
    #[doc = "Bits 0:7 - SDCN"]
    #[inline(always)]
    pub fn sdcn(&self) -> SDCN_R {
        SDCN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - SES"]
    #[inline(always)]
    pub fn ses(&self) -> SES_R {
        SES_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - SDCN"]
    #[inline(always)]
    pub fn sdcn(&mut self) -> SDCN_W {
        SDCN_W::new(self)
    }
    #[doc = "Bit 16 - SES"]
    #[inline(always)]
    pub fn ses(&mut self) -> SES_W {
        SES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI_RS_DELAY\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_rs_delay](index.html) module"]
pub struct QSPI_RS_DELAY_SPEC;
impl crate::RegisterSpec for QSPI_RS_DELAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_rs_delay::R](R) reader structure"]
impl crate::Readable for QSPI_RS_DELAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_rs_delay::W](W) writer structure"]
impl crate::Writable for QSPI_RS_DELAY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_RS_DELAY to value 0"]
impl crate::Resettable for QSPI_RS_DELAY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
