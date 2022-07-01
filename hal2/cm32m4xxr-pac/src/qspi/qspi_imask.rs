#[doc = "Register `QSPI_IMASK` reader"]
pub struct R(crate::R<QSPI_IMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_IMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_IMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_IMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_IMASK` writer"]
pub struct W(crate::W<QSPI_IMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_IMASK_SPEC>;
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
impl From<crate::W<QSPI_IMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_IMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXFEIM` reader - TXFEIM"]
pub type TXFEIM_R = crate::BitReader<bool>;
#[doc = "Field `TXFEIM` writer - TXFEIM"]
pub type TXFEIM_W<'a> = crate::BitWriter<'a, u32, QSPI_IMASK_SPEC, bool, 0>;
#[doc = "Field `TXFOIM` reader - TXFOIM"]
pub type TXFOIM_R = crate::BitReader<bool>;
#[doc = "Field `TXFOIM` writer - TXFOIM"]
pub type TXFOIM_W<'a> = crate::BitWriter<'a, u32, QSPI_IMASK_SPEC, bool, 1>;
#[doc = "Field `RXFUIM` reader - RXFUIM"]
pub type RXFUIM_R = crate::BitReader<bool>;
#[doc = "Field `RXFUIM` writer - RXFUIM"]
pub type RXFUIM_W<'a> = crate::BitWriter<'a, u32, QSPI_IMASK_SPEC, bool, 2>;
#[doc = "Field `RXFOIM` reader - RXFOIM"]
pub type RXFOIM_R = crate::BitReader<bool>;
#[doc = "Field `RXFOIM` writer - RXFOIM"]
pub type RXFOIM_W<'a> = crate::BitWriter<'a, u32, QSPI_IMASK_SPEC, bool, 3>;
#[doc = "Field `RXFFIM` reader - RXFFIM"]
pub type RXFFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXFFIM` writer - RXFFIM"]
pub type RXFFIM_W<'a> = crate::BitWriter<'a, u32, QSPI_IMASK_SPEC, bool, 4>;
#[doc = "Field `MMCIM` reader - MMCIM"]
pub type MMCIM_R = crate::BitReader<bool>;
#[doc = "Field `MMCIM` writer - MMCIM"]
pub type MMCIM_W<'a> = crate::BitWriter<'a, u32, QSPI_IMASK_SPEC, bool, 5>;
#[doc = "Field `XRXOIM` reader - XRXOIM"]
pub type XRXOIM_R = crate::BitReader<bool>;
#[doc = "Field `XRXOIM` writer - XRXOIM"]
pub type XRXOIM_W<'a> = crate::BitWriter<'a, u32, QSPI_IMASK_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 0 - TXFEIM"]
    #[inline(always)]
    pub fn txfeim(&self) -> TXFEIM_R {
        TXFEIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXFOIM"]
    #[inline(always)]
    pub fn txfoim(&self) -> TXFOIM_R {
        TXFOIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXFUIM"]
    #[inline(always)]
    pub fn rxfuim(&self) -> RXFUIM_R {
        RXFUIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RXFOIM"]
    #[inline(always)]
    pub fn rxfoim(&self) -> RXFOIM_R {
        RXFOIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXFFIM"]
    #[inline(always)]
    pub fn rxffim(&self) -> RXFFIM_R {
        RXFFIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMCIM"]
    #[inline(always)]
    pub fn mmcim(&self) -> MMCIM_R {
        MMCIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - XRXOIM"]
    #[inline(always)]
    pub fn xrxoim(&self) -> XRXOIM_R {
        XRXOIM_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXFEIM"]
    #[inline(always)]
    pub fn txfeim(&mut self) -> TXFEIM_W {
        TXFEIM_W::new(self)
    }
    #[doc = "Bit 1 - TXFOIM"]
    #[inline(always)]
    pub fn txfoim(&mut self) -> TXFOIM_W {
        TXFOIM_W::new(self)
    }
    #[doc = "Bit 2 - RXFUIM"]
    #[inline(always)]
    pub fn rxfuim(&mut self) -> RXFUIM_W {
        RXFUIM_W::new(self)
    }
    #[doc = "Bit 3 - RXFOIM"]
    #[inline(always)]
    pub fn rxfoim(&mut self) -> RXFOIM_W {
        RXFOIM_W::new(self)
    }
    #[doc = "Bit 4 - RXFFIM"]
    #[inline(always)]
    pub fn rxffim(&mut self) -> RXFFIM_W {
        RXFFIM_W::new(self)
    }
    #[doc = "Bit 5 - MMCIM"]
    #[inline(always)]
    pub fn mmcim(&mut self) -> MMCIM_W {
        MMCIM_W::new(self)
    }
    #[doc = "Bit 6 - XRXOIM"]
    #[inline(always)]
    pub fn xrxoim(&mut self) -> XRXOIM_W {
        XRXOIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI_IMASK\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_imask](index.html) module"]
pub struct QSPI_IMASK_SPEC;
impl crate::RegisterSpec for QSPI_IMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_imask::R](R) reader structure"]
impl crate::Readable for QSPI_IMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_imask::W](W) writer structure"]
impl crate::Writable for QSPI_IMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_IMASK to value 0x7f"]
impl crate::Resettable for QSPI_IMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7f
    }
}
