#[doc = "Register `QSPI_DMA_CTRL` reader"]
pub struct R(crate::R<QSPI_DMA_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_DMA_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_DMA_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_DMA_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_DMA_CTRL` writer"]
pub struct W(crate::W<QSPI_DMA_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_DMA_CTRL_SPEC>;
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
impl From<crate::W<QSPI_DMA_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_DMA_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_DMA_EN` reader - RX_DMA_EN"]
pub type RX_DMA_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_DMA_EN` writer - RX_DMA_EN"]
pub type RX_DMA_EN_W<'a> = crate::BitWriter<'a, u32, QSPI_DMA_CTRL_SPEC, bool, 0>;
#[doc = "Field `TX_DMA_EN` reader - TX_DMA_EN"]
pub type TX_DMA_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_DMA_EN` writer - TX_DMA_EN"]
pub type TX_DMA_EN_W<'a> = crate::BitWriter<'a, u32, QSPI_DMA_CTRL_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - RX_DMA_EN"]
    #[inline(always)]
    pub fn rx_dma_en(&self) -> RX_DMA_EN_R {
        RX_DMA_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX_DMA_EN"]
    #[inline(always)]
    pub fn tx_dma_en(&self) -> TX_DMA_EN_R {
        TX_DMA_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX_DMA_EN"]
    #[inline(always)]
    pub fn rx_dma_en(&mut self) -> RX_DMA_EN_W {
        RX_DMA_EN_W::new(self)
    }
    #[doc = "Bit 1 - TX_DMA_EN"]
    #[inline(always)]
    pub fn tx_dma_en(&mut self) -> TX_DMA_EN_W {
        TX_DMA_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI_DMA_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_dma_ctrl](index.html) module"]
pub struct QSPI_DMA_CTRL_SPEC;
impl crate::RegisterSpec for QSPI_DMA_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_dma_ctrl::R](R) reader structure"]
impl crate::Readable for QSPI_DMA_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_dma_ctrl::W](W) writer structure"]
impl crate::Writable for QSPI_DMA_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_DMA_CTRL to value 0"]
impl crate::Resettable for QSPI_DMA_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
