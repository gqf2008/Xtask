#[doc = "Register `TSC_THRHD9` reader"]
pub struct R(crate::R<TSC_THRHD9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSC_THRHD9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSC_THRHD9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSC_THRHD9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSC_THRHD9` writer"]
pub struct W(crate::W<TSC_THRHD9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSC_THRHD9_SPEC>;
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
impl From<crate::W<TSC_THRHD9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSC_THRHD9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASE9` reader - BASE9"]
pub type BASE9_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BASE9` writer - BASE9"]
pub type BASE9_W<'a> = crate::FieldWriter<'a, u32, TSC_THRHD9_SPEC, u16, u16, 11, 0>;
#[doc = "Field `DELTA9` reader - DELTA9"]
pub type DELTA9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELTA9` writer - DELTA9"]
pub type DELTA9_W<'a> = crate::FieldWriter<'a, u32, TSC_THRHD9_SPEC, u8, u8, 8, 16>;
impl R {
    #[doc = "Bits 0:10 - BASE9"]
    #[inline(always)]
    pub fn base9(&self) -> BASE9_R {
        BASE9_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:23 - DELTA9"]
    #[inline(always)]
    pub fn delta9(&self) -> DELTA9_R {
        DELTA9_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - BASE9"]
    #[inline(always)]
    pub fn base9(&mut self) -> BASE9_W {
        BASE9_W::new(self)
    }
    #[doc = "Bits 16:23 - DELTA9"]
    #[inline(always)]
    pub fn delta9(&mut self) -> DELTA9_W {
        DELTA9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSC_THRHD9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsc_thrhd9](index.html) module"]
pub struct TSC_THRHD9_SPEC;
impl crate::RegisterSpec for TSC_THRHD9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsc_thrhd9::R](R) reader structure"]
impl crate::Readable for TSC_THRHD9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsc_thrhd9::W](W) writer structure"]
impl crate::Writable for TSC_THRHD9_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSC_THRHD9 to value 0x0080_0400"]
impl crate::Resettable for TSC_THRHD9_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_0400
    }
}
