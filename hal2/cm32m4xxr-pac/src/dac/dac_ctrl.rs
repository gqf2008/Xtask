#[doc = "Register `DAC_CTRL` reader"]
pub struct R(crate::R<DAC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_CTRL` writer"]
pub struct W(crate::W<DAC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_CTRL_SPEC>;
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
impl From<crate::W<DAC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH1EN` reader - CH1EN"]
pub type CH1EN_R = crate::BitReader<bool>;
#[doc = "Field `CH1EN` writer - CH1EN"]
pub type CH1EN_W<'a> = crate::BitWriter<'a, u32, DAC_CTRL_SPEC, bool, 0>;
#[doc = "Field `B1EN` reader - B1EN"]
pub type B1EN_R = crate::BitReader<bool>;
#[doc = "Field `B1EN` writer - B1EN"]
pub type B1EN_W<'a> = crate::BitWriter<'a, u32, DAC_CTRL_SPEC, bool, 1>;
#[doc = "Field `T1EN` reader - T1EN"]
pub type T1EN_R = crate::BitReader<bool>;
#[doc = "Field `T1EN` writer - T1EN"]
pub type T1EN_W<'a> = crate::BitWriter<'a, u32, DAC_CTRL_SPEC, bool, 2>;
#[doc = "Field `T1SEL` reader - T1SEL"]
pub type T1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T1SEL` writer - T1SEL"]
pub type T1SEL_W<'a> = crate::FieldWriter<'a, u32, DAC_CTRL_SPEC, u8, u8, 3, 3>;
#[doc = "Field `W1EN` reader - W1EN"]
pub type W1EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `W1EN` writer - W1EN"]
pub type W1EN_W<'a> = crate::FieldWriter<'a, u32, DAC_CTRL_SPEC, u8, u8, 2, 6>;
#[doc = "Field `MA1SEL` reader - MA1SEL"]
pub type MA1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MA1SEL` writer - MA1SEL"]
pub type MA1SEL_W<'a> = crate::FieldWriter<'a, u32, DAC_CTRL_SPEC, u8, u8, 4, 8>;
#[doc = "Field `DMA1EN` reader - DMA1EN"]
pub type DMA1EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA1EN` writer - DMA1EN"]
pub type DMA1EN_W<'a> = crate::BitWriter<'a, u32, DAC_CTRL_SPEC, bool, 12>;
#[doc = "Field `__CH2EN` reader - __CH2EN"]
pub type __CH2EN_R = crate::BitReader<bool>;
#[doc = "Field `__CH2EN` writer - __CH2EN"]
pub type __CH2EN_W<'a> = crate::BitWriter<'a, u32, DAC_CTRL_SPEC, bool, 16>;
#[doc = "Field `__B2EN` reader - __B2EN"]
pub type __B2EN_R = crate::BitReader<bool>;
#[doc = "Field `__B2EN` writer - __B2EN"]
pub type __B2EN_W<'a> = crate::BitWriter<'a, u32, DAC_CTRL_SPEC, bool, 17>;
#[doc = "Field `__T2EN` reader - __T2EN"]
pub type __T2EN_R = crate::BitReader<bool>;
#[doc = "Field `__T2EN` writer - __T2EN"]
pub type __T2EN_W<'a> = crate::BitWriter<'a, u32, DAC_CTRL_SPEC, bool, 18>;
#[doc = "Field `T2SEL` reader - T2SEL"]
pub type T2SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T2SEL` writer - T2SEL"]
pub type T2SEL_W<'a> = crate::FieldWriter<'a, u32, DAC_CTRL_SPEC, u8, u8, 3, 19>;
#[doc = "Field `W2EN` reader - W2EN"]
pub type W2EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `W2EN` writer - W2EN"]
pub type W2EN_W<'a> = crate::FieldWriter<'a, u32, DAC_CTRL_SPEC, u8, u8, 2, 22>;
#[doc = "Field `MA2SEL` reader - MA2SEL"]
pub type MA2SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MA2SEL` writer - MA2SEL"]
pub type MA2SEL_W<'a> = crate::FieldWriter<'a, u32, DAC_CTRL_SPEC, u8, u8, 4, 24>;
#[doc = "Field `DMA2EN` reader - DMA2EN"]
pub type DMA2EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA2EN` writer - DMA2EN"]
pub type DMA2EN_W<'a> = crate::BitWriter<'a, u32, DAC_CTRL_SPEC, bool, 28>;
impl R {
    #[doc = "Bit 0 - CH1EN"]
    #[inline(always)]
    pub fn ch1en(&self) -> CH1EN_R {
        CH1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - B1EN"]
    #[inline(always)]
    pub fn b1en(&self) -> B1EN_R {
        B1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - T1EN"]
    #[inline(always)]
    pub fn t1en(&self) -> T1EN_R {
        T1EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - T1SEL"]
    #[inline(always)]
    pub fn t1sel(&self) -> T1SEL_R {
        T1SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - W1EN"]
    #[inline(always)]
    pub fn w1en(&self) -> W1EN_R {
        W1EN_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - MA1SEL"]
    #[inline(always)]
    pub fn ma1sel(&self) -> MA1SEL_R {
        MA1SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DMA1EN"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - __CH2EN"]
    #[inline(always)]
    pub fn __ch2en(&self) -> __CH2EN_R {
        __CH2EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - __B2EN"]
    #[inline(always)]
    pub fn __b2en(&self) -> __B2EN_R {
        __B2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - __T2EN"]
    #[inline(always)]
    pub fn __t2en(&self) -> __T2EN_R {
        __T2EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - T2SEL"]
    #[inline(always)]
    pub fn t2sel(&self) -> T2SEL_R {
        T2SEL_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - W2EN"]
    #[inline(always)]
    pub fn w2en(&self) -> W2EN_R {
        W2EN_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - MA2SEL"]
    #[inline(always)]
    pub fn ma2sel(&self) -> MA2SEL_R {
        MA2SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - DMA2EN"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH1EN"]
    #[inline(always)]
    pub fn ch1en(&mut self) -> CH1EN_W {
        CH1EN_W::new(self)
    }
    #[doc = "Bit 1 - B1EN"]
    #[inline(always)]
    pub fn b1en(&mut self) -> B1EN_W {
        B1EN_W::new(self)
    }
    #[doc = "Bit 2 - T1EN"]
    #[inline(always)]
    pub fn t1en(&mut self) -> T1EN_W {
        T1EN_W::new(self)
    }
    #[doc = "Bits 3:5 - T1SEL"]
    #[inline(always)]
    pub fn t1sel(&mut self) -> T1SEL_W {
        T1SEL_W::new(self)
    }
    #[doc = "Bits 6:7 - W1EN"]
    #[inline(always)]
    pub fn w1en(&mut self) -> W1EN_W {
        W1EN_W::new(self)
    }
    #[doc = "Bits 8:11 - MA1SEL"]
    #[inline(always)]
    pub fn ma1sel(&mut self) -> MA1SEL_W {
        MA1SEL_W::new(self)
    }
    #[doc = "Bit 12 - DMA1EN"]
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W {
        DMA1EN_W::new(self)
    }
    #[doc = "Bit 16 - __CH2EN"]
    #[inline(always)]
    pub fn __ch2en(&mut self) -> __CH2EN_W {
        __CH2EN_W::new(self)
    }
    #[doc = "Bit 17 - __B2EN"]
    #[inline(always)]
    pub fn __b2en(&mut self) -> __B2EN_W {
        __B2EN_W::new(self)
    }
    #[doc = "Bit 18 - __T2EN"]
    #[inline(always)]
    pub fn __t2en(&mut self) -> __T2EN_W {
        __T2EN_W::new(self)
    }
    #[doc = "Bits 19:21 - T2SEL"]
    #[inline(always)]
    pub fn t2sel(&mut self) -> T2SEL_W {
        T2SEL_W::new(self)
    }
    #[doc = "Bits 22:23 - W2EN"]
    #[inline(always)]
    pub fn w2en(&mut self) -> W2EN_W {
        W2EN_W::new(self)
    }
    #[doc = "Bits 24:27 - MA2SEL"]
    #[inline(always)]
    pub fn ma2sel(&mut self) -> MA2SEL_W {
        MA2SEL_W::new(self)
    }
    #[doc = "Bit 28 - DMA2EN"]
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W {
        DMA2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_ctrl](index.html) module"]
pub struct DAC_CTRL_SPEC;
impl crate::RegisterSpec for DAC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_ctrl::R](R) reader structure"]
impl crate::Readable for DAC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_ctrl::W](W) writer structure"]
impl crate::Writable for DAC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC_CTRL to value 0"]
impl crate::Resettable for DAC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
