#[doc = "Register `GPIOx_POD` reader"]
pub struct R(crate::R<GPIOX_POD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOX_POD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOX_POD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOX_POD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOx_POD` writer"]
pub struct W(crate::W<GPIOX_POD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOX_POD_SPEC>;
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
impl From<crate::W<GPIOX_POD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOX_POD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POD0` reader - POD0"]
pub type POD0_R = crate::BitReader<bool>;
#[doc = "Field `POD0` writer - POD0"]
pub type POD0_W<'a> = crate::BitWriter<'a, u32, GPIOX_POD_SPEC, bool, 0>;
#[doc = "Field `POD1` reader - POD1"]
pub type POD1_R = crate::BitReader<bool>;
#[doc = "Field `POD1` writer - POD1"]
pub type POD1_W<'a> = crate::BitWriter<'a, u32, GPIOX_POD_SPEC, bool, 1>;
#[doc = "Field `POD2` reader - POD2"]
pub type POD2_R = crate::BitReader<bool>;
#[doc = "Field `POD2` writer - POD2"]
pub type POD2_W<'a> = crate::BitWriter<'a, u32, GPIOX_POD_SPEC, bool, 2>;
#[doc = "Field `POD3` reader - POD3"]
pub type POD3_R = crate::BitReader<bool>;
#[doc = "Field `POD3` writer - POD3"]
pub type POD3_W<'a> = crate::BitWriter<'a, u32, GPIOX_POD_SPEC, bool, 3>;
#[doc = "Field `POD4` reader - POD4"]
pub type POD4_R = crate::BitReader<bool>;
#[doc = "Field `POD4` writer - POD4"]
pub type POD4_W<'a> = crate::BitWriter<'a, u32, GPIOX_POD_SPEC, bool, 4>;
#[doc = "Field `POD5` reader - POD5"]
pub type POD5_R = crate::BitReader<bool>;
#[doc = "Field `POD5` writer - POD5"]
pub type POD5_W<'a> = crate::BitWriter<'a, u32, GPIOX_POD_SPEC, bool, 5>;
#[doc = "Field `POD6` reader - POD6"]
pub type POD6_R = crate::BitReader<bool>;
#[doc = "Field `POD6` writer - POD6"]
pub type POD6_W<'a> = crate::BitWriter<'a, u32, GPIOX_POD_SPEC, bool, 6>;
#[doc = "Field `POD7` reader - POD7"]
pub type POD7_R = crate::BitReader<bool>;
#[doc = "Field `POD7` writer - POD7"]
pub type POD7_W<'a> = crate::BitWriter<'a, u32, GPIOX_POD_SPEC, bool, 7>;
#[doc = "Field `POD8` reader - POD8"]
pub type POD8_R = crate::BitReader<bool>;
#[doc = "Field `POD8` writer - POD8"]
pub type POD8_W<'a> = crate::BitWriter<'a, u32, GPIOX_POD_SPEC, bool, 8>;
#[doc = "Field `POD9` reader - POD9"]
pub type POD9_R = crate::BitReader<bool>;
#[doc = "Field `POD9` writer - POD9"]
pub type POD9_W<'a> = crate::BitWriter<'a, u32, GPIOX_POD_SPEC, bool, 9>;
#[doc = "Field `POD10` reader - POD10"]
pub type POD10_R = crate::BitReader<bool>;
#[doc = "Field `POD10` writer - POD10"]
pub type POD10_W<'a> = crate::BitWriter<'a, u32, GPIOX_POD_SPEC, bool, 10>;
#[doc = "Field `POD11` reader - POD11"]
pub type POD11_R = crate::BitReader<bool>;
#[doc = "Field `POD11` writer - POD11"]
pub type POD11_W<'a> = crate::BitWriter<'a, u32, GPIOX_POD_SPEC, bool, 11>;
#[doc = "Field `POD12` reader - POD12"]
pub type POD12_R = crate::BitReader<bool>;
#[doc = "Field `POD12` writer - POD12"]
pub type POD12_W<'a> = crate::BitWriter<'a, u32, GPIOX_POD_SPEC, bool, 12>;
#[doc = "Field `POD13` reader - POD13"]
pub type POD13_R = crate::BitReader<bool>;
#[doc = "Field `POD13` writer - POD13"]
pub type POD13_W<'a> = crate::BitWriter<'a, u32, GPIOX_POD_SPEC, bool, 13>;
#[doc = "Field `POD14` reader - POD14"]
pub type POD14_R = crate::BitReader<bool>;
#[doc = "Field `POD14` writer - POD14"]
pub type POD14_W<'a> = crate::BitWriter<'a, u32, GPIOX_POD_SPEC, bool, 14>;
#[doc = "Field `POD15` reader - POD15"]
pub type POD15_R = crate::BitReader<bool>;
#[doc = "Field `POD15` writer - POD15"]
pub type POD15_W<'a> = crate::BitWriter<'a, u32, GPIOX_POD_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - POD0"]
    #[inline(always)]
    pub fn pod0(&self) -> POD0_R {
        POD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - POD1"]
    #[inline(always)]
    pub fn pod1(&self) -> POD1_R {
        POD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - POD2"]
    #[inline(always)]
    pub fn pod2(&self) -> POD2_R {
        POD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - POD3"]
    #[inline(always)]
    pub fn pod3(&self) -> POD3_R {
        POD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - POD4"]
    #[inline(always)]
    pub fn pod4(&self) -> POD4_R {
        POD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - POD5"]
    #[inline(always)]
    pub fn pod5(&self) -> POD5_R {
        POD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - POD6"]
    #[inline(always)]
    pub fn pod6(&self) -> POD6_R {
        POD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - POD7"]
    #[inline(always)]
    pub fn pod7(&self) -> POD7_R {
        POD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - POD8"]
    #[inline(always)]
    pub fn pod8(&self) -> POD8_R {
        POD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - POD9"]
    #[inline(always)]
    pub fn pod9(&self) -> POD9_R {
        POD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - POD10"]
    #[inline(always)]
    pub fn pod10(&self) -> POD10_R {
        POD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - POD11"]
    #[inline(always)]
    pub fn pod11(&self) -> POD11_R {
        POD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - POD12"]
    #[inline(always)]
    pub fn pod12(&self) -> POD12_R {
        POD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - POD13"]
    #[inline(always)]
    pub fn pod13(&self) -> POD13_R {
        POD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - POD14"]
    #[inline(always)]
    pub fn pod14(&self) -> POD14_R {
        POD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - POD15"]
    #[inline(always)]
    pub fn pod15(&self) -> POD15_R {
        POD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - POD0"]
    #[inline(always)]
    pub fn pod0(&mut self) -> POD0_W {
        POD0_W::new(self)
    }
    #[doc = "Bit 1 - POD1"]
    #[inline(always)]
    pub fn pod1(&mut self) -> POD1_W {
        POD1_W::new(self)
    }
    #[doc = "Bit 2 - POD2"]
    #[inline(always)]
    pub fn pod2(&mut self) -> POD2_W {
        POD2_W::new(self)
    }
    #[doc = "Bit 3 - POD3"]
    #[inline(always)]
    pub fn pod3(&mut self) -> POD3_W {
        POD3_W::new(self)
    }
    #[doc = "Bit 4 - POD4"]
    #[inline(always)]
    pub fn pod4(&mut self) -> POD4_W {
        POD4_W::new(self)
    }
    #[doc = "Bit 5 - POD5"]
    #[inline(always)]
    pub fn pod5(&mut self) -> POD5_W {
        POD5_W::new(self)
    }
    #[doc = "Bit 6 - POD6"]
    #[inline(always)]
    pub fn pod6(&mut self) -> POD6_W {
        POD6_W::new(self)
    }
    #[doc = "Bit 7 - POD7"]
    #[inline(always)]
    pub fn pod7(&mut self) -> POD7_W {
        POD7_W::new(self)
    }
    #[doc = "Bit 8 - POD8"]
    #[inline(always)]
    pub fn pod8(&mut self) -> POD8_W {
        POD8_W::new(self)
    }
    #[doc = "Bit 9 - POD9"]
    #[inline(always)]
    pub fn pod9(&mut self) -> POD9_W {
        POD9_W::new(self)
    }
    #[doc = "Bit 10 - POD10"]
    #[inline(always)]
    pub fn pod10(&mut self) -> POD10_W {
        POD10_W::new(self)
    }
    #[doc = "Bit 11 - POD11"]
    #[inline(always)]
    pub fn pod11(&mut self) -> POD11_W {
        POD11_W::new(self)
    }
    #[doc = "Bit 12 - POD12"]
    #[inline(always)]
    pub fn pod12(&mut self) -> POD12_W {
        POD12_W::new(self)
    }
    #[doc = "Bit 13 - POD13"]
    #[inline(always)]
    pub fn pod13(&mut self) -> POD13_W {
        POD13_W::new(self)
    }
    #[doc = "Bit 14 - POD14"]
    #[inline(always)]
    pub fn pod14(&mut self) -> POD14_W {
        POD14_W::new(self)
    }
    #[doc = "Bit 15 - POD15"]
    #[inline(always)]
    pub fn pod15(&mut self) -> POD15_W {
        POD15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOx_POD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiox_pod](index.html) module"]
pub struct GPIOX_POD_SPEC;
impl crate::RegisterSpec for GPIOX_POD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiox_pod::R](R) reader structure"]
impl crate::Readable for GPIOX_POD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiox_pod::W](W) writer structure"]
impl crate::Writable for GPIOX_POD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOx_POD to value 0xa000"]
impl crate::Resettable for GPIOX_POD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa000
    }
}
