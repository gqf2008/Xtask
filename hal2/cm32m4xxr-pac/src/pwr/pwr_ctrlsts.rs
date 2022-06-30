#[doc = "Register `PWR_CTRLSTS` reader"]
pub struct R(crate::R<PWR_CTRLSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CTRLSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CTRLSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CTRLSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_CTRLSTS` writer"]
pub struct W(crate::W<PWR_CTRLSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_CTRLSTS_SPEC>;
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
impl From<crate::W<PWR_CTRLSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_CTRLSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUPF` reader - WKUPF"]
pub type WKUPF_R = crate::BitReader<bool>;
#[doc = "Field `WKUPF` writer - WKUPF"]
pub type WKUPF_W<'a> = crate::BitWriter<'a, u32, PWR_CTRLSTS_SPEC, bool, 0>;
#[doc = "Field `SBF` reader - SBF"]
pub type SBF_R = crate::BitReader<bool>;
#[doc = "Field `SBF` writer - SBF"]
pub type SBF_W<'a> = crate::BitWriter<'a, u32, PWR_CTRLSTS_SPEC, bool, 1>;
#[doc = "Field `PVDO` reader - PVDO"]
pub type PVDO_R = crate::BitReader<bool>;
#[doc = "Field `PVDO` writer - PVDO"]
pub type PVDO_W<'a> = crate::BitWriter<'a, u32, PWR_CTRLSTS_SPEC, bool, 2>;
#[doc = "Field `VBATF` reader - VBATF"]
pub type VBATF_R = crate::BitReader<bool>;
#[doc = "Field `VBATF` writer - VBATF"]
pub type VBATF_W<'a> = crate::BitWriter<'a, u32, PWR_CTRLSTS_SPEC, bool, 3>;
#[doc = "Field `WKUPEN` reader - WKUPEN"]
pub type WKUPEN_R = crate::BitReader<bool>;
#[doc = "Field `WKUPEN` writer - WKUPEN"]
pub type WKUPEN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRLSTS_SPEC, bool, 8>;
impl R {
    #[doc = "Bit 0 - WKUPF"]
    #[inline(always)]
    pub fn wkupf(&self) -> WKUPF_R {
        WKUPF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SBF"]
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PVDO"]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VBATF"]
    #[inline(always)]
    pub fn vbatf(&self) -> VBATF_R {
        VBATF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - WKUPEN"]
    #[inline(always)]
    pub fn wkupen(&self) -> WKUPEN_R {
        WKUPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WKUPF"]
    #[inline(always)]
    pub fn wkupf(&mut self) -> WKUPF_W {
        WKUPF_W::new(self)
    }
    #[doc = "Bit 1 - SBF"]
    #[inline(always)]
    pub fn sbf(&mut self) -> SBF_W {
        SBF_W::new(self)
    }
    #[doc = "Bit 2 - PVDO"]
    #[inline(always)]
    pub fn pvdo(&mut self) -> PVDO_W {
        PVDO_W::new(self)
    }
    #[doc = "Bit 3 - VBATF"]
    #[inline(always)]
    pub fn vbatf(&mut self) -> VBATF_W {
        VBATF_W::new(self)
    }
    #[doc = "Bit 8 - WKUPEN"]
    #[inline(always)]
    pub fn wkupen(&mut self) -> WKUPEN_W {
        WKUPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWR_CTRLSTS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_ctrlsts](index.html) module"]
pub struct PWR_CTRLSTS_SPEC;
impl crate::RegisterSpec for PWR_CTRLSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_ctrlsts::R](R) reader structure"]
impl crate::Readable for PWR_CTRLSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_ctrlsts::W](W) writer structure"]
impl crate::Writable for PWR_CTRLSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_CTRLSTS to value 0"]
impl crate::Resettable for PWR_CTRLSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
