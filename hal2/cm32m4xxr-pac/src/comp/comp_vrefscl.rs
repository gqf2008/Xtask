#[doc = "Register `COMP_VREFSCL` reader"]
pub struct R(crate::R<COMP_VREFSCL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP_VREFSCL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP_VREFSCL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP_VREFSCL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP_VREFSCL` writer"]
pub struct W(crate::W<COMP_VREFSCL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP_VREFSCL_SPEC>;
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
impl From<crate::W<COMP_VREFSCL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP_VREFSCL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VV1EN` reader - VV1EN"]
pub type VV1EN_R = crate::BitReader<bool>;
#[doc = "Field `VV1EN` writer - VV1EN"]
pub type VV1EN_W<'a> = crate::BitWriter<'a, u32, COMP_VREFSCL_SPEC, bool, 0>;
#[doc = "Field `VV1TRM` reader - VV1TRM"]
pub type VV1TRM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VV1TRM` writer - VV1TRM"]
pub type VV1TRM_W<'a> = crate::FieldWriter<'a, u32, COMP_VREFSCL_SPEC, u8, u8, 6, 1>;
#[doc = "Field `VV2EN` reader - VV2EN"]
pub type VV2EN_R = crate::BitReader<bool>;
#[doc = "Field `VV2EN` writer - VV2EN"]
pub type VV2EN_W<'a> = crate::BitWriter<'a, u32, COMP_VREFSCL_SPEC, bool, 7>;
#[doc = "Field `VV2TRM` reader - VV2TRM"]
pub type VV2TRM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VV2TRM` writer - VV2TRM"]
pub type VV2TRM_W<'a> = crate::FieldWriter<'a, u32, COMP_VREFSCL_SPEC, u8, u8, 6, 8>;
impl R {
    #[doc = "Bit 0 - VV1EN"]
    #[inline(always)]
    pub fn vv1en(&self) -> VV1EN_R {
        VV1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - VV1TRM"]
    #[inline(always)]
    pub fn vv1trm(&self) -> VV1TRM_R {
        VV1TRM_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bit 7 - VV2EN"]
    #[inline(always)]
    pub fn vv2en(&self) -> VV2EN_R {
        VV2EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - VV2TRM"]
    #[inline(always)]
    pub fn vv2trm(&self) -> VV2TRM_R {
        VV2TRM_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - VV1EN"]
    #[inline(always)]
    pub fn vv1en(&mut self) -> VV1EN_W {
        VV1EN_W::new(self)
    }
    #[doc = "Bits 1:6 - VV1TRM"]
    #[inline(always)]
    pub fn vv1trm(&mut self) -> VV1TRM_W {
        VV1TRM_W::new(self)
    }
    #[doc = "Bit 7 - VV2EN"]
    #[inline(always)]
    pub fn vv2en(&mut self) -> VV2EN_W {
        VV2EN_W::new(self)
    }
    #[doc = "Bits 8:13 - VV2TRM"]
    #[inline(always)]
    pub fn vv2trm(&mut self) -> VV2TRM_W {
        VV2TRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "COMP_VREFSCL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_vrefscl](index.html) module"]
pub struct COMP_VREFSCL_SPEC;
impl crate::RegisterSpec for COMP_VREFSCL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp_vrefscl::R](R) reader structure"]
impl crate::Readable for COMP_VREFSCL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp_vrefscl::W](W) writer structure"]
impl crate::Writable for COMP_VREFSCL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP_VREFSCL to value 0"]
impl crate::Resettable for COMP_VREFSCL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
