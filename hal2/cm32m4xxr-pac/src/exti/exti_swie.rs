#[doc = "Register `EXTI_SWIE` reader"]
pub struct R(crate::R<EXTI_SWIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_SWIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_SWIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_SWIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_SWIE` writer"]
pub struct W(crate::W<EXTI_SWIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_SWIE_SPEC>;
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
impl From<crate::W<EXTI_SWIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_SWIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWIE` reader - SWIE"]
pub type SWIE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SWIE` writer - SWIE"]
pub type SWIE_W<'a> = crate::FieldWriter<'a, u32, EXTI_SWIE_SPEC, u32, u32, 22, 0>;
impl R {
    #[doc = "Bits 0:21 - SWIE"]
    #[inline(always)]
    pub fn swie(&self) -> SWIE_R {
        SWIE_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - SWIE"]
    #[inline(always)]
    pub fn swie(&mut self) -> SWIE_W {
        SWIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI_SWIE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_swie](index.html) module"]
pub struct EXTI_SWIE_SPEC;
impl crate::RegisterSpec for EXTI_SWIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_swie::R](R) reader structure"]
impl crate::Readable for EXTI_SWIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_swie::W](W) writer structure"]
impl crate::Writable for EXTI_SWIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTI_SWIE to value 0"]
impl crate::Resettable for EXTI_SWIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
