#[doc = "Register `GPIOx_SR_CFG` reader"]
pub struct R(crate::R<GPIOX_SR_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOX_SR_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOX_SR_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOX_SR_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOx_SR_CFG` writer"]
pub struct W(crate::W<GPIOX_SR_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOX_SR_CFG_SPEC>;
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
impl From<crate::W<GPIOX_SR_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOX_SR_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SR_CFG0` reader - SR_CFG0"]
pub type SR_CFG0_R = crate::BitReader<bool>;
#[doc = "Field `SR_CFG0` writer - SR_CFG0"]
pub type SR_CFG0_W<'a> = crate::BitWriter<'a, u32, GPIOX_SR_CFG_SPEC, bool, 0>;
#[doc = "Field `SR_CFG1` reader - SR_CFG1"]
pub type SR_CFG1_R = crate::BitReader<bool>;
#[doc = "Field `SR_CFG1` writer - SR_CFG1"]
pub type SR_CFG1_W<'a> = crate::BitWriter<'a, u32, GPIOX_SR_CFG_SPEC, bool, 1>;
#[doc = "Field `SR_CFG2` reader - SR_CFG2"]
pub type SR_CFG2_R = crate::BitReader<bool>;
#[doc = "Field `SR_CFG2` writer - SR_CFG2"]
pub type SR_CFG2_W<'a> = crate::BitWriter<'a, u32, GPIOX_SR_CFG_SPEC, bool, 2>;
#[doc = "Field `SR_CFG3` reader - SR_CFG3"]
pub type SR_CFG3_R = crate::BitReader<bool>;
#[doc = "Field `SR_CFG3` writer - SR_CFG3"]
pub type SR_CFG3_W<'a> = crate::BitWriter<'a, u32, GPIOX_SR_CFG_SPEC, bool, 3>;
#[doc = "Field `SR_CFG4` reader - SR_CFG4"]
pub type SR_CFG4_R = crate::BitReader<bool>;
#[doc = "Field `SR_CFG4` writer - SR_CFG4"]
pub type SR_CFG4_W<'a> = crate::BitWriter<'a, u32, GPIOX_SR_CFG_SPEC, bool, 4>;
#[doc = "Field `SR_CFG5` reader - SR_CFG5"]
pub type SR_CFG5_R = crate::BitReader<bool>;
#[doc = "Field `SR_CFG5` writer - SR_CFG5"]
pub type SR_CFG5_W<'a> = crate::BitWriter<'a, u32, GPIOX_SR_CFG_SPEC, bool, 5>;
#[doc = "Field `SR_CFG6` reader - SR_CFG6"]
pub type SR_CFG6_R = crate::BitReader<bool>;
#[doc = "Field `SR_CFG6` writer - SR_CFG6"]
pub type SR_CFG6_W<'a> = crate::BitWriter<'a, u32, GPIOX_SR_CFG_SPEC, bool, 6>;
#[doc = "Field `SR_CFG7` reader - SR_CFG7"]
pub type SR_CFG7_R = crate::BitReader<bool>;
#[doc = "Field `SR_CFG7` writer - SR_CFG7"]
pub type SR_CFG7_W<'a> = crate::BitWriter<'a, u32, GPIOX_SR_CFG_SPEC, bool, 7>;
#[doc = "Field `SR_CFG8` reader - SR_CFG8"]
pub type SR_CFG8_R = crate::BitReader<bool>;
#[doc = "Field `SR_CFG8` writer - SR_CFG8"]
pub type SR_CFG8_W<'a> = crate::BitWriter<'a, u32, GPIOX_SR_CFG_SPEC, bool, 8>;
#[doc = "Field `SR_CFG9` reader - SR_CFG9"]
pub type SR_CFG9_R = crate::BitReader<bool>;
#[doc = "Field `SR_CFG9` writer - SR_CFG9"]
pub type SR_CFG9_W<'a> = crate::BitWriter<'a, u32, GPIOX_SR_CFG_SPEC, bool, 9>;
#[doc = "Field `SR_CFG10` reader - SR_CFG10"]
pub type SR_CFG10_R = crate::BitReader<bool>;
#[doc = "Field `SR_CFG10` writer - SR_CFG10"]
pub type SR_CFG10_W<'a> = crate::BitWriter<'a, u32, GPIOX_SR_CFG_SPEC, bool, 10>;
#[doc = "Field `SR_CFG11` reader - SR_CFG11"]
pub type SR_CFG11_R = crate::BitReader<bool>;
#[doc = "Field `SR_CFG11` writer - SR_CFG11"]
pub type SR_CFG11_W<'a> = crate::BitWriter<'a, u32, GPIOX_SR_CFG_SPEC, bool, 11>;
#[doc = "Field `SR_CFG12` reader - SR_CFG12"]
pub type SR_CFG12_R = crate::BitReader<bool>;
#[doc = "Field `SR_CFG12` writer - SR_CFG12"]
pub type SR_CFG12_W<'a> = crate::BitWriter<'a, u32, GPIOX_SR_CFG_SPEC, bool, 12>;
#[doc = "Field `SR_CFG13` reader - SR_CFG13"]
pub type SR_CFG13_R = crate::BitReader<bool>;
#[doc = "Field `SR_CFG13` writer - SR_CFG13"]
pub type SR_CFG13_W<'a> = crate::BitWriter<'a, u32, GPIOX_SR_CFG_SPEC, bool, 13>;
#[doc = "Field `SR_CFG14` reader - SR_CFG14"]
pub type SR_CFG14_R = crate::BitReader<bool>;
#[doc = "Field `SR_CFG14` writer - SR_CFG14"]
pub type SR_CFG14_W<'a> = crate::BitWriter<'a, u32, GPIOX_SR_CFG_SPEC, bool, 14>;
#[doc = "Field `SR_CFG15` reader - SR_CFG15"]
pub type SR_CFG15_R = crate::BitReader<bool>;
#[doc = "Field `SR_CFG15` writer - SR_CFG15"]
pub type SR_CFG15_W<'a> = crate::BitWriter<'a, u32, GPIOX_SR_CFG_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - SR_CFG0"]
    #[inline(always)]
    pub fn sr_cfg0(&self) -> SR_CFG0_R {
        SR_CFG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SR_CFG1"]
    #[inline(always)]
    pub fn sr_cfg1(&self) -> SR_CFG1_R {
        SR_CFG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SR_CFG2"]
    #[inline(always)]
    pub fn sr_cfg2(&self) -> SR_CFG2_R {
        SR_CFG2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SR_CFG3"]
    #[inline(always)]
    pub fn sr_cfg3(&self) -> SR_CFG3_R {
        SR_CFG3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SR_CFG4"]
    #[inline(always)]
    pub fn sr_cfg4(&self) -> SR_CFG4_R {
        SR_CFG4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SR_CFG5"]
    #[inline(always)]
    pub fn sr_cfg5(&self) -> SR_CFG5_R {
        SR_CFG5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SR_CFG6"]
    #[inline(always)]
    pub fn sr_cfg6(&self) -> SR_CFG6_R {
        SR_CFG6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SR_CFG7"]
    #[inline(always)]
    pub fn sr_cfg7(&self) -> SR_CFG7_R {
        SR_CFG7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SR_CFG8"]
    #[inline(always)]
    pub fn sr_cfg8(&self) -> SR_CFG8_R {
        SR_CFG8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SR_CFG9"]
    #[inline(always)]
    pub fn sr_cfg9(&self) -> SR_CFG9_R {
        SR_CFG9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SR_CFG10"]
    #[inline(always)]
    pub fn sr_cfg10(&self) -> SR_CFG10_R {
        SR_CFG10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SR_CFG11"]
    #[inline(always)]
    pub fn sr_cfg11(&self) -> SR_CFG11_R {
        SR_CFG11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SR_CFG12"]
    #[inline(always)]
    pub fn sr_cfg12(&self) -> SR_CFG12_R {
        SR_CFG12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SR_CFG13"]
    #[inline(always)]
    pub fn sr_cfg13(&self) -> SR_CFG13_R {
        SR_CFG13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SR_CFG14"]
    #[inline(always)]
    pub fn sr_cfg14(&self) -> SR_CFG14_R {
        SR_CFG14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SR_CFG15"]
    #[inline(always)]
    pub fn sr_cfg15(&self) -> SR_CFG15_R {
        SR_CFG15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SR_CFG0"]
    #[inline(always)]
    pub fn sr_cfg0(&mut self) -> SR_CFG0_W {
        SR_CFG0_W::new(self)
    }
    #[doc = "Bit 1 - SR_CFG1"]
    #[inline(always)]
    pub fn sr_cfg1(&mut self) -> SR_CFG1_W {
        SR_CFG1_W::new(self)
    }
    #[doc = "Bit 2 - SR_CFG2"]
    #[inline(always)]
    pub fn sr_cfg2(&mut self) -> SR_CFG2_W {
        SR_CFG2_W::new(self)
    }
    #[doc = "Bit 3 - SR_CFG3"]
    #[inline(always)]
    pub fn sr_cfg3(&mut self) -> SR_CFG3_W {
        SR_CFG3_W::new(self)
    }
    #[doc = "Bit 4 - SR_CFG4"]
    #[inline(always)]
    pub fn sr_cfg4(&mut self) -> SR_CFG4_W {
        SR_CFG4_W::new(self)
    }
    #[doc = "Bit 5 - SR_CFG5"]
    #[inline(always)]
    pub fn sr_cfg5(&mut self) -> SR_CFG5_W {
        SR_CFG5_W::new(self)
    }
    #[doc = "Bit 6 - SR_CFG6"]
    #[inline(always)]
    pub fn sr_cfg6(&mut self) -> SR_CFG6_W {
        SR_CFG6_W::new(self)
    }
    #[doc = "Bit 7 - SR_CFG7"]
    #[inline(always)]
    pub fn sr_cfg7(&mut self) -> SR_CFG7_W {
        SR_CFG7_W::new(self)
    }
    #[doc = "Bit 8 - SR_CFG8"]
    #[inline(always)]
    pub fn sr_cfg8(&mut self) -> SR_CFG8_W {
        SR_CFG8_W::new(self)
    }
    #[doc = "Bit 9 - SR_CFG9"]
    #[inline(always)]
    pub fn sr_cfg9(&mut self) -> SR_CFG9_W {
        SR_CFG9_W::new(self)
    }
    #[doc = "Bit 10 - SR_CFG10"]
    #[inline(always)]
    pub fn sr_cfg10(&mut self) -> SR_CFG10_W {
        SR_CFG10_W::new(self)
    }
    #[doc = "Bit 11 - SR_CFG11"]
    #[inline(always)]
    pub fn sr_cfg11(&mut self) -> SR_CFG11_W {
        SR_CFG11_W::new(self)
    }
    #[doc = "Bit 12 - SR_CFG12"]
    #[inline(always)]
    pub fn sr_cfg12(&mut self) -> SR_CFG12_W {
        SR_CFG12_W::new(self)
    }
    #[doc = "Bit 13 - SR_CFG13"]
    #[inline(always)]
    pub fn sr_cfg13(&mut self) -> SR_CFG13_W {
        SR_CFG13_W::new(self)
    }
    #[doc = "Bit 14 - SR_CFG14"]
    #[inline(always)]
    pub fn sr_cfg14(&mut self) -> SR_CFG14_W {
        SR_CFG14_W::new(self)
    }
    #[doc = "Bit 15 - SR_CFG15"]
    #[inline(always)]
    pub fn sr_cfg15(&mut self) -> SR_CFG15_W {
        SR_CFG15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOx_SR_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiox_sr_cfg](index.html) module"]
pub struct GPIOX_SR_CFG_SPEC;
impl crate::RegisterSpec for GPIOX_SR_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiox_sr_cfg::R](R) reader structure"]
impl crate::Readable for GPIOX_SR_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiox_sr_cfg::W](W) writer structure"]
impl crate::Writable for GPIOX_SR_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOx_SR_CFG to value 0x023f"]
impl crate::Resettable for GPIOX_SR_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x023f
    }
}
