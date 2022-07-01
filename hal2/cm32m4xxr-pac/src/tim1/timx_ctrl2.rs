#[doc = "Register `TIMx_CTRL2` reader"]
pub struct R(crate::R<TIMX_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_CTRL2` writer"]
pub struct W(crate::W<TIMX_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_CTRL2_SPEC>;
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
impl From<crate::W<TIMX_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCPCTL` reader - CCPCTL"]
pub type CCPCTL_R = crate::BitReader<bool>;
#[doc = "Field `CCPCTL` writer - CCPCTL"]
pub type CCPCTL_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL2_SPEC, bool, 0>;
#[doc = "Field `CCUSEL` reader - CCUSEL"]
pub type CCUSEL_R = crate::BitReader<bool>;
#[doc = "Field `CCUSEL` writer - CCUSEL"]
pub type CCUSEL_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL2_SPEC, bool, 2>;
#[doc = "Field `CCDSEL` reader - CCDSEL"]
pub type CCDSEL_R = crate::BitReader<bool>;
#[doc = "Field `CCDSEL` writer - CCDSEL"]
pub type CCDSEL_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL2_SPEC, bool, 3>;
#[doc = "Field `MMSEL` reader - MMSEL"]
pub type MMSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MMSEL` writer - MMSEL"]
pub type MMSEL_W<'a> = crate::FieldWriter<'a, u32, TIMX_CTRL2_SPEC, u8, u8, 3, 4>;
#[doc = "Field `TI1SEL` reader - TI1SEL"]
pub type TI1SEL_R = crate::BitReader<bool>;
#[doc = "Field `TI1SEL` writer - TI1SEL"]
pub type TI1SEL_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL2_SPEC, bool, 7>;
#[doc = "Field `OI1` reader - OI1"]
pub type OI1_R = crate::BitReader<bool>;
#[doc = "Field `OI1` writer - OI1"]
pub type OI1_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL2_SPEC, bool, 8>;
#[doc = "Field `OI1N` reader - OI1N"]
pub type OI1N_R = crate::BitReader<bool>;
#[doc = "Field `OI1N` writer - OI1N"]
pub type OI1N_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL2_SPEC, bool, 9>;
#[doc = "Field `OI2` reader - OI2"]
pub type OI2_R = crate::BitReader<bool>;
#[doc = "Field `OI2` writer - OI2"]
pub type OI2_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL2_SPEC, bool, 10>;
#[doc = "Field `OI2N` reader - OI2N"]
pub type OI2N_R = crate::BitReader<bool>;
#[doc = "Field `OI2N` writer - OI2N"]
pub type OI2N_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL2_SPEC, bool, 11>;
#[doc = "Field `OI3` reader - OI3"]
pub type OI3_R = crate::BitReader<bool>;
#[doc = "Field `OI3` writer - OI3"]
pub type OI3_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL2_SPEC, bool, 12>;
#[doc = "Field `OI3N` reader - OI3N"]
pub type OI3N_R = crate::BitReader<bool>;
#[doc = "Field `OI3N` writer - OI3N"]
pub type OI3N_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL2_SPEC, bool, 13>;
#[doc = "Field `OI4` reader - OI4"]
pub type OI4_R = crate::BitReader<bool>;
#[doc = "Field `OI4` writer - OI4"]
pub type OI4_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL2_SPEC, bool, 14>;
#[doc = "Field `OI5` reader - OI5"]
pub type OI5_R = crate::BitReader<bool>;
#[doc = "Field `OI5` writer - OI5"]
pub type OI5_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL2_SPEC, bool, 16>;
#[doc = "Field `OI6` reader - OI6"]
pub type OI6_R = crate::BitReader<bool>;
#[doc = "Field `OI6` writer - OI6"]
pub type OI6_W<'a> = crate::BitWriter<'a, u32, TIMX_CTRL2_SPEC, bool, 18>;
impl R {
    #[doc = "Bit 0 - CCPCTL"]
    #[inline(always)]
    pub fn ccpctl(&self) -> CCPCTL_R {
        CCPCTL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - CCUSEL"]
    #[inline(always)]
    pub fn ccusel(&self) -> CCUSEL_R {
        CCUSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CCDSEL"]
    #[inline(always)]
    pub fn ccdsel(&self) -> CCDSEL_R {
        CCDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - MMSEL"]
    #[inline(always)]
    pub fn mmsel(&self) -> MMSEL_R {
        MMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - TI1SEL"]
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OI1"]
    #[inline(always)]
    pub fn oi1(&self) -> OI1_R {
        OI1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OI1N"]
    #[inline(always)]
    pub fn oi1n(&self) -> OI1N_R {
        OI1N_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OI2"]
    #[inline(always)]
    pub fn oi2(&self) -> OI2_R {
        OI2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OI2N"]
    #[inline(always)]
    pub fn oi2n(&self) -> OI2N_R {
        OI2N_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OI3"]
    #[inline(always)]
    pub fn oi3(&self) -> OI3_R {
        OI3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - OI3N"]
    #[inline(always)]
    pub fn oi3n(&self) -> OI3N_R {
        OI3N_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - OI4"]
    #[inline(always)]
    pub fn oi4(&self) -> OI4_R {
        OI4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - OI5"]
    #[inline(always)]
    pub fn oi5(&self) -> OI5_R {
        OI5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - OI6"]
    #[inline(always)]
    pub fn oi6(&self) -> OI6_R {
        OI6_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCPCTL"]
    #[inline(always)]
    pub fn ccpctl(&mut self) -> CCPCTL_W {
        CCPCTL_W::new(self)
    }
    #[doc = "Bit 2 - CCUSEL"]
    #[inline(always)]
    pub fn ccusel(&mut self) -> CCUSEL_W {
        CCUSEL_W::new(self)
    }
    #[doc = "Bit 3 - CCDSEL"]
    #[inline(always)]
    pub fn ccdsel(&mut self) -> CCDSEL_W {
        CCDSEL_W::new(self)
    }
    #[doc = "Bits 4:6 - MMSEL"]
    #[inline(always)]
    pub fn mmsel(&mut self) -> MMSEL_W {
        MMSEL_W::new(self)
    }
    #[doc = "Bit 7 - TI1SEL"]
    #[inline(always)]
    pub fn ti1sel(&mut self) -> TI1SEL_W {
        TI1SEL_W::new(self)
    }
    #[doc = "Bit 8 - OI1"]
    #[inline(always)]
    pub fn oi1(&mut self) -> OI1_W {
        OI1_W::new(self)
    }
    #[doc = "Bit 9 - OI1N"]
    #[inline(always)]
    pub fn oi1n(&mut self) -> OI1N_W {
        OI1N_W::new(self)
    }
    #[doc = "Bit 10 - OI2"]
    #[inline(always)]
    pub fn oi2(&mut self) -> OI2_W {
        OI2_W::new(self)
    }
    #[doc = "Bit 11 - OI2N"]
    #[inline(always)]
    pub fn oi2n(&mut self) -> OI2N_W {
        OI2N_W::new(self)
    }
    #[doc = "Bit 12 - OI3"]
    #[inline(always)]
    pub fn oi3(&mut self) -> OI3_W {
        OI3_W::new(self)
    }
    #[doc = "Bit 13 - OI3N"]
    #[inline(always)]
    pub fn oi3n(&mut self) -> OI3N_W {
        OI3N_W::new(self)
    }
    #[doc = "Bit 14 - OI4"]
    #[inline(always)]
    pub fn oi4(&mut self) -> OI4_W {
        OI4_W::new(self)
    }
    #[doc = "Bit 16 - OI5"]
    #[inline(always)]
    pub fn oi5(&mut self) -> OI5_W {
        OI5_W::new(self)
    }
    #[doc = "Bit 18 - OI6"]
    #[inline(always)]
    pub fn oi6(&mut self) -> OI6_W {
        OI6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMx_CTRL2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ctrl2](index.html) module"]
pub struct TIMX_CTRL2_SPEC;
impl crate::RegisterSpec for TIMX_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timx_ctrl2::R](R) reader structure"]
impl crate::Readable for TIMX_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_ctrl2::W](W) writer structure"]
impl crate::Writable for TIMX_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMx_CTRL2 to value 0"]
impl crate::Resettable for TIMX_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
