#[doc = "Register `DMA_CHCFG7` reader"]
pub struct R(crate::R<DMA_CHCFG7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CHCFG7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CHCFG7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CHCFG7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CHCFG7` writer"]
pub struct W(crate::W<DMA_CHCFG7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CHCFG7_SPEC>;
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
impl From<crate::W<DMA_CHCFG7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CHCFG7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHEN` reader - CHEN"]
pub type CHEN_R = crate::BitReader<bool>;
#[doc = "Field `CHEN` writer - CHEN"]
pub type CHEN_W<'a> = crate::BitWriter<'a, u32, DMA_CHCFG7_SPEC, bool, 0>;
#[doc = "Field `TXCIE` reader - TXCIE"]
pub type TXCIE_R = crate::BitReader<bool>;
#[doc = "Field `TXCIE` writer - TXCIE"]
pub type TXCIE_W<'a> = crate::BitWriter<'a, u32, DMA_CHCFG7_SPEC, bool, 1>;
#[doc = "Field `HTXIE` reader - HTXIE"]
pub type HTXIE_R = crate::BitReader<bool>;
#[doc = "Field `HTXIE` writer - HTXIE"]
pub type HTXIE_W<'a> = crate::BitWriter<'a, u32, DMA_CHCFG7_SPEC, bool, 2>;
#[doc = "Field `ERRIE` reader - ERRIE"]
pub type ERRIE_R = crate::BitReader<bool>;
#[doc = "Field `ERRIE` writer - ERRIE"]
pub type ERRIE_W<'a> = crate::BitWriter<'a, u32, DMA_CHCFG7_SPEC, bool, 3>;
#[doc = "Field `DIR` reader - DIR"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - DIR"]
pub type DIR_W<'a> = crate::BitWriter<'a, u32, DMA_CHCFG7_SPEC, bool, 4>;
#[doc = "Field `CIRC` reader - CIRC"]
pub type CIRC_R = crate::BitReader<bool>;
#[doc = "Field `CIRC` writer - CIRC"]
pub type CIRC_W<'a> = crate::BitWriter<'a, u32, DMA_CHCFG7_SPEC, bool, 5>;
#[doc = "Field `PINC` reader - PINC"]
pub type PINC_R = crate::BitReader<bool>;
#[doc = "Field `PINC` writer - PINC"]
pub type PINC_W<'a> = crate::BitWriter<'a, u32, DMA_CHCFG7_SPEC, bool, 6>;
#[doc = "Field `MINC` reader - MINC"]
pub type MINC_R = crate::BitReader<bool>;
#[doc = "Field `MINC` writer - MINC"]
pub type MINC_W<'a> = crate::BitWriter<'a, u32, DMA_CHCFG7_SPEC, bool, 7>;
#[doc = "Field `PSIZE` reader - PSIZE"]
pub type PSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSIZE` writer - PSIZE"]
pub type PSIZE_W<'a> = crate::FieldWriter<'a, u32, DMA_CHCFG7_SPEC, u8, u8, 2, 8>;
#[doc = "Field `MSIZE` reader - MSIZE"]
pub type MSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSIZE` writer - MSIZE"]
pub type MSIZE_W<'a> = crate::FieldWriter<'a, u32, DMA_CHCFG7_SPEC, u8, u8, 2, 10>;
#[doc = "Field `PRIOLVL` reader - PRIOLVL"]
pub type PRIOLVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIOLVL` writer - PRIOLVL"]
pub type PRIOLVL_W<'a> = crate::FieldWriter<'a, u32, DMA_CHCFG7_SPEC, u8, u8, 2, 12>;
#[doc = "Field `MEM2MEM` reader - MEM2MEM"]
pub type MEM2MEM_R = crate::BitReader<bool>;
#[doc = "Field `MEM2MEM` writer - MEM2MEM"]
pub type MEM2MEM_W<'a> = crate::BitWriter<'a, u32, DMA_CHCFG7_SPEC, bool, 14>;
impl R {
    #[doc = "Bit 0 - CHEN"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXCIE"]
    #[inline(always)]
    pub fn txcie(&self) -> TXCIE_R {
        TXCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HTXIE"]
    #[inline(always)]
    pub fn htxie(&self) -> HTXIE_R {
        HTXIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ERRIE"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CIRC"]
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PINC"]
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MINC"]
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - PSIZE"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - MSIZE"]
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PRIOLVL"]
    #[inline(always)]
    pub fn priolvl(&self) -> PRIOLVL_R {
        PRIOLVL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - MEM2MEM"]
    #[inline(always)]
    pub fn mem2mem(&self) -> MEM2MEM_R {
        MEM2MEM_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CHEN"]
    #[inline(always)]
    pub fn chen(&mut self) -> CHEN_W {
        CHEN_W::new(self)
    }
    #[doc = "Bit 1 - TXCIE"]
    #[inline(always)]
    pub fn txcie(&mut self) -> TXCIE_W {
        TXCIE_W::new(self)
    }
    #[doc = "Bit 2 - HTXIE"]
    #[inline(always)]
    pub fn htxie(&mut self) -> HTXIE_W {
        HTXIE_W::new(self)
    }
    #[doc = "Bit 3 - ERRIE"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 4 - DIR"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W::new(self)
    }
    #[doc = "Bit 5 - CIRC"]
    #[inline(always)]
    pub fn circ(&mut self) -> CIRC_W {
        CIRC_W::new(self)
    }
    #[doc = "Bit 6 - PINC"]
    #[inline(always)]
    pub fn pinc(&mut self) -> PINC_W {
        PINC_W::new(self)
    }
    #[doc = "Bit 7 - MINC"]
    #[inline(always)]
    pub fn minc(&mut self) -> MINC_W {
        MINC_W::new(self)
    }
    #[doc = "Bits 8:9 - PSIZE"]
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W {
        PSIZE_W::new(self)
    }
    #[doc = "Bits 10:11 - MSIZE"]
    #[inline(always)]
    pub fn msize(&mut self) -> MSIZE_W {
        MSIZE_W::new(self)
    }
    #[doc = "Bits 12:13 - PRIOLVL"]
    #[inline(always)]
    pub fn priolvl(&mut self) -> PRIOLVL_W {
        PRIOLVL_W::new(self)
    }
    #[doc = "Bit 14 - MEM2MEM"]
    #[inline(always)]
    pub fn mem2mem(&mut self) -> MEM2MEM_W {
        MEM2MEM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_CHCFG7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_chcfg7](index.html) module"]
pub struct DMA_CHCFG7_SPEC;
impl crate::RegisterSpec for DMA_CHCFG7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_chcfg7::R](R) reader structure"]
impl crate::Readable for DMA_CHCFG7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_chcfg7::W](W) writer structure"]
impl crate::Writable for DMA_CHCFG7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_CHCFG7 to value 0"]
impl crate::Resettable for DMA_CHCFG7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
