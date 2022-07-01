#[doc = "Register `AFIO_EXTI_CFG2` reader"]
pub struct R(crate::R<AFIO_EXTI_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIO_EXTI_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIO_EXTI_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIO_EXTI_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFIO_EXTI_CFG2` writer"]
pub struct W(crate::W<AFIO_EXTI_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFIO_EXTI_CFG2_SPEC>;
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
impl From<crate::W<AFIO_EXTI_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFIO_EXTI_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI4_CFG` reader - EXTI4_CFG"]
pub type EXTI4_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI4_CFG` writer - EXTI4_CFG"]
pub type EXTI4_CFG_W<'a> = crate::FieldWriter<'a, u32, AFIO_EXTI_CFG2_SPEC, u8, u8, 4, 0>;
#[doc = "Field `EXTI5_CFG` reader - EXTI5_CFG"]
pub type EXTI5_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI5_CFG` writer - EXTI5_CFG"]
pub type EXTI5_CFG_W<'a> = crate::FieldWriter<'a, u32, AFIO_EXTI_CFG2_SPEC, u8, u8, 4, 4>;
#[doc = "Field `EXTI6_CFG` reader - EXTI6_CFG"]
pub type EXTI6_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI6_CFG` writer - EXTI6_CFG"]
pub type EXTI6_CFG_W<'a> = crate::FieldWriter<'a, u32, AFIO_EXTI_CFG2_SPEC, u8, u8, 4, 8>;
#[doc = "Field `EXTI7_CFG` reader - EXTI7_CFG"]
pub type EXTI7_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI7_CFG` writer - EXTI7_CFG"]
pub type EXTI7_CFG_W<'a> = crate::FieldWriter<'a, u32, AFIO_EXTI_CFG2_SPEC, u8, u8, 4, 12>;
impl R {
    #[doc = "Bits 0:3 - EXTI4_CFG"]
    #[inline(always)]
    pub fn exti4_cfg(&self) -> EXTI4_CFG_R {
        EXTI4_CFG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI5_CFG"]
    #[inline(always)]
    pub fn exti5_cfg(&self) -> EXTI5_CFG_R {
        EXTI5_CFG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI6_CFG"]
    #[inline(always)]
    pub fn exti6_cfg(&self) -> EXTI6_CFG_R {
        EXTI6_CFG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI7_CFG"]
    #[inline(always)]
    pub fn exti7_cfg(&self) -> EXTI7_CFG_R {
        EXTI7_CFG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI4_CFG"]
    #[inline(always)]
    pub fn exti4_cfg(&mut self) -> EXTI4_CFG_W {
        EXTI4_CFG_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI5_CFG"]
    #[inline(always)]
    pub fn exti5_cfg(&mut self) -> EXTI5_CFG_W {
        EXTI5_CFG_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI6_CFG"]
    #[inline(always)]
    pub fn exti6_cfg(&mut self) -> EXTI6_CFG_W {
        EXTI6_CFG_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTI7_CFG"]
    #[inline(always)]
    pub fn exti7_cfg(&mut self) -> EXTI7_CFG_W {
        EXTI7_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO_EXTI_CFG2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_exti_cfg2](index.html) module"]
pub struct AFIO_EXTI_CFG2_SPEC;
impl crate::RegisterSpec for AFIO_EXTI_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afio_exti_cfg2::R](R) reader structure"]
impl crate::Readable for AFIO_EXTI_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afio_exti_cfg2::W](W) writer structure"]
impl crate::Writable for AFIO_EXTI_CFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFIO_EXTI_CFG2 to value 0"]
impl crate::Resettable for AFIO_EXTI_CFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
