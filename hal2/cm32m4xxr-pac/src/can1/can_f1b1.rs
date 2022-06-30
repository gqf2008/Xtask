#[doc = "Register `CAN_F1B1` reader"]
pub struct R(crate::R<CAN_F1B1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_F1B1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAN_F1B1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAN_F1B1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_F1B1` writer"]
pub struct W(crate::W<CAN_F1B1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_F1B1_SPEC>;
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
impl From<crate::W<CAN_F1B1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAN_F1B1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FBC` reader - FBC"]
pub type FBC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FBC` writer - FBC"]
pub type FBC_W<'a> = crate::FieldWriter<'a, u32, CAN_F1B1_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - FBC"]
    #[inline(always)]
    pub fn fbc(&self) -> FBC_R {
        FBC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FBC"]
    #[inline(always)]
    pub fn fbc(&mut self) -> FBC_W {
        FBC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN_F1B1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_f1b1](index.html) module"]
pub struct CAN_F1B1_SPEC;
impl crate::RegisterSpec for CAN_F1B1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_f1b1::R](R) reader structure"]
impl crate::Readable for CAN_F1B1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_f1b1::W](W) writer structure"]
impl crate::Writable for CAN_F1B1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_F1B1 to value 0"]
impl crate::Resettable for CAN_F1B1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
