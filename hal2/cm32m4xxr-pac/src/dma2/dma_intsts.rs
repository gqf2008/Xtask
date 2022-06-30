#[doc = "Register `DMA_INTSTS` reader"]
pub struct R(crate::R<DMA_INTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_INTSTS` writer"]
pub struct W(crate::W<DMA_INTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_INTSTS_SPEC>;
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
impl From<crate::W<DMA_INTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_INTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GLBF1` reader - GLBF1"]
pub type GLBF1_R = crate::BitReader<bool>;
#[doc = "Field `GLBF1` writer - GLBF1"]
pub type GLBF1_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 0>;
#[doc = "Field `TXCF1` reader - TXCF1"]
pub type TXCF1_R = crate::BitReader<bool>;
#[doc = "Field `TXCF1` writer - TXCF1"]
pub type TXCF1_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 1>;
#[doc = "Field `HTXF1` reader - HTXF1"]
pub type HTXF1_R = crate::BitReader<bool>;
#[doc = "Field `HTXF1` writer - HTXF1"]
pub type HTXF1_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 2>;
#[doc = "Field `ERRF1` reader - ERRF1"]
pub type ERRF1_R = crate::BitReader<bool>;
#[doc = "Field `ERRF1` writer - ERRF1"]
pub type ERRF1_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 3>;
#[doc = "Field `GLBF2` reader - GLBF2"]
pub type GLBF2_R = crate::BitReader<bool>;
#[doc = "Field `GLBF2` writer - GLBF2"]
pub type GLBF2_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 4>;
#[doc = "Field `TXCF2` reader - TXCF2"]
pub type TXCF2_R = crate::BitReader<bool>;
#[doc = "Field `TXCF2` writer - TXCF2"]
pub type TXCF2_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 5>;
#[doc = "Field `HTXF2` reader - HTXF2"]
pub type HTXF2_R = crate::BitReader<bool>;
#[doc = "Field `HTXF2` writer - HTXF2"]
pub type HTXF2_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 6>;
#[doc = "Field `ERRF2` reader - ERRF2"]
pub type ERRF2_R = crate::BitReader<bool>;
#[doc = "Field `ERRF2` writer - ERRF2"]
pub type ERRF2_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 7>;
#[doc = "Field `GLBF3` reader - GLBF3"]
pub type GLBF3_R = crate::BitReader<bool>;
#[doc = "Field `GLBF3` writer - GLBF3"]
pub type GLBF3_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 8>;
#[doc = "Field `TXCF3` reader - TXCF3"]
pub type TXCF3_R = crate::BitReader<bool>;
#[doc = "Field `TXCF3` writer - TXCF3"]
pub type TXCF3_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 9>;
#[doc = "Field `HTXF3` reader - HTXF3"]
pub type HTXF3_R = crate::BitReader<bool>;
#[doc = "Field `HTXF3` writer - HTXF3"]
pub type HTXF3_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 10>;
#[doc = "Field `ERRF3` reader - ERRF3"]
pub type ERRF3_R = crate::BitReader<bool>;
#[doc = "Field `ERRF3` writer - ERRF3"]
pub type ERRF3_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 11>;
#[doc = "Field `GLBF4` reader - GLBF4"]
pub type GLBF4_R = crate::BitReader<bool>;
#[doc = "Field `GLBF4` writer - GLBF4"]
pub type GLBF4_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 12>;
#[doc = "Field `TXCF4` reader - TXCF4"]
pub type TXCF4_R = crate::BitReader<bool>;
#[doc = "Field `TXCF4` writer - TXCF4"]
pub type TXCF4_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 13>;
#[doc = "Field `HTXF4` reader - HTXF4"]
pub type HTXF4_R = crate::BitReader<bool>;
#[doc = "Field `HTXF4` writer - HTXF4"]
pub type HTXF4_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 14>;
#[doc = "Field `ERRF4` reader - ERRF4"]
pub type ERRF4_R = crate::BitReader<bool>;
#[doc = "Field `ERRF4` writer - ERRF4"]
pub type ERRF4_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 15>;
#[doc = "Field `GLBF5` reader - GLBF5"]
pub type GLBF5_R = crate::BitReader<bool>;
#[doc = "Field `GLBF5` writer - GLBF5"]
pub type GLBF5_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 16>;
#[doc = "Field `TXCF5` reader - TXCF5"]
pub type TXCF5_R = crate::BitReader<bool>;
#[doc = "Field `TXCF5` writer - TXCF5"]
pub type TXCF5_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 17>;
#[doc = "Field `HTXF5` reader - HTXF5"]
pub type HTXF5_R = crate::BitReader<bool>;
#[doc = "Field `HTXF5` writer - HTXF5"]
pub type HTXF5_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 18>;
#[doc = "Field `ERRF5` reader - ERRF5"]
pub type ERRF5_R = crate::BitReader<bool>;
#[doc = "Field `ERRF5` writer - ERRF5"]
pub type ERRF5_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 19>;
#[doc = "Field `GLBF6` reader - GLBF6"]
pub type GLBF6_R = crate::BitReader<bool>;
#[doc = "Field `GLBF6` writer - GLBF6"]
pub type GLBF6_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 20>;
#[doc = "Field `TXCF6` reader - TXCF6"]
pub type TXCF6_R = crate::BitReader<bool>;
#[doc = "Field `TXCF6` writer - TXCF6"]
pub type TXCF6_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 21>;
#[doc = "Field `HTXF6` reader - HTXF6"]
pub type HTXF6_R = crate::BitReader<bool>;
#[doc = "Field `HTXF6` writer - HTXF6"]
pub type HTXF6_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 22>;
#[doc = "Field `ERRF6` reader - ERRF6"]
pub type ERRF6_R = crate::BitReader<bool>;
#[doc = "Field `ERRF6` writer - ERRF6"]
pub type ERRF6_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 23>;
#[doc = "Field `GLBF7` reader - GLBF7"]
pub type GLBF7_R = crate::BitReader<bool>;
#[doc = "Field `GLBF7` writer - GLBF7"]
pub type GLBF7_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 24>;
#[doc = "Field `TXCF7` reader - TXCF7"]
pub type TXCF7_R = crate::BitReader<bool>;
#[doc = "Field `TXCF7` writer - TXCF7"]
pub type TXCF7_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 25>;
#[doc = "Field `HTXF7` reader - HTXF7"]
pub type HTXF7_R = crate::BitReader<bool>;
#[doc = "Field `HTXF7` writer - HTXF7"]
pub type HTXF7_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 26>;
#[doc = "Field `ERRF7` reader - ERRF7"]
pub type ERRF7_R = crate::BitReader<bool>;
#[doc = "Field `ERRF7` writer - ERRF7"]
pub type ERRF7_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 27>;
#[doc = "Field `GLBF8` reader - GLBF8"]
pub type GLBF8_R = crate::BitReader<bool>;
#[doc = "Field `GLBF8` writer - GLBF8"]
pub type GLBF8_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 28>;
#[doc = "Field `TXCF8` reader - TXCF8"]
pub type TXCF8_R = crate::BitReader<bool>;
#[doc = "Field `TXCF8` writer - TXCF8"]
pub type TXCF8_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 29>;
#[doc = "Field `HTXF8` reader - HTXF8"]
pub type HTXF8_R = crate::BitReader<bool>;
#[doc = "Field `HTXF8` writer - HTXF8"]
pub type HTXF8_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 30>;
#[doc = "Field `ERRF8` reader - ERRF8"]
pub type ERRF8_R = crate::BitReader<bool>;
#[doc = "Field `ERRF8` writer - ERRF8"]
pub type ERRF8_W<'a> = crate::BitWriter<'a, u32, DMA_INTSTS_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - GLBF1"]
    #[inline(always)]
    pub fn glbf1(&self) -> GLBF1_R {
        GLBF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXCF1"]
    #[inline(always)]
    pub fn txcf1(&self) -> TXCF1_R {
        TXCF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HTXF1"]
    #[inline(always)]
    pub fn htxf1(&self) -> HTXF1_R {
        HTXF1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ERRF1"]
    #[inline(always)]
    pub fn errf1(&self) -> ERRF1_R {
        ERRF1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GLBF2"]
    #[inline(always)]
    pub fn glbf2(&self) -> GLBF2_R {
        GLBF2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXCF2"]
    #[inline(always)]
    pub fn txcf2(&self) -> TXCF2_R {
        TXCF2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HTXF2"]
    #[inline(always)]
    pub fn htxf2(&self) -> HTXF2_R {
        HTXF2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ERRF2"]
    #[inline(always)]
    pub fn errf2(&self) -> ERRF2_R {
        ERRF2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GLBF3"]
    #[inline(always)]
    pub fn glbf3(&self) -> GLBF3_R {
        GLBF3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TXCF3"]
    #[inline(always)]
    pub fn txcf3(&self) -> TXCF3_R {
        TXCF3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HTXF3"]
    #[inline(always)]
    pub fn htxf3(&self) -> HTXF3_R {
        HTXF3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ERRF3"]
    #[inline(always)]
    pub fn errf3(&self) -> ERRF3_R {
        ERRF3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GLBF4"]
    #[inline(always)]
    pub fn glbf4(&self) -> GLBF4_R {
        GLBF4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TXCF4"]
    #[inline(always)]
    pub fn txcf4(&self) -> TXCF4_R {
        TXCF4_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - HTXF4"]
    #[inline(always)]
    pub fn htxf4(&self) -> HTXF4_R {
        HTXF4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ERRF4"]
    #[inline(always)]
    pub fn errf4(&self) -> ERRF4_R {
        ERRF4_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GLBF5"]
    #[inline(always)]
    pub fn glbf5(&self) -> GLBF5_R {
        GLBF5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TXCF5"]
    #[inline(always)]
    pub fn txcf5(&self) -> TXCF5_R {
        TXCF5_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HTXF5"]
    #[inline(always)]
    pub fn htxf5(&self) -> HTXF5_R {
        HTXF5_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ERRF5"]
    #[inline(always)]
    pub fn errf5(&self) -> ERRF5_R {
        ERRF5_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GLBF6"]
    #[inline(always)]
    pub fn glbf6(&self) -> GLBF6_R {
        GLBF6_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TXCF6"]
    #[inline(always)]
    pub fn txcf6(&self) -> TXCF6_R {
        TXCF6_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - HTXF6"]
    #[inline(always)]
    pub fn htxf6(&self) -> HTXF6_R {
        HTXF6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ERRF6"]
    #[inline(always)]
    pub fn errf6(&self) -> ERRF6_R {
        ERRF6_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GLBF7"]
    #[inline(always)]
    pub fn glbf7(&self) -> GLBF7_R {
        GLBF7_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TXCF7"]
    #[inline(always)]
    pub fn txcf7(&self) -> TXCF7_R {
        TXCF7_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - HTXF7"]
    #[inline(always)]
    pub fn htxf7(&self) -> HTXF7_R {
        HTXF7_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - ERRF7"]
    #[inline(always)]
    pub fn errf7(&self) -> ERRF7_R {
        ERRF7_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GLBF8"]
    #[inline(always)]
    pub fn glbf8(&self) -> GLBF8_R {
        GLBF8_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TXCF8"]
    #[inline(always)]
    pub fn txcf8(&self) -> TXCF8_R {
        TXCF8_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - HTXF8"]
    #[inline(always)]
    pub fn htxf8(&self) -> HTXF8_R {
        HTXF8_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ERRF8"]
    #[inline(always)]
    pub fn errf8(&self) -> ERRF8_R {
        ERRF8_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GLBF1"]
    #[inline(always)]
    pub fn glbf1(&mut self) -> GLBF1_W {
        GLBF1_W::new(self)
    }
    #[doc = "Bit 1 - TXCF1"]
    #[inline(always)]
    pub fn txcf1(&mut self) -> TXCF1_W {
        TXCF1_W::new(self)
    }
    #[doc = "Bit 2 - HTXF1"]
    #[inline(always)]
    pub fn htxf1(&mut self) -> HTXF1_W {
        HTXF1_W::new(self)
    }
    #[doc = "Bit 3 - ERRF1"]
    #[inline(always)]
    pub fn errf1(&mut self) -> ERRF1_W {
        ERRF1_W::new(self)
    }
    #[doc = "Bit 4 - GLBF2"]
    #[inline(always)]
    pub fn glbf2(&mut self) -> GLBF2_W {
        GLBF2_W::new(self)
    }
    #[doc = "Bit 5 - TXCF2"]
    #[inline(always)]
    pub fn txcf2(&mut self) -> TXCF2_W {
        TXCF2_W::new(self)
    }
    #[doc = "Bit 6 - HTXF2"]
    #[inline(always)]
    pub fn htxf2(&mut self) -> HTXF2_W {
        HTXF2_W::new(self)
    }
    #[doc = "Bit 7 - ERRF2"]
    #[inline(always)]
    pub fn errf2(&mut self) -> ERRF2_W {
        ERRF2_W::new(self)
    }
    #[doc = "Bit 8 - GLBF3"]
    #[inline(always)]
    pub fn glbf3(&mut self) -> GLBF3_W {
        GLBF3_W::new(self)
    }
    #[doc = "Bit 9 - TXCF3"]
    #[inline(always)]
    pub fn txcf3(&mut self) -> TXCF3_W {
        TXCF3_W::new(self)
    }
    #[doc = "Bit 10 - HTXF3"]
    #[inline(always)]
    pub fn htxf3(&mut self) -> HTXF3_W {
        HTXF3_W::new(self)
    }
    #[doc = "Bit 11 - ERRF3"]
    #[inline(always)]
    pub fn errf3(&mut self) -> ERRF3_W {
        ERRF3_W::new(self)
    }
    #[doc = "Bit 12 - GLBF4"]
    #[inline(always)]
    pub fn glbf4(&mut self) -> GLBF4_W {
        GLBF4_W::new(self)
    }
    #[doc = "Bit 13 - TXCF4"]
    #[inline(always)]
    pub fn txcf4(&mut self) -> TXCF4_W {
        TXCF4_W::new(self)
    }
    #[doc = "Bit 14 - HTXF4"]
    #[inline(always)]
    pub fn htxf4(&mut self) -> HTXF4_W {
        HTXF4_W::new(self)
    }
    #[doc = "Bit 15 - ERRF4"]
    #[inline(always)]
    pub fn errf4(&mut self) -> ERRF4_W {
        ERRF4_W::new(self)
    }
    #[doc = "Bit 16 - GLBF5"]
    #[inline(always)]
    pub fn glbf5(&mut self) -> GLBF5_W {
        GLBF5_W::new(self)
    }
    #[doc = "Bit 17 - TXCF5"]
    #[inline(always)]
    pub fn txcf5(&mut self) -> TXCF5_W {
        TXCF5_W::new(self)
    }
    #[doc = "Bit 18 - HTXF5"]
    #[inline(always)]
    pub fn htxf5(&mut self) -> HTXF5_W {
        HTXF5_W::new(self)
    }
    #[doc = "Bit 19 - ERRF5"]
    #[inline(always)]
    pub fn errf5(&mut self) -> ERRF5_W {
        ERRF5_W::new(self)
    }
    #[doc = "Bit 20 - GLBF6"]
    #[inline(always)]
    pub fn glbf6(&mut self) -> GLBF6_W {
        GLBF6_W::new(self)
    }
    #[doc = "Bit 21 - TXCF6"]
    #[inline(always)]
    pub fn txcf6(&mut self) -> TXCF6_W {
        TXCF6_W::new(self)
    }
    #[doc = "Bit 22 - HTXF6"]
    #[inline(always)]
    pub fn htxf6(&mut self) -> HTXF6_W {
        HTXF6_W::new(self)
    }
    #[doc = "Bit 23 - ERRF6"]
    #[inline(always)]
    pub fn errf6(&mut self) -> ERRF6_W {
        ERRF6_W::new(self)
    }
    #[doc = "Bit 24 - GLBF7"]
    #[inline(always)]
    pub fn glbf7(&mut self) -> GLBF7_W {
        GLBF7_W::new(self)
    }
    #[doc = "Bit 25 - TXCF7"]
    #[inline(always)]
    pub fn txcf7(&mut self) -> TXCF7_W {
        TXCF7_W::new(self)
    }
    #[doc = "Bit 26 - HTXF7"]
    #[inline(always)]
    pub fn htxf7(&mut self) -> HTXF7_W {
        HTXF7_W::new(self)
    }
    #[doc = "Bit 27 - ERRF7"]
    #[inline(always)]
    pub fn errf7(&mut self) -> ERRF7_W {
        ERRF7_W::new(self)
    }
    #[doc = "Bit 28 - GLBF8"]
    #[inline(always)]
    pub fn glbf8(&mut self) -> GLBF8_W {
        GLBF8_W::new(self)
    }
    #[doc = "Bit 29 - TXCF8"]
    #[inline(always)]
    pub fn txcf8(&mut self) -> TXCF8_W {
        TXCF8_W::new(self)
    }
    #[doc = "Bit 30 - HTXF8"]
    #[inline(always)]
    pub fn htxf8(&mut self) -> HTXF8_W {
        HTXF8_W::new(self)
    }
    #[doc = "Bit 31 - ERRF8"]
    #[inline(always)]
    pub fn errf8(&mut self) -> ERRF8_W {
        ERRF8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_INTSTS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_intsts](index.html) module"]
pub struct DMA_INTSTS_SPEC;
impl crate::RegisterSpec for DMA_INTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_intsts::R](R) reader structure"]
impl crate::Readable for DMA_INTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_intsts::W](W) writer structure"]
impl crate::Writable for DMA_INTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_INTSTS to value 0"]
impl crate::Resettable for DMA_INTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
