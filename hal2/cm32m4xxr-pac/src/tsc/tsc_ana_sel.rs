#[doc = "Register `TSC_ANA_SEL` reader"]
pub struct R(crate::R<TSC_ANA_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSC_ANA_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSC_ANA_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSC_ANA_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSC_ANA_SEL` writer"]
pub struct W(crate::W<TSC_ANA_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSC_ANA_SEL_SPEC>;
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
impl From<crate::W<TSC_ANA_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSC_ANA_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SP_OPT` reader - SP_OPT"]
pub type SP_OPT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SP_OPT` writer - SP_OPT"]
pub type SP_OPT_W<'a> = crate::FieldWriter<'a, u32, TSC_ANA_SEL_SPEC, u8, u8, 2, 4>;
#[doc = "Field `PAD_OPT` reader - PAD_OPT"]
pub type PAD_OPT_R = crate::BitReader<bool>;
#[doc = "Field `PAD_OPT` writer - PAD_OPT"]
pub type PAD_OPT_W<'a> = crate::BitWriter<'a, u32, TSC_ANA_SEL_SPEC, bool, 6>;
impl R {
    #[doc = "Bits 4:5 - SP_OPT"]
    #[inline(always)]
    pub fn sp_opt(&self) -> SP_OPT_R {
        SP_OPT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - PAD_OPT"]
    #[inline(always)]
    pub fn pad_opt(&self) -> PAD_OPT_R {
        PAD_OPT_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:5 - SP_OPT"]
    #[inline(always)]
    pub fn sp_opt(&mut self) -> SP_OPT_W {
        SP_OPT_W::new(self)
    }
    #[doc = "Bit 6 - PAD_OPT"]
    #[inline(always)]
    pub fn pad_opt(&mut self) -> PAD_OPT_W {
        PAD_OPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSC_ANA_SEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsc_ana_sel](index.html) module"]
pub struct TSC_ANA_SEL_SPEC;
impl crate::RegisterSpec for TSC_ANA_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsc_ana_sel::R](R) reader structure"]
impl crate::Readable for TSC_ANA_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsc_ana_sel::W](W) writer structure"]
impl crate::Writable for TSC_ANA_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSC_ANA_SEL to value 0"]
impl crate::Resettable for TSC_ANA_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
