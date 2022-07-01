#[doc = "Register `TSC_THRHD1` reader"]
pub struct R(crate::R<TSC_THRHD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSC_THRHD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSC_THRHD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSC_THRHD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSC_THRHD1` writer"]
pub struct W(crate::W<TSC_THRHD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSC_THRHD1_SPEC>;
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
impl From<crate::W<TSC_THRHD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSC_THRHD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASE1` reader - BASE1"]
pub type BASE1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BASE1` writer - BASE1"]
pub type BASE1_W<'a> = crate::FieldWriter<'a, u32, TSC_THRHD1_SPEC, u16, u16, 11, 0>;
#[doc = "Field `DELTA1` reader - DELTA1"]
pub type DELTA1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELTA1` writer - DELTA1"]
pub type DELTA1_W<'a> = crate::FieldWriter<'a, u32, TSC_THRHD1_SPEC, u8, u8, 8, 16>;
impl R {
    #[doc = "Bits 0:10 - BASE1"]
    #[inline(always)]
    pub fn base1(&self) -> BASE1_R {
        BASE1_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:23 - DELTA1"]
    #[inline(always)]
    pub fn delta1(&self) -> DELTA1_R {
        DELTA1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - BASE1"]
    #[inline(always)]
    pub fn base1(&mut self) -> BASE1_W {
        BASE1_W::new(self)
    }
    #[doc = "Bits 16:23 - DELTA1"]
    #[inline(always)]
    pub fn delta1(&mut self) -> DELTA1_W {
        DELTA1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSC_THRHD1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsc_thrhd1](index.html) module"]
pub struct TSC_THRHD1_SPEC;
impl crate::RegisterSpec for TSC_THRHD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsc_thrhd1::R](R) reader structure"]
impl crate::Readable for TSC_THRHD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsc_thrhd1::W](W) writer structure"]
impl crate::Writable for TSC_THRHD1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSC_THRHD1 to value 0x0080_0400"]
impl crate::Resettable for TSC_THRHD1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_0400
    }
}
