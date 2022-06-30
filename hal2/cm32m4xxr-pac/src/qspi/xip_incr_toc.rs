#[doc = "Register `XIP_INCR_TOC` reader"]
pub struct R(crate::R<XIP_INCR_TOC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XIP_INCR_TOC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XIP_INCR_TOC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XIP_INCR_TOC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XIP_INCR_TOC` writer"]
pub struct W(crate::W<XIP_INCR_TOC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XIP_INCR_TOC_SPEC>;
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
impl From<crate::W<XIP_INCR_TOC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XIP_INCR_TOC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITOC` reader - ITOC"]
pub type ITOC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ITOC` writer - ITOC"]
pub type ITOC_W<'a> = crate::FieldWriter<'a, u32, XIP_INCR_TOC_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - ITOC"]
    #[inline(always)]
    pub fn itoc(&self) -> ITOC_R {
        ITOC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ITOC"]
    #[inline(always)]
    pub fn itoc(&mut self) -> ITOC_W {
        ITOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XIP_INCR_TOC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xip_incr_toc](index.html) module"]
pub struct XIP_INCR_TOC_SPEC;
impl crate::RegisterSpec for XIP_INCR_TOC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xip_incr_toc::R](R) reader structure"]
impl crate::Readable for XIP_INCR_TOC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xip_incr_toc::W](W) writer structure"]
impl crate::Writable for XIP_INCR_TOC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XIP_INCR_TOC to value 0"]
impl crate::Resettable for XIP_INCR_TOC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
