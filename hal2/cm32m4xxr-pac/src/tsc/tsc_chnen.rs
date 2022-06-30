#[doc = "Register `TSC_CHNEN` reader"]
pub struct R(crate::R<TSC_CHNEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSC_CHNEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSC_CHNEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSC_CHNEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSC_CHNEN` writer"]
pub struct W(crate::W<TSC_CHNEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSC_CHNEN_SPEC>;
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
impl From<crate::W<TSC_CHNEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSC_CHNEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHN_SEL` reader - CHN_SEL"]
pub type CHN_SEL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CHN_SEL` writer - CHN_SEL"]
pub type CHN_SEL_W<'a> = crate::FieldWriter<'a, u32, TSC_CHNEN_SPEC, u32, u32, 24, 0>;
impl R {
    #[doc = "Bits 0:23 - CHN_SEL"]
    #[inline(always)]
    pub fn chn_sel(&self) -> CHN_SEL_R {
        CHN_SEL_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - CHN_SEL"]
    #[inline(always)]
    pub fn chn_sel(&mut self) -> CHN_SEL_W {
        CHN_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSC_CHNEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsc_chnen](index.html) module"]
pub struct TSC_CHNEN_SPEC;
impl crate::RegisterSpec for TSC_CHNEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsc_chnen::R](R) reader structure"]
impl crate::Readable for TSC_CHNEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsc_chnen::W](W) writer structure"]
impl crate::Writable for TSC_CHNEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSC_CHNEN to value 0"]
impl crate::Resettable for TSC_CHNEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
