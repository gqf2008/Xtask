#[doc = "Register `TIMx_CCMOD1` reader"]
pub struct R(crate::R<TIMX_CCMOD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_CCMOD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_CCMOD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_CCMOD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_CCMOD1` writer"]
pub struct W(crate::W<TIMX_CCMOD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_CCMOD1_SPEC>;
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
impl From<crate::W<TIMX_CCMOD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_CCMOD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC1SEL` reader - CC1SEL"]
pub type CC1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CC1SEL` writer - CC1SEL"]
pub type CC1SEL_W<'a> = crate::FieldWriter<'a, u32, TIMX_CCMOD1_SPEC, u8, u8, 2, 0>;
#[doc = "Field `OC1FEN` reader - OC1FEN"]
pub type OC1FEN_R = crate::BitReader<bool>;
#[doc = "Field `OC1FEN` writer - OC1FEN"]
pub type OC1FEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCMOD1_SPEC, bool, 2>;
#[doc = "Field `OC1PEN` reader - OC1PEN"]
pub type OC1PEN_R = crate::BitReader<bool>;
#[doc = "Field `OC1PEN` writer - OC1PEN"]
pub type OC1PEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCMOD1_SPEC, bool, 3>;
#[doc = "Field `OC1M` reader - OC1M"]
pub type OC1M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OC1M` writer - OC1M"]
pub type OC1M_W<'a> = crate::FieldWriter<'a, u32, TIMX_CCMOD1_SPEC, u8, u8, 3, 4>;
#[doc = "Field `OC1CEN` reader - OC1CEN"]
pub type OC1CEN_R = crate::BitReader<bool>;
#[doc = "Field `OC1CEN` writer - OC1CEN"]
pub type OC1CEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCMOD1_SPEC, bool, 7>;
#[doc = "Field `CC2SEL` reader - CC2SEL"]
pub type CC2SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CC2SEL` writer - CC2SEL"]
pub type CC2SEL_W<'a> = crate::FieldWriter<'a, u32, TIMX_CCMOD1_SPEC, u8, u8, 2, 8>;
#[doc = "Field `OC2FEN` reader - OC2FEN"]
pub type OC2FEN_R = crate::BitReader<bool>;
#[doc = "Field `OC2FEN` writer - OC2FEN"]
pub type OC2FEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCMOD1_SPEC, bool, 10>;
#[doc = "Field `OC2PEN` reader - OC2PEN"]
pub type OC2PEN_R = crate::BitReader<bool>;
#[doc = "Field `OC2PEN` writer - OC2PEN"]
pub type OC2PEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCMOD1_SPEC, bool, 11>;
#[doc = "Field `OC2M` reader - OC2M"]
pub type OC2M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OC2M` writer - OC2M"]
pub type OC2M_W<'a> = crate::FieldWriter<'a, u32, TIMX_CCMOD1_SPEC, u8, u8, 3, 12>;
#[doc = "Field `OC2CEN` reader - OC2CEN"]
pub type OC2CEN_R = crate::BitReader<bool>;
#[doc = "Field `OC2CEN` writer - OC2CEN"]
pub type OC2CEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCMOD1_SPEC, bool, 15>;
impl R {
    #[doc = "Bits 0:1 - CC1SEL"]
    #[inline(always)]
    pub fn cc1sel(&self) -> CC1SEL_R {
        CC1SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - OC1FEN"]
    #[inline(always)]
    pub fn oc1fen(&self) -> OC1FEN_R {
        OC1FEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OC1PEN"]
    #[inline(always)]
    pub fn oc1pen(&self) -> OC1PEN_R {
        OC1PEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - OC1M"]
    #[inline(always)]
    pub fn oc1m(&self) -> OC1M_R {
        OC1M_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - OC1CEN"]
    #[inline(always)]
    pub fn oc1cen(&self) -> OC1CEN_R {
        OC1CEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - CC2SEL"]
    #[inline(always)]
    pub fn cc2sel(&self) -> CC2SEL_R {
        CC2SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - OC2FEN"]
    #[inline(always)]
    pub fn oc2fen(&self) -> OC2FEN_R {
        OC2FEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OC2PEN"]
    #[inline(always)]
    pub fn oc2pen(&self) -> OC2PEN_R {
        OC2PEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - OC2M"]
    #[inline(always)]
    pub fn oc2m(&self) -> OC2M_R {
        OC2M_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - OC2CEN"]
    #[inline(always)]
    pub fn oc2cen(&self) -> OC2CEN_R {
        OC2CEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC1SEL"]
    #[inline(always)]
    pub fn cc1sel(&mut self) -> CC1SEL_W {
        CC1SEL_W::new(self)
    }
    #[doc = "Bit 2 - OC1FEN"]
    #[inline(always)]
    pub fn oc1fen(&mut self) -> OC1FEN_W {
        OC1FEN_W::new(self)
    }
    #[doc = "Bit 3 - OC1PEN"]
    #[inline(always)]
    pub fn oc1pen(&mut self) -> OC1PEN_W {
        OC1PEN_W::new(self)
    }
    #[doc = "Bits 4:6 - OC1M"]
    #[inline(always)]
    pub fn oc1m(&mut self) -> OC1M_W {
        OC1M_W::new(self)
    }
    #[doc = "Bit 7 - OC1CEN"]
    #[inline(always)]
    pub fn oc1cen(&mut self) -> OC1CEN_W {
        OC1CEN_W::new(self)
    }
    #[doc = "Bits 8:9 - CC2SEL"]
    #[inline(always)]
    pub fn cc2sel(&mut self) -> CC2SEL_W {
        CC2SEL_W::new(self)
    }
    #[doc = "Bit 10 - OC2FEN"]
    #[inline(always)]
    pub fn oc2fen(&mut self) -> OC2FEN_W {
        OC2FEN_W::new(self)
    }
    #[doc = "Bit 11 - OC2PEN"]
    #[inline(always)]
    pub fn oc2pen(&mut self) -> OC2PEN_W {
        OC2PEN_W::new(self)
    }
    #[doc = "Bits 12:14 - OC2M"]
    #[inline(always)]
    pub fn oc2m(&mut self) -> OC2M_W {
        OC2M_W::new(self)
    }
    #[doc = "Bit 15 - OC2CEN"]
    #[inline(always)]
    pub fn oc2cen(&mut self) -> OC2CEN_W {
        OC2CEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMx_CCMOD1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccmod1](index.html) module"]
pub struct TIMX_CCMOD1_SPEC;
impl crate::RegisterSpec for TIMX_CCMOD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timx_ccmod1::R](R) reader structure"]
impl crate::Readable for TIMX_CCMOD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_ccmod1::W](W) writer structure"]
impl crate::Writable for TIMX_CCMOD1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMx_CCMOD1 to value 0"]
impl crate::Resettable for TIMX_CCMOD1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
