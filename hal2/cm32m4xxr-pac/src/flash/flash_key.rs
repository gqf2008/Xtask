#[doc = "Register `FLASH_KEY` reader"]
pub struct R(crate::R<FLASH_KEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_KEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_KEY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_KEY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_KEY` writer"]
pub struct W(crate::W<FLASH_KEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_KEY_SPEC>;
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
impl From<crate::W<FLASH_KEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_KEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FKEY` reader - FKEY"]
pub type FKEY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FKEY` writer - FKEY"]
pub type FKEY_W<'a> = crate::FieldWriter<'a, u32, FLASH_KEY_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - FKEY"]
    #[inline(always)]
    pub fn fkey(&self) -> FKEY_R {
        FKEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FKEY"]
    #[inline(always)]
    pub fn fkey(&mut self) -> FKEY_W {
        FKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH_KEY\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_key](index.html) module"]
pub struct FLASH_KEY_SPEC;
impl crate::RegisterSpec for FLASH_KEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_key::R](R) reader structure"]
impl crate::Readable for FLASH_KEY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_key::W](W) writer structure"]
impl crate::Writable for FLASH_KEY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_KEY to value 0"]
impl crate::Resettable for FLASH_KEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
