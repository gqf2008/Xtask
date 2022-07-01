#[doc = "Register `QSPI_RISTS` reader"]
pub struct R(crate::R<QSPI_RISTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_RISTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_RISTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_RISTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_RISTS` writer"]
pub struct W(crate::W<QSPI_RISTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_RISTS_SPEC>;
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
impl From<crate::W<QSPI_RISTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_RISTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXFERIS` reader - TXFERIS"]
pub type TXFERIS_R = crate::BitReader<bool>;
#[doc = "Field `TXFERIS` writer - TXFERIS"]
pub type TXFERIS_W<'a> = crate::BitWriter<'a, u32, QSPI_RISTS_SPEC, bool, 0>;
#[doc = "Field `TXFORIS` reader - TXFORIS"]
pub type TXFORIS_R = crate::BitReader<bool>;
#[doc = "Field `TXFORIS` writer - TXFORIS"]
pub type TXFORIS_W<'a> = crate::BitWriter<'a, u32, QSPI_RISTS_SPEC, bool, 1>;
#[doc = "Field `RXFURIS` reader - RXFURIS"]
pub type RXFURIS_R = crate::BitReader<bool>;
#[doc = "Field `RXFURIS` writer - RXFURIS"]
pub type RXFURIS_W<'a> = crate::BitWriter<'a, u32, QSPI_RISTS_SPEC, bool, 2>;
#[doc = "Field `RXFORIS` reader - RXFORIS"]
pub type RXFORIS_R = crate::BitReader<bool>;
#[doc = "Field `RXFORIS` writer - RXFORIS"]
pub type RXFORIS_W<'a> = crate::BitWriter<'a, u32, QSPI_RISTS_SPEC, bool, 3>;
#[doc = "Field `RXFFRIS` reader - RXFFRIS"]
pub type RXFFRIS_R = crate::BitReader<bool>;
#[doc = "Field `RXFFRIS` writer - RXFFRIS"]
pub type RXFFRIS_W<'a> = crate::BitWriter<'a, u32, QSPI_RISTS_SPEC, bool, 4>;
#[doc = "Field `MMCRIS` reader - MMCRIS"]
pub type MMCRIS_R = crate::BitReader<bool>;
#[doc = "Field `MMCRIS` writer - MMCRIS"]
pub type MMCRIS_W<'a> = crate::BitWriter<'a, u32, QSPI_RISTS_SPEC, bool, 5>;
#[doc = "Field `XRXORIS` reader - XRXORIS"]
pub type XRXORIS_R = crate::BitReader<bool>;
#[doc = "Field `XRXORIS` writer - XRXORIS"]
pub type XRXORIS_W<'a> = crate::BitWriter<'a, u32, QSPI_RISTS_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 0 - TXFERIS"]
    #[inline(always)]
    pub fn txferis(&self) -> TXFERIS_R {
        TXFERIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXFORIS"]
    #[inline(always)]
    pub fn txforis(&self) -> TXFORIS_R {
        TXFORIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXFURIS"]
    #[inline(always)]
    pub fn rxfuris(&self) -> RXFURIS_R {
        RXFURIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RXFORIS"]
    #[inline(always)]
    pub fn rxforis(&self) -> RXFORIS_R {
        RXFORIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXFFRIS"]
    #[inline(always)]
    pub fn rxffris(&self) -> RXFFRIS_R {
        RXFFRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMCRIS"]
    #[inline(always)]
    pub fn mmcris(&self) -> MMCRIS_R {
        MMCRIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - XRXORIS"]
    #[inline(always)]
    pub fn xrxoris(&self) -> XRXORIS_R {
        XRXORIS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXFERIS"]
    #[inline(always)]
    pub fn txferis(&mut self) -> TXFERIS_W {
        TXFERIS_W::new(self)
    }
    #[doc = "Bit 1 - TXFORIS"]
    #[inline(always)]
    pub fn txforis(&mut self) -> TXFORIS_W {
        TXFORIS_W::new(self)
    }
    #[doc = "Bit 2 - RXFURIS"]
    #[inline(always)]
    pub fn rxfuris(&mut self) -> RXFURIS_W {
        RXFURIS_W::new(self)
    }
    #[doc = "Bit 3 - RXFORIS"]
    #[inline(always)]
    pub fn rxforis(&mut self) -> RXFORIS_W {
        RXFORIS_W::new(self)
    }
    #[doc = "Bit 4 - RXFFRIS"]
    #[inline(always)]
    pub fn rxffris(&mut self) -> RXFFRIS_W {
        RXFFRIS_W::new(self)
    }
    #[doc = "Bit 5 - MMCRIS"]
    #[inline(always)]
    pub fn mmcris(&mut self) -> MMCRIS_W {
        MMCRIS_W::new(self)
    }
    #[doc = "Bit 6 - XRXORIS"]
    #[inline(always)]
    pub fn xrxoris(&mut self) -> XRXORIS_W {
        XRXORIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI_RISTS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_rists](index.html) module"]
pub struct QSPI_RISTS_SPEC;
impl crate::RegisterSpec for QSPI_RISTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_rists::R](R) reader structure"]
impl crate::Readable for QSPI_RISTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_rists::W](W) writer structure"]
impl crate::Writable for QSPI_RISTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_RISTS to value 0"]
impl crate::Resettable for QSPI_RISTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
