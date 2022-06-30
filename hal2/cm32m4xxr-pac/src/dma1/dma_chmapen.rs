#[doc = "Register `DMA_CHMAPEN` reader"]
pub struct R(crate::R<DMA_CHMAPEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CHMAPEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CHMAPEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CHMAPEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CHMAPEN` writer"]
pub struct W(crate::W<DMA_CHMAPEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CHMAPEN_SPEC>;
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
impl From<crate::W<DMA_CHMAPEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CHMAPEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAP_EN` reader - MAP_EN"]
pub type MAP_EN_R = crate::BitReader<bool>;
#[doc = "Field `MAP_EN` writer - MAP_EN"]
pub type MAP_EN_W<'a> = crate::BitWriter<'a, u32, DMA_CHMAPEN_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - MAP_EN"]
    #[inline(always)]
    pub fn map_en(&self) -> MAP_EN_R {
        MAP_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MAP_EN"]
    #[inline(always)]
    pub fn map_en(&mut self) -> MAP_EN_W {
        MAP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_CHMAPEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chmapen](index.html) module"]
pub struct DMA_CHMAPEN_SPEC;
impl crate::RegisterSpec for DMA_CHMAPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_chmapen::R](R) reader structure"]
impl crate::Readable for DMA_CHMAPEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_chmapen::W](W) writer structure"]
impl crate::Writable for DMA_CHMAPEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_CHMAPEN to value 0"]
impl crate::Resettable for DMA_CHMAPEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
