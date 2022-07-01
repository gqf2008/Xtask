#[doc = "Register `LRC` reader"]
pub struct R(crate::R<LRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LRC` writer"]
pub struct W(crate::W<LRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LRC_SPEC>;
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
impl From<crate::W<LRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LRCDAT` reader - LRCDAT"]
pub type LRCDAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LRCDAT` writer - LRCDAT"]
pub type LRCDAT_W<'a> = crate::FieldWriter<'a, u32, LRC_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - LRCDAT"]
    #[inline(always)]
    pub fn lrcdat(&self) -> LRCDAT_R {
        LRCDAT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LRCDAT"]
    #[inline(always)]
    pub fn lrcdat(&mut self) -> LRCDAT_W {
        LRCDAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LRC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lrc](index.html) module"]
pub struct LRC_SPEC;
impl crate::RegisterSpec for LRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lrc::R](R) reader structure"]
impl crate::Readable for LRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lrc::W](W) writer structure"]
impl crate::Writable for LRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LRC to value 0"]
impl crate::Resettable for LRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
