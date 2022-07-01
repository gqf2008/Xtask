#[doc = "Register `GPIOx_PBC` reader"]
pub struct R(crate::R<GPIOX_PBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOX_PBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOX_PBC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOX_PBC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOx_PBC` writer"]
pub struct W(crate::W<GPIOX_PBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOX_PBC_SPEC>;
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
impl From<crate::W<GPIOX_PBC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOX_PBC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBC0` reader - PBC0"]
pub type PBC0_R = crate::BitReader<bool>;
#[doc = "Field `PBC0` writer - PBC0"]
pub type PBC0_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBC_SPEC, bool, 0>;
#[doc = "Field `PBC1` reader - PBC1"]
pub type PBC1_R = crate::BitReader<bool>;
#[doc = "Field `PBC1` writer - PBC1"]
pub type PBC1_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBC_SPEC, bool, 1>;
#[doc = "Field `PBC2` reader - PBC2"]
pub type PBC2_R = crate::BitReader<bool>;
#[doc = "Field `PBC2` writer - PBC2"]
pub type PBC2_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBC_SPEC, bool, 2>;
#[doc = "Field `PBC3` reader - PBC3"]
pub type PBC3_R = crate::BitReader<bool>;
#[doc = "Field `PBC3` writer - PBC3"]
pub type PBC3_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBC_SPEC, bool, 3>;
#[doc = "Field `PBC4` reader - PBC4"]
pub type PBC4_R = crate::BitReader<bool>;
#[doc = "Field `PBC4` writer - PBC4"]
pub type PBC4_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBC_SPEC, bool, 4>;
#[doc = "Field `PBC5` reader - PBC5"]
pub type PBC5_R = crate::BitReader<bool>;
#[doc = "Field `PBC5` writer - PBC5"]
pub type PBC5_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBC_SPEC, bool, 5>;
#[doc = "Field `PBC6` reader - PBC6"]
pub type PBC6_R = crate::BitReader<bool>;
#[doc = "Field `PBC6` writer - PBC6"]
pub type PBC6_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBC_SPEC, bool, 6>;
#[doc = "Field `PBC7` reader - PBC7"]
pub type PBC7_R = crate::BitReader<bool>;
#[doc = "Field `PBC7` writer - PBC7"]
pub type PBC7_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBC_SPEC, bool, 7>;
#[doc = "Field `PBC8` reader - PBC8"]
pub type PBC8_R = crate::BitReader<bool>;
#[doc = "Field `PBC8` writer - PBC8"]
pub type PBC8_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBC_SPEC, bool, 8>;
#[doc = "Field `PBC9` reader - PBC9"]
pub type PBC9_R = crate::BitReader<bool>;
#[doc = "Field `PBC9` writer - PBC9"]
pub type PBC9_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBC_SPEC, bool, 9>;
#[doc = "Field `PBC10` reader - PBC10"]
pub type PBC10_R = crate::BitReader<bool>;
#[doc = "Field `PBC10` writer - PBC10"]
pub type PBC10_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBC_SPEC, bool, 10>;
#[doc = "Field `PBC11` reader - PBC11"]
pub type PBC11_R = crate::BitReader<bool>;
#[doc = "Field `PBC11` writer - PBC11"]
pub type PBC11_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBC_SPEC, bool, 11>;
#[doc = "Field `PBC12` reader - PBC12"]
pub type PBC12_R = crate::BitReader<bool>;
#[doc = "Field `PBC12` writer - PBC12"]
pub type PBC12_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBC_SPEC, bool, 12>;
#[doc = "Field `PBC13` reader - PBC13"]
pub type PBC13_R = crate::BitReader<bool>;
#[doc = "Field `PBC13` writer - PBC13"]
pub type PBC13_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBC_SPEC, bool, 13>;
#[doc = "Field `PBC14` reader - PBC14"]
pub type PBC14_R = crate::BitReader<bool>;
#[doc = "Field `PBC14` writer - PBC14"]
pub type PBC14_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBC_SPEC, bool, 14>;
#[doc = "Field `PBC15` reader - PBC15"]
pub type PBC15_R = crate::BitReader<bool>;
#[doc = "Field `PBC15` writer - PBC15"]
pub type PBC15_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBC_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - PBC0"]
    #[inline(always)]
    pub fn pbc0(&self) -> PBC0_R {
        PBC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PBC1"]
    #[inline(always)]
    pub fn pbc1(&self) -> PBC1_R {
        PBC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PBC2"]
    #[inline(always)]
    pub fn pbc2(&self) -> PBC2_R {
        PBC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PBC3"]
    #[inline(always)]
    pub fn pbc3(&self) -> PBC3_R {
        PBC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PBC4"]
    #[inline(always)]
    pub fn pbc4(&self) -> PBC4_R {
        PBC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PBC5"]
    #[inline(always)]
    pub fn pbc5(&self) -> PBC5_R {
        PBC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PBC6"]
    #[inline(always)]
    pub fn pbc6(&self) -> PBC6_R {
        PBC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PBC7"]
    #[inline(always)]
    pub fn pbc7(&self) -> PBC7_R {
        PBC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PBC8"]
    #[inline(always)]
    pub fn pbc8(&self) -> PBC8_R {
        PBC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PBC9"]
    #[inline(always)]
    pub fn pbc9(&self) -> PBC9_R {
        PBC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PBC10"]
    #[inline(always)]
    pub fn pbc10(&self) -> PBC10_R {
        PBC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PBC11"]
    #[inline(always)]
    pub fn pbc11(&self) -> PBC11_R {
        PBC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PBC12"]
    #[inline(always)]
    pub fn pbc12(&self) -> PBC12_R {
        PBC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PBC13"]
    #[inline(always)]
    pub fn pbc13(&self) -> PBC13_R {
        PBC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PBC14"]
    #[inline(always)]
    pub fn pbc14(&self) -> PBC14_R {
        PBC14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PBC15"]
    #[inline(always)]
    pub fn pbc15(&self) -> PBC15_R {
        PBC15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PBC0"]
    #[inline(always)]
    pub fn pbc0(&mut self) -> PBC0_W {
        PBC0_W::new(self)
    }
    #[doc = "Bit 1 - PBC1"]
    #[inline(always)]
    pub fn pbc1(&mut self) -> PBC1_W {
        PBC1_W::new(self)
    }
    #[doc = "Bit 2 - PBC2"]
    #[inline(always)]
    pub fn pbc2(&mut self) -> PBC2_W {
        PBC2_W::new(self)
    }
    #[doc = "Bit 3 - PBC3"]
    #[inline(always)]
    pub fn pbc3(&mut self) -> PBC3_W {
        PBC3_W::new(self)
    }
    #[doc = "Bit 4 - PBC4"]
    #[inline(always)]
    pub fn pbc4(&mut self) -> PBC4_W {
        PBC4_W::new(self)
    }
    #[doc = "Bit 5 - PBC5"]
    #[inline(always)]
    pub fn pbc5(&mut self) -> PBC5_W {
        PBC5_W::new(self)
    }
    #[doc = "Bit 6 - PBC6"]
    #[inline(always)]
    pub fn pbc6(&mut self) -> PBC6_W {
        PBC6_W::new(self)
    }
    #[doc = "Bit 7 - PBC7"]
    #[inline(always)]
    pub fn pbc7(&mut self) -> PBC7_W {
        PBC7_W::new(self)
    }
    #[doc = "Bit 8 - PBC8"]
    #[inline(always)]
    pub fn pbc8(&mut self) -> PBC8_W {
        PBC8_W::new(self)
    }
    #[doc = "Bit 9 - PBC9"]
    #[inline(always)]
    pub fn pbc9(&mut self) -> PBC9_W {
        PBC9_W::new(self)
    }
    #[doc = "Bit 10 - PBC10"]
    #[inline(always)]
    pub fn pbc10(&mut self) -> PBC10_W {
        PBC10_W::new(self)
    }
    #[doc = "Bit 11 - PBC11"]
    #[inline(always)]
    pub fn pbc11(&mut self) -> PBC11_W {
        PBC11_W::new(self)
    }
    #[doc = "Bit 12 - PBC12"]
    #[inline(always)]
    pub fn pbc12(&mut self) -> PBC12_W {
        PBC12_W::new(self)
    }
    #[doc = "Bit 13 - PBC13"]
    #[inline(always)]
    pub fn pbc13(&mut self) -> PBC13_W {
        PBC13_W::new(self)
    }
    #[doc = "Bit 14 - PBC14"]
    #[inline(always)]
    pub fn pbc14(&mut self) -> PBC14_W {
        PBC14_W::new(self)
    }
    #[doc = "Bit 15 - PBC15"]
    #[inline(always)]
    pub fn pbc15(&mut self) -> PBC15_W {
        PBC15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOx_PBC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiox_pbc](index.html) module"]
pub struct GPIOX_PBC_SPEC;
impl crate::RegisterSpec for GPIOX_PBC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiox_pbc::R](R) reader structure"]
impl crate::Readable for GPIOX_PBC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiox_pbc::W](W) writer structure"]
impl crate::Writable for GPIOX_PBC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOx_PBC to value 0"]
impl crate::Resettable for GPIOX_PBC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
