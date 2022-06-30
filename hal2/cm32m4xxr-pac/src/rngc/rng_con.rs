#[doc = "Register `RNG_CON` reader"]
pub struct R(crate::R<RNG_CON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNG_CON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RNG_CON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RNG_CON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RNG_CON` writer"]
pub struct W(crate::W<RNG_CON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RNG_CON_SPEC>;
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
impl From<crate::W<RNG_CON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RNG_CON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RING0_EN` reader - RING0_EN"]
pub type RING0_EN_R = crate::BitReader<bool>;
#[doc = "Field `RING0_EN` writer - RING0_EN"]
pub type RING0_EN_W<'a> = crate::BitWriter<'a, u32, RNG_CON_SPEC, bool, 0>;
#[doc = "Field `RING1_EN` reader - RING1_EN"]
pub type RING1_EN_R = crate::BitReader<bool>;
#[doc = "Field `RING1_EN` writer - RING1_EN"]
pub type RING1_EN_W<'a> = crate::BitWriter<'a, u32, RNG_CON_SPEC, bool, 1>;
#[doc = "Field `RNG_MODE` reader - RNG_MODE"]
pub type RNG_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RNG_MODE` writer - RNG_MODE"]
pub type RNG_MODE_W<'a> = crate::FieldWriter<'a, u32, RNG_CON_SPEC, u8, u8, 2, 5>;
impl R {
    #[doc = "Bit 0 - RING0_EN"]
    #[inline(always)]
    pub fn ring0_en(&self) -> RING0_EN_R {
        RING0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RING1_EN"]
    #[inline(always)]
    pub fn ring1_en(&self) -> RING1_EN_R {
        RING1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 5:6 - RNG_MODE"]
    #[inline(always)]
    pub fn rng_mode(&self) -> RNG_MODE_R {
        RNG_MODE_R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RING0_EN"]
    #[inline(always)]
    pub fn ring0_en(&mut self) -> RING0_EN_W {
        RING0_EN_W::new(self)
    }
    #[doc = "Bit 1 - RING1_EN"]
    #[inline(always)]
    pub fn ring1_en(&mut self) -> RING1_EN_W {
        RING1_EN_W::new(self)
    }
    #[doc = "Bits 5:6 - RNG_MODE"]
    #[inline(always)]
    pub fn rng_mode(&mut self) -> RNG_MODE_W {
        RNG_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RNG_CON\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_con](index.html) module"]
pub struct RNG_CON_SPEC;
impl crate::RegisterSpec for RNG_CON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rng_con::R](R) reader structure"]
impl crate::Readable for RNG_CON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rng_con::W](W) writer structure"]
impl crate::Writable for RNG_CON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RNG_CON to value 0"]
impl crate::Resettable for RNG_CON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
