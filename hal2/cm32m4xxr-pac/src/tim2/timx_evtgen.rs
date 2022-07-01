#[doc = "Register `TIMx_EVTGEN` reader"]
pub struct R(crate::R<TIMX_EVTGEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_EVTGEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_EVTGEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_EVTGEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_EVTGEN` writer"]
pub struct W(crate::W<TIMX_EVTGEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_EVTGEN_SPEC>;
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
impl From<crate::W<TIMX_EVTGEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_EVTGEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UDGN` reader - UDGN"]
pub type UDGN_R = crate::BitReader<bool>;
#[doc = "Field `UDGN` writer - UDGN"]
pub type UDGN_W<'a> = crate::BitWriter<'a, u32, TIMX_EVTGEN_SPEC, bool, 0>;
#[doc = "Field `CC1GN` reader - CC1GN"]
pub type CC1GN_R = crate::BitReader<bool>;
#[doc = "Field `CC1GN` writer - CC1GN"]
pub type CC1GN_W<'a> = crate::BitWriter<'a, u32, TIMX_EVTGEN_SPEC, bool, 1>;
#[doc = "Field `CC2GN` reader - CC2GN"]
pub type CC2GN_R = crate::BitReader<bool>;
#[doc = "Field `CC2GN` writer - CC2GN"]
pub type CC2GN_W<'a> = crate::BitWriter<'a, u32, TIMX_EVTGEN_SPEC, bool, 2>;
#[doc = "Field `CC3GN` reader - CC3GN"]
pub type CC3GN_R = crate::BitReader<bool>;
#[doc = "Field `CC3GN` writer - CC3GN"]
pub type CC3GN_W<'a> = crate::BitWriter<'a, u32, TIMX_EVTGEN_SPEC, bool, 3>;
#[doc = "Field `CC4GN` reader - CC4GN"]
pub type CC4GN_R = crate::BitReader<bool>;
#[doc = "Field `CC4GN` writer - CC4GN"]
pub type CC4GN_W<'a> = crate::BitWriter<'a, u32, TIMX_EVTGEN_SPEC, bool, 4>;
#[doc = "Field `CCUDGN` reader - CCUDGN"]
pub type CCUDGN_R = crate::BitReader<bool>;
#[doc = "Field `CCUDGN` writer - CCUDGN"]
pub type CCUDGN_W<'a> = crate::BitWriter<'a, u32, TIMX_EVTGEN_SPEC, bool, 5>;
#[doc = "Field `TGN` reader - TGN"]
pub type TGN_R = crate::BitReader<bool>;
#[doc = "Field `TGN` writer - TGN"]
pub type TGN_W<'a> = crate::BitWriter<'a, u32, TIMX_EVTGEN_SPEC, bool, 6>;
#[doc = "Field `BGN` reader - BGN"]
pub type BGN_R = crate::BitReader<bool>;
#[doc = "Field `BGN` writer - BGN"]
pub type BGN_W<'a> = crate::BitWriter<'a, u32, TIMX_EVTGEN_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - UDGN"]
    #[inline(always)]
    pub fn udgn(&self) -> UDGN_R {
        UDGN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC1GN"]
    #[inline(always)]
    pub fn cc1gn(&self) -> CC1GN_R {
        CC1GN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC2GN"]
    #[inline(always)]
    pub fn cc2gn(&self) -> CC2GN_R {
        CC2GN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CC3GN"]
    #[inline(always)]
    pub fn cc3gn(&self) -> CC3GN_R {
        CC3GN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CC4GN"]
    #[inline(always)]
    pub fn cc4gn(&self) -> CC4GN_R {
        CC4GN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CCUDGN"]
    #[inline(always)]
    pub fn ccudgn(&self) -> CCUDGN_R {
        CCUDGN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TGN"]
    #[inline(always)]
    pub fn tgn(&self) -> TGN_R {
        TGN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BGN"]
    #[inline(always)]
    pub fn bgn(&self) -> BGN_R {
        BGN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UDGN"]
    #[inline(always)]
    pub fn udgn(&mut self) -> UDGN_W {
        UDGN_W::new(self)
    }
    #[doc = "Bit 1 - CC1GN"]
    #[inline(always)]
    pub fn cc1gn(&mut self) -> CC1GN_W {
        CC1GN_W::new(self)
    }
    #[doc = "Bit 2 - CC2GN"]
    #[inline(always)]
    pub fn cc2gn(&mut self) -> CC2GN_W {
        CC2GN_W::new(self)
    }
    #[doc = "Bit 3 - CC3GN"]
    #[inline(always)]
    pub fn cc3gn(&mut self) -> CC3GN_W {
        CC3GN_W::new(self)
    }
    #[doc = "Bit 4 - CC4GN"]
    #[inline(always)]
    pub fn cc4gn(&mut self) -> CC4GN_W {
        CC4GN_W::new(self)
    }
    #[doc = "Bit 5 - CCUDGN"]
    #[inline(always)]
    pub fn ccudgn(&mut self) -> CCUDGN_W {
        CCUDGN_W::new(self)
    }
    #[doc = "Bit 6 - TGN"]
    #[inline(always)]
    pub fn tgn(&mut self) -> TGN_W {
        TGN_W::new(self)
    }
    #[doc = "Bit 7 - BGN"]
    #[inline(always)]
    pub fn bgn(&mut self) -> BGN_W {
        BGN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMx_EVTGEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_evtgen](index.html) module"]
pub struct TIMX_EVTGEN_SPEC;
impl crate::RegisterSpec for TIMX_EVTGEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timx_evtgen::R](R) reader structure"]
impl crate::Readable for TIMX_EVTGEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_evtgen::W](W) writer structure"]
impl crate::Writable for TIMX_EVTGEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMx_EVTGEN to value 0"]
impl crate::Resettable for TIMX_EVTGEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
