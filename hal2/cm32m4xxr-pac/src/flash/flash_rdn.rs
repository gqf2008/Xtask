#[doc = "Register `FLASH_RDN` reader"]
pub struct R(crate::R<FLASH_RDN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_RDN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_RDN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_RDN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_RDN` writer"]
pub struct W(crate::W<FLASH_RDN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_RDN_SPEC>;
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
impl From<crate::W<FLASH_RDN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_RDN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_RDN0` reader - FLASH_RDN0"]
pub type FLASH_RDN0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FLASH_RDN0` writer - FLASH_RDN0"]
pub type FLASH_RDN0_W<'a> = crate::FieldWriter<'a, u32, FLASH_RDN_SPEC, u16, u16, 9, 0>;
#[doc = "Field `FLASH_RDN1` reader - FLASH_RDN1"]
pub type FLASH_RDN1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FLASH_RDN1` writer - FLASH_RDN1"]
pub type FLASH_RDN1_W<'a> = crate::FieldWriter<'a, u32, FLASH_RDN_SPEC, u16, u16, 9, 16>;
impl R {
    #[doc = "Bits 0:8 - FLASH_RDN0"]
    #[inline(always)]
    pub fn flash_rdn0(&self) -> FLASH_RDN0_R {
        FLASH_RDN0_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - FLASH_RDN1"]
    #[inline(always)]
    pub fn flash_rdn1(&self) -> FLASH_RDN1_R {
        FLASH_RDN1_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - FLASH_RDN0"]
    #[inline(always)]
    pub fn flash_rdn0(&mut self) -> FLASH_RDN0_W {
        FLASH_RDN0_W::new(self)
    }
    #[doc = "Bits 16:24 - FLASH_RDN1"]
    #[inline(always)]
    pub fn flash_rdn1(&mut self) -> FLASH_RDN1_W {
        FLASH_RDN1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH_RDN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_rdn](index.html) module"]
pub struct FLASH_RDN_SPEC;
impl crate::RegisterSpec for FLASH_RDN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_rdn::R](R) reader structure"]
impl crate::Readable for FLASH_RDN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_rdn::W](W) writer structure"]
impl crate::Writable for FLASH_RDN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_RDN to value 0"]
impl crate::Resettable for FLASH_RDN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
