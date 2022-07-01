#[doc = "Register `RCC_CLKINT` reader"]
pub struct R(crate::R<RCC_CLKINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CLKINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CLKINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CLKINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_CLKINT` writer"]
pub struct W(crate::W<RCC_CLKINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CLKINT_SPEC>;
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
impl From<crate::W<RCC_CLKINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CLKINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSIRDIF` reader - LSIRDIF"]
pub type LSIRDIF_R = crate::BitReader<bool>;
#[doc = "Field `LSIRDIF` writer - LSIRDIF"]
pub type LSIRDIF_W<'a> = crate::BitWriter<'a, u32, RCC_CLKINT_SPEC, bool, 0>;
#[doc = "Field `LSERDIF` reader - LSERDIF"]
pub type LSERDIF_R = crate::BitReader<bool>;
#[doc = "Field `LSERDIF` writer - LSERDIF"]
pub type LSERDIF_W<'a> = crate::BitWriter<'a, u32, RCC_CLKINT_SPEC, bool, 1>;
#[doc = "Field `HSIRDIF` reader - HSIRDIF"]
pub type HSIRDIF_R = crate::BitReader<bool>;
#[doc = "Field `HSIRDIF` writer - HSIRDIF"]
pub type HSIRDIF_W<'a> = crate::BitWriter<'a, u32, RCC_CLKINT_SPEC, bool, 2>;
#[doc = "Field `HSERDIF` reader - HSERDIF"]
pub type HSERDIF_R = crate::BitReader<bool>;
#[doc = "Field `HSERDIF` writer - HSERDIF"]
pub type HSERDIF_W<'a> = crate::BitWriter<'a, u32, RCC_CLKINT_SPEC, bool, 3>;
#[doc = "Field `PLLRDIF` reader - PLLRDIF"]
pub type PLLRDIF_R = crate::BitReader<bool>;
#[doc = "Field `PLLRDIF` writer - PLLRDIF"]
pub type PLLRDIF_W<'a> = crate::BitWriter<'a, u32, RCC_CLKINT_SPEC, bool, 4>;
#[doc = "Field `CLKSSIF` reader - CLKSSIF"]
pub type CLKSSIF_R = crate::BitReader<bool>;
#[doc = "Field `CLKSSIF` writer - CLKSSIF"]
pub type CLKSSIF_W<'a> = crate::BitWriter<'a, u32, RCC_CLKINT_SPEC, bool, 7>;
#[doc = "Field `LSIRDIEN` reader - LSIRDIEN"]
pub type LSIRDIEN_R = crate::BitReader<bool>;
#[doc = "Field `LSIRDIEN` writer - LSIRDIEN"]
pub type LSIRDIEN_W<'a> = crate::BitWriter<'a, u32, RCC_CLKINT_SPEC, bool, 8>;
#[doc = "Field `LSERDIEN` reader - LSERDIEN"]
pub type LSERDIEN_R = crate::BitReader<bool>;
#[doc = "Field `LSERDIEN` writer - LSERDIEN"]
pub type LSERDIEN_W<'a> = crate::BitWriter<'a, u32, RCC_CLKINT_SPEC, bool, 9>;
#[doc = "Field `HSIRDIEN` reader - HSIRDIEN"]
pub type HSIRDIEN_R = crate::BitReader<bool>;
#[doc = "Field `HSIRDIEN` writer - HSIRDIEN"]
pub type HSIRDIEN_W<'a> = crate::BitWriter<'a, u32, RCC_CLKINT_SPEC, bool, 10>;
#[doc = "Field `HSERDIEN` reader - HSERDIEN"]
pub type HSERDIEN_R = crate::BitReader<bool>;
#[doc = "Field `HSERDIEN` writer - HSERDIEN"]
pub type HSERDIEN_W<'a> = crate::BitWriter<'a, u32, RCC_CLKINT_SPEC, bool, 11>;
#[doc = "Field `PLLRDIEN` reader - PLLRDIEN"]
pub type PLLRDIEN_R = crate::BitReader<bool>;
#[doc = "Field `PLLRDIEN` writer - PLLRDIEN"]
pub type PLLRDIEN_W<'a> = crate::BitWriter<'a, u32, RCC_CLKINT_SPEC, bool, 12>;
#[doc = "Field `LSIRDICLR` reader - LSIRDICLR"]
pub type LSIRDICLR_R = crate::BitReader<bool>;
#[doc = "Field `LSIRDICLR` writer - LSIRDICLR"]
pub type LSIRDICLR_W<'a> = crate::BitWriter<'a, u32, RCC_CLKINT_SPEC, bool, 16>;
#[doc = "Field `LSERDICLR` reader - LSERDICLR"]
pub type LSERDICLR_R = crate::BitReader<bool>;
#[doc = "Field `LSERDICLR` writer - LSERDICLR"]
pub type LSERDICLR_W<'a> = crate::BitWriter<'a, u32, RCC_CLKINT_SPEC, bool, 17>;
#[doc = "Field `HSIRDICLR` reader - HSIRDICLR"]
pub type HSIRDICLR_R = crate::BitReader<bool>;
#[doc = "Field `HSIRDICLR` writer - HSIRDICLR"]
pub type HSIRDICLR_W<'a> = crate::BitWriter<'a, u32, RCC_CLKINT_SPEC, bool, 18>;
#[doc = "Field `HSERDICLR` reader - HSERDICLR"]
pub type HSERDICLR_R = crate::BitReader<bool>;
#[doc = "Field `HSERDICLR` writer - HSERDICLR"]
pub type HSERDICLR_W<'a> = crate::BitWriter<'a, u32, RCC_CLKINT_SPEC, bool, 19>;
#[doc = "Field `PLLRDICLR` reader - PLLRDICLR"]
pub type PLLRDICLR_R = crate::BitReader<bool>;
#[doc = "Field `PLLRDICLR` writer - PLLRDICLR"]
pub type PLLRDICLR_W<'a> = crate::BitWriter<'a, u32, RCC_CLKINT_SPEC, bool, 20>;
#[doc = "Field `CLKSSICLR` reader - CLKSSICLR"]
pub type CLKSSICLR_R = crate::BitReader<bool>;
#[doc = "Field `CLKSSICLR` writer - CLKSSICLR"]
pub type CLKSSICLR_W<'a> = crate::BitWriter<'a, u32, RCC_CLKINT_SPEC, bool, 23>;
impl R {
    #[doc = "Bit 0 - LSIRDIF"]
    #[inline(always)]
    pub fn lsirdif(&self) -> LSIRDIF_R {
        LSIRDIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSERDIF"]
    #[inline(always)]
    pub fn lserdif(&self) -> LSERDIF_R {
        LSERDIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSIRDIF"]
    #[inline(always)]
    pub fn hsirdif(&self) -> HSIRDIF_R {
        HSIRDIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSERDIF"]
    #[inline(always)]
    pub fn hserdif(&self) -> HSERDIF_R {
        HSERDIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLLRDIF"]
    #[inline(always)]
    pub fn pllrdif(&self) -> PLLRDIF_R {
        PLLRDIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - CLKSSIF"]
    #[inline(always)]
    pub fn clkssif(&self) -> CLKSSIF_R {
        CLKSSIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LSIRDIEN"]
    #[inline(always)]
    pub fn lsirdien(&self) -> LSIRDIEN_R {
        LSIRDIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSERDIEN"]
    #[inline(always)]
    pub fn lserdien(&self) -> LSERDIEN_R {
        LSERDIEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSIRDIEN"]
    #[inline(always)]
    pub fn hsirdien(&self) -> HSIRDIEN_R {
        HSIRDIEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSERDIEN"]
    #[inline(always)]
    pub fn hserdien(&self) -> HSERDIEN_R {
        HSERDIEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PLLRDIEN"]
    #[inline(always)]
    pub fn pllrdien(&self) -> PLLRDIEN_R {
        PLLRDIEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - LSIRDICLR"]
    #[inline(always)]
    pub fn lsirdiclr(&self) -> LSIRDICLR_R {
        LSIRDICLR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - LSERDICLR"]
    #[inline(always)]
    pub fn lserdiclr(&self) -> LSERDICLR_R {
        LSERDICLR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSIRDICLR"]
    #[inline(always)]
    pub fn hsirdiclr(&self) -> HSIRDICLR_R {
        HSIRDICLR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HSERDICLR"]
    #[inline(always)]
    pub fn hserdiclr(&self) -> HSERDICLR_R {
        HSERDICLR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PLLRDICLR"]
    #[inline(always)]
    pub fn pllrdiclr(&self) -> PLLRDICLR_R {
        PLLRDICLR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 23 - CLKSSICLR"]
    #[inline(always)]
    pub fn clkssiclr(&self) -> CLKSSICLR_R {
        CLKSSICLR_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSIRDIF"]
    #[inline(always)]
    pub fn lsirdif(&mut self) -> LSIRDIF_W {
        LSIRDIF_W::new(self)
    }
    #[doc = "Bit 1 - LSERDIF"]
    #[inline(always)]
    pub fn lserdif(&mut self) -> LSERDIF_W {
        LSERDIF_W::new(self)
    }
    #[doc = "Bit 2 - HSIRDIF"]
    #[inline(always)]
    pub fn hsirdif(&mut self) -> HSIRDIF_W {
        HSIRDIF_W::new(self)
    }
    #[doc = "Bit 3 - HSERDIF"]
    #[inline(always)]
    pub fn hserdif(&mut self) -> HSERDIF_W {
        HSERDIF_W::new(self)
    }
    #[doc = "Bit 4 - PLLRDIF"]
    #[inline(always)]
    pub fn pllrdif(&mut self) -> PLLRDIF_W {
        PLLRDIF_W::new(self)
    }
    #[doc = "Bit 7 - CLKSSIF"]
    #[inline(always)]
    pub fn clkssif(&mut self) -> CLKSSIF_W {
        CLKSSIF_W::new(self)
    }
    #[doc = "Bit 8 - LSIRDIEN"]
    #[inline(always)]
    pub fn lsirdien(&mut self) -> LSIRDIEN_W {
        LSIRDIEN_W::new(self)
    }
    #[doc = "Bit 9 - LSERDIEN"]
    #[inline(always)]
    pub fn lserdien(&mut self) -> LSERDIEN_W {
        LSERDIEN_W::new(self)
    }
    #[doc = "Bit 10 - HSIRDIEN"]
    #[inline(always)]
    pub fn hsirdien(&mut self) -> HSIRDIEN_W {
        HSIRDIEN_W::new(self)
    }
    #[doc = "Bit 11 - HSERDIEN"]
    #[inline(always)]
    pub fn hserdien(&mut self) -> HSERDIEN_W {
        HSERDIEN_W::new(self)
    }
    #[doc = "Bit 12 - PLLRDIEN"]
    #[inline(always)]
    pub fn pllrdien(&mut self) -> PLLRDIEN_W {
        PLLRDIEN_W::new(self)
    }
    #[doc = "Bit 16 - LSIRDICLR"]
    #[inline(always)]
    pub fn lsirdiclr(&mut self) -> LSIRDICLR_W {
        LSIRDICLR_W::new(self)
    }
    #[doc = "Bit 17 - LSERDICLR"]
    #[inline(always)]
    pub fn lserdiclr(&mut self) -> LSERDICLR_W {
        LSERDICLR_W::new(self)
    }
    #[doc = "Bit 18 - HSIRDICLR"]
    #[inline(always)]
    pub fn hsirdiclr(&mut self) -> HSIRDICLR_W {
        HSIRDICLR_W::new(self)
    }
    #[doc = "Bit 19 - HSERDICLR"]
    #[inline(always)]
    pub fn hserdiclr(&mut self) -> HSERDICLR_W {
        HSERDICLR_W::new(self)
    }
    #[doc = "Bit 20 - PLLRDICLR"]
    #[inline(always)]
    pub fn pllrdiclr(&mut self) -> PLLRDICLR_W {
        PLLRDICLR_W::new(self)
    }
    #[doc = "Bit 23 - CLKSSICLR"]
    #[inline(always)]
    pub fn clkssiclr(&mut self) -> CLKSSICLR_W {
        CLKSSICLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC_CLKINT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_clkint](index.html) module"]
pub struct RCC_CLKINT_SPEC;
impl crate::RegisterSpec for RCC_CLKINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_clkint::R](R) reader structure"]
impl crate::Readable for RCC_CLKINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_clkint::W](W) writer structure"]
impl crate::Writable for RCC_CLKINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_CLKINT to value 0"]
impl crate::Resettable for RCC_CLKINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
