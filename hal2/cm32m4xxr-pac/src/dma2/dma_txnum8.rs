#[doc = "Register `DMA_TXNUM8` reader"]
pub struct R(crate::R<DMA_TXNUM8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_TXNUM8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_TXNUM8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_TXNUM8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_TXNUM8` writer"]
pub struct W(crate::W<DMA_TXNUM8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_TXNUM8_SPEC>;
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
impl From<crate::W<DMA_TXNUM8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_TXNUM8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDTX` reader - NDTX"]
pub type NDTX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NDTX` writer - NDTX"]
pub type NDTX_W<'a> = crate::FieldWriter<'a, u32, DMA_TXNUM8_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - NDTX"]
    #[inline(always)]
    pub fn ndtx(&self) -> NDTX_R {
        NDTX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - NDTX"]
    #[inline(always)]
    pub fn ndtx(&mut self) -> NDTX_W {
        NDTX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_TXNUM8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_txnum8](index.html) module"]
pub struct DMA_TXNUM8_SPEC;
impl crate::RegisterSpec for DMA_TXNUM8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_txnum8::R](R) reader structure"]
impl crate::Readable for DMA_TXNUM8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_txnum8::W](W) writer structure"]
impl crate::Writable for DMA_TXNUM8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_TXNUM8 to value 0"]
impl crate::Resettable for DMA_TXNUM8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
