#[doc = "Register `GPIOx_PLOCK_CFG` reader"]
pub struct R(crate::R<GPIOX_PLOCK_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOX_PLOCK_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOX_PLOCK_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOX_PLOCK_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOx_PLOCK_CFG` writer"]
pub struct W(crate::W<GPIOX_PLOCK_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOX_PLOCK_CFG_SPEC>;
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
impl From<crate::W<GPIOX_PLOCK_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOX_PLOCK_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLOCK_CFG0` reader - PLOCK_CFG0"]
pub type PLOCK_CFG0_R = crate::BitReader<bool>;
#[doc = "Field `PLOCK_CFG0` writer - PLOCK_CFG0"]
pub type PLOCK_CFG0_W<'a> = crate::BitWriter<'a, u32, GPIOX_PLOCK_CFG_SPEC, bool, 0>;
#[doc = "Field `PLOCK_CFG1` reader - PLOCK_CFG1"]
pub type PLOCK_CFG1_R = crate::BitReader<bool>;
#[doc = "Field `PLOCK_CFG1` writer - PLOCK_CFG1"]
pub type PLOCK_CFG1_W<'a> = crate::BitWriter<'a, u32, GPIOX_PLOCK_CFG_SPEC, bool, 1>;
#[doc = "Field `PLOCK_CFG2` reader - PLOCK_CFG2"]
pub type PLOCK_CFG2_R = crate::BitReader<bool>;
#[doc = "Field `PLOCK_CFG2` writer - PLOCK_CFG2"]
pub type PLOCK_CFG2_W<'a> = crate::BitWriter<'a, u32, GPIOX_PLOCK_CFG_SPEC, bool, 2>;
#[doc = "Field `PLOCK_CFG3` reader - PLOCK_CFG3"]
pub type PLOCK_CFG3_R = crate::BitReader<bool>;
#[doc = "Field `PLOCK_CFG3` writer - PLOCK_CFG3"]
pub type PLOCK_CFG3_W<'a> = crate::BitWriter<'a, u32, GPIOX_PLOCK_CFG_SPEC, bool, 3>;
#[doc = "Field `PLOCK_CFG4` reader - PLOCK_CFG4"]
pub type PLOCK_CFG4_R = crate::BitReader<bool>;
#[doc = "Field `PLOCK_CFG4` writer - PLOCK_CFG4"]
pub type PLOCK_CFG4_W<'a> = crate::BitWriter<'a, u32, GPIOX_PLOCK_CFG_SPEC, bool, 4>;
#[doc = "Field `PLOCK_CFG5` reader - PLOCK_CFG5"]
pub type PLOCK_CFG5_R = crate::BitReader<bool>;
#[doc = "Field `PLOCK_CFG5` writer - PLOCK_CFG5"]
pub type PLOCK_CFG5_W<'a> = crate::BitWriter<'a, u32, GPIOX_PLOCK_CFG_SPEC, bool, 5>;
#[doc = "Field `PLOCK_CFG6` reader - PLOCK_CFG6"]
pub type PLOCK_CFG6_R = crate::BitReader<bool>;
#[doc = "Field `PLOCK_CFG6` writer - PLOCK_CFG6"]
pub type PLOCK_CFG6_W<'a> = crate::BitWriter<'a, u32, GPIOX_PLOCK_CFG_SPEC, bool, 6>;
#[doc = "Field `PLOCK_CFG7` reader - PLOCK_CFG7"]
pub type PLOCK_CFG7_R = crate::BitReader<bool>;
#[doc = "Field `PLOCK_CFG7` writer - PLOCK_CFG7"]
pub type PLOCK_CFG7_W<'a> = crate::BitWriter<'a, u32, GPIOX_PLOCK_CFG_SPEC, bool, 7>;
#[doc = "Field `PLOCK_CFG8` reader - PLOCK_CFG8"]
pub type PLOCK_CFG8_R = crate::BitReader<bool>;
#[doc = "Field `PLOCK_CFG8` writer - PLOCK_CFG8"]
pub type PLOCK_CFG8_W<'a> = crate::BitWriter<'a, u32, GPIOX_PLOCK_CFG_SPEC, bool, 8>;
#[doc = "Field `PLOCK_CFG9` reader - PLOCK_CFG9"]
pub type PLOCK_CFG9_R = crate::BitReader<bool>;
#[doc = "Field `PLOCK_CFG9` writer - PLOCK_CFG9"]
pub type PLOCK_CFG9_W<'a> = crate::BitWriter<'a, u32, GPIOX_PLOCK_CFG_SPEC, bool, 9>;
#[doc = "Field `PLOCK_CFG10` reader - PLOCK_CFG10"]
pub type PLOCK_CFG10_R = crate::BitReader<bool>;
#[doc = "Field `PLOCK_CFG10` writer - PLOCK_CFG10"]
pub type PLOCK_CFG10_W<'a> = crate::BitWriter<'a, u32, GPIOX_PLOCK_CFG_SPEC, bool, 10>;
#[doc = "Field `PLOCK_CFG11` reader - PLOCK_CFG11"]
pub type PLOCK_CFG11_R = crate::BitReader<bool>;
#[doc = "Field `PLOCK_CFG11` writer - PLOCK_CFG11"]
pub type PLOCK_CFG11_W<'a> = crate::BitWriter<'a, u32, GPIOX_PLOCK_CFG_SPEC, bool, 11>;
#[doc = "Field `PLOCK_CFG12` reader - PLOCK_CFG12"]
pub type PLOCK_CFG12_R = crate::BitReader<bool>;
#[doc = "Field `PLOCK_CFG12` writer - PLOCK_CFG12"]
pub type PLOCK_CFG12_W<'a> = crate::BitWriter<'a, u32, GPIOX_PLOCK_CFG_SPEC, bool, 12>;
#[doc = "Field `PLOCK_CFG13` reader - PLOCK_CFG13"]
pub type PLOCK_CFG13_R = crate::BitReader<bool>;
#[doc = "Field `PLOCK_CFG13` writer - PLOCK_CFG13"]
pub type PLOCK_CFG13_W<'a> = crate::BitWriter<'a, u32, GPIOX_PLOCK_CFG_SPEC, bool, 13>;
#[doc = "Field `PLOCK_CFG14` reader - PLOCK_CFG14"]
pub type PLOCK_CFG14_R = crate::BitReader<bool>;
#[doc = "Field `PLOCK_CFG14` writer - PLOCK_CFG14"]
pub type PLOCK_CFG14_W<'a> = crate::BitWriter<'a, u32, GPIOX_PLOCK_CFG_SPEC, bool, 14>;
#[doc = "Field `PLOCK_CFG15` reader - PLOCK_CFG15"]
pub type PLOCK_CFG15_R = crate::BitReader<bool>;
#[doc = "Field `PLOCK_CFG15` writer - PLOCK_CFG15"]
pub type PLOCK_CFG15_W<'a> = crate::BitWriter<'a, u32, GPIOX_PLOCK_CFG_SPEC, bool, 15>;
#[doc = "Field `PLOCKK_CFG` reader - PLOCKK_CFG"]
pub type PLOCKK_CFG_R = crate::BitReader<bool>;
#[doc = "Field `PLOCKK_CFG` writer - PLOCKK_CFG"]
pub type PLOCKK_CFG_W<'a> = crate::BitWriter<'a, u32, GPIOX_PLOCK_CFG_SPEC, bool, 16>;
impl R {
    #[doc = "Bit 0 - PLOCK_CFG0"]
    #[inline(always)]
    pub fn plock_cfg0(&self) -> PLOCK_CFG0_R {
        PLOCK_CFG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLOCK_CFG1"]
    #[inline(always)]
    pub fn plock_cfg1(&self) -> PLOCK_CFG1_R {
        PLOCK_CFG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PLOCK_CFG2"]
    #[inline(always)]
    pub fn plock_cfg2(&self) -> PLOCK_CFG2_R {
        PLOCK_CFG2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PLOCK_CFG3"]
    #[inline(always)]
    pub fn plock_cfg3(&self) -> PLOCK_CFG3_R {
        PLOCK_CFG3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLOCK_CFG4"]
    #[inline(always)]
    pub fn plock_cfg4(&self) -> PLOCK_CFG4_R {
        PLOCK_CFG4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLOCK_CFG5"]
    #[inline(always)]
    pub fn plock_cfg5(&self) -> PLOCK_CFG5_R {
        PLOCK_CFG5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLOCK_CFG6"]
    #[inline(always)]
    pub fn plock_cfg6(&self) -> PLOCK_CFG6_R {
        PLOCK_CFG6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLOCK_CFG7"]
    #[inline(always)]
    pub fn plock_cfg7(&self) -> PLOCK_CFG7_R {
        PLOCK_CFG7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PLOCK_CFG8"]
    #[inline(always)]
    pub fn plock_cfg8(&self) -> PLOCK_CFG8_R {
        PLOCK_CFG8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PLOCK_CFG9"]
    #[inline(always)]
    pub fn plock_cfg9(&self) -> PLOCK_CFG9_R {
        PLOCK_CFG9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PLOCK_CFG10"]
    #[inline(always)]
    pub fn plock_cfg10(&self) -> PLOCK_CFG10_R {
        PLOCK_CFG10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PLOCK_CFG11"]
    #[inline(always)]
    pub fn plock_cfg11(&self) -> PLOCK_CFG11_R {
        PLOCK_CFG11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PLOCK_CFG12"]
    #[inline(always)]
    pub fn plock_cfg12(&self) -> PLOCK_CFG12_R {
        PLOCK_CFG12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PLOCK_CFG13"]
    #[inline(always)]
    pub fn plock_cfg13(&self) -> PLOCK_CFG13_R {
        PLOCK_CFG13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PLOCK_CFG14"]
    #[inline(always)]
    pub fn plock_cfg14(&self) -> PLOCK_CFG14_R {
        PLOCK_CFG14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PLOCK_CFG15"]
    #[inline(always)]
    pub fn plock_cfg15(&self) -> PLOCK_CFG15_R {
        PLOCK_CFG15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PLOCKK_CFG"]
    #[inline(always)]
    pub fn plockk_cfg(&self) -> PLOCKK_CFG_R {
        PLOCKK_CFG_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLOCK_CFG0"]
    #[inline(always)]
    pub fn plock_cfg0(&mut self) -> PLOCK_CFG0_W {
        PLOCK_CFG0_W::new(self)
    }
    #[doc = "Bit 1 - PLOCK_CFG1"]
    #[inline(always)]
    pub fn plock_cfg1(&mut self) -> PLOCK_CFG1_W {
        PLOCK_CFG1_W::new(self)
    }
    #[doc = "Bit 2 - PLOCK_CFG2"]
    #[inline(always)]
    pub fn plock_cfg2(&mut self) -> PLOCK_CFG2_W {
        PLOCK_CFG2_W::new(self)
    }
    #[doc = "Bit 3 - PLOCK_CFG3"]
    #[inline(always)]
    pub fn plock_cfg3(&mut self) -> PLOCK_CFG3_W {
        PLOCK_CFG3_W::new(self)
    }
    #[doc = "Bit 4 - PLOCK_CFG4"]
    #[inline(always)]
    pub fn plock_cfg4(&mut self) -> PLOCK_CFG4_W {
        PLOCK_CFG4_W::new(self)
    }
    #[doc = "Bit 5 - PLOCK_CFG5"]
    #[inline(always)]
    pub fn plock_cfg5(&mut self) -> PLOCK_CFG5_W {
        PLOCK_CFG5_W::new(self)
    }
    #[doc = "Bit 6 - PLOCK_CFG6"]
    #[inline(always)]
    pub fn plock_cfg6(&mut self) -> PLOCK_CFG6_W {
        PLOCK_CFG6_W::new(self)
    }
    #[doc = "Bit 7 - PLOCK_CFG7"]
    #[inline(always)]
    pub fn plock_cfg7(&mut self) -> PLOCK_CFG7_W {
        PLOCK_CFG7_W::new(self)
    }
    #[doc = "Bit 8 - PLOCK_CFG8"]
    #[inline(always)]
    pub fn plock_cfg8(&mut self) -> PLOCK_CFG8_W {
        PLOCK_CFG8_W::new(self)
    }
    #[doc = "Bit 9 - PLOCK_CFG9"]
    #[inline(always)]
    pub fn plock_cfg9(&mut self) -> PLOCK_CFG9_W {
        PLOCK_CFG9_W::new(self)
    }
    #[doc = "Bit 10 - PLOCK_CFG10"]
    #[inline(always)]
    pub fn plock_cfg10(&mut self) -> PLOCK_CFG10_W {
        PLOCK_CFG10_W::new(self)
    }
    #[doc = "Bit 11 - PLOCK_CFG11"]
    #[inline(always)]
    pub fn plock_cfg11(&mut self) -> PLOCK_CFG11_W {
        PLOCK_CFG11_W::new(self)
    }
    #[doc = "Bit 12 - PLOCK_CFG12"]
    #[inline(always)]
    pub fn plock_cfg12(&mut self) -> PLOCK_CFG12_W {
        PLOCK_CFG12_W::new(self)
    }
    #[doc = "Bit 13 - PLOCK_CFG13"]
    #[inline(always)]
    pub fn plock_cfg13(&mut self) -> PLOCK_CFG13_W {
        PLOCK_CFG13_W::new(self)
    }
    #[doc = "Bit 14 - PLOCK_CFG14"]
    #[inline(always)]
    pub fn plock_cfg14(&mut self) -> PLOCK_CFG14_W {
        PLOCK_CFG14_W::new(self)
    }
    #[doc = "Bit 15 - PLOCK_CFG15"]
    #[inline(always)]
    pub fn plock_cfg15(&mut self) -> PLOCK_CFG15_W {
        PLOCK_CFG15_W::new(self)
    }
    #[doc = "Bit 16 - PLOCKK_CFG"]
    #[inline(always)]
    pub fn plockk_cfg(&mut self) -> PLOCKK_CFG_W {
        PLOCKK_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOx_PLOCK_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiox_plock_cfg](index.html) module"]
pub struct GPIOX_PLOCK_CFG_SPEC;
impl crate::RegisterSpec for GPIOX_PLOCK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiox_plock_cfg::R](R) reader structure"]
impl crate::Readable for GPIOX_PLOCK_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiox_plock_cfg::W](W) writer structure"]
impl crate::Writable for GPIOX_PLOCK_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOx_PLOCK_CFG to value 0"]
impl crate::Resettable for GPIOX_PLOCK_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
