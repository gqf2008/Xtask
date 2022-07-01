#[doc = "Register `CRC16D` reader"]
pub struct R(crate::R<CRC16D_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC16D_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC16D_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC16D_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC16D` writer"]
pub struct W(crate::W<CRC16D_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC16D_SPEC>;
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
impl From<crate::W<CRC16D_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC16D_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC16D` reader - CRC16D"]
pub type CRC16D_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CRC16D` writer - CRC16D"]
pub type CRC16D_W<'a> = crate::FieldWriter<'a, u32, CRC16D_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - CRC16D"]
    #[inline(always)]
    pub fn crc16d(&self) -> CRC16D_R {
        CRC16D_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC16D"]
    #[inline(always)]
    pub fn crc16d(&mut self) -> CRC16D_W {
        CRC16D_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC16D\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc16d](index.html) module"]
pub struct CRC16D_SPEC;
impl crate::RegisterSpec for CRC16D_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc16d::R](R) reader structure"]
impl crate::Readable for CRC16D_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc16d::W](W) writer structure"]
impl crate::Writable for CRC16D_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC16D to value 0"]
impl crate::Resettable for CRC16D_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
