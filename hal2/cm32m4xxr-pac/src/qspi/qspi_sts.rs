#[doc = "Register `QSPI_STS` reader"]
pub struct R(crate::R<QSPI_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_STS` writer"]
pub struct W(crate::W<QSPI_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_STS_SPEC>;
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
impl From<crate::W<QSPI_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSY` reader - BUSY"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - BUSY"]
pub type BUSY_W<'a> = crate::BitWriter<'a, u32, QSPI_STS_SPEC, bool, 0>;
#[doc = "Field `TXFNF` reader - TXFNF"]
pub type TXFNF_R = crate::BitReader<bool>;
#[doc = "Field `TXFNF` writer - TXFNF"]
pub type TXFNF_W<'a> = crate::BitWriter<'a, u32, QSPI_STS_SPEC, bool, 1>;
#[doc = "Field `TXFE` reader - TXFE"]
pub type TXFE_R = crate::BitReader<bool>;
#[doc = "Field `TXFE` writer - TXFE"]
pub type TXFE_W<'a> = crate::BitWriter<'a, u32, QSPI_STS_SPEC, bool, 2>;
#[doc = "Field `RXFNE` reader - RXFNE"]
pub type RXFNE_R = crate::BitReader<bool>;
#[doc = "Field `RXFNE` writer - RXFNE"]
pub type RXFNE_W<'a> = crate::BitWriter<'a, u32, QSPI_STS_SPEC, bool, 3>;
#[doc = "Field `RXFF` reader - RXFF"]
pub type RXFF_R = crate::BitReader<bool>;
#[doc = "Field `RXFF` writer - RXFF"]
pub type RXFF_W<'a> = crate::BitWriter<'a, u32, QSPI_STS_SPEC, bool, 4>;
#[doc = "Field `TX_ERR` reader - TX_ERR"]
pub type TX_ERR_R = crate::BitReader<bool>;
#[doc = "Field `TX_ERR` writer - TX_ERR"]
pub type TX_ERR_W<'a> = crate::BitWriter<'a, u32, QSPI_STS_SPEC, bool, 5>;
#[doc = "Field `DC_ERR` reader - DC_ERR"]
pub type DC_ERR_R = crate::BitReader<bool>;
#[doc = "Field `DC_ERR` writer - DC_ERR"]
pub type DC_ERR_W<'a> = crate::BitWriter<'a, u32, QSPI_STS_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 0 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXFNF"]
    #[inline(always)]
    pub fn txfnf(&self) -> TXFNF_R {
        TXFNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXFE"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RXFNE"]
    #[inline(always)]
    pub fn rxfne(&self) -> RXFNE_R {
        RXFNE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXFF"]
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX_ERR"]
    #[inline(always)]
    pub fn tx_err(&self) -> TX_ERR_R {
        TX_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DC_ERR"]
    #[inline(always)]
    pub fn dc_err(&self) -> DC_ERR_R {
        DC_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BUSY"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W::new(self)
    }
    #[doc = "Bit 1 - TXFNF"]
    #[inline(always)]
    pub fn txfnf(&mut self) -> TXFNF_W {
        TXFNF_W::new(self)
    }
    #[doc = "Bit 2 - TXFE"]
    #[inline(always)]
    pub fn txfe(&mut self) -> TXFE_W {
        TXFE_W::new(self)
    }
    #[doc = "Bit 3 - RXFNE"]
    #[inline(always)]
    pub fn rxfne(&mut self) -> RXFNE_W {
        RXFNE_W::new(self)
    }
    #[doc = "Bit 4 - RXFF"]
    #[inline(always)]
    pub fn rxff(&mut self) -> RXFF_W {
        RXFF_W::new(self)
    }
    #[doc = "Bit 5 - TX_ERR"]
    #[inline(always)]
    pub fn tx_err(&mut self) -> TX_ERR_W {
        TX_ERR_W::new(self)
    }
    #[doc = "Bit 6 - DC_ERR"]
    #[inline(always)]
    pub fn dc_err(&mut self) -> DC_ERR_W {
        DC_ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI_STS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_sts](index.html) module"]
pub struct QSPI_STS_SPEC;
impl crate::RegisterSpec for QSPI_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_sts::R](R) reader structure"]
impl crate::Readable for QSPI_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_sts::W](W) writer structure"]
impl crate::Writable for QSPI_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_STS to value 0x06"]
impl crate::Resettable for QSPI_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}
