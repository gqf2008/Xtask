#[doc = "Register `DMA_INTCLR` reader"]
pub struct R(crate::R<DMA_INTCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INTCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INTCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INTCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_INTCLR` writer"]
pub struct W(crate::W<DMA_INTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_INTCLR_SPEC>;
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
impl From<crate::W<DMA_INTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_INTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CGLBF1` reader - CGLBF1"]
pub type CGLBF1_R = crate::BitReader<bool>;
#[doc = "Field `CGLBF1` writer - CGLBF1"]
pub type CGLBF1_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 0>;
#[doc = "Field `CTXCF1` reader - CTXCF1"]
pub type CTXCF1_R = crate::BitReader<bool>;
#[doc = "Field `CTXCF1` writer - CTXCF1"]
pub type CTXCF1_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 1>;
#[doc = "Field `CHTXF1` reader - CHTXF1"]
pub type CHTXF1_R = crate::BitReader<bool>;
#[doc = "Field `CHTXF1` writer - CHTXF1"]
pub type CHTXF1_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 2>;
#[doc = "Field `CERRF1` reader - CERRF1"]
pub type CERRF1_R = crate::BitReader<bool>;
#[doc = "Field `CERRF1` writer - CERRF1"]
pub type CERRF1_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 3>;
#[doc = "Field `CGLBF2` reader - CGLBF2"]
pub type CGLBF2_R = crate::BitReader<bool>;
#[doc = "Field `CGLBF2` writer - CGLBF2"]
pub type CGLBF2_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 4>;
#[doc = "Field `CTXCF2` reader - CTXCF2"]
pub type CTXCF2_R = crate::BitReader<bool>;
#[doc = "Field `CTXCF2` writer - CTXCF2"]
pub type CTXCF2_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 5>;
#[doc = "Field `CHTXF2` reader - CHTXF2"]
pub type CHTXF2_R = crate::BitReader<bool>;
#[doc = "Field `CHTXF2` writer - CHTXF2"]
pub type CHTXF2_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 6>;
#[doc = "Field `CERRF2` reader - CERRF2"]
pub type CERRF2_R = crate::BitReader<bool>;
#[doc = "Field `CERRF2` writer - CERRF2"]
pub type CERRF2_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 7>;
#[doc = "Field `CGLBF3` reader - CGLBF3"]
pub type CGLBF3_R = crate::BitReader<bool>;
#[doc = "Field `CGLBF3` writer - CGLBF3"]
pub type CGLBF3_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 8>;
#[doc = "Field `CTXCF3` reader - CTXCF3"]
pub type CTXCF3_R = crate::BitReader<bool>;
#[doc = "Field `CTXCF3` writer - CTXCF3"]
pub type CTXCF3_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 9>;
#[doc = "Field `CHTXF3` reader - CHTXF3"]
pub type CHTXF3_R = crate::BitReader<bool>;
#[doc = "Field `CHTXF3` writer - CHTXF3"]
pub type CHTXF3_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 10>;
#[doc = "Field `CERRF3` reader - CERRF3"]
pub type CERRF3_R = crate::BitReader<bool>;
#[doc = "Field `CERRF3` writer - CERRF3"]
pub type CERRF3_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 11>;
#[doc = "Field `CGLBF4` reader - CGLBF4"]
pub type CGLBF4_R = crate::BitReader<bool>;
#[doc = "Field `CGLBF4` writer - CGLBF4"]
pub type CGLBF4_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 12>;
#[doc = "Field `CTXCF4` reader - CTXCF4"]
pub type CTXCF4_R = crate::BitReader<bool>;
#[doc = "Field `CTXCF4` writer - CTXCF4"]
pub type CTXCF4_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 13>;
#[doc = "Field `CHTXF4` reader - CHTXF4"]
pub type CHTXF4_R = crate::BitReader<bool>;
#[doc = "Field `CHTXF4` writer - CHTXF4"]
pub type CHTXF4_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 14>;
#[doc = "Field `CERRF4` reader - CERRF4"]
pub type CERRF4_R = crate::BitReader<bool>;
#[doc = "Field `CERRF4` writer - CERRF4"]
pub type CERRF4_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 15>;
#[doc = "Field `CGLBF5` reader - CGLBF5"]
pub type CGLBF5_R = crate::BitReader<bool>;
#[doc = "Field `CGLBF5` writer - CGLBF5"]
pub type CGLBF5_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 16>;
#[doc = "Field `CTXCF5` reader - CTXCF5"]
pub type CTXCF5_R = crate::BitReader<bool>;
#[doc = "Field `CTXCF5` writer - CTXCF5"]
pub type CTXCF5_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 17>;
#[doc = "Field `CHTXF5` reader - CHTXF5"]
pub type CHTXF5_R = crate::BitReader<bool>;
#[doc = "Field `CHTXF5` writer - CHTXF5"]
pub type CHTXF5_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 18>;
#[doc = "Field `CERRF5` reader - CERRF5"]
pub type CERRF5_R = crate::BitReader<bool>;
#[doc = "Field `CERRF5` writer - CERRF5"]
pub type CERRF5_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 19>;
#[doc = "Field `CGLBF6` reader - CGLBF6"]
pub type CGLBF6_R = crate::BitReader<bool>;
#[doc = "Field `CGLBF6` writer - CGLBF6"]
pub type CGLBF6_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 20>;
#[doc = "Field `CTXCF6` reader - CTXCF6"]
pub type CTXCF6_R = crate::BitReader<bool>;
#[doc = "Field `CTXCF6` writer - CTXCF6"]
pub type CTXCF6_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 21>;
#[doc = "Field `CHTXF6` reader - CHTXF6"]
pub type CHTXF6_R = crate::BitReader<bool>;
#[doc = "Field `CHTXF6` writer - CHTXF6"]
pub type CHTXF6_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 22>;
#[doc = "Field `CERRF6` reader - CERRF6"]
pub type CERRF6_R = crate::BitReader<bool>;
#[doc = "Field `CERRF6` writer - CERRF6"]
pub type CERRF6_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 23>;
#[doc = "Field `CGLBF7` reader - CGLBF7"]
pub type CGLBF7_R = crate::BitReader<bool>;
#[doc = "Field `CGLBF7` writer - CGLBF7"]
pub type CGLBF7_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 24>;
#[doc = "Field `CTXCF7` reader - CTXCF7"]
pub type CTXCF7_R = crate::BitReader<bool>;
#[doc = "Field `CTXCF7` writer - CTXCF7"]
pub type CTXCF7_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 25>;
#[doc = "Field `CHTXF7` reader - CHTXF7"]
pub type CHTXF7_R = crate::BitReader<bool>;
#[doc = "Field `CHTXF7` writer - CHTXF7"]
pub type CHTXF7_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 26>;
#[doc = "Field `CERRF7` reader - CERRF7"]
pub type CERRF7_R = crate::BitReader<bool>;
#[doc = "Field `CERRF7` writer - CERRF7"]
pub type CERRF7_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 27>;
#[doc = "Field `CGLBF8` reader - CGLBF8"]
pub type CGLBF8_R = crate::BitReader<bool>;
#[doc = "Field `CGLBF8` writer - CGLBF8"]
pub type CGLBF8_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 28>;
#[doc = "Field `CTXCF8` reader - CTXCF8"]
pub type CTXCF8_R = crate::BitReader<bool>;
#[doc = "Field `CTXCF8` writer - CTXCF8"]
pub type CTXCF8_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 29>;
#[doc = "Field `CHTXF8` reader - CHTXF8"]
pub type CHTXF8_R = crate::BitReader<bool>;
#[doc = "Field `CHTXF8` writer - CHTXF8"]
pub type CHTXF8_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 30>;
#[doc = "Field `CERRF8` reader - CERRF8"]
pub type CERRF8_R = crate::BitReader<bool>;
#[doc = "Field `CERRF8` writer - CERRF8"]
pub type CERRF8_W<'a> = crate::BitWriter<'a, u32, DMA_INTCLR_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - CGLBF1"]
    #[inline(always)]
    pub fn cglbf1(&self) -> CGLBF1_R {
        CGLBF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTXCF1"]
    #[inline(always)]
    pub fn ctxcf1(&self) -> CTXCF1_R {
        CTXCF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CHTXF1"]
    #[inline(always)]
    pub fn chtxf1(&self) -> CHTXF1_R {
        CHTXF1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CERRF1"]
    #[inline(always)]
    pub fn cerrf1(&self) -> CERRF1_R {
        CERRF1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CGLBF2"]
    #[inline(always)]
    pub fn cglbf2(&self) -> CGLBF2_R {
        CGLBF2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CTXCF2"]
    #[inline(always)]
    pub fn ctxcf2(&self) -> CTXCF2_R {
        CTXCF2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CHTXF2"]
    #[inline(always)]
    pub fn chtxf2(&self) -> CHTXF2_R {
        CHTXF2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CERRF2"]
    #[inline(always)]
    pub fn cerrf2(&self) -> CERRF2_R {
        CERRF2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CGLBF3"]
    #[inline(always)]
    pub fn cglbf3(&self) -> CGLBF3_R {
        CGLBF3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTXCF3"]
    #[inline(always)]
    pub fn ctxcf3(&self) -> CTXCF3_R {
        CTXCF3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CHTXF3"]
    #[inline(always)]
    pub fn chtxf3(&self) -> CHTXF3_R {
        CHTXF3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CERRF3"]
    #[inline(always)]
    pub fn cerrf3(&self) -> CERRF3_R {
        CERRF3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CGLBF4"]
    #[inline(always)]
    pub fn cglbf4(&self) -> CGLBF4_R {
        CGLBF4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CTXCF4"]
    #[inline(always)]
    pub fn ctxcf4(&self) -> CTXCF4_R {
        CTXCF4_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CHTXF4"]
    #[inline(always)]
    pub fn chtxf4(&self) -> CHTXF4_R {
        CHTXF4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CERRF4"]
    #[inline(always)]
    pub fn cerrf4(&self) -> CERRF4_R {
        CERRF4_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CGLBF5"]
    #[inline(always)]
    pub fn cglbf5(&self) -> CGLBF5_R {
        CGLBF5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CTXCF5"]
    #[inline(always)]
    pub fn ctxcf5(&self) -> CTXCF5_R {
        CTXCF5_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CHTXF5"]
    #[inline(always)]
    pub fn chtxf5(&self) -> CHTXF5_R {
        CHTXF5_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CERRF5"]
    #[inline(always)]
    pub fn cerrf5(&self) -> CERRF5_R {
        CERRF5_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CGLBF6"]
    #[inline(always)]
    pub fn cglbf6(&self) -> CGLBF6_R {
        CGLBF6_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CTXCF6"]
    #[inline(always)]
    pub fn ctxcf6(&self) -> CTXCF6_R {
        CTXCF6_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CHTXF6"]
    #[inline(always)]
    pub fn chtxf6(&self) -> CHTXF6_R {
        CHTXF6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CERRF6"]
    #[inline(always)]
    pub fn cerrf6(&self) -> CERRF6_R {
        CERRF6_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CGLBF7"]
    #[inline(always)]
    pub fn cglbf7(&self) -> CGLBF7_R {
        CGLBF7_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CTXCF7"]
    #[inline(always)]
    pub fn ctxcf7(&self) -> CTXCF7_R {
        CTXCF7_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CHTXF7"]
    #[inline(always)]
    pub fn chtxf7(&self) -> CHTXF7_R {
        CHTXF7_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CERRF7"]
    #[inline(always)]
    pub fn cerrf7(&self) -> CERRF7_R {
        CERRF7_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - CGLBF8"]
    #[inline(always)]
    pub fn cglbf8(&self) -> CGLBF8_R {
        CGLBF8_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - CTXCF8"]
    #[inline(always)]
    pub fn ctxcf8(&self) -> CTXCF8_R {
        CTXCF8_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - CHTXF8"]
    #[inline(always)]
    pub fn chtxf8(&self) -> CHTXF8_R {
        CHTXF8_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CERRF8"]
    #[inline(always)]
    pub fn cerrf8(&self) -> CERRF8_R {
        CERRF8_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CGLBF1"]
    #[inline(always)]
    pub fn cglbf1(&mut self) -> CGLBF1_W {
        CGLBF1_W::new(self)
    }
    #[doc = "Bit 1 - CTXCF1"]
    #[inline(always)]
    pub fn ctxcf1(&mut self) -> CTXCF1_W {
        CTXCF1_W::new(self)
    }
    #[doc = "Bit 2 - CHTXF1"]
    #[inline(always)]
    pub fn chtxf1(&mut self) -> CHTXF1_W {
        CHTXF1_W::new(self)
    }
    #[doc = "Bit 3 - CERRF1"]
    #[inline(always)]
    pub fn cerrf1(&mut self) -> CERRF1_W {
        CERRF1_W::new(self)
    }
    #[doc = "Bit 4 - CGLBF2"]
    #[inline(always)]
    pub fn cglbf2(&mut self) -> CGLBF2_W {
        CGLBF2_W::new(self)
    }
    #[doc = "Bit 5 - CTXCF2"]
    #[inline(always)]
    pub fn ctxcf2(&mut self) -> CTXCF2_W {
        CTXCF2_W::new(self)
    }
    #[doc = "Bit 6 - CHTXF2"]
    #[inline(always)]
    pub fn chtxf2(&mut self) -> CHTXF2_W {
        CHTXF2_W::new(self)
    }
    #[doc = "Bit 7 - CERRF2"]
    #[inline(always)]
    pub fn cerrf2(&mut self) -> CERRF2_W {
        CERRF2_W::new(self)
    }
    #[doc = "Bit 8 - CGLBF3"]
    #[inline(always)]
    pub fn cglbf3(&mut self) -> CGLBF3_W {
        CGLBF3_W::new(self)
    }
    #[doc = "Bit 9 - CTXCF3"]
    #[inline(always)]
    pub fn ctxcf3(&mut self) -> CTXCF3_W {
        CTXCF3_W::new(self)
    }
    #[doc = "Bit 10 - CHTXF3"]
    #[inline(always)]
    pub fn chtxf3(&mut self) -> CHTXF3_W {
        CHTXF3_W::new(self)
    }
    #[doc = "Bit 11 - CERRF3"]
    #[inline(always)]
    pub fn cerrf3(&mut self) -> CERRF3_W {
        CERRF3_W::new(self)
    }
    #[doc = "Bit 12 - CGLBF4"]
    #[inline(always)]
    pub fn cglbf4(&mut self) -> CGLBF4_W {
        CGLBF4_W::new(self)
    }
    #[doc = "Bit 13 - CTXCF4"]
    #[inline(always)]
    pub fn ctxcf4(&mut self) -> CTXCF4_W {
        CTXCF4_W::new(self)
    }
    #[doc = "Bit 14 - CHTXF4"]
    #[inline(always)]
    pub fn chtxf4(&mut self) -> CHTXF4_W {
        CHTXF4_W::new(self)
    }
    #[doc = "Bit 15 - CERRF4"]
    #[inline(always)]
    pub fn cerrf4(&mut self) -> CERRF4_W {
        CERRF4_W::new(self)
    }
    #[doc = "Bit 16 - CGLBF5"]
    #[inline(always)]
    pub fn cglbf5(&mut self) -> CGLBF5_W {
        CGLBF5_W::new(self)
    }
    #[doc = "Bit 17 - CTXCF5"]
    #[inline(always)]
    pub fn ctxcf5(&mut self) -> CTXCF5_W {
        CTXCF5_W::new(self)
    }
    #[doc = "Bit 18 - CHTXF5"]
    #[inline(always)]
    pub fn chtxf5(&mut self) -> CHTXF5_W {
        CHTXF5_W::new(self)
    }
    #[doc = "Bit 19 - CERRF5"]
    #[inline(always)]
    pub fn cerrf5(&mut self) -> CERRF5_W {
        CERRF5_W::new(self)
    }
    #[doc = "Bit 20 - CGLBF6"]
    #[inline(always)]
    pub fn cglbf6(&mut self) -> CGLBF6_W {
        CGLBF6_W::new(self)
    }
    #[doc = "Bit 21 - CTXCF6"]
    #[inline(always)]
    pub fn ctxcf6(&mut self) -> CTXCF6_W {
        CTXCF6_W::new(self)
    }
    #[doc = "Bit 22 - CHTXF6"]
    #[inline(always)]
    pub fn chtxf6(&mut self) -> CHTXF6_W {
        CHTXF6_W::new(self)
    }
    #[doc = "Bit 23 - CERRF6"]
    #[inline(always)]
    pub fn cerrf6(&mut self) -> CERRF6_W {
        CERRF6_W::new(self)
    }
    #[doc = "Bit 24 - CGLBF7"]
    #[inline(always)]
    pub fn cglbf7(&mut self) -> CGLBF7_W {
        CGLBF7_W::new(self)
    }
    #[doc = "Bit 25 - CTXCF7"]
    #[inline(always)]
    pub fn ctxcf7(&mut self) -> CTXCF7_W {
        CTXCF7_W::new(self)
    }
    #[doc = "Bit 26 - CHTXF7"]
    #[inline(always)]
    pub fn chtxf7(&mut self) -> CHTXF7_W {
        CHTXF7_W::new(self)
    }
    #[doc = "Bit 27 - CERRF7"]
    #[inline(always)]
    pub fn cerrf7(&mut self) -> CERRF7_W {
        CERRF7_W::new(self)
    }
    #[doc = "Bit 28 - CGLBF8"]
    #[inline(always)]
    pub fn cglbf8(&mut self) -> CGLBF8_W {
        CGLBF8_W::new(self)
    }
    #[doc = "Bit 29 - CTXCF8"]
    #[inline(always)]
    pub fn ctxcf8(&mut self) -> CTXCF8_W {
        CTXCF8_W::new(self)
    }
    #[doc = "Bit 30 - CHTXF8"]
    #[inline(always)]
    pub fn chtxf8(&mut self) -> CHTXF8_W {
        CHTXF8_W::new(self)
    }
    #[doc = "Bit 31 - CERRF8"]
    #[inline(always)]
    pub fn cerrf8(&mut self) -> CERRF8_W {
        CERRF8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_INTCLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_intclr](index.html) module"]
pub struct DMA_INTCLR_SPEC;
impl crate::RegisterSpec for DMA_INTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_intclr::R](R) reader structure"]
impl crate::Readable for DMA_INTCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_intclr::W](W) writer structure"]
impl crate::Writable for DMA_INTCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_INTCLR to value 0"]
impl crate::Resettable for DMA_INTCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
