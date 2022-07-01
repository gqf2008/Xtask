#[doc = "Register `XIP_MODE` reader"]
pub struct R(crate::R<XIP_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XIP_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XIP_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XIP_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XIP_MODE` writer"]
pub struct W(crate::W<XIP_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XIP_MODE_SPEC>;
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
impl From<crate::W<XIP_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XIP_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XIP_MD_BITS` reader - XIP_MD_BITS"]
pub type XIP_MD_BITS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `XIP_MD_BITS` writer - XIP_MD_BITS"]
pub type XIP_MD_BITS_W<'a> = crate::FieldWriter<'a, u32, XIP_MODE_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - XIP_MD_BITS"]
    #[inline(always)]
    pub fn xip_md_bits(&self) -> XIP_MD_BITS_R {
        XIP_MD_BITS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - XIP_MD_BITS"]
    #[inline(always)]
    pub fn xip_md_bits(&mut self) -> XIP_MD_BITS_W {
        XIP_MD_BITS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XIP_MODE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xip_mode](index.html) module"]
pub struct XIP_MODE_SPEC;
impl crate::RegisterSpec for XIP_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xip_mode::R](R) reader structure"]
impl crate::Readable for XIP_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xip_mode::W](W) writer structure"]
impl crate::Writable for XIP_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XIP_MODE to value 0"]
impl crate::Resettable for XIP_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
