#[doc = "Register `BKP_CTRL` reader"]
pub struct R(crate::R<BKP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BKP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BKP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BKP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BKP_CTRL` writer"]
pub struct W(crate::W<BKP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BKP_CTRL_SPEC>;
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
impl From<crate::W<BKP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BKP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TP_EN` reader - TP_EN"]
pub type TP_EN_R = crate::BitReader<bool>;
#[doc = "Field `TP_EN` writer - TP_EN"]
pub type TP_EN_W<'a> = crate::BitWriter<'a, u32, BKP_CTRL_SPEC, bool, 0>;
#[doc = "Field `TP_ALEV` reader - TP_ALEV"]
pub type TP_ALEV_R = crate::BitReader<bool>;
#[doc = "Field `TP_ALEV` writer - TP_ALEV"]
pub type TP_ALEV_W<'a> = crate::BitWriter<'a, u32, BKP_CTRL_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - TP_EN"]
    #[inline(always)]
    pub fn tp_en(&self) -> TP_EN_R {
        TP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TP_ALEV"]
    #[inline(always)]
    pub fn tp_alev(&self) -> TP_ALEV_R {
        TP_ALEV_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TP_EN"]
    #[inline(always)]
    pub fn tp_en(&mut self) -> TP_EN_W {
        TP_EN_W::new(self)
    }
    #[doc = "Bit 1 - TP_ALEV"]
    #[inline(always)]
    pub fn tp_alev(&mut self) -> TP_ALEV_W {
        TP_ALEV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BKP_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp_ctrl](index.html) module"]
pub struct BKP_CTRL_SPEC;
impl crate::RegisterSpec for BKP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bkp_ctrl::R](R) reader structure"]
impl crate::Readable for BKP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bkp_ctrl::W](W) writer structure"]
impl crate::Writable for BKP_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BKP_CTRL to value 0"]
impl crate::Resettable for BKP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
