#[doc = "Register `TIMx_CCMOD2` reader"]
pub struct R(crate::R<TIMX_CCMOD2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_CCMOD2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_CCMOD2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_CCMOD2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_CCMOD2` writer"]
pub struct W(crate::W<TIMX_CCMOD2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_CCMOD2_SPEC>;
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
impl From<crate::W<TIMX_CCMOD2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_CCMOD2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC3SEL` reader - CC3SEL"]
pub type CC3SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CC3SEL` writer - CC3SEL"]
pub type CC3SEL_W<'a> = crate::FieldWriter<'a, u32, TIMX_CCMOD2_SPEC, u8, u8, 2, 0>;
#[doc = "Field `OC3FEN` reader - OC3FEN"]
pub type OC3FEN_R = crate::BitReader<bool>;
#[doc = "Field `OC3FEN` writer - OC3FEN"]
pub type OC3FEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCMOD2_SPEC, bool, 2>;
#[doc = "Field `OC3PEN` reader - OC3PEN"]
pub type OC3PEN_R = crate::BitReader<bool>;
#[doc = "Field `OC3PEN` writer - OC3PEN"]
pub type OC3PEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCMOD2_SPEC, bool, 3>;
#[doc = "Field `OC3M` reader - OC3M"]
pub type OC3M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OC3M` writer - OC3M"]
pub type OC3M_W<'a> = crate::FieldWriter<'a, u32, TIMX_CCMOD2_SPEC, u8, u8, 3, 4>;
#[doc = "Field `OC3CEN` reader - OC3CEN"]
pub type OC3CEN_R = crate::BitReader<bool>;
#[doc = "Field `OC3CEN` writer - OC3CEN"]
pub type OC3CEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCMOD2_SPEC, bool, 7>;
#[doc = "Field `CC4SEL` reader - CC4SEL"]
pub type CC4SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CC4SEL` writer - CC4SEL"]
pub type CC4SEL_W<'a> = crate::FieldWriter<'a, u32, TIMX_CCMOD2_SPEC, u8, u8, 2, 8>;
#[doc = "Field `OC4FEN` reader - OC4FEN"]
pub type OC4FEN_R = crate::BitReader<bool>;
#[doc = "Field `OC4FEN` writer - OC4FEN"]
pub type OC4FEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCMOD2_SPEC, bool, 10>;
#[doc = "Field `OC4PEN` reader - OC4PEN"]
pub type OC4PEN_R = crate::BitReader<bool>;
#[doc = "Field `OC4PEN` writer - OC4PEN"]
pub type OC4PEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCMOD2_SPEC, bool, 11>;
#[doc = "Field `OC4M` reader - OC4M"]
pub type OC4M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OC4M` writer - OC4M"]
pub type OC4M_W<'a> = crate::FieldWriter<'a, u32, TIMX_CCMOD2_SPEC, u8, u8, 3, 12>;
#[doc = "Field `OC4CEN` reader - OC4CEN"]
pub type OC4CEN_R = crate::BitReader<bool>;
#[doc = "Field `OC4CEN` writer - OC4CEN"]
pub type OC4CEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCMOD2_SPEC, bool, 15>;
impl R {
    #[doc = "Bits 0:1 - CC3SEL"]
    #[inline(always)]
    pub fn cc3sel(&self) -> CC3SEL_R {
        CC3SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - OC3FEN"]
    #[inline(always)]
    pub fn oc3fen(&self) -> OC3FEN_R {
        OC3FEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OC3PEN"]
    #[inline(always)]
    pub fn oc3pen(&self) -> OC3PEN_R {
        OC3PEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - OC3M"]
    #[inline(always)]
    pub fn oc3m(&self) -> OC3M_R {
        OC3M_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - OC3CEN"]
    #[inline(always)]
    pub fn oc3cen(&self) -> OC3CEN_R {
        OC3CEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - CC4SEL"]
    #[inline(always)]
    pub fn cc4sel(&self) -> CC4SEL_R {
        CC4SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - OC4FEN"]
    #[inline(always)]
    pub fn oc4fen(&self) -> OC4FEN_R {
        OC4FEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OC4PEN"]
    #[inline(always)]
    pub fn oc4pen(&self) -> OC4PEN_R {
        OC4PEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - OC4M"]
    #[inline(always)]
    pub fn oc4m(&self) -> OC4M_R {
        OC4M_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - OC4CEN"]
    #[inline(always)]
    pub fn oc4cen(&self) -> OC4CEN_R {
        OC4CEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC3SEL"]
    #[inline(always)]
    pub fn cc3sel(&mut self) -> CC3SEL_W {
        CC3SEL_W::new(self)
    }
    #[doc = "Bit 2 - OC3FEN"]
    #[inline(always)]
    pub fn oc3fen(&mut self) -> OC3FEN_W {
        OC3FEN_W::new(self)
    }
    #[doc = "Bit 3 - OC3PEN"]
    #[inline(always)]
    pub fn oc3pen(&mut self) -> OC3PEN_W {
        OC3PEN_W::new(self)
    }
    #[doc = "Bits 4:6 - OC3M"]
    #[inline(always)]
    pub fn oc3m(&mut self) -> OC3M_W {
        OC3M_W::new(self)
    }
    #[doc = "Bit 7 - OC3CEN"]
    #[inline(always)]
    pub fn oc3cen(&mut self) -> OC3CEN_W {
        OC3CEN_W::new(self)
    }
    #[doc = "Bits 8:9 - CC4SEL"]
    #[inline(always)]
    pub fn cc4sel(&mut self) -> CC4SEL_W {
        CC4SEL_W::new(self)
    }
    #[doc = "Bit 10 - OC4FEN"]
    #[inline(always)]
    pub fn oc4fen(&mut self) -> OC4FEN_W {
        OC4FEN_W::new(self)
    }
    #[doc = "Bit 11 - OC4PEN"]
    #[inline(always)]
    pub fn oc4pen(&mut self) -> OC4PEN_W {
        OC4PEN_W::new(self)
    }
    #[doc = "Bits 12:14 - OC4M"]
    #[inline(always)]
    pub fn oc4m(&mut self) -> OC4M_W {
        OC4M_W::new(self)
    }
    #[doc = "Bit 15 - OC4CEN"]
    #[inline(always)]
    pub fn oc4cen(&mut self) -> OC4CEN_W {
        OC4CEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMx_CCMOD2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccmod2](index.html) module"]
pub struct TIMX_CCMOD2_SPEC;
impl crate::RegisterSpec for TIMX_CCMOD2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timx_ccmod2::R](R) reader structure"]
impl crate::Readable for TIMX_CCMOD2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_ccmod2::W](W) writer structure"]
impl crate::Writable for TIMX_CCMOD2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMx_CCMOD2 to value 0"]
impl crate::Resettable for TIMX_CCMOD2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
