#[doc = "Register `FLASH_ECCR` reader"]
pub struct R(crate::R<FLASH_ECCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_ECCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_ECCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_ECCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_ECCR` writer"]
pub struct W(crate::W<FLASH_ECCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_ECCR_SPEC>;
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
impl From<crate::W<FLASH_ECCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_ECCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECCLW` reader - ECCLW"]
pub type ECCLW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ECCLW` writer - ECCLW"]
pub type ECCLW_W<'a> = crate::FieldWriter<'a, u32, FLASH_ECCR_SPEC, u8, u8, 6, 0>;
#[doc = "Field `ECCHW` reader - ECCHW"]
pub type ECCHW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ECCHW` writer - ECCHW"]
pub type ECCHW_W<'a> = crate::FieldWriter<'a, u32, FLASH_ECCR_SPEC, u8, u8, 6, 8>;
impl R {
    #[doc = "Bits 0:5 - ECCLW"]
    #[inline(always)]
    pub fn ecclw(&self) -> ECCLW_R {
        ECCLW_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - ECCHW"]
    #[inline(always)]
    pub fn ecchw(&self) -> ECCHW_R {
        ECCHW_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - ECCLW"]
    #[inline(always)]
    pub fn ecclw(&mut self) -> ECCLW_W {
        ECCLW_W::new(self)
    }
    #[doc = "Bits 8:13 - ECCHW"]
    #[inline(always)]
    pub fn ecchw(&mut self) -> ECCHW_W {
        ECCHW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH_ECCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_eccr](index.html) module"]
pub struct FLASH_ECCR_SPEC;
impl crate::RegisterSpec for FLASH_ECCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_eccr::R](R) reader structure"]
impl crate::Readable for FLASH_ECCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_eccr::W](W) writer structure"]
impl crate::Writable for FLASH_ECCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_ECCR to value 0"]
impl crate::Resettable for FLASH_ECCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
