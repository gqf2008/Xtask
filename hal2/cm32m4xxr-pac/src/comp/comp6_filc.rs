#[doc = "Register `COMP6_FILC` reader"]
pub struct R(crate::R<COMP6_FILC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP6_FILC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP6_FILC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP6_FILC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP6_FILC` writer"]
pub struct W(crate::W<COMP6_FILC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP6_FILC_SPEC>;
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
impl From<crate::W<COMP6_FILC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP6_FILC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILEN` reader - FILEN"]
pub type FILEN_R = crate::BitReader<bool>;
#[doc = "Field `FILEN` writer - FILEN"]
pub type FILEN_W<'a> = crate::BitWriter<'a, u32, COMP6_FILC_SPEC, bool, 0>;
#[doc = "Field `THRESH` reader - THRESH"]
pub type THRESH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THRESH` writer - THRESH"]
pub type THRESH_W<'a> = crate::FieldWriter<'a, u32, COMP6_FILC_SPEC, u8, u8, 5, 1>;
#[doc = "Field `SAMPW` reader - SAMPW"]
pub type SAMPW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMPW` writer - SAMPW"]
pub type SAMPW_W<'a> = crate::FieldWriter<'a, u32, COMP6_FILC_SPEC, u8, u8, 5, 6>;
impl R {
    #[doc = "Bit 0 - FILEN"]
    #[inline(always)]
    pub fn filen(&self) -> FILEN_R {
        FILEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - THRESH"]
    #[inline(always)]
    pub fn thresh(&self) -> THRESH_R {
        THRESH_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - SAMPW"]
    #[inline(always)]
    pub fn sampw(&self) -> SAMPW_R {
        SAMPW_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FILEN"]
    #[inline(always)]
    pub fn filen(&mut self) -> FILEN_W {
        FILEN_W::new(self)
    }
    #[doc = "Bits 1:5 - THRESH"]
    #[inline(always)]
    pub fn thresh(&mut self) -> THRESH_W {
        THRESH_W::new(self)
    }
    #[doc = "Bits 6:10 - SAMPW"]
    #[inline(always)]
    pub fn sampw(&mut self) -> SAMPW_W {
        SAMPW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "COMP6_FILC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp6_filc](index.html) module"]
pub struct COMP6_FILC_SPEC;
impl crate::RegisterSpec for COMP6_FILC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp6_filc::R](R) reader structure"]
impl crate::Readable for COMP6_FILC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp6_filc::W](W) writer structure"]
impl crate::Writable for COMP6_FILC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP6_FILC to value 0"]
impl crate::Resettable for COMP6_FILC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
