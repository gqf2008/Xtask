#[doc = "Register `TIMx_STS` reader"]
pub struct R(crate::R<TIMX_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_STS` writer"]
pub struct W(crate::W<TIMX_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_STS_SPEC>;
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
impl From<crate::W<TIMX_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UDITF` reader - UDITF"]
pub type UDITF_R = crate::BitReader<bool>;
#[doc = "Field `UDITF` writer - UDITF"]
pub type UDITF_W<'a> = crate::BitWriter<'a, u32, TIMX_STS_SPEC, bool, 0>;
#[doc = "Field `CC1ITF` reader - CC1ITF"]
pub type CC1ITF_R = crate::BitReader<bool>;
#[doc = "Field `CC1ITF` writer - CC1ITF"]
pub type CC1ITF_W<'a> = crate::BitWriter<'a, u32, TIMX_STS_SPEC, bool, 1>;
#[doc = "Field `CC2ITF` reader - CC2ITF"]
pub type CC2ITF_R = crate::BitReader<bool>;
#[doc = "Field `CC2ITF` writer - CC2ITF"]
pub type CC2ITF_W<'a> = crate::BitWriter<'a, u32, TIMX_STS_SPEC, bool, 2>;
#[doc = "Field `CC3ITF` reader - CC3ITF"]
pub type CC3ITF_R = crate::BitReader<bool>;
#[doc = "Field `CC3ITF` writer - CC3ITF"]
pub type CC3ITF_W<'a> = crate::BitWriter<'a, u32, TIMX_STS_SPEC, bool, 3>;
#[doc = "Field `CC4ITF` reader - CC4ITF"]
pub type CC4ITF_R = crate::BitReader<bool>;
#[doc = "Field `CC4ITF` writer - CC4ITF"]
pub type CC4ITF_W<'a> = crate::BitWriter<'a, u32, TIMX_STS_SPEC, bool, 4>;
#[doc = "Field `COMITF` reader - COMITF"]
pub type COMITF_R = crate::BitReader<bool>;
#[doc = "Field `COMITF` writer - COMITF"]
pub type COMITF_W<'a> = crate::BitWriter<'a, u32, TIMX_STS_SPEC, bool, 5>;
#[doc = "Field `TITF` reader - TITF"]
pub type TITF_R = crate::BitReader<bool>;
#[doc = "Field `TITF` writer - TITF"]
pub type TITF_W<'a> = crate::BitWriter<'a, u32, TIMX_STS_SPEC, bool, 6>;
#[doc = "Field `BITF` reader - BITF"]
pub type BITF_R = crate::BitReader<bool>;
#[doc = "Field `BITF` writer - BITF"]
pub type BITF_W<'a> = crate::BitWriter<'a, u32, TIMX_STS_SPEC, bool, 7>;
#[doc = "Field `CC1OCF` reader - CC1OCF"]
pub type CC1OCF_R = crate::BitReader<bool>;
#[doc = "Field `CC1OCF` writer - CC1OCF"]
pub type CC1OCF_W<'a> = crate::BitWriter<'a, u32, TIMX_STS_SPEC, bool, 9>;
#[doc = "Field `CC2OCF` reader - CC2OCF"]
pub type CC2OCF_R = crate::BitReader<bool>;
#[doc = "Field `CC2OCF` writer - CC2OCF"]
pub type CC2OCF_W<'a> = crate::BitWriter<'a, u32, TIMX_STS_SPEC, bool, 10>;
#[doc = "Field `CC3OCF` reader - CC3OCF"]
pub type CC3OCF_R = crate::BitReader<bool>;
#[doc = "Field `CC3OCF` writer - CC3OCF"]
pub type CC3OCF_W<'a> = crate::BitWriter<'a, u32, TIMX_STS_SPEC, bool, 11>;
#[doc = "Field `CC4OCF` reader - CC4OCF"]
pub type CC4OCF_R = crate::BitReader<bool>;
#[doc = "Field `CC4OCF` writer - CC4OCF"]
pub type CC4OCF_W<'a> = crate::BitWriter<'a, u32, TIMX_STS_SPEC, bool, 12>;
#[doc = "Field `CC5ITF` reader - CC5ITF"]
pub type CC5ITF_R = crate::BitReader<bool>;
#[doc = "Field `CC5ITF` writer - CC5ITF"]
pub type CC5ITF_W<'a> = crate::BitWriter<'a, u32, TIMX_STS_SPEC, bool, 16>;
#[doc = "Field `CC6ITF` reader - CC6ITF"]
pub type CC6ITF_R = crate::BitReader<bool>;
#[doc = "Field `CC6ITF` writer - CC6ITF"]
pub type CC6ITF_W<'a> = crate::BitWriter<'a, u32, TIMX_STS_SPEC, bool, 17>;
impl R {
    #[doc = "Bit 0 - UDITF"]
    #[inline(always)]
    pub fn uditf(&self) -> UDITF_R {
        UDITF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC1ITF"]
    #[inline(always)]
    pub fn cc1itf(&self) -> CC1ITF_R {
        CC1ITF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC2ITF"]
    #[inline(always)]
    pub fn cc2itf(&self) -> CC2ITF_R {
        CC2ITF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CC3ITF"]
    #[inline(always)]
    pub fn cc3itf(&self) -> CC3ITF_R {
        CC3ITF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CC4ITF"]
    #[inline(always)]
    pub fn cc4itf(&self) -> CC4ITF_R {
        CC4ITF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COMITF"]
    #[inline(always)]
    pub fn comitf(&self) -> COMITF_R {
        COMITF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TITF"]
    #[inline(always)]
    pub fn titf(&self) -> TITF_R {
        TITF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BITF"]
    #[inline(always)]
    pub fn bitf(&self) -> BITF_R {
        BITF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - CC1OCF"]
    #[inline(always)]
    pub fn cc1ocf(&self) -> CC1OCF_R {
        CC1OCF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CC2OCF"]
    #[inline(always)]
    pub fn cc2ocf(&self) -> CC2OCF_R {
        CC2OCF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CC3OCF"]
    #[inline(always)]
    pub fn cc3ocf(&self) -> CC3OCF_R {
        CC3OCF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CC4OCF"]
    #[inline(always)]
    pub fn cc4ocf(&self) -> CC4OCF_R {
        CC4OCF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - CC5ITF"]
    #[inline(always)]
    pub fn cc5itf(&self) -> CC5ITF_R {
        CC5ITF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CC6ITF"]
    #[inline(always)]
    pub fn cc6itf(&self) -> CC6ITF_R {
        CC6ITF_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UDITF"]
    #[inline(always)]
    pub fn uditf(&mut self) -> UDITF_W {
        UDITF_W::new(self)
    }
    #[doc = "Bit 1 - CC1ITF"]
    #[inline(always)]
    pub fn cc1itf(&mut self) -> CC1ITF_W {
        CC1ITF_W::new(self)
    }
    #[doc = "Bit 2 - CC2ITF"]
    #[inline(always)]
    pub fn cc2itf(&mut self) -> CC2ITF_W {
        CC2ITF_W::new(self)
    }
    #[doc = "Bit 3 - CC3ITF"]
    #[inline(always)]
    pub fn cc3itf(&mut self) -> CC3ITF_W {
        CC3ITF_W::new(self)
    }
    #[doc = "Bit 4 - CC4ITF"]
    #[inline(always)]
    pub fn cc4itf(&mut self) -> CC4ITF_W {
        CC4ITF_W::new(self)
    }
    #[doc = "Bit 5 - COMITF"]
    #[inline(always)]
    pub fn comitf(&mut self) -> COMITF_W {
        COMITF_W::new(self)
    }
    #[doc = "Bit 6 - TITF"]
    #[inline(always)]
    pub fn titf(&mut self) -> TITF_W {
        TITF_W::new(self)
    }
    #[doc = "Bit 7 - BITF"]
    #[inline(always)]
    pub fn bitf(&mut self) -> BITF_W {
        BITF_W::new(self)
    }
    #[doc = "Bit 9 - CC1OCF"]
    #[inline(always)]
    pub fn cc1ocf(&mut self) -> CC1OCF_W {
        CC1OCF_W::new(self)
    }
    #[doc = "Bit 10 - CC2OCF"]
    #[inline(always)]
    pub fn cc2ocf(&mut self) -> CC2OCF_W {
        CC2OCF_W::new(self)
    }
    #[doc = "Bit 11 - CC3OCF"]
    #[inline(always)]
    pub fn cc3ocf(&mut self) -> CC3OCF_W {
        CC3OCF_W::new(self)
    }
    #[doc = "Bit 12 - CC4OCF"]
    #[inline(always)]
    pub fn cc4ocf(&mut self) -> CC4OCF_W {
        CC4OCF_W::new(self)
    }
    #[doc = "Bit 16 - CC5ITF"]
    #[inline(always)]
    pub fn cc5itf(&mut self) -> CC5ITF_W {
        CC5ITF_W::new(self)
    }
    #[doc = "Bit 17 - CC6ITF"]
    #[inline(always)]
    pub fn cc6itf(&mut self) -> CC6ITF_W {
        CC6ITF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMx_STS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_sts](index.html) module"]
pub struct TIMX_STS_SPEC;
impl crate::RegisterSpec for TIMX_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timx_sts::R](R) reader structure"]
impl crate::Readable for TIMX_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_sts::W](W) writer structure"]
impl crate::Writable for TIMX_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMx_STS to value 0"]
impl crate::Resettable for TIMX_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
