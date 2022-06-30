#[doc = "Register `TSC_THRHD17` reader"]
pub struct R(crate::R<TSC_THRHD17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSC_THRHD17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSC_THRHD17_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSC_THRHD17_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSC_THRHD17` writer"]
pub struct W(crate::W<TSC_THRHD17_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSC_THRHD17_SPEC>;
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
impl From<crate::W<TSC_THRHD17_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSC_THRHD17_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASE17` reader - BASE17"]
pub type BASE17_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BASE17` writer - BASE17"]
pub type BASE17_W<'a> = crate::FieldWriter<'a, u32, TSC_THRHD17_SPEC, u16, u16, 11, 0>;
#[doc = "Field `DELTA17` reader - DELTA17"]
pub type DELTA17_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELTA17` writer - DELTA17"]
pub type DELTA17_W<'a> = crate::FieldWriter<'a, u32, TSC_THRHD17_SPEC, u8, u8, 8, 16>;
impl R {
    #[doc = "Bits 0:10 - BASE17"]
    #[inline(always)]
    pub fn base17(&self) -> BASE17_R {
        BASE17_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:23 - DELTA17"]
    #[inline(always)]
    pub fn delta17(&self) -> DELTA17_R {
        DELTA17_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - BASE17"]
    #[inline(always)]
    pub fn base17(&mut self) -> BASE17_W {
        BASE17_W::new(self)
    }
    #[doc = "Bits 16:23 - DELTA17"]
    #[inline(always)]
    pub fn delta17(&mut self) -> DELTA17_W {
        DELTA17_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSC_THRHD17\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsc_thrhd17](index.html) module"]
pub struct TSC_THRHD17_SPEC;
impl crate::RegisterSpec for TSC_THRHD17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsc_thrhd17::R](R) reader structure"]
impl crate::Readable for TSC_THRHD17_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsc_thrhd17::W](W) writer structure"]
impl crate::Writable for TSC_THRHD17_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSC_THRHD17 to value 0x0080_0400"]
impl crate::Resettable for TSC_THRHD17_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_0400
    }
}
