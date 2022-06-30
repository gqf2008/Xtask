#[doc = "Register `TSC_THRHD5` reader"]
pub struct R(crate::R<TSC_THRHD5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSC_THRHD5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSC_THRHD5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSC_THRHD5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSC_THRHD5` writer"]
pub struct W(crate::W<TSC_THRHD5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSC_THRHD5_SPEC>;
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
impl From<crate::W<TSC_THRHD5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSC_THRHD5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASE5` reader - BASE5"]
pub type BASE5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BASE5` writer - BASE5"]
pub type BASE5_W<'a> = crate::FieldWriter<'a, u32, TSC_THRHD5_SPEC, u16, u16, 11, 0>;
#[doc = "Field `DELTA5` reader - DELTA5"]
pub type DELTA5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELTA5` writer - DELTA5"]
pub type DELTA5_W<'a> = crate::FieldWriter<'a, u32, TSC_THRHD5_SPEC, u8, u8, 8, 16>;
impl R {
    #[doc = "Bits 0:10 - BASE5"]
    #[inline(always)]
    pub fn base5(&self) -> BASE5_R {
        BASE5_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:23 - DELTA5"]
    #[inline(always)]
    pub fn delta5(&self) -> DELTA5_R {
        DELTA5_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - BASE5"]
    #[inline(always)]
    pub fn base5(&mut self) -> BASE5_W {
        BASE5_W::new(self)
    }
    #[doc = "Bits 16:23 - DELTA5"]
    #[inline(always)]
    pub fn delta5(&mut self) -> DELTA5_W {
        DELTA5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSC_THRHD5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsc_thrhd5](index.html) module"]
pub struct TSC_THRHD5_SPEC;
impl crate::RegisterSpec for TSC_THRHD5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsc_thrhd5::R](R) reader structure"]
impl crate::Readable for TSC_THRHD5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsc_thrhd5::W](W) writer structure"]
impl crate::Writable for TSC_THRHD5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSC_THRHD5 to value 0x0080_0400"]
impl crate::Resettable for TSC_THRHD5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_0400
    }
}
