#[doc = "Register `TSC_THRHD0` reader"]
pub struct R(crate::R<TSC_THRHD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSC_THRHD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSC_THRHD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSC_THRHD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSC_THRHD0` writer"]
pub struct W(crate::W<TSC_THRHD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSC_THRHD0_SPEC>;
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
impl From<crate::W<TSC_THRHD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSC_THRHD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASE0` reader - BASE0"]
pub type BASE0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BASE0` writer - BASE0"]
pub type BASE0_W<'a> = crate::FieldWriter<'a, u32, TSC_THRHD0_SPEC, u16, u16, 11, 0>;
#[doc = "Field `DELTA0` reader - DELTA0"]
pub type DELTA0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELTA0` writer - DELTA0"]
pub type DELTA0_W<'a> = crate::FieldWriter<'a, u32, TSC_THRHD0_SPEC, u8, u8, 8, 16>;
impl R {
    #[doc = "Bits 0:10 - BASE0"]
    #[inline(always)]
    pub fn base0(&self) -> BASE0_R {
        BASE0_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:23 - DELTA0"]
    #[inline(always)]
    pub fn delta0(&self) -> DELTA0_R {
        DELTA0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - BASE0"]
    #[inline(always)]
    pub fn base0(&mut self) -> BASE0_W {
        BASE0_W::new(self)
    }
    #[doc = "Bits 16:23 - DELTA0"]
    #[inline(always)]
    pub fn delta0(&mut self) -> DELTA0_W {
        DELTA0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSC_THRHD0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsc_thrhd0](index.html) module"]
pub struct TSC_THRHD0_SPEC;
impl crate::RegisterSpec for TSC_THRHD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsc_thrhd0::R](R) reader structure"]
impl crate::Readable for TSC_THRHD0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsc_thrhd0::W](W) writer structure"]
impl crate::Writable for TSC_THRHD0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSC_THRHD0 to value 0x0080_0400"]
impl crate::Resettable for TSC_THRHD0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_0400
    }
}
