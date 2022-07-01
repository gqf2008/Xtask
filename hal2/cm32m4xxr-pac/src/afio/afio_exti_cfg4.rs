#[doc = "Register `AFIO_EXTI_CFG4` reader"]
pub struct R(crate::R<AFIO_EXTI_CFG4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIO_EXTI_CFG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIO_EXTI_CFG4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIO_EXTI_CFG4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFIO_EXTI_CFG4` writer"]
pub struct W(crate::W<AFIO_EXTI_CFG4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFIO_EXTI_CFG4_SPEC>;
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
impl From<crate::W<AFIO_EXTI_CFG4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFIO_EXTI_CFG4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI12_CFG` reader - EXTI12_CFG"]
pub type EXTI12_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI12_CFG` writer - EXTI12_CFG"]
pub type EXTI12_CFG_W<'a> = crate::FieldWriter<'a, u32, AFIO_EXTI_CFG4_SPEC, u8, u8, 4, 0>;
#[doc = "Field `EXTI13_CFG` reader - EXTI13_CFG"]
pub type EXTI13_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI13_CFG` writer - EXTI13_CFG"]
pub type EXTI13_CFG_W<'a> = crate::FieldWriter<'a, u32, AFIO_EXTI_CFG4_SPEC, u8, u8, 4, 4>;
#[doc = "Field `EXTI14_CFG` reader - EXTI14_CFG"]
pub type EXTI14_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI14_CFG` writer - EXTI14_CFG"]
pub type EXTI14_CFG_W<'a> = crate::FieldWriter<'a, u32, AFIO_EXTI_CFG4_SPEC, u8, u8, 4, 8>;
#[doc = "Field `EXTI15_CFG` reader - EXTI15_CFG"]
pub type EXTI15_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTI15_CFG` writer - EXTI15_CFG"]
pub type EXTI15_CFG_W<'a> = crate::FieldWriter<'a, u32, AFIO_EXTI_CFG4_SPEC, u8, u8, 4, 12>;
impl R {
    #[doc = "Bits 0:3 - EXTI12_CFG"]
    #[inline(always)]
    pub fn exti12_cfg(&self) -> EXTI12_CFG_R {
        EXTI12_CFG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI13_CFG"]
    #[inline(always)]
    pub fn exti13_cfg(&self) -> EXTI13_CFG_R {
        EXTI13_CFG_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI14_CFG"]
    #[inline(always)]
    pub fn exti14_cfg(&self) -> EXTI14_CFG_R {
        EXTI14_CFG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI15_CFG"]
    #[inline(always)]
    pub fn exti15_cfg(&self) -> EXTI15_CFG_R {
        EXTI15_CFG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI12_CFG"]
    #[inline(always)]
    pub fn exti12_cfg(&mut self) -> EXTI12_CFG_W {
        EXTI12_CFG_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI13_CFG"]
    #[inline(always)]
    pub fn exti13_cfg(&mut self) -> EXTI13_CFG_W {
        EXTI13_CFG_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI14_CFG"]
    #[inline(always)]
    pub fn exti14_cfg(&mut self) -> EXTI14_CFG_W {
        EXTI14_CFG_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTI15_CFG"]
    #[inline(always)]
    pub fn exti15_cfg(&mut self) -> EXTI15_CFG_W {
        EXTI15_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO_EXTI_CFG4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_exti_cfg4](index.html) module"]
pub struct AFIO_EXTI_CFG4_SPEC;
impl crate::RegisterSpec for AFIO_EXTI_CFG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afio_exti_cfg4::R](R) reader structure"]
impl crate::Readable for AFIO_EXTI_CFG4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afio_exti_cfg4::W](W) writer structure"]
impl crate::Writable for AFIO_EXTI_CFG4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFIO_EXTI_CFG4 to value 0"]
impl crate::Resettable for AFIO_EXTI_CFG4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
