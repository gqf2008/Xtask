#[doc = "Register `TSC_ANA_CTRL` reader"]
pub struct R(crate::R<TSC_ANA_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSC_ANA_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSC_ANA_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSC_ANA_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSC_ANA_CTRL` writer"]
pub struct W(crate::W<TSC_ANA_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSC_ANA_CTRL_SPEC>;
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
impl From<crate::W<TSC_ANA_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSC_ANA_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_PAD_MUX` reader - SW_PAD_MUX"]
pub type SW_PAD_MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SW_PAD_MUX` writer - SW_PAD_MUX"]
pub type SW_PAD_MUX_W<'a> = crate::FieldWriter<'a, u32, TSC_ANA_CTRL_SPEC, u8, u8, 5, 0>;
#[doc = "Field `SW_TSC_EN` reader - SW_TSC_EN"]
pub type SW_TSC_EN_R = crate::BitReader<bool>;
#[doc = "Field `SW_TSC_EN` writer - SW_TSC_EN"]
pub type SW_TSC_EN_W<'a> = crate::BitWriter<'a, u32, TSC_ANA_CTRL_SPEC, bool, 5>;
impl R {
    #[doc = "Bits 0:4 - SW_PAD_MUX"]
    #[inline(always)]
    pub fn sw_pad_mux(&self) -> SW_PAD_MUX_R {
        SW_PAD_MUX_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - SW_TSC_EN"]
    #[inline(always)]
    pub fn sw_tsc_en(&self) -> SW_TSC_EN_R {
        SW_TSC_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - SW_PAD_MUX"]
    #[inline(always)]
    pub fn sw_pad_mux(&mut self) -> SW_PAD_MUX_W {
        SW_PAD_MUX_W::new(self)
    }
    #[doc = "Bit 5 - SW_TSC_EN"]
    #[inline(always)]
    pub fn sw_tsc_en(&mut self) -> SW_TSC_EN_W {
        SW_TSC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSC_ANA_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsc_ana_ctrl](index.html) module"]
pub struct TSC_ANA_CTRL_SPEC;
impl crate::RegisterSpec for TSC_ANA_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsc_ana_ctrl::R](R) reader structure"]
impl crate::Readable for TSC_ANA_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsc_ana_ctrl::W](W) writer structure"]
impl crate::Writable for TSC_ANA_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSC_ANA_CTRL to value 0"]
impl crate::Resettable for TSC_ANA_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
