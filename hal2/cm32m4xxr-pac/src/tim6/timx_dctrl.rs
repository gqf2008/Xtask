#[doc = "Register `TIMx_DCTRL` reader"]
pub struct R(crate::R<TIMX_DCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_DCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_DCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_DCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_DCTRL` writer"]
pub struct W(crate::W<TIMX_DCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_DCTRL_SPEC>;
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
impl From<crate::W<TIMX_DCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_DCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBADDR` reader - DBADDR"]
pub type DBADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBADDR` writer - DBADDR"]
pub type DBADDR_W<'a> = crate::FieldWriter<'a, u32, TIMX_DCTRL_SPEC, u8, u8, 5, 0>;
#[doc = "Field `DBLEN` reader - DBLEN"]
pub type DBLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBLEN` writer - DBLEN"]
pub type DBLEN_W<'a> = crate::FieldWriter<'a, u32, TIMX_DCTRL_SPEC, u8, u8, 5, 8>;
impl R {
    #[doc = "Bits 0:4 - DBADDR"]
    #[inline(always)]
    pub fn dbaddr(&self) -> DBADDR_R {
        DBADDR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DBLEN"]
    #[inline(always)]
    pub fn dblen(&self) -> DBLEN_R {
        DBLEN_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DBADDR"]
    #[inline(always)]
    pub fn dbaddr(&mut self) -> DBADDR_W {
        DBADDR_W::new(self)
    }
    #[doc = "Bits 8:12 - DBLEN"]
    #[inline(always)]
    pub fn dblen(&mut self) -> DBLEN_W {
        DBLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMx_DCTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_dctrl](index.html) module"]
pub struct TIMX_DCTRL_SPEC;
impl crate::RegisterSpec for TIMX_DCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timx_dctrl::R](R) reader structure"]
impl crate::Readable for TIMX_DCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_dctrl::W](W) writer structure"]
impl crate::Writable for TIMX_DCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMx_DCTRL to value 0"]
impl crate::Resettable for TIMX_DCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
