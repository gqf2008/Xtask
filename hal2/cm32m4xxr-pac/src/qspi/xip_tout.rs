#[doc = "Register `XIP_TOUT` reader"]
pub struct R(crate::R<XIP_TOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XIP_TOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XIP_TOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XIP_TOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XIP_TOUT` writer"]
pub struct W(crate::W<XIP_TOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XIP_TOUT_SPEC>;
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
impl From<crate::W<XIP_TOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XIP_TOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTOUT` reader - XTOUT"]
pub type XTOUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XTOUT` writer - XTOUT"]
pub type XTOUT_W<'a> = crate::FieldWriter<'a, u32, XIP_TOUT_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - XTOUT"]
    #[inline(always)]
    pub fn xtout(&self) -> XTOUT_R {
        XTOUT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - XTOUT"]
    #[inline(always)]
    pub fn xtout(&mut self) -> XTOUT_W {
        XTOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XIP_TOUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xip_tout](index.html) module"]
pub struct XIP_TOUT_SPEC;
impl crate::RegisterSpec for XIP_TOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xip_tout::R](R) reader structure"]
impl crate::Readable for XIP_TOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xip_tout::W](W) writer structure"]
impl crate::Writable for XIP_TOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XIP_TOUT to value 0"]
impl crate::Resettable for XIP_TOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
