#[doc = "Register `EXTI_IMASK` reader"]
pub struct R(crate::R<EXTI_IMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_IMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_IMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_IMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_IMASK` writer"]
pub struct W(crate::W<EXTI_IMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_IMASK_SPEC>;
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
impl From<crate::W<EXTI_IMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_IMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IMASK` reader - IMASK"]
pub type IMASK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IMASK` writer - IMASK"]
pub type IMASK_W<'a> = crate::FieldWriter<'a, u32, EXTI_IMASK_SPEC, u32, u32, 22, 0>;
impl R {
    #[doc = "Bits 0:21 - IMASK"]
    #[inline(always)]
    pub fn imask(&self) -> IMASK_R {
        IMASK_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - IMASK"]
    #[inline(always)]
    pub fn imask(&mut self) -> IMASK_W {
        IMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI_IMASK\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_imask](index.html) module"]
pub struct EXTI_IMASK_SPEC;
impl crate::RegisterSpec for EXTI_IMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_imask::R](R) reader structure"]
impl crate::Readable for EXTI_IMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_imask::W](W) writer structure"]
impl crate::Writable for EXTI_IMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTI_IMASK to value 0"]
impl crate::Resettable for EXTI_IMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
