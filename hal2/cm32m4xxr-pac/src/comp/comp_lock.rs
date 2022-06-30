#[doc = "Register `COMP_LOCK` reader"]
pub struct R(crate::R<COMP_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP_LOCK` writer"]
pub struct W(crate::W<COMP_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP_LOCK_SPEC>;
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
impl From<crate::W<COMP_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP1LK` reader - CMP1LK"]
pub type CMP1LK_R = crate::BitReader<bool>;
#[doc = "Field `CMP1LK` writer - CMP1LK"]
pub type CMP1LK_W<'a> = crate::BitWriter<'a, u32, COMP_LOCK_SPEC, bool, 0>;
#[doc = "Field `CMP2LK` reader - CMP2LK"]
pub type CMP2LK_R = crate::BitReader<bool>;
#[doc = "Field `CMP2LK` writer - CMP2LK"]
pub type CMP2LK_W<'a> = crate::BitWriter<'a, u32, COMP_LOCK_SPEC, bool, 1>;
#[doc = "Field `CMP3LK` reader - CMP3LK"]
pub type CMP3LK_R = crate::BitReader<bool>;
#[doc = "Field `CMP3LK` writer - CMP3LK"]
pub type CMP3LK_W<'a> = crate::BitWriter<'a, u32, COMP_LOCK_SPEC, bool, 2>;
#[doc = "Field `CMP4LK` reader - CMP4LK"]
pub type CMP4LK_R = crate::BitReader<bool>;
#[doc = "Field `CMP4LK` writer - CMP4LK"]
pub type CMP4LK_W<'a> = crate::BitWriter<'a, u32, COMP_LOCK_SPEC, bool, 3>;
#[doc = "Field `CMP5LK` reader - CMP5LK"]
pub type CMP5LK_R = crate::BitReader<bool>;
#[doc = "Field `CMP5LK` writer - CMP5LK"]
pub type CMP5LK_W<'a> = crate::BitWriter<'a, u32, COMP_LOCK_SPEC, bool, 4>;
#[doc = "Field `CMP6LK` reader - CMP6LK"]
pub type CMP6LK_R = crate::BitReader<bool>;
#[doc = "Field `CMP6LK` writer - CMP6LK"]
pub type CMP6LK_W<'a> = crate::BitWriter<'a, u32, COMP_LOCK_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - CMP1LK"]
    #[inline(always)]
    pub fn cmp1lk(&self) -> CMP1LK_R {
        CMP1LK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMP2LK"]
    #[inline(always)]
    pub fn cmp2lk(&self) -> CMP2LK_R {
        CMP2LK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CMP3LK"]
    #[inline(always)]
    pub fn cmp3lk(&self) -> CMP3LK_R {
        CMP3LK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMP4LK"]
    #[inline(always)]
    pub fn cmp4lk(&self) -> CMP4LK_R {
        CMP4LK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CMP5LK"]
    #[inline(always)]
    pub fn cmp5lk(&self) -> CMP5LK_R {
        CMP5LK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CMP6LK"]
    #[inline(always)]
    pub fn cmp6lk(&self) -> CMP6LK_R {
        CMP6LK_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMP1LK"]
    #[inline(always)]
    pub fn cmp1lk(&mut self) -> CMP1LK_W {
        CMP1LK_W::new(self)
    }
    #[doc = "Bit 1 - CMP2LK"]
    #[inline(always)]
    pub fn cmp2lk(&mut self) -> CMP2LK_W {
        CMP2LK_W::new(self)
    }
    #[doc = "Bit 2 - CMP3LK"]
    #[inline(always)]
    pub fn cmp3lk(&mut self) -> CMP3LK_W {
        CMP3LK_W::new(self)
    }
    #[doc = "Bit 3 - CMP4LK"]
    #[inline(always)]
    pub fn cmp4lk(&mut self) -> CMP4LK_W {
        CMP4LK_W::new(self)
    }
    #[doc = "Bit 4 - CMP5LK"]
    #[inline(always)]
    pub fn cmp5lk(&mut self) -> CMP5LK_W {
        CMP5LK_W::new(self)
    }
    #[doc = "Bit 5 - CMP6LK"]
    #[inline(always)]
    pub fn cmp6lk(&mut self) -> CMP6LK_W {
        CMP6LK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "COMP_LOCK\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_lock](index.html) module"]
pub struct COMP_LOCK_SPEC;
impl crate::RegisterSpec for COMP_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp_lock::R](R) reader structure"]
impl crate::Readable for COMP_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp_lock::W](W) writer structure"]
impl crate::Writable for COMP_LOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP_LOCK to value 0"]
impl crate::Resettable for COMP_LOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
