#[doc = "Register `RNG_DATA` reader"]
pub struct R(crate::R<RNG_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNG_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RNG_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RNG_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RNG_DATA` writer"]
pub struct W(crate::W<RNG_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RNG_DATA_SPEC>;
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
impl From<crate::W<RNG_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RNG_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RNG_DATA` reader - RNG_DATA"]
pub type RNG_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RNG_DATA` writer - RNG_DATA"]
pub type RNG_DATA_W<'a> = crate::FieldWriter<'a, u32, RNG_DATA_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - RNG_DATA"]
    #[inline(always)]
    pub fn rng_data(&self) -> RNG_DATA_R {
        RNG_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RNG_DATA"]
    #[inline(always)]
    pub fn rng_data(&mut self) -> RNG_DATA_W {
        RNG_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RNG_DATA\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_data](index.html) module"]
pub struct RNG_DATA_SPEC;
impl crate::RegisterSpec for RNG_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rng_data::R](R) reader structure"]
impl crate::Readable for RNG_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rng_data::W](W) writer structure"]
impl crate::Writable for RNG_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RNG_DATA to value 0"]
impl crate::Resettable for RNG_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
