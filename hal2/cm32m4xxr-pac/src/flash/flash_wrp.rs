#[doc = "Register `FLASH_WRP` reader"]
pub struct R(crate::R<FLASH_WRP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_WRP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_WRP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_WRP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_WRP` writer"]
pub struct W(crate::W<FLASH_WRP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_WRP_SPEC>;
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
impl From<crate::W<FLASH_WRP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_WRP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRPT` reader - WRPT"]
pub type WRPT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WRPT` writer - WRPT"]
pub type WRPT_W<'a> = crate::FieldWriter<'a, u32, FLASH_WRP_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - WRPT"]
    #[inline(always)]
    pub fn wrpt(&self) -> WRPT_R {
        WRPT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WRPT"]
    #[inline(always)]
    pub fn wrpt(&mut self) -> WRPT_W {
        WRPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH_WRP\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_wrp](index.html) module"]
pub struct FLASH_WRP_SPEC;
impl crate::RegisterSpec for FLASH_WRP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_wrp::R](R) reader structure"]
impl crate::Readable for FLASH_WRP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_wrp::W](W) writer structure"]
impl crate::Writable for FLASH_WRP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_WRP to value 0xffff_ffff"]
impl crate::Resettable for FLASH_WRP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
