#[doc = "Register `DMA_MADDR6` reader"]
pub struct R(crate::R<DMA_MADDR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_MADDR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_MADDR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_MADDR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_MADDR6` writer"]
pub struct W(crate::W<DMA_MADDR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_MADDR6_SPEC>;
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
impl From<crate::W<DMA_MADDR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_MADDR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - ADDR"]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - ADDR"]
pub type ADDR_W<'a> = crate::FieldWriter<'a, u32, DMA_MADDR6_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - ADDR"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ADDR"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_MADDR6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_maddr6](index.html) module"]
pub struct DMA_MADDR6_SPEC;
impl crate::RegisterSpec for DMA_MADDR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_maddr6::R](R) reader structure"]
impl crate::Readable for DMA_MADDR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_maddr6::W](W) writer structure"]
impl crate::Writable for DMA_MADDR6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_MADDR6 to value 0"]
impl crate::Resettable for DMA_MADDR6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
