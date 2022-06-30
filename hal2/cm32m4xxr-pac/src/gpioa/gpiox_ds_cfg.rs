#[doc = "Register `GPIOx_DS_CFG` reader"]
pub struct R(crate::R<GPIOX_DS_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOX_DS_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOX_DS_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOX_DS_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOx_DS_CFG` writer"]
pub struct W(crate::W<GPIOX_DS_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOX_DS_CFG_SPEC>;
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
impl From<crate::W<GPIOX_DS_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOX_DS_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DS_CFG0` reader - DS_CFG0"]
pub type DS_CFG0_R = crate::BitReader<bool>;
#[doc = "Field `DS_CFG0` writer - DS_CFG0"]
pub type DS_CFG0_W<'a> = crate::BitWriter<'a, u32, GPIOX_DS_CFG_SPEC, bool, 0>;
#[doc = "Field `DS_CFG1` reader - DS_CFG1"]
pub type DS_CFG1_R = crate::BitReader<bool>;
#[doc = "Field `DS_CFG1` writer - DS_CFG1"]
pub type DS_CFG1_W<'a> = crate::BitWriter<'a, u32, GPIOX_DS_CFG_SPEC, bool, 1>;
#[doc = "Field `DS_CFG2` reader - DS_CFG2"]
pub type DS_CFG2_R = crate::BitReader<bool>;
#[doc = "Field `DS_CFG2` writer - DS_CFG2"]
pub type DS_CFG2_W<'a> = crate::BitWriter<'a, u32, GPIOX_DS_CFG_SPEC, bool, 2>;
#[doc = "Field `DS_CFG3` reader - DS_CFG3"]
pub type DS_CFG3_R = crate::BitReader<bool>;
#[doc = "Field `DS_CFG3` writer - DS_CFG3"]
pub type DS_CFG3_W<'a> = crate::BitWriter<'a, u32, GPIOX_DS_CFG_SPEC, bool, 3>;
#[doc = "Field `DS_CFG4` reader - DS_CFG4"]
pub type DS_CFG4_R = crate::BitReader<bool>;
#[doc = "Field `DS_CFG4` writer - DS_CFG4"]
pub type DS_CFG4_W<'a> = crate::BitWriter<'a, u32, GPIOX_DS_CFG_SPEC, bool, 4>;
#[doc = "Field `DS_CFG5` reader - DS_CFG5"]
pub type DS_CFG5_R = crate::BitReader<bool>;
#[doc = "Field `DS_CFG5` writer - DS_CFG5"]
pub type DS_CFG5_W<'a> = crate::BitWriter<'a, u32, GPIOX_DS_CFG_SPEC, bool, 5>;
#[doc = "Field `DS_CFG6` reader - DS_CFG6"]
pub type DS_CFG6_R = crate::BitReader<bool>;
#[doc = "Field `DS_CFG6` writer - DS_CFG6"]
pub type DS_CFG6_W<'a> = crate::BitWriter<'a, u32, GPIOX_DS_CFG_SPEC, bool, 6>;
#[doc = "Field `DS_CFG7` reader - DS_CFG7"]
pub type DS_CFG7_R = crate::BitReader<bool>;
#[doc = "Field `DS_CFG7` writer - DS_CFG7"]
pub type DS_CFG7_W<'a> = crate::BitWriter<'a, u32, GPIOX_DS_CFG_SPEC, bool, 7>;
#[doc = "Field `DS_CFG8` reader - DS_CFG8"]
pub type DS_CFG8_R = crate::BitReader<bool>;
#[doc = "Field `DS_CFG8` writer - DS_CFG8"]
pub type DS_CFG8_W<'a> = crate::BitWriter<'a, u32, GPIOX_DS_CFG_SPEC, bool, 8>;
#[doc = "Field `DS_CFG9` reader - DS_CFG9"]
pub type DS_CFG9_R = crate::BitReader<bool>;
#[doc = "Field `DS_CFG9` writer - DS_CFG9"]
pub type DS_CFG9_W<'a> = crate::BitWriter<'a, u32, GPIOX_DS_CFG_SPEC, bool, 9>;
#[doc = "Field `DS_CFG10` reader - DS_CFG10"]
pub type DS_CFG10_R = crate::BitReader<bool>;
#[doc = "Field `DS_CFG10` writer - DS_CFG10"]
pub type DS_CFG10_W<'a> = crate::BitWriter<'a, u32, GPIOX_DS_CFG_SPEC, bool, 10>;
#[doc = "Field `DS_CFG11` reader - DS_CFG11"]
pub type DS_CFG11_R = crate::BitReader<bool>;
#[doc = "Field `DS_CFG11` writer - DS_CFG11"]
pub type DS_CFG11_W<'a> = crate::BitWriter<'a, u32, GPIOX_DS_CFG_SPEC, bool, 11>;
#[doc = "Field `DS_CFG12` reader - DS_CFG12"]
pub type DS_CFG12_R = crate::BitReader<bool>;
#[doc = "Field `DS_CFG12` writer - DS_CFG12"]
pub type DS_CFG12_W<'a> = crate::BitWriter<'a, u32, GPIOX_DS_CFG_SPEC, bool, 12>;
#[doc = "Field `DS_CFG13` reader - DS_CFG13"]
pub type DS_CFG13_R = crate::BitReader<bool>;
#[doc = "Field `DS_CFG13` writer - DS_CFG13"]
pub type DS_CFG13_W<'a> = crate::BitWriter<'a, u32, GPIOX_DS_CFG_SPEC, bool, 13>;
#[doc = "Field `DS_CFG14` reader - DS_CFG14"]
pub type DS_CFG14_R = crate::BitReader<bool>;
#[doc = "Field `DS_CFG14` writer - DS_CFG14"]
pub type DS_CFG14_W<'a> = crate::BitWriter<'a, u32, GPIOX_DS_CFG_SPEC, bool, 14>;
#[doc = "Field `DS_CFG15` reader - DS_CFG15"]
pub type DS_CFG15_R = crate::BitReader<bool>;
#[doc = "Field `DS_CFG15` writer - DS_CFG15"]
pub type DS_CFG15_W<'a> = crate::BitWriter<'a, u32, GPIOX_DS_CFG_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - DS_CFG0"]
    #[inline(always)]
    pub fn ds_cfg0(&self) -> DS_CFG0_R {
        DS_CFG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DS_CFG1"]
    #[inline(always)]
    pub fn ds_cfg1(&self) -> DS_CFG1_R {
        DS_CFG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DS_CFG2"]
    #[inline(always)]
    pub fn ds_cfg2(&self) -> DS_CFG2_R {
        DS_CFG2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DS_CFG3"]
    #[inline(always)]
    pub fn ds_cfg3(&self) -> DS_CFG3_R {
        DS_CFG3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DS_CFG4"]
    #[inline(always)]
    pub fn ds_cfg4(&self) -> DS_CFG4_R {
        DS_CFG4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DS_CFG5"]
    #[inline(always)]
    pub fn ds_cfg5(&self) -> DS_CFG5_R {
        DS_CFG5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DS_CFG6"]
    #[inline(always)]
    pub fn ds_cfg6(&self) -> DS_CFG6_R {
        DS_CFG6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DS_CFG7"]
    #[inline(always)]
    pub fn ds_cfg7(&self) -> DS_CFG7_R {
        DS_CFG7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DS_CFG8"]
    #[inline(always)]
    pub fn ds_cfg8(&self) -> DS_CFG8_R {
        DS_CFG8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DS_CFG9"]
    #[inline(always)]
    pub fn ds_cfg9(&self) -> DS_CFG9_R {
        DS_CFG9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DS_CFG10"]
    #[inline(always)]
    pub fn ds_cfg10(&self) -> DS_CFG10_R {
        DS_CFG10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DS_CFG11"]
    #[inline(always)]
    pub fn ds_cfg11(&self) -> DS_CFG11_R {
        DS_CFG11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DS_CFG12"]
    #[inline(always)]
    pub fn ds_cfg12(&self) -> DS_CFG12_R {
        DS_CFG12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DS_CFG13"]
    #[inline(always)]
    pub fn ds_cfg13(&self) -> DS_CFG13_R {
        DS_CFG13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DS_CFG14"]
    #[inline(always)]
    pub fn ds_cfg14(&self) -> DS_CFG14_R {
        DS_CFG14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DS_CFG15"]
    #[inline(always)]
    pub fn ds_cfg15(&self) -> DS_CFG15_R {
        DS_CFG15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DS_CFG0"]
    #[inline(always)]
    pub fn ds_cfg0(&mut self) -> DS_CFG0_W {
        DS_CFG0_W::new(self)
    }
    #[doc = "Bit 1 - DS_CFG1"]
    #[inline(always)]
    pub fn ds_cfg1(&mut self) -> DS_CFG1_W {
        DS_CFG1_W::new(self)
    }
    #[doc = "Bit 2 - DS_CFG2"]
    #[inline(always)]
    pub fn ds_cfg2(&mut self) -> DS_CFG2_W {
        DS_CFG2_W::new(self)
    }
    #[doc = "Bit 3 - DS_CFG3"]
    #[inline(always)]
    pub fn ds_cfg3(&mut self) -> DS_CFG3_W {
        DS_CFG3_W::new(self)
    }
    #[doc = "Bit 4 - DS_CFG4"]
    #[inline(always)]
    pub fn ds_cfg4(&mut self) -> DS_CFG4_W {
        DS_CFG4_W::new(self)
    }
    #[doc = "Bit 5 - DS_CFG5"]
    #[inline(always)]
    pub fn ds_cfg5(&mut self) -> DS_CFG5_W {
        DS_CFG5_W::new(self)
    }
    #[doc = "Bit 6 - DS_CFG6"]
    #[inline(always)]
    pub fn ds_cfg6(&mut self) -> DS_CFG6_W {
        DS_CFG6_W::new(self)
    }
    #[doc = "Bit 7 - DS_CFG7"]
    #[inline(always)]
    pub fn ds_cfg7(&mut self) -> DS_CFG7_W {
        DS_CFG7_W::new(self)
    }
    #[doc = "Bit 8 - DS_CFG8"]
    #[inline(always)]
    pub fn ds_cfg8(&mut self) -> DS_CFG8_W {
        DS_CFG8_W::new(self)
    }
    #[doc = "Bit 9 - DS_CFG9"]
    #[inline(always)]
    pub fn ds_cfg9(&mut self) -> DS_CFG9_W {
        DS_CFG9_W::new(self)
    }
    #[doc = "Bit 10 - DS_CFG10"]
    #[inline(always)]
    pub fn ds_cfg10(&mut self) -> DS_CFG10_W {
        DS_CFG10_W::new(self)
    }
    #[doc = "Bit 11 - DS_CFG11"]
    #[inline(always)]
    pub fn ds_cfg11(&mut self) -> DS_CFG11_W {
        DS_CFG11_W::new(self)
    }
    #[doc = "Bit 12 - DS_CFG12"]
    #[inline(always)]
    pub fn ds_cfg12(&mut self) -> DS_CFG12_W {
        DS_CFG12_W::new(self)
    }
    #[doc = "Bit 13 - DS_CFG13"]
    #[inline(always)]
    pub fn ds_cfg13(&mut self) -> DS_CFG13_W {
        DS_CFG13_W::new(self)
    }
    #[doc = "Bit 14 - DS_CFG14"]
    #[inline(always)]
    pub fn ds_cfg14(&mut self) -> DS_CFG14_W {
        DS_CFG14_W::new(self)
    }
    #[doc = "Bit 15 - DS_CFG15"]
    #[inline(always)]
    pub fn ds_cfg15(&mut self) -> DS_CFG15_W {
        DS_CFG15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOx_DS_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiox_ds_cfg](index.html) module"]
pub struct GPIOX_DS_CFG_SPEC;
impl crate::RegisterSpec for GPIOX_DS_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiox_ds_cfg::R](R) reader structure"]
impl crate::Readable for GPIOX_DS_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiox_ds_cfg::W](W) writer structure"]
impl crate::Writable for GPIOX_DS_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOx_DS_CFG to value 0xf03f"]
impl crate::Resettable for GPIOX_DS_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf03f
    }
}
