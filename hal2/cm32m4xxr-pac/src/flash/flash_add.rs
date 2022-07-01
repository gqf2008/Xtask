#[doc = "Register `FLASH_ADD` reader"]
pub struct R(crate::R<FLASH_ADD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_ADD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_ADD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_ADD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_ADD` writer"]
pub struct W(crate::W<FLASH_ADD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_ADD_SPEC>;
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
impl From<crate::W<FLASH_ADD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_ADD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FADD` reader - FADD"]
pub type FADD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FADD` writer - FADD"]
pub type FADD_W<'a> = crate::FieldWriter<'a, u32, FLASH_ADD_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - FADD"]
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FADD"]
    #[inline(always)]
    pub fn fadd(&mut self) -> FADD_W {
        FADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH_ADD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_add](index.html) module"]
pub struct FLASH_ADD_SPEC;
impl crate::RegisterSpec for FLASH_ADD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_add::R](R) reader structure"]
impl crate::Readable for FLASH_ADD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_add::W](W) writer structure"]
impl crate::Writable for FLASH_ADD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_ADD to value 0"]
impl crate::Resettable for FLASH_ADD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
