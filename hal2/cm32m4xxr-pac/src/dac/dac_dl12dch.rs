#[doc = "Register `DAC_DL12DCH` reader"]
pub struct R(crate::R<DAC_DL12DCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_DL12DCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_DL12DCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_DL12DCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_DL12DCH` writer"]
pub struct W(crate::W<DAC_DL12DCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_DL12DCH_SPEC>;
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
impl From<crate::W<DAC_DL12DCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_DL12DCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACCH1D` reader - DACCH1D"]
pub type DACCH1D_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DACCH1D` writer - DACCH1D"]
pub type DACCH1D_W<'a> = crate::FieldWriter<'a, u32, DAC_DL12DCH_SPEC, u16, u16, 12, 4>;
#[doc = "Field `DACCH2D` reader - DACCH2D"]
pub type DACCH2D_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DACCH2D` writer - DACCH2D"]
pub type DACCH2D_W<'a> = crate::FieldWriter<'a, u32, DAC_DL12DCH_SPEC, u16, u16, 12, 20>;
impl R {
    #[doc = "Bits 4:15 - DACCH1D"]
    #[inline(always)]
    pub fn dacch1d(&self) -> DACCH1D_R {
        DACCH1D_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - DACCH2D"]
    #[inline(always)]
    pub fn dacch2d(&self) -> DACCH2D_R {
        DACCH2D_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - DACCH1D"]
    #[inline(always)]
    pub fn dacch1d(&mut self) -> DACCH1D_W {
        DACCH1D_W::new(self)
    }
    #[doc = "Bits 20:31 - DACCH2D"]
    #[inline(always)]
    pub fn dacch2d(&mut self) -> DACCH2D_W {
        DACCH2D_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC_DL12DCH\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_dl12dch](index.html) module"]
pub struct DAC_DL12DCH_SPEC;
impl crate::RegisterSpec for DAC_DL12DCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_dl12dch::R](R) reader structure"]
impl crate::Readable for DAC_DL12DCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_dl12dch::W](W) writer structure"]
impl crate::Writable for DAC_DL12DCH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC_DL12DCH to value 0"]
impl crate::Resettable for DAC_DL12DCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
