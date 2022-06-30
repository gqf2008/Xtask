#[doc = "Register `GPIOx_PID` reader"]
pub struct R(crate::R<GPIOX_PID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOX_PID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOX_PID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOX_PID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOx_PID` writer"]
pub struct W(crate::W<GPIOX_PID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOX_PID_SPEC>;
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
impl From<crate::W<GPIOX_PID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOX_PID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PID0` reader - PID0"]
pub type PID0_R = crate::BitReader<bool>;
#[doc = "Field `PID0` writer - PID0"]
pub type PID0_W<'a> = crate::BitWriter<'a, u32, GPIOX_PID_SPEC, bool, 0>;
#[doc = "Field `PID1` reader - PID1"]
pub type PID1_R = crate::BitReader<bool>;
#[doc = "Field `PID1` writer - PID1"]
pub type PID1_W<'a> = crate::BitWriter<'a, u32, GPIOX_PID_SPEC, bool, 1>;
#[doc = "Field `PID2` reader - PID2"]
pub type PID2_R = crate::BitReader<bool>;
#[doc = "Field `PID2` writer - PID2"]
pub type PID2_W<'a> = crate::BitWriter<'a, u32, GPIOX_PID_SPEC, bool, 2>;
#[doc = "Field `PID3` reader - PID3"]
pub type PID3_R = crate::BitReader<bool>;
#[doc = "Field `PID3` writer - PID3"]
pub type PID3_W<'a> = crate::BitWriter<'a, u32, GPIOX_PID_SPEC, bool, 3>;
#[doc = "Field `PID4` reader - PID4"]
pub type PID4_R = crate::BitReader<bool>;
#[doc = "Field `PID4` writer - PID4"]
pub type PID4_W<'a> = crate::BitWriter<'a, u32, GPIOX_PID_SPEC, bool, 4>;
#[doc = "Field `PID5` reader - PID5"]
pub type PID5_R = crate::BitReader<bool>;
#[doc = "Field `PID5` writer - PID5"]
pub type PID5_W<'a> = crate::BitWriter<'a, u32, GPIOX_PID_SPEC, bool, 5>;
#[doc = "Field `PID6` reader - PID6"]
pub type PID6_R = crate::BitReader<bool>;
#[doc = "Field `PID6` writer - PID6"]
pub type PID6_W<'a> = crate::BitWriter<'a, u32, GPIOX_PID_SPEC, bool, 6>;
#[doc = "Field `PID7` reader - PID7"]
pub type PID7_R = crate::BitReader<bool>;
#[doc = "Field `PID7` writer - PID7"]
pub type PID7_W<'a> = crate::BitWriter<'a, u32, GPIOX_PID_SPEC, bool, 7>;
#[doc = "Field `PID8` reader - PID8"]
pub type PID8_R = crate::BitReader<bool>;
#[doc = "Field `PID8` writer - PID8"]
pub type PID8_W<'a> = crate::BitWriter<'a, u32, GPIOX_PID_SPEC, bool, 8>;
#[doc = "Field `PID9` reader - PID9"]
pub type PID9_R = crate::BitReader<bool>;
#[doc = "Field `PID9` writer - PID9"]
pub type PID9_W<'a> = crate::BitWriter<'a, u32, GPIOX_PID_SPEC, bool, 9>;
#[doc = "Field `PID10` reader - PID10"]
pub type PID10_R = crate::BitReader<bool>;
#[doc = "Field `PID10` writer - PID10"]
pub type PID10_W<'a> = crate::BitWriter<'a, u32, GPIOX_PID_SPEC, bool, 10>;
#[doc = "Field `PID11` reader - PID11"]
pub type PID11_R = crate::BitReader<bool>;
#[doc = "Field `PID11` writer - PID11"]
pub type PID11_W<'a> = crate::BitWriter<'a, u32, GPIOX_PID_SPEC, bool, 11>;
#[doc = "Field `PID12` reader - PID12"]
pub type PID12_R = crate::BitReader<bool>;
#[doc = "Field `PID12` writer - PID12"]
pub type PID12_W<'a> = crate::BitWriter<'a, u32, GPIOX_PID_SPEC, bool, 12>;
#[doc = "Field `PID13` reader - PID13"]
pub type PID13_R = crate::BitReader<bool>;
#[doc = "Field `PID13` writer - PID13"]
pub type PID13_W<'a> = crate::BitWriter<'a, u32, GPIOX_PID_SPEC, bool, 13>;
#[doc = "Field `PID14` reader - PID14"]
pub type PID14_R = crate::BitReader<bool>;
#[doc = "Field `PID14` writer - PID14"]
pub type PID14_W<'a> = crate::BitWriter<'a, u32, GPIOX_PID_SPEC, bool, 14>;
#[doc = "Field `PID15` reader - PID15"]
pub type PID15_R = crate::BitReader<bool>;
#[doc = "Field `PID15` writer - PID15"]
pub type PID15_W<'a> = crate::BitWriter<'a, u32, GPIOX_PID_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - PID0"]
    #[inline(always)]
    pub fn pid0(&self) -> PID0_R {
        PID0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PID1"]
    #[inline(always)]
    pub fn pid1(&self) -> PID1_R {
        PID1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PID2"]
    #[inline(always)]
    pub fn pid2(&self) -> PID2_R {
        PID2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PID3"]
    #[inline(always)]
    pub fn pid3(&self) -> PID3_R {
        PID3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PID4"]
    #[inline(always)]
    pub fn pid4(&self) -> PID4_R {
        PID4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PID5"]
    #[inline(always)]
    pub fn pid5(&self) -> PID5_R {
        PID5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PID6"]
    #[inline(always)]
    pub fn pid6(&self) -> PID6_R {
        PID6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PID7"]
    #[inline(always)]
    pub fn pid7(&self) -> PID7_R {
        PID7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PID8"]
    #[inline(always)]
    pub fn pid8(&self) -> PID8_R {
        PID8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PID9"]
    #[inline(always)]
    pub fn pid9(&self) -> PID9_R {
        PID9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PID10"]
    #[inline(always)]
    pub fn pid10(&self) -> PID10_R {
        PID10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PID11"]
    #[inline(always)]
    pub fn pid11(&self) -> PID11_R {
        PID11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PID12"]
    #[inline(always)]
    pub fn pid12(&self) -> PID12_R {
        PID12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PID13"]
    #[inline(always)]
    pub fn pid13(&self) -> PID13_R {
        PID13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PID14"]
    #[inline(always)]
    pub fn pid14(&self) -> PID14_R {
        PID14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PID15"]
    #[inline(always)]
    pub fn pid15(&self) -> PID15_R {
        PID15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PID0"]
    #[inline(always)]
    pub fn pid0(&mut self) -> PID0_W {
        PID0_W::new(self)
    }
    #[doc = "Bit 1 - PID1"]
    #[inline(always)]
    pub fn pid1(&mut self) -> PID1_W {
        PID1_W::new(self)
    }
    #[doc = "Bit 2 - PID2"]
    #[inline(always)]
    pub fn pid2(&mut self) -> PID2_W {
        PID2_W::new(self)
    }
    #[doc = "Bit 3 - PID3"]
    #[inline(always)]
    pub fn pid3(&mut self) -> PID3_W {
        PID3_W::new(self)
    }
    #[doc = "Bit 4 - PID4"]
    #[inline(always)]
    pub fn pid4(&mut self) -> PID4_W {
        PID4_W::new(self)
    }
    #[doc = "Bit 5 - PID5"]
    #[inline(always)]
    pub fn pid5(&mut self) -> PID5_W {
        PID5_W::new(self)
    }
    #[doc = "Bit 6 - PID6"]
    #[inline(always)]
    pub fn pid6(&mut self) -> PID6_W {
        PID6_W::new(self)
    }
    #[doc = "Bit 7 - PID7"]
    #[inline(always)]
    pub fn pid7(&mut self) -> PID7_W {
        PID7_W::new(self)
    }
    #[doc = "Bit 8 - PID8"]
    #[inline(always)]
    pub fn pid8(&mut self) -> PID8_W {
        PID8_W::new(self)
    }
    #[doc = "Bit 9 - PID9"]
    #[inline(always)]
    pub fn pid9(&mut self) -> PID9_W {
        PID9_W::new(self)
    }
    #[doc = "Bit 10 - PID10"]
    #[inline(always)]
    pub fn pid10(&mut self) -> PID10_W {
        PID10_W::new(self)
    }
    #[doc = "Bit 11 - PID11"]
    #[inline(always)]
    pub fn pid11(&mut self) -> PID11_W {
        PID11_W::new(self)
    }
    #[doc = "Bit 12 - PID12"]
    #[inline(always)]
    pub fn pid12(&mut self) -> PID12_W {
        PID12_W::new(self)
    }
    #[doc = "Bit 13 - PID13"]
    #[inline(always)]
    pub fn pid13(&mut self) -> PID13_W {
        PID13_W::new(self)
    }
    #[doc = "Bit 14 - PID14"]
    #[inline(always)]
    pub fn pid14(&mut self) -> PID14_W {
        PID14_W::new(self)
    }
    #[doc = "Bit 15 - PID15"]
    #[inline(always)]
    pub fn pid15(&mut self) -> PID15_W {
        PID15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOx_PID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiox_pid](index.html) module"]
pub struct GPIOX_PID_SPEC;
impl crate::RegisterSpec for GPIOX_PID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiox_pid::R](R) reader structure"]
impl crate::Readable for GPIOX_PID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiox_pid::W](W) writer structure"]
impl crate::Writable for GPIOX_PID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOx_PID to value 0"]
impl crate::Resettable for GPIOX_PID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
