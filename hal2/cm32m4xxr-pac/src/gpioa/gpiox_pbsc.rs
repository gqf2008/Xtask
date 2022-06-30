#[doc = "Register `GPIOx_PBSC` reader"]
pub struct R(crate::R<GPIOX_PBSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOX_PBSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOX_PBSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOX_PBSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOx_PBSC` writer"]
pub struct W(crate::W<GPIOX_PBSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOX_PBSC_SPEC>;
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
impl From<crate::W<GPIOX_PBSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOX_PBSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBS0` reader - PBS0"]
pub type PBS0_R = crate::BitReader<bool>;
#[doc = "Field `PBS0` writer - PBS0"]
pub type PBS0_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 0>;
#[doc = "Field `PBS1` reader - PBS1"]
pub type PBS1_R = crate::BitReader<bool>;
#[doc = "Field `PBS1` writer - PBS1"]
pub type PBS1_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 1>;
#[doc = "Field `PBS2` reader - PBS2"]
pub type PBS2_R = crate::BitReader<bool>;
#[doc = "Field `PBS2` writer - PBS2"]
pub type PBS2_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 2>;
#[doc = "Field `PBS3` reader - PBS3"]
pub type PBS3_R = crate::BitReader<bool>;
#[doc = "Field `PBS3` writer - PBS3"]
pub type PBS3_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 3>;
#[doc = "Field `PBS4` reader - PBS4"]
pub type PBS4_R = crate::BitReader<bool>;
#[doc = "Field `PBS4` writer - PBS4"]
pub type PBS4_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 4>;
#[doc = "Field `PBS5` reader - PBS5"]
pub type PBS5_R = crate::BitReader<bool>;
#[doc = "Field `PBS5` writer - PBS5"]
pub type PBS5_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 5>;
#[doc = "Field `PBS6` reader - PBS6"]
pub type PBS6_R = crate::BitReader<bool>;
#[doc = "Field `PBS6` writer - PBS6"]
pub type PBS6_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 6>;
#[doc = "Field `PBS7` reader - PBS7"]
pub type PBS7_R = crate::BitReader<bool>;
#[doc = "Field `PBS7` writer - PBS7"]
pub type PBS7_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 7>;
#[doc = "Field `PBS8` reader - PBS8"]
pub type PBS8_R = crate::BitReader<bool>;
#[doc = "Field `PBS8` writer - PBS8"]
pub type PBS8_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 8>;
#[doc = "Field `PBS9` reader - PBS9"]
pub type PBS9_R = crate::BitReader<bool>;
#[doc = "Field `PBS9` writer - PBS9"]
pub type PBS9_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 9>;
#[doc = "Field `PBS10` reader - PBS10"]
pub type PBS10_R = crate::BitReader<bool>;
#[doc = "Field `PBS10` writer - PBS10"]
pub type PBS10_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 10>;
#[doc = "Field `PBS11` reader - PBS11"]
pub type PBS11_R = crate::BitReader<bool>;
#[doc = "Field `PBS11` writer - PBS11"]
pub type PBS11_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 11>;
#[doc = "Field `PBS12` reader - PBS12"]
pub type PBS12_R = crate::BitReader<bool>;
#[doc = "Field `PBS12` writer - PBS12"]
pub type PBS12_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 12>;
#[doc = "Field `PBS13` reader - PBS13"]
pub type PBS13_R = crate::BitReader<bool>;
#[doc = "Field `PBS13` writer - PBS13"]
pub type PBS13_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 13>;
#[doc = "Field `PBS14` reader - PBS14"]
pub type PBS14_R = crate::BitReader<bool>;
#[doc = "Field `PBS14` writer - PBS14"]
pub type PBS14_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 14>;
#[doc = "Field `PBS15` reader - PBS15"]
pub type PBS15_R = crate::BitReader<bool>;
#[doc = "Field `PBS15` writer - PBS15"]
pub type PBS15_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 15>;
#[doc = "Field `PBC0` reader - PBC0"]
pub type PBC0_R = crate::BitReader<bool>;
#[doc = "Field `PBC0` writer - PBC0"]
pub type PBC0_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 16>;
#[doc = "Field `PBC1` reader - PBC1"]
pub type PBC1_R = crate::BitReader<bool>;
#[doc = "Field `PBC1` writer - PBC1"]
pub type PBC1_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 17>;
#[doc = "Field `PBC2` reader - PBC2"]
pub type PBC2_R = crate::BitReader<bool>;
#[doc = "Field `PBC2` writer - PBC2"]
pub type PBC2_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 18>;
#[doc = "Field `PBC3` reader - PBC3"]
pub type PBC3_R = crate::BitReader<bool>;
#[doc = "Field `PBC3` writer - PBC3"]
pub type PBC3_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 19>;
#[doc = "Field `PBC4` reader - PBC4"]
pub type PBC4_R = crate::BitReader<bool>;
#[doc = "Field `PBC4` writer - PBC4"]
pub type PBC4_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 20>;
#[doc = "Field `PBC5` reader - PBC5"]
pub type PBC5_R = crate::BitReader<bool>;
#[doc = "Field `PBC5` writer - PBC5"]
pub type PBC5_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 21>;
#[doc = "Field `PBC6` reader - PBC6"]
pub type PBC6_R = crate::BitReader<bool>;
#[doc = "Field `PBC6` writer - PBC6"]
pub type PBC6_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 22>;
#[doc = "Field `PBC7` reader - PBC7"]
pub type PBC7_R = crate::BitReader<bool>;
#[doc = "Field `PBC7` writer - PBC7"]
pub type PBC7_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 23>;
#[doc = "Field `PBC8` reader - PBC8"]
pub type PBC8_R = crate::BitReader<bool>;
#[doc = "Field `PBC8` writer - PBC8"]
pub type PBC8_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 24>;
#[doc = "Field `PBC9` reader - PBC9"]
pub type PBC9_R = crate::BitReader<bool>;
#[doc = "Field `PBC9` writer - PBC9"]
pub type PBC9_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 25>;
#[doc = "Field `PBC10` reader - PBC10"]
pub type PBC10_R = crate::BitReader<bool>;
#[doc = "Field `PBC10` writer - PBC10"]
pub type PBC10_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 26>;
#[doc = "Field `PBC11` reader - PBC11"]
pub type PBC11_R = crate::BitReader<bool>;
#[doc = "Field `PBC11` writer - PBC11"]
pub type PBC11_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 27>;
#[doc = "Field `PBC12` reader - PBC12"]
pub type PBC12_R = crate::BitReader<bool>;
#[doc = "Field `PBC12` writer - PBC12"]
pub type PBC12_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 28>;
#[doc = "Field `PBC13` reader - PBC13"]
pub type PBC13_R = crate::BitReader<bool>;
#[doc = "Field `PBC13` writer - PBC13"]
pub type PBC13_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 29>;
#[doc = "Field `PBC14` reader - PBC14"]
pub type PBC14_R = crate::BitReader<bool>;
#[doc = "Field `PBC14` writer - PBC14"]
pub type PBC14_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 30>;
#[doc = "Field `PBC15` reader - PBC15"]
pub type PBC15_R = crate::BitReader<bool>;
#[doc = "Field `PBC15` writer - PBC15"]
pub type PBC15_W<'a> = crate::BitWriter<'a, u32, GPIOX_PBSC_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - PBS0"]
    #[inline(always)]
    pub fn pbs0(&self) -> PBS0_R {
        PBS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PBS1"]
    #[inline(always)]
    pub fn pbs1(&self) -> PBS1_R {
        PBS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PBS2"]
    #[inline(always)]
    pub fn pbs2(&self) -> PBS2_R {
        PBS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PBS3"]
    #[inline(always)]
    pub fn pbs3(&self) -> PBS3_R {
        PBS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PBS4"]
    #[inline(always)]
    pub fn pbs4(&self) -> PBS4_R {
        PBS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PBS5"]
    #[inline(always)]
    pub fn pbs5(&self) -> PBS5_R {
        PBS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PBS6"]
    #[inline(always)]
    pub fn pbs6(&self) -> PBS6_R {
        PBS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PBS7"]
    #[inline(always)]
    pub fn pbs7(&self) -> PBS7_R {
        PBS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PBS8"]
    #[inline(always)]
    pub fn pbs8(&self) -> PBS8_R {
        PBS8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PBS9"]
    #[inline(always)]
    pub fn pbs9(&self) -> PBS9_R {
        PBS9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PBS10"]
    #[inline(always)]
    pub fn pbs10(&self) -> PBS10_R {
        PBS10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PBS11"]
    #[inline(always)]
    pub fn pbs11(&self) -> PBS11_R {
        PBS11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PBS12"]
    #[inline(always)]
    pub fn pbs12(&self) -> PBS12_R {
        PBS12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PBS13"]
    #[inline(always)]
    pub fn pbs13(&self) -> PBS13_R {
        PBS13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PBS14"]
    #[inline(always)]
    pub fn pbs14(&self) -> PBS14_R {
        PBS14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PBS15"]
    #[inline(always)]
    pub fn pbs15(&self) -> PBS15_R {
        PBS15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PBC0"]
    #[inline(always)]
    pub fn pbc0(&self) -> PBC0_R {
        PBC0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PBC1"]
    #[inline(always)]
    pub fn pbc1(&self) -> PBC1_R {
        PBC1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PBC2"]
    #[inline(always)]
    pub fn pbc2(&self) -> PBC2_R {
        PBC2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PBC3"]
    #[inline(always)]
    pub fn pbc3(&self) -> PBC3_R {
        PBC3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PBC4"]
    #[inline(always)]
    pub fn pbc4(&self) -> PBC4_R {
        PBC4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PBC5"]
    #[inline(always)]
    pub fn pbc5(&self) -> PBC5_R {
        PBC5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PBC6"]
    #[inline(always)]
    pub fn pbc6(&self) -> PBC6_R {
        PBC6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PBC7"]
    #[inline(always)]
    pub fn pbc7(&self) -> PBC7_R {
        PBC7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PBC8"]
    #[inline(always)]
    pub fn pbc8(&self) -> PBC8_R {
        PBC8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PBC9"]
    #[inline(always)]
    pub fn pbc9(&self) -> PBC9_R {
        PBC9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PBC10"]
    #[inline(always)]
    pub fn pbc10(&self) -> PBC10_R {
        PBC10_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PBC11"]
    #[inline(always)]
    pub fn pbc11(&self) -> PBC11_R {
        PBC11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PBC12"]
    #[inline(always)]
    pub fn pbc12(&self) -> PBC12_R {
        PBC12_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PBC13"]
    #[inline(always)]
    pub fn pbc13(&self) -> PBC13_R {
        PBC13_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - PBC14"]
    #[inline(always)]
    pub fn pbc14(&self) -> PBC14_R {
        PBC14_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PBC15"]
    #[inline(always)]
    pub fn pbc15(&self) -> PBC15_R {
        PBC15_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PBS0"]
    #[inline(always)]
    pub fn pbs0(&mut self) -> PBS0_W {
        PBS0_W::new(self)
    }
    #[doc = "Bit 1 - PBS1"]
    #[inline(always)]
    pub fn pbs1(&mut self) -> PBS1_W {
        PBS1_W::new(self)
    }
    #[doc = "Bit 2 - PBS2"]
    #[inline(always)]
    pub fn pbs2(&mut self) -> PBS2_W {
        PBS2_W::new(self)
    }
    #[doc = "Bit 3 - PBS3"]
    #[inline(always)]
    pub fn pbs3(&mut self) -> PBS3_W {
        PBS3_W::new(self)
    }
    #[doc = "Bit 4 - PBS4"]
    #[inline(always)]
    pub fn pbs4(&mut self) -> PBS4_W {
        PBS4_W::new(self)
    }
    #[doc = "Bit 5 - PBS5"]
    #[inline(always)]
    pub fn pbs5(&mut self) -> PBS5_W {
        PBS5_W::new(self)
    }
    #[doc = "Bit 6 - PBS6"]
    #[inline(always)]
    pub fn pbs6(&mut self) -> PBS6_W {
        PBS6_W::new(self)
    }
    #[doc = "Bit 7 - PBS7"]
    #[inline(always)]
    pub fn pbs7(&mut self) -> PBS7_W {
        PBS7_W::new(self)
    }
    #[doc = "Bit 8 - PBS8"]
    #[inline(always)]
    pub fn pbs8(&mut self) -> PBS8_W {
        PBS8_W::new(self)
    }
    #[doc = "Bit 9 - PBS9"]
    #[inline(always)]
    pub fn pbs9(&mut self) -> PBS9_W {
        PBS9_W::new(self)
    }
    #[doc = "Bit 10 - PBS10"]
    #[inline(always)]
    pub fn pbs10(&mut self) -> PBS10_W {
        PBS10_W::new(self)
    }
    #[doc = "Bit 11 - PBS11"]
    #[inline(always)]
    pub fn pbs11(&mut self) -> PBS11_W {
        PBS11_W::new(self)
    }
    #[doc = "Bit 12 - PBS12"]
    #[inline(always)]
    pub fn pbs12(&mut self) -> PBS12_W {
        PBS12_W::new(self)
    }
    #[doc = "Bit 13 - PBS13"]
    #[inline(always)]
    pub fn pbs13(&mut self) -> PBS13_W {
        PBS13_W::new(self)
    }
    #[doc = "Bit 14 - PBS14"]
    #[inline(always)]
    pub fn pbs14(&mut self) -> PBS14_W {
        PBS14_W::new(self)
    }
    #[doc = "Bit 15 - PBS15"]
    #[inline(always)]
    pub fn pbs15(&mut self) -> PBS15_W {
        PBS15_W::new(self)
    }
    #[doc = "Bit 16 - PBC0"]
    #[inline(always)]
    pub fn pbc0(&mut self) -> PBC0_W {
        PBC0_W::new(self)
    }
    #[doc = "Bit 17 - PBC1"]
    #[inline(always)]
    pub fn pbc1(&mut self) -> PBC1_W {
        PBC1_W::new(self)
    }
    #[doc = "Bit 18 - PBC2"]
    #[inline(always)]
    pub fn pbc2(&mut self) -> PBC2_W {
        PBC2_W::new(self)
    }
    #[doc = "Bit 19 - PBC3"]
    #[inline(always)]
    pub fn pbc3(&mut self) -> PBC3_W {
        PBC3_W::new(self)
    }
    #[doc = "Bit 20 - PBC4"]
    #[inline(always)]
    pub fn pbc4(&mut self) -> PBC4_W {
        PBC4_W::new(self)
    }
    #[doc = "Bit 21 - PBC5"]
    #[inline(always)]
    pub fn pbc5(&mut self) -> PBC5_W {
        PBC5_W::new(self)
    }
    #[doc = "Bit 22 - PBC6"]
    #[inline(always)]
    pub fn pbc6(&mut self) -> PBC6_W {
        PBC6_W::new(self)
    }
    #[doc = "Bit 23 - PBC7"]
    #[inline(always)]
    pub fn pbc7(&mut self) -> PBC7_W {
        PBC7_W::new(self)
    }
    #[doc = "Bit 24 - PBC8"]
    #[inline(always)]
    pub fn pbc8(&mut self) -> PBC8_W {
        PBC8_W::new(self)
    }
    #[doc = "Bit 25 - PBC9"]
    #[inline(always)]
    pub fn pbc9(&mut self) -> PBC9_W {
        PBC9_W::new(self)
    }
    #[doc = "Bit 26 - PBC10"]
    #[inline(always)]
    pub fn pbc10(&mut self) -> PBC10_W {
        PBC10_W::new(self)
    }
    #[doc = "Bit 27 - PBC11"]
    #[inline(always)]
    pub fn pbc11(&mut self) -> PBC11_W {
        PBC11_W::new(self)
    }
    #[doc = "Bit 28 - PBC12"]
    #[inline(always)]
    pub fn pbc12(&mut self) -> PBC12_W {
        PBC12_W::new(self)
    }
    #[doc = "Bit 29 - PBC13"]
    #[inline(always)]
    pub fn pbc13(&mut self) -> PBC13_W {
        PBC13_W::new(self)
    }
    #[doc = "Bit 30 - PBC14"]
    #[inline(always)]
    pub fn pbc14(&mut self) -> PBC14_W {
        PBC14_W::new(self)
    }
    #[doc = "Bit 31 - PBC15"]
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
#[doc = "GPIOx_PBSC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiox_pbsc](index.html) module"]
pub struct GPIOX_PBSC_SPEC;
impl crate::RegisterSpec for GPIOX_PBSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiox_pbsc::R](R) reader structure"]
impl crate::Readable for GPIOX_PBSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiox_pbsc::W](W) writer structure"]
impl crate::Writable for GPIOX_PBSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOx_PBSC to value 0"]
impl crate::Resettable for GPIOX_PBSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
