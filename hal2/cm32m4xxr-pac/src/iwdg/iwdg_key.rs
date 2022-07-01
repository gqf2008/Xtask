#[doc = "Register `IWDG_KEY` reader"]
pub struct R(crate::R<IWDG_KEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IWDG_KEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IWDG_KEY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IWDG_KEY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IWDG_KEY` writer"]
pub struct W(crate::W<IWDG_KEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IWDG_KEY_SPEC>;
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
impl From<crate::W<IWDG_KEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IWDG_KEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEYV` reader - KEYV"]
pub type KEYV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `KEYV` writer - KEYV"]
pub type KEYV_W<'a> = crate::FieldWriter<'a, u32, IWDG_KEY_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - KEYV"]
    #[inline(always)]
    pub fn keyv(&self) -> KEYV_R {
        KEYV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - KEYV"]
    #[inline(always)]
    pub fn keyv(&mut self) -> KEYV_W {
        KEYV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IWDG_KEY\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iwdg_key](index.html) module"]
pub struct IWDG_KEY_SPEC;
impl crate::RegisterSpec for IWDG_KEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iwdg_key::R](R) reader structure"]
impl crate::Readable for IWDG_KEY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iwdg_key::W](W) writer structure"]
impl crate::Writable for IWDG_KEY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IWDG_KEY to value 0"]
impl crate::Resettable for IWDG_KEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
