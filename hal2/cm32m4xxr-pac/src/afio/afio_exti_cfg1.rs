#[doc = "Register `AFIO_EXTI_CFG1` reader"]
pub struct R(crate::R<AFIO_EXTI_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIO_EXTI_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIO_EXTI_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIO_EXTI_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFIO_EXTI_CFG1` writer"]
pub struct W(crate::W<AFIO_EXTI_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFIO_EXTI_CFG1_SPEC>;
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
impl From<crate::W<AFIO_EXTI_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFIO_EXTI_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI0_CFG` reader - EXTI0_CFG"]
pub type EXTI0_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI0_CFG` writer - EXTI0_CFG"]
pub type EXTI0_CFG_W<'a> = crate::FieldWriter<'a, u32, AFIO_EXTI_CFG1_SPEC, u8, u8, 4, 0>;
#[doc = "Field `EXTI1_CFG` reader - EXTI1_CFG"]
pub type EXTI1_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI1_CFG` writer - EXTI1_CFG"]
pub type EXTI1_CFG_W<'a> = crate::FieldWriter<'a, u32, AFIO_EXTI_CFG1_SPEC, u8, u8, 4, 4>;
#[doc = "Field `EXTI2_CFG` reader - EXTI2_CFG"]
pub type EXTI2_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI2_CFG` writer - EXTI2_CFG"]
pub type EXTI2_CFG_W<'a> = crate::FieldWriter<'a, u32, AFIO_EXTI_CFG1_SPEC, u8, u8, 4, 8>;
#[doc = "Field `EXTI3_CFG` reader - EXTI3_CFG"]
pub type EXTI3_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI3_CFG` writer - EXTI3_CFG"]
pub type EXTI3_CFG_W<'a> = crate::FieldWriter<'a, u32, AFIO_EXTI_CFG1_SPEC, u8, u8, 4, 12>;
impl R {
    #[doc = "Bits 0:3 - EXTI0_CFG"]
    #[inline(always)]
    pub fn exti0_cfg(&self) -> EXTI0_CFG_R {
        EXTI0_CFG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI1_CFG"]
    #[inline(always)]
    pub fn exti1_cfg(&self) -> EXTI1_CFG_R {
        EXTI1_CFG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI2_CFG"]
    #[inline(always)]
    pub fn exti2_cfg(&self) -> EXTI2_CFG_R {
        EXTI2_CFG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI3_CFG"]
    #[inline(always)]
    pub fn exti3_cfg(&self) -> EXTI3_CFG_R {
        EXTI3_CFG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI0_CFG"]
    #[inline(always)]
    pub fn exti0_cfg(&mut self) -> EXTI0_CFG_W {
        EXTI0_CFG_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI1_CFG"]
    #[inline(always)]
    pub fn exti1_cfg(&mut self) -> EXTI1_CFG_W {
        EXTI1_CFG_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI2_CFG"]
    #[inline(always)]
    pub fn exti2_cfg(&mut self) -> EXTI2_CFG_W {
        EXTI2_CFG_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTI3_CFG"]
    #[inline(always)]
    pub fn exti3_cfg(&mut self) -> EXTI3_CFG_W {
        EXTI3_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO_EXTI_CFG1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_exti_cfg1](index.html) module"]
pub struct AFIO_EXTI_CFG1_SPEC;
impl crate::RegisterSpec for AFIO_EXTI_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afio_exti_cfg1::R](R) reader structure"]
impl crate::Readable for AFIO_EXTI_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afio_exti_cfg1::W](W) writer structure"]
impl crate::Writable for AFIO_EXTI_CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFIO_EXTI_CFG1 to value 0"]
impl crate::Resettable for AFIO_EXTI_CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
