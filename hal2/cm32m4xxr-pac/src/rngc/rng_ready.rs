#[doc = "Register `RNG_READY` reader"]
pub struct R(crate::R<RNG_READY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNG_READY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RNG_READY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RNG_READY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RNG_READY` writer"]
pub struct W(crate::W<RNG_READY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RNG_READY_SPEC>;
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
impl From<crate::W<RNG_READY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RNG_READY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RNG_READY` reader - RNG_READY"]
pub type RNG_READY_R = crate::BitReader<bool>;
#[doc = "Field `RNG_READY` writer - RNG_READY"]
pub type RNG_READY_W<'a> = crate::BitWriter<'a, u32, RNG_READY_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - RNG_READY"]
    #[inline(always)]
    pub fn rng_ready(&self) -> RNG_READY_R {
        RNG_READY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RNG_READY"]
    #[inline(always)]
    pub fn rng_ready(&mut self) -> RNG_READY_W {
        RNG_READY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RNG_READY\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_ready](index.html) module"]
pub struct RNG_READY_SPEC;
impl crate::RegisterSpec for RNG_READY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rng_ready::R](R) reader structure"]
impl crate::Readable for RNG_READY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rng_ready::W](W) writer structure"]
impl crate::Writable for RNG_READY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RNG_READY to value 0"]
impl crate::Resettable for RNG_READY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
