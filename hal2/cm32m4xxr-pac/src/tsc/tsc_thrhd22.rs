#[doc = "Register `TSC_THRHD22` reader"]
pub struct R(crate::R<TSC_THRHD22_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSC_THRHD22_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSC_THRHD22_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSC_THRHD22_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSC_THRHD22` writer"]
pub struct W(crate::W<TSC_THRHD22_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSC_THRHD22_SPEC>;
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
impl From<crate::W<TSC_THRHD22_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSC_THRHD22_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASE22` reader - BASE22"]
pub type BASE22_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BASE22` writer - BASE22"]
pub type BASE22_W<'a> = crate::FieldWriter<'a, u32, TSC_THRHD22_SPEC, u16, u16, 11, 0>;
#[doc = "Field `DELTA22` reader - DELTA22"]
pub type DELTA22_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELTA22` writer - DELTA22"]
pub type DELTA22_W<'a> = crate::FieldWriter<'a, u32, TSC_THRHD22_SPEC, u8, u8, 8, 16>;
impl R {
    #[doc = "Bits 0:10 - BASE22"]
    #[inline(always)]
    pub fn base22(&self) -> BASE22_R {
        BASE22_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:23 - DELTA22"]
    #[inline(always)]
    pub fn delta22(&self) -> DELTA22_R {
        DELTA22_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - BASE22"]
    #[inline(always)]
    pub fn base22(&mut self) -> BASE22_W {
        BASE22_W::new(self)
    }
    #[doc = "Bits 16:23 - DELTA22"]
    #[inline(always)]
    pub fn delta22(&mut self) -> DELTA22_W {
        DELTA22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSC_THRHD22\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsc_thrhd22](index.html) module"]
pub struct TSC_THRHD22_SPEC;
impl crate::RegisterSpec for TSC_THRHD22_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsc_thrhd22::R](R) reader structure"]
impl crate::Readable for TSC_THRHD22_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsc_thrhd22::W](W) writer structure"]
impl crate::Writable for TSC_THRHD22_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSC_THRHD22 to value 0x0080_0400"]
impl crate::Resettable for TSC_THRHD22_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_0400
    }
}
