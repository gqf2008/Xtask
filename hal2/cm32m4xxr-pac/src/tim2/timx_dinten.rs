#[doc = "Register `TIMx_DINTEN` reader"]
pub struct R(crate::R<TIMX_DINTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_DINTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_DINTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_DINTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_DINTEN` writer"]
pub struct W(crate::W<TIMX_DINTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_DINTEN_SPEC>;
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
impl From<crate::W<TIMX_DINTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_DINTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UIEN` reader - UIEN"]
pub type UIEN_R = crate::BitReader<bool>;
#[doc = "Field `UIEN` writer - UIEN"]
pub type UIEN_W<'a> = crate::BitWriter<'a, u32, TIMX_DINTEN_SPEC, bool, 0>;
#[doc = "Field `CC1IEN` reader - CC1IEN"]
pub type CC1IEN_R = crate::BitReader<bool>;
#[doc = "Field `CC1IEN` writer - CC1IEN"]
pub type CC1IEN_W<'a> = crate::BitWriter<'a, u32, TIMX_DINTEN_SPEC, bool, 1>;
#[doc = "Field `CC2IEN` reader - CC2IEN"]
pub type CC2IEN_R = crate::BitReader<bool>;
#[doc = "Field `CC2IEN` writer - CC2IEN"]
pub type CC2IEN_W<'a> = crate::BitWriter<'a, u32, TIMX_DINTEN_SPEC, bool, 2>;
#[doc = "Field `CC3IEN` reader - CC3IEN"]
pub type CC3IEN_R = crate::BitReader<bool>;
#[doc = "Field `CC3IEN` writer - CC3IEN"]
pub type CC3IEN_W<'a> = crate::BitWriter<'a, u32, TIMX_DINTEN_SPEC, bool, 3>;
#[doc = "Field `CC4IEN` reader - CC4IEN"]
pub type CC4IEN_R = crate::BitReader<bool>;
#[doc = "Field `CC4IEN` writer - CC4IEN"]
pub type CC4IEN_W<'a> = crate::BitWriter<'a, u32, TIMX_DINTEN_SPEC, bool, 4>;
#[doc = "Field `COMIEN` reader - COMIEN"]
pub type COMIEN_R = crate::BitReader<bool>;
#[doc = "Field `COMIEN` writer - COMIEN"]
pub type COMIEN_W<'a> = crate::BitWriter<'a, u32, TIMX_DINTEN_SPEC, bool, 5>;
#[doc = "Field `TIEN` reader - TIEN"]
pub type TIEN_R = crate::BitReader<bool>;
#[doc = "Field `TIEN` writer - TIEN"]
pub type TIEN_W<'a> = crate::BitWriter<'a, u32, TIMX_DINTEN_SPEC, bool, 6>;
#[doc = "Field `BIEN` reader - BIEN"]
pub type BIEN_R = crate::BitReader<bool>;
#[doc = "Field `BIEN` writer - BIEN"]
pub type BIEN_W<'a> = crate::BitWriter<'a, u32, TIMX_DINTEN_SPEC, bool, 7>;
#[doc = "Field `UDEN` reader - UDEN"]
pub type UDEN_R = crate::BitReader<bool>;
#[doc = "Field `UDEN` writer - UDEN"]
pub type UDEN_W<'a> = crate::BitWriter<'a, u32, TIMX_DINTEN_SPEC, bool, 8>;
#[doc = "Field `CC1DEN` reader - CC1DEN"]
pub type CC1DEN_R = crate::BitReader<bool>;
#[doc = "Field `CC1DEN` writer - CC1DEN"]
pub type CC1DEN_W<'a> = crate::BitWriter<'a, u32, TIMX_DINTEN_SPEC, bool, 9>;
#[doc = "Field `CC2DEN` reader - CC2DEN"]
pub type CC2DEN_R = crate::BitReader<bool>;
#[doc = "Field `CC2DEN` writer - CC2DEN"]
pub type CC2DEN_W<'a> = crate::BitWriter<'a, u32, TIMX_DINTEN_SPEC, bool, 10>;
#[doc = "Field `CC3DEN` reader - CC3DEN"]
pub type CC3DEN_R = crate::BitReader<bool>;
#[doc = "Field `CC3DEN` writer - CC3DEN"]
pub type CC3DEN_W<'a> = crate::BitWriter<'a, u32, TIMX_DINTEN_SPEC, bool, 11>;
#[doc = "Field `CC4DEN` reader - CC4DEN"]
pub type CC4DEN_R = crate::BitReader<bool>;
#[doc = "Field `CC4DEN` writer - CC4DEN"]
pub type CC4DEN_W<'a> = crate::BitWriter<'a, u32, TIMX_DINTEN_SPEC, bool, 12>;
#[doc = "Field `COMDEN` reader - COMDEN"]
pub type COMDEN_R = crate::BitReader<bool>;
#[doc = "Field `COMDEN` writer - COMDEN"]
pub type COMDEN_W<'a> = crate::BitWriter<'a, u32, TIMX_DINTEN_SPEC, bool, 13>;
#[doc = "Field `TDEN` reader - TDEN"]
pub type TDEN_R = crate::BitReader<bool>;
#[doc = "Field `TDEN` writer - TDEN"]
pub type TDEN_W<'a> = crate::BitWriter<'a, u32, TIMX_DINTEN_SPEC, bool, 14>;
impl R {
    #[doc = "Bit 0 - UIEN"]
    #[inline(always)]
    pub fn uien(&self) -> UIEN_R {
        UIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC1IEN"]
    #[inline(always)]
    pub fn cc1ien(&self) -> CC1IEN_R {
        CC1IEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC2IEN"]
    #[inline(always)]
    pub fn cc2ien(&self) -> CC2IEN_R {
        CC2IEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CC3IEN"]
    #[inline(always)]
    pub fn cc3ien(&self) -> CC3IEN_R {
        CC3IEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CC4IEN"]
    #[inline(always)]
    pub fn cc4ien(&self) -> CC4IEN_R {
        CC4IEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COMIEN"]
    #[inline(always)]
    pub fn comien(&self) -> COMIEN_R {
        COMIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIEN"]
    #[inline(always)]
    pub fn tien(&self) -> TIEN_R {
        TIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BIEN"]
    #[inline(always)]
    pub fn bien(&self) -> BIEN_R {
        BIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UDEN"]
    #[inline(always)]
    pub fn uden(&self) -> UDEN_R {
        UDEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CC1DEN"]
    #[inline(always)]
    pub fn cc1den(&self) -> CC1DEN_R {
        CC1DEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CC2DEN"]
    #[inline(always)]
    pub fn cc2den(&self) -> CC2DEN_R {
        CC2DEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CC3DEN"]
    #[inline(always)]
    pub fn cc3den(&self) -> CC3DEN_R {
        CC3DEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CC4DEN"]
    #[inline(always)]
    pub fn cc4den(&self) -> CC4DEN_R {
        CC4DEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - COMDEN"]
    #[inline(always)]
    pub fn comden(&self) -> COMDEN_R {
        COMDEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TDEN"]
    #[inline(always)]
    pub fn tden(&self) -> TDEN_R {
        TDEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UIEN"]
    #[inline(always)]
    pub fn uien(&mut self) -> UIEN_W {
        UIEN_W::new(self)
    }
    #[doc = "Bit 1 - CC1IEN"]
    #[inline(always)]
    pub fn cc1ien(&mut self) -> CC1IEN_W {
        CC1IEN_W::new(self)
    }
    #[doc = "Bit 2 - CC2IEN"]
    #[inline(always)]
    pub fn cc2ien(&mut self) -> CC2IEN_W {
        CC2IEN_W::new(self)
    }
    #[doc = "Bit 3 - CC3IEN"]
    #[inline(always)]
    pub fn cc3ien(&mut self) -> CC3IEN_W {
        CC3IEN_W::new(self)
    }
    #[doc = "Bit 4 - CC4IEN"]
    #[inline(always)]
    pub fn cc4ien(&mut self) -> CC4IEN_W {
        CC4IEN_W::new(self)
    }
    #[doc = "Bit 5 - COMIEN"]
    #[inline(always)]
    pub fn comien(&mut self) -> COMIEN_W {
        COMIEN_W::new(self)
    }
    #[doc = "Bit 6 - TIEN"]
    #[inline(always)]
    pub fn tien(&mut self) -> TIEN_W {
        TIEN_W::new(self)
    }
    #[doc = "Bit 7 - BIEN"]
    #[inline(always)]
    pub fn bien(&mut self) -> BIEN_W {
        BIEN_W::new(self)
    }
    #[doc = "Bit 8 - UDEN"]
    #[inline(always)]
    pub fn uden(&mut self) -> UDEN_W {
        UDEN_W::new(self)
    }
    #[doc = "Bit 9 - CC1DEN"]
    #[inline(always)]
    pub fn cc1den(&mut self) -> CC1DEN_W {
        CC1DEN_W::new(self)
    }
    #[doc = "Bit 10 - CC2DEN"]
    #[inline(always)]
    pub fn cc2den(&mut self) -> CC2DEN_W {
        CC2DEN_W::new(self)
    }
    #[doc = "Bit 11 - CC3DEN"]
    #[inline(always)]
    pub fn cc3den(&mut self) -> CC3DEN_W {
        CC3DEN_W::new(self)
    }
    #[doc = "Bit 12 - CC4DEN"]
    #[inline(always)]
    pub fn cc4den(&mut self) -> CC4DEN_W {
        CC4DEN_W::new(self)
    }
    #[doc = "Bit 13 - COMDEN"]
    #[inline(always)]
    pub fn comden(&mut self) -> COMDEN_W {
        COMDEN_W::new(self)
    }
    #[doc = "Bit 14 - TDEN"]
    #[inline(always)]
    pub fn tden(&mut self) -> TDEN_W {
        TDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMx_DINTEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_dinten](index.html) module"]
pub struct TIMX_DINTEN_SPEC;
impl crate::RegisterSpec for TIMX_DINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timx_dinten::R](R) reader structure"]
impl crate::Readable for TIMX_DINTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_dinten::W](W) writer structure"]
impl crate::Writable for TIMX_DINTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMx_DINTEN to value 0"]
impl crate::Resettable for TIMX_DINTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
