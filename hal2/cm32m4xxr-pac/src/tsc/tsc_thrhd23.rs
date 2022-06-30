#[doc = "Register `TSC_THRHD23` reader"]
pub struct R(crate::R<TSC_THRHD23_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSC_THRHD23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSC_THRHD23_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSC_THRHD23_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSC_THRHD23` writer"]
pub struct W(crate::W<TSC_THRHD23_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSC_THRHD23_SPEC>;
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
impl From<crate::W<TSC_THRHD23_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSC_THRHD23_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASE23` reader - BASE23"]
pub type BASE23_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BASE23` writer - BASE23"]
pub type BASE23_W<'a> = crate::FieldWriter<'a, u32, TSC_THRHD23_SPEC, u16, u16, 11, 0>;
#[doc = "Field `DELTA23` reader - DELTA23"]
pub type DELTA23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELTA23` writer - DELTA23"]
pub type DELTA23_W<'a> = crate::FieldWriter<'a, u32, TSC_THRHD23_SPEC, u8, u8, 8, 16>;
impl R {
    #[doc = "Bits 0:10 - BASE23"]
    #[inline(always)]
    pub fn base23(&self) -> BASE23_R {
        BASE23_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:23 - DELTA23"]
    #[inline(always)]
    pub fn delta23(&self) -> DELTA23_R {
        DELTA23_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - BASE23"]
    #[inline(always)]
    pub fn base23(&mut self) -> BASE23_W {
        BASE23_W::new(self)
    }
    #[doc = "Bits 16:23 - DELTA23"]
    #[inline(always)]
    pub fn delta23(&mut self) -> DELTA23_W {
        DELTA23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSC_THRHD23\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsc_thrhd23](index.html) module"]
pub struct TSC_THRHD23_SPEC;
impl crate::RegisterSpec for TSC_THRHD23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsc_thrhd23::R](R) reader structure"]
impl crate::Readable for TSC_THRHD23_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsc_thrhd23::W](W) writer structure"]
impl crate::Writable for TSC_THRHD23_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSC_THRHD23 to value 0x0080_0400"]
impl crate::Resettable for TSC_THRHD23_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_0400
    }
}
