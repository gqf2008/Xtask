#[doc = "Register `TIMx_CCEN` reader"]
pub struct R(crate::R<TIMX_CCEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_CCEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_CCEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_CCEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_CCEN` writer"]
pub struct W(crate::W<TIMX_CCEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_CCEN_SPEC>;
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
impl From<crate::W<TIMX_CCEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_CCEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC1EN` reader - CC1EN"]
pub type CC1EN_R = crate::BitReader<bool>;
#[doc = "Field `CC1EN` writer - CC1EN"]
pub type CC1EN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCEN_SPEC, bool, 0>;
#[doc = "Field `CC1P` reader - CC1P"]
pub type CC1P_R = crate::BitReader<bool>;
#[doc = "Field `CC1P` writer - CC1P"]
pub type CC1P_W<'a> = crate::BitWriter<'a, u32, TIMX_CCEN_SPEC, bool, 1>;
#[doc = "Field `CC1NEN` reader - CC1NEN"]
pub type CC1NEN_R = crate::BitReader<bool>;
#[doc = "Field `CC1NEN` writer - CC1NEN"]
pub type CC1NEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCEN_SPEC, bool, 2>;
#[doc = "Field `CC1NP` reader - CC1NP"]
pub type CC1NP_R = crate::BitReader<bool>;
#[doc = "Field `CC1NP` writer - CC1NP"]
pub type CC1NP_W<'a> = crate::BitWriter<'a, u32, TIMX_CCEN_SPEC, bool, 3>;
#[doc = "Field `CC2EN` reader - CC2EN"]
pub type CC2EN_R = crate::BitReader<bool>;
#[doc = "Field `CC2EN` writer - CC2EN"]
pub type CC2EN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCEN_SPEC, bool, 4>;
#[doc = "Field `CC2P` reader - CC2P"]
pub type CC2P_R = crate::BitReader<bool>;
#[doc = "Field `CC2P` writer - CC2P"]
pub type CC2P_W<'a> = crate::BitWriter<'a, u32, TIMX_CCEN_SPEC, bool, 5>;
#[doc = "Field `CC2NEN` reader - CC2NEN"]
pub type CC2NEN_R = crate::BitReader<bool>;
#[doc = "Field `CC2NEN` writer - CC2NEN"]
pub type CC2NEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCEN_SPEC, bool, 6>;
#[doc = "Field `CC2NP` reader - CC2NP"]
pub type CC2NP_R = crate::BitReader<bool>;
#[doc = "Field `CC2NP` writer - CC2NP"]
pub type CC2NP_W<'a> = crate::BitWriter<'a, u32, TIMX_CCEN_SPEC, bool, 7>;
#[doc = "Field `CC3EN` reader - CC3EN"]
pub type CC3EN_R = crate::BitReader<bool>;
#[doc = "Field `CC3EN` writer - CC3EN"]
pub type CC3EN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCEN_SPEC, bool, 8>;
#[doc = "Field `CC3P` reader - CC3P"]
pub type CC3P_R = crate::BitReader<bool>;
#[doc = "Field `CC3P` writer - CC3P"]
pub type CC3P_W<'a> = crate::BitWriter<'a, u32, TIMX_CCEN_SPEC, bool, 9>;
#[doc = "Field `CC3NEN` reader - CC3NEN"]
pub type CC3NEN_R = crate::BitReader<bool>;
#[doc = "Field `CC3NEN` writer - CC3NEN"]
pub type CC3NEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCEN_SPEC, bool, 10>;
#[doc = "Field `CC3NP` reader - CC3NP"]
pub type CC3NP_R = crate::BitReader<bool>;
#[doc = "Field `CC3NP` writer - CC3NP"]
pub type CC3NP_W<'a> = crate::BitWriter<'a, u32, TIMX_CCEN_SPEC, bool, 11>;
#[doc = "Field `CC4EN` reader - CC4EN"]
pub type CC4EN_R = crate::BitReader<bool>;
#[doc = "Field `CC4EN` writer - CC4EN"]
pub type CC4EN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCEN_SPEC, bool, 12>;
#[doc = "Field `CC4P` reader - CC4P"]
pub type CC4P_R = crate::BitReader<bool>;
#[doc = "Field `CC4P` writer - CC4P"]
pub type CC4P_W<'a> = crate::BitWriter<'a, u32, TIMX_CCEN_SPEC, bool, 13>;
#[doc = "Field `CC5EN` reader - CC5EN"]
pub type CC5EN_R = crate::BitReader<bool>;
#[doc = "Field `CC5EN` writer - CC5EN"]
pub type CC5EN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCEN_SPEC, bool, 16>;
#[doc = "Field `CC5P` reader - CC5P"]
pub type CC5P_R = crate::BitReader<bool>;
#[doc = "Field `CC5P` writer - CC5P"]
pub type CC5P_W<'a> = crate::BitWriter<'a, u32, TIMX_CCEN_SPEC, bool, 17>;
#[doc = "Field `CC6EN` reader - CC6EN"]
pub type CC6EN_R = crate::BitReader<bool>;
#[doc = "Field `CC6EN` writer - CC6EN"]
pub type CC6EN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCEN_SPEC, bool, 20>;
#[doc = "Field `CC6P` reader - CC6P"]
pub type CC6P_R = crate::BitReader<bool>;
#[doc = "Field `CC6P` writer - CC6P"]
pub type CC6P_W<'a> = crate::BitWriter<'a, u32, TIMX_CCEN_SPEC, bool, 21>;
impl R {
    #[doc = "Bit 0 - CC1EN"]
    #[inline(always)]
    pub fn cc1en(&self) -> CC1EN_R {
        CC1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC1P"]
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC1NEN"]
    #[inline(always)]
    pub fn cc1nen(&self) -> CC1NEN_R {
        CC1NEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CC1NP"]
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CC2EN"]
    #[inline(always)]
    pub fn cc2en(&self) -> CC2EN_R {
        CC2EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CC2P"]
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CC2NEN"]
    #[inline(always)]
    pub fn cc2nen(&self) -> CC2NEN_R {
        CC2NEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CC2NP"]
    #[inline(always)]
    pub fn cc2np(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CC3EN"]
    #[inline(always)]
    pub fn cc3en(&self) -> CC3EN_R {
        CC3EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CC3P"]
    #[inline(always)]
    pub fn cc3p(&self) -> CC3P_R {
        CC3P_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CC3NEN"]
    #[inline(always)]
    pub fn cc3nen(&self) -> CC3NEN_R {
        CC3NEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CC3NP"]
    #[inline(always)]
    pub fn cc3np(&self) -> CC3NP_R {
        CC3NP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CC4EN"]
    #[inline(always)]
    pub fn cc4en(&self) -> CC4EN_R {
        CC4EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CC4P"]
    #[inline(always)]
    pub fn cc4p(&self) -> CC4P_R {
        CC4P_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - CC5EN"]
    #[inline(always)]
    pub fn cc5en(&self) -> CC5EN_R {
        CC5EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CC5P"]
    #[inline(always)]
    pub fn cc5p(&self) -> CC5P_R {
        CC5P_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - CC6EN"]
    #[inline(always)]
    pub fn cc6en(&self) -> CC6EN_R {
        CC6EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CC6P"]
    #[inline(always)]
    pub fn cc6p(&self) -> CC6P_R {
        CC6P_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CC1EN"]
    #[inline(always)]
    pub fn cc1en(&mut self) -> CC1EN_W {
        CC1EN_W::new(self)
    }
    #[doc = "Bit 1 - CC1P"]
    #[inline(always)]
    pub fn cc1p(&mut self) -> CC1P_W {
        CC1P_W::new(self)
    }
    #[doc = "Bit 2 - CC1NEN"]
    #[inline(always)]
    pub fn cc1nen(&mut self) -> CC1NEN_W {
        CC1NEN_W::new(self)
    }
    #[doc = "Bit 3 - CC1NP"]
    #[inline(always)]
    pub fn cc1np(&mut self) -> CC1NP_W {
        CC1NP_W::new(self)
    }
    #[doc = "Bit 4 - CC2EN"]
    #[inline(always)]
    pub fn cc2en(&mut self) -> CC2EN_W {
        CC2EN_W::new(self)
    }
    #[doc = "Bit 5 - CC2P"]
    #[inline(always)]
    pub fn cc2p(&mut self) -> CC2P_W {
        CC2P_W::new(self)
    }
    #[doc = "Bit 6 - CC2NEN"]
    #[inline(always)]
    pub fn cc2nen(&mut self) -> CC2NEN_W {
        CC2NEN_W::new(self)
    }
    #[doc = "Bit 7 - CC2NP"]
    #[inline(always)]
    pub fn cc2np(&mut self) -> CC2NP_W {
        CC2NP_W::new(self)
    }
    #[doc = "Bit 8 - CC3EN"]
    #[inline(always)]
    pub fn cc3en(&mut self) -> CC3EN_W {
        CC3EN_W::new(self)
    }
    #[doc = "Bit 9 - CC3P"]
    #[inline(always)]
    pub fn cc3p(&mut self) -> CC3P_W {
        CC3P_W::new(self)
    }
    #[doc = "Bit 10 - CC3NEN"]
    #[inline(always)]
    pub fn cc3nen(&mut self) -> CC3NEN_W {
        CC3NEN_W::new(self)
    }
    #[doc = "Bit 11 - CC3NP"]
    #[inline(always)]
    pub fn cc3np(&mut self) -> CC3NP_W {
        CC3NP_W::new(self)
    }
    #[doc = "Bit 12 - CC4EN"]
    #[inline(always)]
    pub fn cc4en(&mut self) -> CC4EN_W {
        CC4EN_W::new(self)
    }
    #[doc = "Bit 13 - CC4P"]
    #[inline(always)]
    pub fn cc4p(&mut self) -> CC4P_W {
        CC4P_W::new(self)
    }
    #[doc = "Bit 16 - CC5EN"]
    #[inline(always)]
    pub fn cc5en(&mut self) -> CC5EN_W {
        CC5EN_W::new(self)
    }
    #[doc = "Bit 17 - CC5P"]
    #[inline(always)]
    pub fn cc5p(&mut self) -> CC5P_W {
        CC5P_W::new(self)
    }
    #[doc = "Bit 20 - CC6EN"]
    #[inline(always)]
    pub fn cc6en(&mut self) -> CC6EN_W {
        CC6EN_W::new(self)
    }
    #[doc = "Bit 21 - CC6P"]
    #[inline(always)]
    pub fn cc6p(&mut self) -> CC6P_W {
        CC6P_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMx_CCEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccen](index.html) module"]
pub struct TIMX_CCEN_SPEC;
impl crate::RegisterSpec for TIMX_CCEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timx_ccen::R](R) reader structure"]
impl crate::Readable for TIMX_CCEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_ccen::W](W) writer structure"]
impl crate::Writable for TIMX_CCEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMx_CCEN to value 0"]
impl crate::Resettable for TIMX_CCEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
