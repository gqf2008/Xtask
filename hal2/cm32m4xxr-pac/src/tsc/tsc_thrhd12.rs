#[doc = "Register `TSC_THRHD12` reader"]
pub struct R(crate::R<TSC_THRHD12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSC_THRHD12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSC_THRHD12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSC_THRHD12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSC_THRHD12` writer"]
pub struct W(crate::W<TSC_THRHD12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSC_THRHD12_SPEC>;
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
impl From<crate::W<TSC_THRHD12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSC_THRHD12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASE12` reader - BASE12"]
pub type BASE12_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BASE12` writer - BASE12"]
pub type BASE12_W<'a> = crate::FieldWriter<'a, u32, TSC_THRHD12_SPEC, u16, u16, 11, 0>;
#[doc = "Field `DELTA12` reader - DELTA12"]
pub type DELTA12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELTA12` writer - DELTA12"]
pub type DELTA12_W<'a> = crate::FieldWriter<'a, u32, TSC_THRHD12_SPEC, u8, u8, 8, 16>;
impl R {
    #[doc = "Bits 0:10 - BASE12"]
    #[inline(always)]
    pub fn base12(&self) -> BASE12_R {
        BASE12_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:23 - DELTA12"]
    #[inline(always)]
    pub fn delta12(&self) -> DELTA12_R {
        DELTA12_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - BASE12"]
    #[inline(always)]
    pub fn base12(&mut self) -> BASE12_W {
        BASE12_W::new(self)
    }
    #[doc = "Bits 16:23 - DELTA12"]
    #[inline(always)]
    pub fn delta12(&mut self) -> DELTA12_W {
        DELTA12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSC_THRHD12\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsc_thrhd12](index.html) module"]
pub struct TSC_THRHD12_SPEC;
impl crate::RegisterSpec for TSC_THRHD12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsc_thrhd12::R](R) reader structure"]
impl crate::Readable for TSC_THRHD12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsc_thrhd12::W](W) writer structure"]
impl crate::Writable for TSC_THRHD12_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSC_THRHD12 to value 0x0080_0400"]
impl crate::Resettable for TSC_THRHD12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_0400
    }
}
