#[doc = "Register `FLASH_CAHR` reader"]
pub struct R(crate::R<FLASH_CAHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_CAHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_CAHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_CAHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_CAHR` writer"]
pub struct W(crate::W<FLASH_CAHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_CAHR_SPEC>;
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
impl From<crate::W<FLASH_CAHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_CAHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKSTRT` reader - LOCKSTRT"]
pub type LOCKSTRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOCKSTRT` writer - LOCKSTRT"]
pub type LOCKSTRT_W<'a> = crate::FieldWriter<'a, u32, FLASH_CAHR_SPEC, u8, u8, 4, 0>;
#[doc = "Field `LOCKSTOP` reader - LOCKSTOP"]
pub type LOCKSTOP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOCKSTOP` writer - LOCKSTOP"]
pub type LOCKSTOP_W<'a> = crate::FieldWriter<'a, u32, FLASH_CAHR_SPEC, u8, u8, 4, 4>;
impl R {
    #[doc = "Bits 0:3 - LOCKSTRT"]
    #[inline(always)]
    pub fn lockstrt(&self) -> LOCKSTRT_R {
        LOCKSTRT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - LOCKSTOP"]
    #[inline(always)]
    pub fn lockstop(&self) -> LOCKSTOP_R {
        LOCKSTOP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - LOCKSTRT"]
    #[inline(always)]
    pub fn lockstrt(&mut self) -> LOCKSTRT_W {
        LOCKSTRT_W::new(self)
    }
    #[doc = "Bits 4:7 - LOCKSTOP"]
    #[inline(always)]
    pub fn lockstop(&mut self) -> LOCKSTOP_W {
        LOCKSTOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH_CAHR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_cahr](index.html) module"]
pub struct FLASH_CAHR_SPEC;
impl crate::RegisterSpec for FLASH_CAHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_cahr::R](R) reader structure"]
impl crate::Readable for FLASH_CAHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_cahr::W](W) writer structure"]
impl crate::Writable for FLASH_CAHR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_CAHR to value 0"]
impl crate::Resettable for FLASH_CAHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
