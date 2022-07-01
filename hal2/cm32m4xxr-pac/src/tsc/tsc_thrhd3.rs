#[doc = "Register `TSC_THRHD3` reader"]
pub struct R(crate::R<TSC_THRHD3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSC_THRHD3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSC_THRHD3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSC_THRHD3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSC_THRHD3` writer"]
pub struct W(crate::W<TSC_THRHD3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSC_THRHD3_SPEC>;
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
impl From<crate::W<TSC_THRHD3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSC_THRHD3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASE3` reader - BASE3"]
pub type BASE3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BASE3` writer - BASE3"]
pub type BASE3_W<'a> = crate::FieldWriter<'a, u32, TSC_THRHD3_SPEC, u16, u16, 11, 0>;
#[doc = "Field `DELTA3` reader - DELTA3"]
pub type DELTA3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELTA3` writer - DELTA3"]
pub type DELTA3_W<'a> = crate::FieldWriter<'a, u32, TSC_THRHD3_SPEC, u8, u8, 8, 16>;
impl R {
    #[doc = "Bits 0:10 - BASE3"]
    #[inline(always)]
    pub fn base3(&self) -> BASE3_R {
        BASE3_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:23 - DELTA3"]
    #[inline(always)]
    pub fn delta3(&self) -> DELTA3_R {
        DELTA3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - BASE3"]
    #[inline(always)]
    pub fn base3(&mut self) -> BASE3_W {
        BASE3_W::new(self)
    }
    #[doc = "Bits 16:23 - DELTA3"]
    #[inline(always)]
    pub fn delta3(&mut self) -> DELTA3_W {
        DELTA3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSC_THRHD3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsc_thrhd3](index.html) module"]
pub struct TSC_THRHD3_SPEC;
impl crate::RegisterSpec for TSC_THRHD3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsc_thrhd3::R](R) reader structure"]
impl crate::Readable for TSC_THRHD3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsc_thrhd3::W](W) writer structure"]
impl crate::Writable for TSC_THRHD3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSC_THRHD3 to value 0x0080_0400"]
impl crate::Resettable for TSC_THRHD3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_0400
    }
}
