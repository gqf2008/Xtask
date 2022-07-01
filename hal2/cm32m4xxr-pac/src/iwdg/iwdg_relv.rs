#[doc = "Register `IWDG_RELV` reader"]
pub struct R(crate::R<IWDG_RELV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IWDG_RELV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IWDG_RELV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IWDG_RELV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IWDG_RELV` writer"]
pub struct W(crate::W<IWDG_RELV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IWDG_RELV_SPEC>;
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
impl From<crate::W<IWDG_RELV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IWDG_RELV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REL` reader - REL"]
pub type REL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REL` writer - REL"]
pub type REL_W<'a> = crate::FieldWriter<'a, u32, IWDG_RELV_SPEC, u16, u16, 12, 0>;
impl R {
    #[doc = "Bits 0:11 - REL"]
    #[inline(always)]
    pub fn rel(&self) -> REL_R {
        REL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - REL"]
    #[inline(always)]
    pub fn rel(&mut self) -> REL_W {
        REL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IWDG_RELV\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iwdg_relv](index.html) module"]
pub struct IWDG_RELV_SPEC;
impl crate::RegisterSpec for IWDG_RELV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iwdg_relv::R](R) reader structure"]
impl crate::Readable for IWDG_RELV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iwdg_relv::W](W) writer structure"]
impl crate::Writable for IWDG_RELV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IWDG_RELV to value 0x0fff"]
impl crate::Resettable for IWDG_RELV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff
    }
}
