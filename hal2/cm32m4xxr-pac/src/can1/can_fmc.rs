#[doc = "Register `CAN_FMC` reader"]
pub struct R(crate::R<CAN_FMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_FMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAN_FMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAN_FMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_FMC` writer"]
pub struct W(crate::W<CAN_FMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_FMC_SPEC>;
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
impl From<crate::W<CAN_FMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAN_FMC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FINITM` reader - FINITM"]
pub type FINITM_R = crate::BitReader<bool>;
#[doc = "Field `FINITM` writer - FINITM"]
pub type FINITM_W<'a> = crate::BitWriter<'a, u32, CAN_FMC_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - FINITM"]
    #[inline(always)]
    pub fn finitm(&self) -> FINITM_R {
        FINITM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FINITM"]
    #[inline(always)]
    pub fn finitm(&mut self) -> FINITM_W {
        FINITM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN_FMC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_fmc](index.html) module"]
pub struct CAN_FMC_SPEC;
impl crate::RegisterSpec for CAN_FMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_fmc::R](R) reader structure"]
impl crate::Readable for CAN_FMC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_fmc::W](W) writer structure"]
impl crate::Writable for CAN_FMC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_FMC to value 0x01"]
impl crate::Resettable for CAN_FMC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
