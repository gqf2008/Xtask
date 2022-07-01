#[doc = "Register `RCC_AHBPCLKEN` reader"]
pub struct R(crate::R<RCC_AHBPCLKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHBPCLKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHBPCLKEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHBPCLKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_AHBPCLKEN` writer"]
pub struct W(crate::W<RCC_AHBPCLKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHBPCLKEN_SPEC>;
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
impl From<crate::W<RCC_AHBPCLKEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHBPCLKEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA1EN` reader - DMA1EN"]
pub type DMA1EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA1EN` writer - DMA1EN"]
pub type DMA1EN_W<'a> = crate::BitWriter<'a, u32, RCC_AHBPCLKEN_SPEC, bool, 0>;
#[doc = "Field `DMA2EN` reader - DMA2EN"]
pub type DMA2EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA2EN` writer - DMA2EN"]
pub type DMA2EN_W<'a> = crate::BitWriter<'a, u32, RCC_AHBPCLKEN_SPEC, bool, 1>;
#[doc = "Field `SRAMEN` reader - SRAMEN"]
pub type SRAMEN_R = crate::BitReader<bool>;
#[doc = "Field `SRAMEN` writer - SRAMEN"]
pub type SRAMEN_W<'a> = crate::BitWriter<'a, u32, RCC_AHBPCLKEN_SPEC, bool, 2>;
#[doc = "Field `FLITFEN` reader - FLITFEN"]
pub type FLITFEN_R = crate::BitReader<bool>;
#[doc = "Field `FLITFEN` writer - FLITFEN"]
pub type FLITFEN_W<'a> = crate::BitWriter<'a, u32, RCC_AHBPCLKEN_SPEC, bool, 4>;
#[doc = "Field `CRCEN` reader - CRCEN"]
pub type CRCEN_R = crate::BitReader<bool>;
#[doc = "Field `CRCEN` writer - CRCEN"]
pub type CRCEN_W<'a> = crate::BitWriter<'a, u32, RCC_AHBPCLKEN_SPEC, bool, 6>;
#[doc = "Field `RNGCEN` reader - RNGCEN"]
pub type RNGCEN_R = crate::BitReader<bool>;
#[doc = "Field `RNGCEN` writer - RNGCEN"]
pub type RNGCEN_W<'a> = crate::BitWriter<'a, u32, RCC_AHBPCLKEN_SPEC, bool, 9>;
#[doc = "Field `SACEN` reader - SACEN"]
pub type SACEN_R = crate::BitReader<bool>;
#[doc = "Field `SACEN` writer - SACEN"]
pub type SACEN_W<'a> = crate::BitWriter<'a, u32, RCC_AHBPCLKEN_SPEC, bool, 11>;
#[doc = "Field `ADC1EN` reader - ADC1EN"]
pub type ADC1EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC1EN` writer - ADC1EN"]
pub type ADC1EN_W<'a> = crate::BitWriter<'a, u32, RCC_AHBPCLKEN_SPEC, bool, 12>;
#[doc = "Field `ADC2EN` reader - ADC2EN"]
pub type ADC2EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC2EN` writer - ADC2EN"]
pub type ADC2EN_W<'a> = crate::BitWriter<'a, u32, RCC_AHBPCLKEN_SPEC, bool, 13>;
#[doc = "Field `ADC3EN` reader - ADC3EN"]
pub type ADC3EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC3EN` writer - ADC3EN"]
pub type ADC3EN_W<'a> = crate::BitWriter<'a, u32, RCC_AHBPCLKEN_SPEC, bool, 14>;
#[doc = "Field `ADC4EN` reader - ADC4EN"]
pub type ADC4EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC4EN` writer - ADC4EN"]
pub type ADC4EN_W<'a> = crate::BitWriter<'a, u32, RCC_AHBPCLKEN_SPEC, bool, 15>;
#[doc = "Field `QSPIEN` reader - QSPIEN"]
pub type QSPIEN_R = crate::BitReader<bool>;
#[doc = "Field `QSPIEN` writer - QSPIEN"]
pub type QSPIEN_W<'a> = crate::BitWriter<'a, u32, RCC_AHBPCLKEN_SPEC, bool, 17>;
impl R {
    #[doc = "Bit 0 - DMA1EN"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2EN"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAMEN"]
    #[inline(always)]
    pub fn sramen(&self) -> SRAMEN_R {
        SRAMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FLITFEN"]
    #[inline(always)]
    pub fn flitfen(&self) -> FLITFEN_R {
        FLITFEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CRCEN"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - RNGCEN"]
    #[inline(always)]
    pub fn rngcen(&self) -> RNGCEN_R {
        RNGCEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - SACEN"]
    #[inline(always)]
    pub fn sacen(&self) -> SACEN_R {
        SACEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ADC1EN"]
    #[inline(always)]
    pub fn adc1en(&self) -> ADC1EN_R {
        ADC1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC2EN"]
    #[inline(always)]
    pub fn adc2en(&self) -> ADC2EN_R {
        ADC2EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC3EN"]
    #[inline(always)]
    pub fn adc3en(&self) -> ADC3EN_R {
        ADC3EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC4EN"]
    #[inline(always)]
    pub fn adc4en(&self) -> ADC4EN_R {
        ADC4EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - QSPIEN"]
    #[inline(always)]
    pub fn qspien(&self) -> QSPIEN_R {
        QSPIEN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1EN"]
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W {
        DMA1EN_W::new(self)
    }
    #[doc = "Bit 1 - DMA2EN"]
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W {
        DMA2EN_W::new(self)
    }
    #[doc = "Bit 2 - SRAMEN"]
    #[inline(always)]
    pub fn sramen(&mut self) -> SRAMEN_W {
        SRAMEN_W::new(self)
    }
    #[doc = "Bit 4 - FLITFEN"]
    #[inline(always)]
    pub fn flitfen(&mut self) -> FLITFEN_W {
        FLITFEN_W::new(self)
    }
    #[doc = "Bit 6 - CRCEN"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 9 - RNGCEN"]
    #[inline(always)]
    pub fn rngcen(&mut self) -> RNGCEN_W {
        RNGCEN_W::new(self)
    }
    #[doc = "Bit 11 - SACEN"]
    #[inline(always)]
    pub fn sacen(&mut self) -> SACEN_W {
        SACEN_W::new(self)
    }
    #[doc = "Bit 12 - ADC1EN"]
    #[inline(always)]
    pub fn adc1en(&mut self) -> ADC1EN_W {
        ADC1EN_W::new(self)
    }
    #[doc = "Bit 13 - ADC2EN"]
    #[inline(always)]
    pub fn adc2en(&mut self) -> ADC2EN_W {
        ADC2EN_W::new(self)
    }
    #[doc = "Bit 14 - ADC3EN"]
    #[inline(always)]
    pub fn adc3en(&mut self) -> ADC3EN_W {
        ADC3EN_W::new(self)
    }
    #[doc = "Bit 15 - ADC4EN"]
    #[inline(always)]
    pub fn adc4en(&mut self) -> ADC4EN_W {
        ADC4EN_W::new(self)
    }
    #[doc = "Bit 17 - QSPIEN"]
    #[inline(always)]
    pub fn qspien(&mut self) -> QSPIEN_W {
        QSPIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC_AHBPCLKEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahbpclken](index.html) module"]
pub struct RCC_AHBPCLKEN_SPEC;
impl crate::RegisterSpec for RCC_AHBPCLKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ahbpclken::R](R) reader structure"]
impl crate::Readable for RCC_AHBPCLKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ahbpclken::W](W) writer structure"]
impl crate::Writable for RCC_AHBPCLKEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_AHBPCLKEN to value 0x14"]
impl crate::Resettable for RCC_AHBPCLKEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x14
    }
}
