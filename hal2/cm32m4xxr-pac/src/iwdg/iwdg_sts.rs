#[doc = "Register `IWDG_STS` reader"]
pub struct R(crate::R<IWDG_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IWDG_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IWDG_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IWDG_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IWDG_STS` writer"]
pub struct W(crate::W<IWDG_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IWDG_STS_SPEC>;
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
impl From<crate::W<IWDG_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IWDG_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PVU` reader - PVU"]
pub type PVU_R = crate::BitReader<bool>;
#[doc = "Field `PVU` writer - PVU"]
pub type PVU_W<'a> = crate::BitWriter<'a, u32, IWDG_STS_SPEC, bool, 0>;
#[doc = "Field `CRVU` reader - CRVU"]
pub type CRVU_R = crate::BitReader<bool>;
#[doc = "Field `CRVU` writer - CRVU"]
pub type CRVU_W<'a> = crate::BitWriter<'a, u32, IWDG_STS_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - PVU"]
    #[inline(always)]
    pub fn pvu(&self) -> PVU_R {
        PVU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CRVU"]
    #[inline(always)]
    pub fn crvu(&self) -> CRVU_R {
        CRVU_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PVU"]
    #[inline(always)]
    pub fn pvu(&mut self) -> PVU_W {
        PVU_W::new(self)
    }
    #[doc = "Bit 1 - CRVU"]
    #[inline(always)]
    pub fn crvu(&mut self) -> CRVU_W {
        CRVU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IWDG_STS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iwdg_sts](index.html) module"]
pub struct IWDG_STS_SPEC;
impl crate::RegisterSpec for IWDG_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iwdg_sts::R](R) reader structure"]
impl crate::Readable for IWDG_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iwdg_sts::W](W) writer structure"]
impl crate::Writable for IWDG_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IWDG_STS to value 0"]
impl crate::Resettable for IWDG_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
