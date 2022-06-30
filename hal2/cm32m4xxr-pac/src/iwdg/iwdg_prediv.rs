#[doc = "Register `IWDG_PREDIV` reader"]
pub struct R(crate::R<IWDG_PREDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IWDG_PREDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IWDG_PREDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IWDG_PREDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IWDG_PREDIV` writer"]
pub struct W(crate::W<IWDG_PREDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IWDG_PREDIV_SPEC>;
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
impl From<crate::W<IWDG_PREDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IWDG_PREDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD` reader - PD"]
pub type PD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PD` writer - PD"]
pub type PD_W<'a> = crate::FieldWriter<'a, u32, IWDG_PREDIV_SPEC, u8, u8, 3, 0>;
impl R {
    #[doc = "Bits 0:2 - PD"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PD"]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W {
        PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IWDG_PREDIV\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iwdg_prediv](index.html) module"]
pub struct IWDG_PREDIV_SPEC;
impl crate::RegisterSpec for IWDG_PREDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iwdg_prediv::R](R) reader structure"]
impl crate::Readable for IWDG_PREDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iwdg_prediv::W](W) writer structure"]
impl crate::Writable for IWDG_PREDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IWDG_PREDIV to value 0"]
impl crate::Resettable for IWDG_PREDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
