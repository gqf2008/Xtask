#[doc = "Register `CRC32CTRL` reader"]
pub struct R(crate::R<CRC32CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC32CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC32CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC32CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC32CTRL` writer"]
pub struct W(crate::W<CRC32CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC32CTRL_SPEC>;
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
impl From<crate::W<CRC32CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC32CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET` reader - RESET"]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - RESET"]
pub type RESET_W<'a> = crate::BitWriter<'a, u32, CRC32CTRL_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - RESET"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RESET"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC32CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc32ctrl](index.html) module"]
pub struct CRC32CTRL_SPEC;
impl crate::RegisterSpec for CRC32CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc32ctrl::R](R) reader structure"]
impl crate::Readable for CRC32CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc32ctrl::W](W) writer structure"]
impl crate::Writable for CRC32CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC32CTRL to value 0"]
impl crate::Resettable for CRC32CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
