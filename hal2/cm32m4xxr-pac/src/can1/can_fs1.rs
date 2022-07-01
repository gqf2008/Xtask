#[doc = "Register `CAN_FS1` reader"]
pub struct R(crate::R<CAN_FS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_FS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAN_FS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAN_FS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_FS1` writer"]
pub struct W(crate::W<CAN_FS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_FS1_SPEC>;
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
impl From<crate::W<CAN_FS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAN_FS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSC` reader - FSC"]
pub type FSC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FSC` writer - FSC"]
pub type FSC_W<'a> = crate::FieldWriter<'a, u32, CAN_FS1_SPEC, u16, u16, 14, 0>;
impl R {
    #[doc = "Bits 0:13 - FSC"]
    #[inline(always)]
    pub fn fsc(&self) -> FSC_R {
        FSC_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - FSC"]
    #[inline(always)]
    pub fn fsc(&mut self) -> FSC_W {
        FSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN_FS1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_fs1](index.html) module"]
pub struct CAN_FS1_SPEC;
impl crate::RegisterSpec for CAN_FS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_fs1::R](R) reader structure"]
impl crate::Readable for CAN_FS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_fs1::W](W) writer structure"]
impl crate::Writable for CAN_FS1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_FS1 to value 0"]
impl crate::Resettable for CAN_FS1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
