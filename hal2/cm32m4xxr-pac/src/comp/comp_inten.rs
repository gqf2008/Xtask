#[doc = "Register `COMP_INTEN` reader"]
pub struct R(crate::R<COMP_INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP_INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP_INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP_INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP_INTEN` writer"]
pub struct W(crate::W<COMP_INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP_INTEN_SPEC>;
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
impl From<crate::W<COMP_INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP_INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP0IEN` reader - CMP0IEN"]
pub type CMP0IEN_R = crate::BitReader<bool>;
#[doc = "Field `CMP0IEN` writer - CMP0IEN"]
pub type CMP0IEN_W<'a> = crate::BitWriter<'a, u32, COMP_INTEN_SPEC, bool, 0>;
#[doc = "Field `CMP1IEN` reader - CMP1IEN"]
pub type CMP1IEN_R = crate::BitReader<bool>;
#[doc = "Field `CMP1IEN` writer - CMP1IEN"]
pub type CMP1IEN_W<'a> = crate::BitWriter<'a, u32, COMP_INTEN_SPEC, bool, 1>;
#[doc = "Field `CMP2IEN` reader - CMP2IEN"]
pub type CMP2IEN_R = crate::BitReader<bool>;
#[doc = "Field `CMP2IEN` writer - CMP2IEN"]
pub type CMP2IEN_W<'a> = crate::BitWriter<'a, u32, COMP_INTEN_SPEC, bool, 2>;
#[doc = "Field `CMP3IEN` reader - CMP3IEN"]
pub type CMP3IEN_R = crate::BitReader<bool>;
#[doc = "Field `CMP3IEN` writer - CMP3IEN"]
pub type CMP3IEN_W<'a> = crate::BitWriter<'a, u32, COMP_INTEN_SPEC, bool, 3>;
#[doc = "Field `CMP4IEN` reader - CMP4IEN"]
pub type CMP4IEN_R = crate::BitReader<bool>;
#[doc = "Field `CMP4IEN` writer - CMP4IEN"]
pub type CMP4IEN_W<'a> = crate::BitWriter<'a, u32, COMP_INTEN_SPEC, bool, 4>;
#[doc = "Field `CMP5IEN` reader - CMP5IEN"]
pub type CMP5IEN_R = crate::BitReader<bool>;
#[doc = "Field `CMP5IEN` writer - CMP5IEN"]
pub type CMP5IEN_W<'a> = crate::BitWriter<'a, u32, COMP_INTEN_SPEC, bool, 5>;
#[doc = "Field `CMP6IEN` reader - CMP6IEN"]
pub type CMP6IEN_R = crate::BitReader<bool>;
#[doc = "Field `CMP6IEN` writer - CMP6IEN"]
pub type CMP6IEN_W<'a> = crate::BitWriter<'a, u32, COMP_INTEN_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 0 - CMP0IEN"]
    #[inline(always)]
    pub fn cmp0ien(&self) -> CMP0IEN_R {
        CMP0IEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMP1IEN"]
    #[inline(always)]
    pub fn cmp1ien(&self) -> CMP1IEN_R {
        CMP1IEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CMP2IEN"]
    #[inline(always)]
    pub fn cmp2ien(&self) -> CMP2IEN_R {
        CMP2IEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMP3IEN"]
    #[inline(always)]
    pub fn cmp3ien(&self) -> CMP3IEN_R {
        CMP3IEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CMP4IEN"]
    #[inline(always)]
    pub fn cmp4ien(&self) -> CMP4IEN_R {
        CMP4IEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CMP5IEN"]
    #[inline(always)]
    pub fn cmp5ien(&self) -> CMP5IEN_R {
        CMP5IEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMP6IEN"]
    #[inline(always)]
    pub fn cmp6ien(&self) -> CMP6IEN_R {
        CMP6IEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMP0IEN"]
    #[inline(always)]
    pub fn cmp0ien(&mut self) -> CMP0IEN_W {
        CMP0IEN_W::new(self)
    }
    #[doc = "Bit 1 - CMP1IEN"]
    #[inline(always)]
    pub fn cmp1ien(&mut self) -> CMP1IEN_W {
        CMP1IEN_W::new(self)
    }
    #[doc = "Bit 2 - CMP2IEN"]
    #[inline(always)]
    pub fn cmp2ien(&mut self) -> CMP2IEN_W {
        CMP2IEN_W::new(self)
    }
    #[doc = "Bit 3 - CMP3IEN"]
    #[inline(always)]
    pub fn cmp3ien(&mut self) -> CMP3IEN_W {
        CMP3IEN_W::new(self)
    }
    #[doc = "Bit 4 - CMP4IEN"]
    #[inline(always)]
    pub fn cmp4ien(&mut self) -> CMP4IEN_W {
        CMP4IEN_W::new(self)
    }
    #[doc = "Bit 5 - CMP5IEN"]
    #[inline(always)]
    pub fn cmp5ien(&mut self) -> CMP5IEN_W {
        CMP5IEN_W::new(self)
    }
    #[doc = "Bit 6 - CMP6IEN"]
    #[inline(always)]
    pub fn cmp6ien(&mut self) -> CMP6IEN_W {
        CMP6IEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "COMP_INTEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_inten](index.html) module"]
pub struct COMP_INTEN_SPEC;
impl crate::RegisterSpec for COMP_INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp_inten::R](R) reader structure"]
impl crate::Readable for COMP_INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp_inten::W](W) writer structure"]
impl crate::Writable for COMP_INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP_INTEN to value 0"]
impl crate::Resettable for COMP_INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
