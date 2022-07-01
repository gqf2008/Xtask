#[doc = "Register `CRC32IDAT` reader"]
pub struct R(crate::R<CRC32IDAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC32IDAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC32IDAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC32IDAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC32IDAT` writer"]
pub struct W(crate::W<CRC32IDAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC32IDAT_SPEC>;
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
impl From<crate::W<CRC32IDAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC32IDAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC32IDAT` reader - CRC32IDAT"]
pub type CRC32IDAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CRC32IDAT` writer - CRC32IDAT"]
pub type CRC32IDAT_W<'a> = crate::FieldWriter<'a, u32, CRC32IDAT_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - CRC32IDAT"]
    #[inline(always)]
    pub fn crc32idat(&self) -> CRC32IDAT_R {
        CRC32IDAT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CRC32IDAT"]
    #[inline(always)]
    pub fn crc32idat(&mut self) -> CRC32IDAT_W {
        CRC32IDAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC32IDAT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc32idat](index.html) module"]
pub struct CRC32IDAT_SPEC;
impl crate::RegisterSpec for CRC32IDAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc32idat::R](R) reader structure"]
impl crate::Readable for CRC32IDAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc32idat::W](W) writer structure"]
impl crate::Writable for CRC32IDAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC32IDAT to value 0"]
impl crate::Resettable for CRC32IDAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
