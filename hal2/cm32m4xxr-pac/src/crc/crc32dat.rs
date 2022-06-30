#[doc = "Register `CRC32DAT` reader"]
pub struct R(crate::R<CRC32DAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC32DAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC32DAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC32DAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC32DAT` writer"]
pub struct W(crate::W<CRC32DAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC32DAT_SPEC>;
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
impl From<crate::W<CRC32DAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC32DAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC32DAT` reader - CRC32DAT"]
pub type CRC32DAT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CRC32DAT` writer - CRC32DAT"]
pub type CRC32DAT_W<'a> = crate::FieldWriter<'a, u32, CRC32DAT_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - CRC32DAT"]
    #[inline(always)]
    pub fn crc32dat(&self) -> CRC32DAT_R {
        CRC32DAT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC32DAT"]
    #[inline(always)]
    pub fn crc32dat(&mut self) -> CRC32DAT_W {
        CRC32DAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC32DAT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc32dat](index.html) module"]
pub struct CRC32DAT_SPEC;
impl crate::RegisterSpec for CRC32DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc32dat::R](R) reader structure"]
impl crate::Readable for CRC32DAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc32dat::W](W) writer structure"]
impl crate::Writable for CRC32DAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC32DAT to value 0xffff_ffff"]
impl crate::Resettable for CRC32DAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
