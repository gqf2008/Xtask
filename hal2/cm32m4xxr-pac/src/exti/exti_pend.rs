#[doc = "Register `EXTI_PEND` reader"]
pub struct R(crate::R<EXTI_PEND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_PEND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_PEND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_PEND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_PEND` writer"]
pub struct W(crate::W<EXTI_PEND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_PEND_SPEC>;
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
impl From<crate::W<EXTI_PEND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_PEND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEND0` reader - PEND0"]
pub type PEND0_R = crate::BitReader<bool>;
#[doc = "Field `PEND0` writer - PEND0"]
pub type PEND0_W<'a> = crate::BitWriter<'a, u32, EXTI_PEND_SPEC, bool, 0>;
#[doc = "Field `PEND1` reader - PEND1"]
pub type PEND1_R = crate::BitReader<bool>;
#[doc = "Field `PEND1` writer - PEND1"]
pub type PEND1_W<'a> = crate::BitWriter<'a, u32, EXTI_PEND_SPEC, bool, 1>;
#[doc = "Field `PEND2` reader - PEND2"]
pub type PEND2_R = crate::BitReader<bool>;
#[doc = "Field `PEND2` writer - PEND2"]
pub type PEND2_W<'a> = crate::BitWriter<'a, u32, EXTI_PEND_SPEC, bool, 2>;
#[doc = "Field `PEND3` reader - PEND3"]
pub type PEND3_R = crate::BitReader<bool>;
#[doc = "Field `PEND3` writer - PEND3"]
pub type PEND3_W<'a> = crate::BitWriter<'a, u32, EXTI_PEND_SPEC, bool, 3>;
#[doc = "Field `PEND4` reader - PEND4"]
pub type PEND4_R = crate::BitReader<bool>;
#[doc = "Field `PEND4` writer - PEND4"]
pub type PEND4_W<'a> = crate::BitWriter<'a, u32, EXTI_PEND_SPEC, bool, 4>;
#[doc = "Field `PEND5` reader - PEND5"]
pub type PEND5_R = crate::BitReader<bool>;
#[doc = "Field `PEND5` writer - PEND5"]
pub type PEND5_W<'a> = crate::BitWriter<'a, u32, EXTI_PEND_SPEC, bool, 5>;
#[doc = "Field `PEND6` reader - PEND6"]
pub type PEND6_R = crate::BitReader<bool>;
#[doc = "Field `PEND6` writer - PEND6"]
pub type PEND6_W<'a> = crate::BitWriter<'a, u32, EXTI_PEND_SPEC, bool, 6>;
#[doc = "Field `PEND7` reader - PEND7"]
pub type PEND7_R = crate::BitReader<bool>;
#[doc = "Field `PEND7` writer - PEND7"]
pub type PEND7_W<'a> = crate::BitWriter<'a, u32, EXTI_PEND_SPEC, bool, 7>;
#[doc = "Field `PEND8` reader - PEND8"]
pub type PEND8_R = crate::BitReader<bool>;
#[doc = "Field `PEND8` writer - PEND8"]
pub type PEND8_W<'a> = crate::BitWriter<'a, u32, EXTI_PEND_SPEC, bool, 8>;
#[doc = "Field `PEND9` reader - PEND9"]
pub type PEND9_R = crate::BitReader<bool>;
#[doc = "Field `PEND9` writer - PEND9"]
pub type PEND9_W<'a> = crate::BitWriter<'a, u32, EXTI_PEND_SPEC, bool, 9>;
#[doc = "Field `PEND10` reader - PEND10"]
pub type PEND10_R = crate::BitReader<bool>;
#[doc = "Field `PEND10` writer - PEND10"]
pub type PEND10_W<'a> = crate::BitWriter<'a, u32, EXTI_PEND_SPEC, bool, 10>;
#[doc = "Field `PEND11` reader - PEND11"]
pub type PEND11_R = crate::BitReader<bool>;
#[doc = "Field `PEND11` writer - PEND11"]
pub type PEND11_W<'a> = crate::BitWriter<'a, u32, EXTI_PEND_SPEC, bool, 11>;
#[doc = "Field `PEND12` reader - PEND12"]
pub type PEND12_R = crate::BitReader<bool>;
#[doc = "Field `PEND12` writer - PEND12"]
pub type PEND12_W<'a> = crate::BitWriter<'a, u32, EXTI_PEND_SPEC, bool, 12>;
#[doc = "Field `PEND13` reader - PEND13"]
pub type PEND13_R = crate::BitReader<bool>;
#[doc = "Field `PEND13` writer - PEND13"]
pub type PEND13_W<'a> = crate::BitWriter<'a, u32, EXTI_PEND_SPEC, bool, 13>;
#[doc = "Field `PEND14` reader - PEND14"]
pub type PEND14_R = crate::BitReader<bool>;
#[doc = "Field `PEND14` writer - PEND14"]
pub type PEND14_W<'a> = crate::BitWriter<'a, u32, EXTI_PEND_SPEC, bool, 14>;
#[doc = "Field `PEND15` reader - PEND15"]
pub type PEND15_R = crate::BitReader<bool>;
#[doc = "Field `PEND15` writer - PEND15"]
pub type PEND15_W<'a> = crate::BitWriter<'a, u32, EXTI_PEND_SPEC, bool, 15>;
#[doc = "Field `PEND16` reader - PEND16"]
pub type PEND16_R = crate::BitReader<bool>;
#[doc = "Field `PEND16` writer - PEND16"]
pub type PEND16_W<'a> = crate::BitWriter<'a, u32, EXTI_PEND_SPEC, bool, 16>;
#[doc = "Field `PEND17` reader - PEND17"]
pub type PEND17_R = crate::BitReader<bool>;
#[doc = "Field `PEND17` writer - PEND17"]
pub type PEND17_W<'a> = crate::BitWriter<'a, u32, EXTI_PEND_SPEC, bool, 17>;
#[doc = "Field `PEND18` reader - PEND18"]
pub type PEND18_R = crate::BitReader<bool>;
#[doc = "Field `PEND18` writer - PEND18"]
pub type PEND18_W<'a> = crate::BitWriter<'a, u32, EXTI_PEND_SPEC, bool, 18>;
#[doc = "Field `PEND19` reader - PEND19"]
pub type PEND19_R = crate::BitReader<bool>;
#[doc = "Field `PEND19` writer - PEND19"]
pub type PEND19_W<'a> = crate::BitWriter<'a, u32, EXTI_PEND_SPEC, bool, 19>;
#[doc = "Field `PEND20` reader - PEND20"]
pub type PEND20_R = crate::BitReader<bool>;
#[doc = "Field `PEND20` writer - PEND20"]
pub type PEND20_W<'a> = crate::BitWriter<'a, u32, EXTI_PEND_SPEC, bool, 20>;
#[doc = "Field `PEND21` reader - PEND21"]
pub type PEND21_R = crate::BitReader<bool>;
#[doc = "Field `PEND21` writer - PEND21"]
pub type PEND21_W<'a> = crate::BitWriter<'a, u32, EXTI_PEND_SPEC, bool, 21>;
impl R {
    #[doc = "Bit 0 - PEND0"]
    #[inline(always)]
    pub fn pend0(&self) -> PEND0_R {
        PEND0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PEND1"]
    #[inline(always)]
    pub fn pend1(&self) -> PEND1_R {
        PEND1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PEND2"]
    #[inline(always)]
    pub fn pend2(&self) -> PEND2_R {
        PEND2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PEND3"]
    #[inline(always)]
    pub fn pend3(&self) -> PEND3_R {
        PEND3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PEND4"]
    #[inline(always)]
    pub fn pend4(&self) -> PEND4_R {
        PEND4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PEND5"]
    #[inline(always)]
    pub fn pend5(&self) -> PEND5_R {
        PEND5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PEND6"]
    #[inline(always)]
    pub fn pend6(&self) -> PEND6_R {
        PEND6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PEND7"]
    #[inline(always)]
    pub fn pend7(&self) -> PEND7_R {
        PEND7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PEND8"]
    #[inline(always)]
    pub fn pend8(&self) -> PEND8_R {
        PEND8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PEND9"]
    #[inline(always)]
    pub fn pend9(&self) -> PEND9_R {
        PEND9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PEND10"]
    #[inline(always)]
    pub fn pend10(&self) -> PEND10_R {
        PEND10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PEND11"]
    #[inline(always)]
    pub fn pend11(&self) -> PEND11_R {
        PEND11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PEND12"]
    #[inline(always)]
    pub fn pend12(&self) -> PEND12_R {
        PEND12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PEND13"]
    #[inline(always)]
    pub fn pend13(&self) -> PEND13_R {
        PEND13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PEND14"]
    #[inline(always)]
    pub fn pend14(&self) -> PEND14_R {
        PEND14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PEND15"]
    #[inline(always)]
    pub fn pend15(&self) -> PEND15_R {
        PEND15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PEND16"]
    #[inline(always)]
    pub fn pend16(&self) -> PEND16_R {
        PEND16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PEND17"]
    #[inline(always)]
    pub fn pend17(&self) -> PEND17_R {
        PEND17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PEND18"]
    #[inline(always)]
    pub fn pend18(&self) -> PEND18_R {
        PEND18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PEND19"]
    #[inline(always)]
    pub fn pend19(&self) -> PEND19_R {
        PEND19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PEND20"]
    #[inline(always)]
    pub fn pend20(&self) -> PEND20_R {
        PEND20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PEND21"]
    #[inline(always)]
    pub fn pend21(&self) -> PEND21_R {
        PEND21_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PEND0"]
    #[inline(always)]
    pub fn pend0(&mut self) -> PEND0_W {
        PEND0_W::new(self)
    }
    #[doc = "Bit 1 - PEND1"]
    #[inline(always)]
    pub fn pend1(&mut self) -> PEND1_W {
        PEND1_W::new(self)
    }
    #[doc = "Bit 2 - PEND2"]
    #[inline(always)]
    pub fn pend2(&mut self) -> PEND2_W {
        PEND2_W::new(self)
    }
    #[doc = "Bit 3 - PEND3"]
    #[inline(always)]
    pub fn pend3(&mut self) -> PEND3_W {
        PEND3_W::new(self)
    }
    #[doc = "Bit 4 - PEND4"]
    #[inline(always)]
    pub fn pend4(&mut self) -> PEND4_W {
        PEND4_W::new(self)
    }
    #[doc = "Bit 5 - PEND5"]
    #[inline(always)]
    pub fn pend5(&mut self) -> PEND5_W {
        PEND5_W::new(self)
    }
    #[doc = "Bit 6 - PEND6"]
    #[inline(always)]
    pub fn pend6(&mut self) -> PEND6_W {
        PEND6_W::new(self)
    }
    #[doc = "Bit 7 - PEND7"]
    #[inline(always)]
    pub fn pend7(&mut self) -> PEND7_W {
        PEND7_W::new(self)
    }
    #[doc = "Bit 8 - PEND8"]
    #[inline(always)]
    pub fn pend8(&mut self) -> PEND8_W {
        PEND8_W::new(self)
    }
    #[doc = "Bit 9 - PEND9"]
    #[inline(always)]
    pub fn pend9(&mut self) -> PEND9_W {
        PEND9_W::new(self)
    }
    #[doc = "Bit 10 - PEND10"]
    #[inline(always)]
    pub fn pend10(&mut self) -> PEND10_W {
        PEND10_W::new(self)
    }
    #[doc = "Bit 11 - PEND11"]
    #[inline(always)]
    pub fn pend11(&mut self) -> PEND11_W {
        PEND11_W::new(self)
    }
    #[doc = "Bit 12 - PEND12"]
    #[inline(always)]
    pub fn pend12(&mut self) -> PEND12_W {
        PEND12_W::new(self)
    }
    #[doc = "Bit 13 - PEND13"]
    #[inline(always)]
    pub fn pend13(&mut self) -> PEND13_W {
        PEND13_W::new(self)
    }
    #[doc = "Bit 14 - PEND14"]
    #[inline(always)]
    pub fn pend14(&mut self) -> PEND14_W {
        PEND14_W::new(self)
    }
    #[doc = "Bit 15 - PEND15"]
    #[inline(always)]
    pub fn pend15(&mut self) -> PEND15_W {
        PEND15_W::new(self)
    }
    #[doc = "Bit 16 - PEND16"]
    #[inline(always)]
    pub fn pend16(&mut self) -> PEND16_W {
        PEND16_W::new(self)
    }
    #[doc = "Bit 17 - PEND17"]
    #[inline(always)]
    pub fn pend17(&mut self) -> PEND17_W {
        PEND17_W::new(self)
    }
    #[doc = "Bit 18 - PEND18"]
    #[inline(always)]
    pub fn pend18(&mut self) -> PEND18_W {
        PEND18_W::new(self)
    }
    #[doc = "Bit 19 - PEND19"]
    #[inline(always)]
    pub fn pend19(&mut self) -> PEND19_W {
        PEND19_W::new(self)
    }
    #[doc = "Bit 20 - PEND20"]
    #[inline(always)]
    pub fn pend20(&mut self) -> PEND20_W {
        PEND20_W::new(self)
    }
    #[doc = "Bit 21 - PEND21"]
    #[inline(always)]
    pub fn pend21(&mut self) -> PEND21_W {
        PEND21_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI_PEND\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_pend](index.html) module"]
pub struct EXTI_PEND_SPEC;
impl crate::RegisterSpec for EXTI_PEND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_pend::R](R) reader structure"]
impl crate::Readable for EXTI_PEND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_pend::W](W) writer structure"]
impl crate::Writable for EXTI_PEND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTI_PEND to value 0"]
impl crate::Resettable for EXTI_PEND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
