#[doc = "Register `TIMx_SMCTRL` reader"]
pub struct R(crate::R<TIMX_SMCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_SMCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_SMCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_SMCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_SMCTRL` writer"]
pub struct W(crate::W<TIMX_SMCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_SMCTRL_SPEC>;
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
impl From<crate::W<TIMX_SMCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_SMCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMSEL` reader - SMSEL"]
pub type SMSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMSEL` writer - SMSEL"]
pub type SMSEL_W<'a> = crate::FieldWriter<'a, u32, TIMX_SMCTRL_SPEC, u8, u8, 3, 0>;
#[doc = "Field `TSEL` reader - TSEL"]
pub type TSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSEL` writer - TSEL"]
pub type TSEL_W<'a> = crate::FieldWriter<'a, u32, TIMX_SMCTRL_SPEC, u8, u8, 3, 4>;
#[doc = "Field `MSMD` reader - MSMD"]
pub type MSMD_R = crate::BitReader<bool>;
#[doc = "Field `MSMD` writer - MSMD"]
pub type MSMD_W<'a> = crate::BitWriter<'a, u32, TIMX_SMCTRL_SPEC, bool, 7>;
#[doc = "Field `EXTF` reader - EXTF"]
pub type EXTF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTF` writer - EXTF"]
pub type EXTF_W<'a> = crate::FieldWriter<'a, u32, TIMX_SMCTRL_SPEC, u8, u8, 4, 8>;
#[doc = "Field `EXTPS` reader - EXTPS"]
pub type EXTPS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTPS` writer - EXTPS"]
pub type EXTPS_W<'a> = crate::FieldWriter<'a, u32, TIMX_SMCTRL_SPEC, u8, u8, 2, 12>;
#[doc = "Field `EXCEN` reader - EXCEN"]
pub type EXCEN_R = crate::BitReader<bool>;
#[doc = "Field `EXCEN` writer - EXCEN"]
pub type EXCEN_W<'a> = crate::BitWriter<'a, u32, TIMX_SMCTRL_SPEC, bool, 14>;
#[doc = "Field `EXTP` reader - EXTP"]
pub type EXTP_R = crate::BitReader<bool>;
#[doc = "Field `EXTP` writer - EXTP"]
pub type EXTP_W<'a> = crate::BitWriter<'a, u32, TIMX_SMCTRL_SPEC, bool, 15>;
impl R {
    #[doc = "Bits 0:2 - SMSEL"]
    #[inline(always)]
    pub fn smsel(&self) -> SMSEL_R {
        SMSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - TSEL"]
    #[inline(always)]
    pub fn tsel(&self) -> TSEL_R {
        TSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - MSMD"]
    #[inline(always)]
    pub fn msmd(&self) -> MSMD_R {
        MSMD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - EXTF"]
    #[inline(always)]
    pub fn extf(&self) -> EXTF_R {
        EXTF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - EXTPS"]
    #[inline(always)]
    pub fn extps(&self) -> EXTPS_R {
        EXTPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - EXCEN"]
    #[inline(always)]
    pub fn excen(&self) -> EXCEN_R {
        EXCEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EXTP"]
    #[inline(always)]
    pub fn extp(&self) -> EXTP_R {
        EXTP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SMSEL"]
    #[inline(always)]
    pub fn smsel(&mut self) -> SMSEL_W {
        SMSEL_W::new(self)
    }
    #[doc = "Bits 4:6 - TSEL"]
    #[inline(always)]
    pub fn tsel(&mut self) -> TSEL_W {
        TSEL_W::new(self)
    }
    #[doc = "Bit 7 - MSMD"]
    #[inline(always)]
    pub fn msmd(&mut self) -> MSMD_W {
        MSMD_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTF"]
    #[inline(always)]
    pub fn extf(&mut self) -> EXTF_W {
        EXTF_W::new(self)
    }
    #[doc = "Bits 12:13 - EXTPS"]
    #[inline(always)]
    pub fn extps(&mut self) -> EXTPS_W {
        EXTPS_W::new(self)
    }
    #[doc = "Bit 14 - EXCEN"]
    #[inline(always)]
    pub fn excen(&mut self) -> EXCEN_W {
        EXCEN_W::new(self)
    }
    #[doc = "Bit 15 - EXTP"]
    #[inline(always)]
    pub fn extp(&mut self) -> EXTP_W {
        EXTP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMx_SMCTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_smctrl](index.html) module"]
pub struct TIMX_SMCTRL_SPEC;
impl crate::RegisterSpec for TIMX_SMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timx_smctrl::R](R) reader structure"]
impl crate::Readable for TIMX_SMCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_smctrl::W](W) writer structure"]
impl crate::Writable for TIMX_SMCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMx_SMCTRL to value 0"]
impl crate::Resettable for TIMX_SMCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
