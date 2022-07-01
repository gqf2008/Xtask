#[doc = "Register `AFIO_EXTI_CFG3` reader"]
pub struct R(crate::R<AFIO_EXTI_CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIO_EXTI_CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIO_EXTI_CFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIO_EXTI_CFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFIO_EXTI_CFG3` writer"]
pub struct W(crate::W<AFIO_EXTI_CFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFIO_EXTI_CFG3_SPEC>;
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
impl From<crate::W<AFIO_EXTI_CFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFIO_EXTI_CFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI8_CFG` reader - EXTI8_CFG"]
pub type EXTI8_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI8_CFG` writer - EXTI8_CFG"]
pub type EXTI8_CFG_W<'a> = crate::FieldWriter<'a, u32, AFIO_EXTI_CFG3_SPEC, u8, u8, 4, 0>;
#[doc = "Field `EXTI9_CFG` reader - EXTI9_CFG"]
pub type EXTI9_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI9_CFG` writer - EXTI9_CFG"]
pub type EXTI9_CFG_W<'a> = crate::FieldWriter<'a, u32, AFIO_EXTI_CFG3_SPEC, u8, u8, 4, 4>;
#[doc = "Field `EXTI10_CFG` reader - EXTI10_CFG"]
pub type EXTI10_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI10_CFG` writer - EXTI10_CFG"]
pub type EXTI10_CFG_W<'a> = crate::FieldWriter<'a, u32, AFIO_EXTI_CFG3_SPEC, u8, u8, 4, 8>;
#[doc = "Field `EXTI11_CFG` reader - EXTI11_CFG"]
pub type EXTI11_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI11_CFG` writer - EXTI11_CFG"]
pub type EXTI11_CFG_W<'a> = crate::FieldWriter<'a, u32, AFIO_EXTI_CFG3_SPEC, u8, u8, 4, 12>;
impl R {
    #[doc = "Bits 0:3 - EXTI8_CFG"]
    #[inline(always)]
    pub fn exti8_cfg(&self) -> EXTI8_CFG_R {
        EXTI8_CFG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI9_CFG"]
    #[inline(always)]
    pub fn exti9_cfg(&self) -> EXTI9_CFG_R {
        EXTI9_CFG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI10_CFG"]
    #[inline(always)]
    pub fn exti10_cfg(&self) -> EXTI10_CFG_R {
        EXTI10_CFG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI11_CFG"]
    #[inline(always)]
    pub fn exti11_cfg(&self) -> EXTI11_CFG_R {
        EXTI11_CFG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI8_CFG"]
    #[inline(always)]
    pub fn exti8_cfg(&mut self) -> EXTI8_CFG_W {
        EXTI8_CFG_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI9_CFG"]
    #[inline(always)]
    pub fn exti9_cfg(&mut self) -> EXTI9_CFG_W {
        EXTI9_CFG_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI10_CFG"]
    #[inline(always)]
    pub fn exti10_cfg(&mut self) -> EXTI10_CFG_W {
        EXTI10_CFG_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTI11_CFG"]
    #[inline(always)]
    pub fn exti11_cfg(&mut self) -> EXTI11_CFG_W {
        EXTI11_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO_EXTI_CFG3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_exti_cfg3](index.html) module"]
pub struct AFIO_EXTI_CFG3_SPEC;
impl crate::RegisterSpec for AFIO_EXTI_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afio_exti_cfg3::R](R) reader structure"]
impl crate::Readable for AFIO_EXTI_CFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afio_exti_cfg3::W](W) writer structure"]
impl crate::Writable for AFIO_EXTI_CFG3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFIO_EXTI_CFG3 to value 0"]
impl crate::Resettable for AFIO_EXTI_CFG3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
