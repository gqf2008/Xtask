#[doc = "Register `TIMx_CTRL1` reader"]
pub struct R(crate::R<TIMX_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_CTRL1` writer"]
pub struct W(crate::W<TIMX_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_CTRL1_SPEC>;
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
impl From<crate::W<TIMX_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTEN` reader - CNTEN"]
pub type CNTEN_R = crate::BitReader<bool>;
#[doc = "Field `CNTEN` writer - CNTEN"]
pub type CNTEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL1_SPEC, bool, 0>;
#[doc = "Field `UPDIS` reader - UPDIS"]
pub type UPDIS_R = crate::BitReader<bool>;
#[doc = "Field `UPDIS` writer - UPDIS"]
pub type UPDIS_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL1_SPEC, bool, 1>;
#[doc = "Field `UPRS` reader - UPRS"]
pub type UPRS_R = crate::BitReader<bool>;
#[doc = "Field `UPRS` writer - UPRS"]
pub type UPRS_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL1_SPEC, bool, 2>;
#[doc = "Field `ONEPM` reader - ONEPM"]
pub type ONEPM_R = crate::BitReader<bool>;
#[doc = "Field `ONEPM` writer - ONEPM"]
pub type ONEPM_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL1_SPEC, bool, 3>;
#[doc = "Field `DIR` reader - DIR"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - DIR"]
pub type DIR_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL1_SPEC, bool, 4>;
#[doc = "Field `CAMSEL` reader - CAMSEL"]
pub type CAMSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAMSEL` writer - CAMSEL"]
pub type CAMSEL_W<'a> = crate::FieldWriter<'a, u32, TIMX_CTRL1_SPEC, u8, u8, 2, 5>;
#[doc = "Field `ARPEN` reader - ARPEN"]
pub type ARPEN_R = crate::BitReader<bool>;
#[doc = "Field `ARPEN` writer - ARPEN"]
pub type ARPEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL1_SPEC, bool, 7>;
#[doc = "Field `CLKD` reader - CLKD"]
pub type CLKD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKD` writer - CLKD"]
pub type CLKD_W<'a> = crate::FieldWriter<'a, u32, TIMX_CTRL1_SPEC, u8, u8, 2, 8>;
#[doc = "Field `IOMBKPEN` reader - IOMBKPEN"]
pub type IOMBKPEN_R = crate::BitReader<bool>;
#[doc = "Field `IOMBKPEN` writer - IOMBKPEN"]
pub type IOMBKPEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL1_SPEC, bool, 10>;
#[doc = "Field `C1SEL` reader - C1SEL"]
pub type C1SEL_R = crate::BitReader<bool>;
#[doc = "Field `C1SEL` writer - C1SEL"]
pub type C1SEL_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL1_SPEC, bool, 11>;
#[doc = "Field `C2SEL` reader - C2SEL"]
pub type C2SEL_R = crate::BitReader<bool>;
#[doc = "Field `C2SEL` writer - C2SEL"]
pub type C2SEL_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL1_SPEC, bool, 12>;
#[doc = "Field `C3SEL` reader - C3SEL"]
pub type C3SEL_R = crate::BitReader<bool>;
#[doc = "Field `C3SEL` writer - C3SEL"]
pub type C3SEL_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL1_SPEC, bool, 13>;
#[doc = "Field `C4SEL` reader - C4SEL"]
pub type C4SEL_R = crate::BitReader<bool>;
#[doc = "Field `C4SEL` writer - C4SEL"]
pub type C4SEL_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL1_SPEC, bool, 14>;
#[doc = "Field `CLRSEL` reader - CLRSEL"]
pub type CLRSEL_R = crate::BitReader<bool>;
#[doc = "Field `CLRSEL` writer - CLRSEL"]
pub type CLRSEL_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL1_SPEC, bool, 15>;
#[doc = "Field `PBKPEN` reader - PBKPEN"]
pub type PBKPEN_R = crate::BitReader<bool>;
#[doc = "Field `PBKPEN` writer - PBKPEN"]
pub type PBKPEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL1_SPEC, bool, 17>;
impl R {
    #[doc = "Bit 0 - CNTEN"]
    #[inline(always)]
    pub fn cnten(&self) -> CNTEN_R {
        CNTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UPDIS"]
    #[inline(always)]
    pub fn updis(&self) -> UPDIS_R {
        UPDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UPRS"]
    #[inline(always)]
    pub fn uprs(&self) -> UPRS_R {
        UPRS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ONEPM"]
    #[inline(always)]
    pub fn onepm(&self) -> ONEPM_R {
        ONEPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - CAMSEL"]
    #[inline(always)]
    pub fn camsel(&self) -> CAMSEL_R {
        CAMSEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - ARPEN"]
    #[inline(always)]
    pub fn arpen(&self) -> ARPEN_R {
        ARPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - CLKD"]
    #[inline(always)]
    pub fn clkd(&self) -> CLKD_R {
        CLKD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - IOMBKPEN"]
    #[inline(always)]
    pub fn iombkpen(&self) -> IOMBKPEN_R {
        IOMBKPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - C1SEL"]
    #[inline(always)]
    pub fn c1sel(&self) -> C1SEL_R {
        C1SEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - C2SEL"]
    #[inline(always)]
    pub fn c2sel(&self) -> C2SEL_R {
        C2SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - C3SEL"]
    #[inline(always)]
    pub fn c3sel(&self) -> C3SEL_R {
        C3SEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - C4SEL"]
    #[inline(always)]
    pub fn c4sel(&self) -> C4SEL_R {
        C4SEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CLRSEL"]
    #[inline(always)]
    pub fn clrsel(&self) -> CLRSEL_R {
        CLRSEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - PBKPEN"]
    #[inline(always)]
    pub fn pbkpen(&self) -> PBKPEN_R {
        PBKPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CNTEN"]
    #[inline(always)]
    pub fn cnten(&mut self) -> CNTEN_W {
        CNTEN_W::new(self)
    }
    #[doc = "Bit 1 - UPDIS"]
    #[inline(always)]
    pub fn updis(&mut self) -> UPDIS_W {
        UPDIS_W::new(self)
    }
    #[doc = "Bit 2 - UPRS"]
    #[inline(always)]
    pub fn uprs(&mut self) -> UPRS_W {
        UPRS_W::new(self)
    }
    #[doc = "Bit 3 - ONEPM"]
    #[inline(always)]
    pub fn onepm(&mut self) -> ONEPM_W {
        ONEPM_W::new(self)
    }
    #[doc = "Bit 4 - DIR"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W::new(self)
    }
    #[doc = "Bits 5:6 - CAMSEL"]
    #[inline(always)]
    pub fn camsel(&mut self) -> CAMSEL_W {
        CAMSEL_W::new(self)
    }
    #[doc = "Bit 7 - ARPEN"]
    #[inline(always)]
    pub fn arpen(&mut self) -> ARPEN_W {
        ARPEN_W::new(self)
    }
    #[doc = "Bits 8:9 - CLKD"]
    #[inline(always)]
    pub fn clkd(&mut self) -> CLKD_W {
        CLKD_W::new(self)
    }
    #[doc = "Bit 10 - IOMBKPEN"]
    #[inline(always)]
    pub fn iombkpen(&mut self) -> IOMBKPEN_W {
        IOMBKPEN_W::new(self)
    }
    #[doc = "Bit 11 - C1SEL"]
    #[inline(always)]
    pub fn c1sel(&mut self) -> C1SEL_W {
        C1SEL_W::new(self)
    }
    #[doc = "Bit 12 - C2SEL"]
    #[inline(always)]
    pub fn c2sel(&mut self) -> C2SEL_W {
        C2SEL_W::new(self)
    }
    #[doc = "Bit 13 - C3SEL"]
    #[inline(always)]
    pub fn c3sel(&mut self) -> C3SEL_W {
        C3SEL_W::new(self)
    }
    #[doc = "Bit 14 - C4SEL"]
    #[inline(always)]
    pub fn c4sel(&mut self) -> C4SEL_W {
        C4SEL_W::new(self)
    }
    #[doc = "Bit 15 - CLRSEL"]
    #[inline(always)]
    pub fn clrsel(&mut self) -> CLRSEL_W {
        CLRSEL_W::new(self)
    }
    #[doc = "Bit 17 - PBKPEN"]
    #[inline(always)]
    pub fn pbkpen(&mut self) -> PBKPEN_W {
        PBKPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMx_CTRL1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ctrl1](index.html) module"]
pub struct TIMX_CTRL1_SPEC;
impl crate::RegisterSpec for TIMX_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timx_ctrl1::R](R) reader structure"]
impl crate::Readable for TIMX_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_ctrl1::W](W) writer structure"]
impl crate::Writable for TIMX_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMx_CTRL1 to value 0"]
impl crate::Resettable for TIMX_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
