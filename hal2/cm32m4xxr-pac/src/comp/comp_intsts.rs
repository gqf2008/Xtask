#[doc = "Register `COMP_INTSTS` reader"]
pub struct R(crate::R<COMP_INTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP_INTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP_INTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP_INTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP_INTSTS` writer"]
pub struct W(crate::W<COMP_INTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP_INTSTS_SPEC>;
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
impl From<crate::W<COMP_INTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP_INTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP1IS` reader - CMP1IS"]
pub type CMP1IS_R = crate::BitReader<bool>;
#[doc = "Field `CMP1IS` writer - CMP1IS"]
pub type CMP1IS_W<'a> = crate::BitWriter<'a, u32, COMP_INTSTS_SPEC, bool, 0>;
#[doc = "Field `CMP2IS` reader - CMP2IS"]
pub type CMP2IS_R = crate::BitReader<bool>;
#[doc = "Field `CMP2IS` writer - CMP2IS"]
pub type CMP2IS_W<'a> = crate::BitWriter<'a, u32, COMP_INTSTS_SPEC, bool, 1>;
#[doc = "Field `CMP3IS` reader - CMP3IS"]
pub type CMP3IS_R = crate::BitReader<bool>;
#[doc = "Field `CMP3IS` writer - CMP3IS"]
pub type CMP3IS_W<'a> = crate::BitWriter<'a, u32, COMP_INTSTS_SPEC, bool, 2>;
#[doc = "Field `CMP4IS` reader - CMP4IS"]
pub type CMP4IS_R = crate::BitReader<bool>;
#[doc = "Field `CMP4IS` writer - CMP4IS"]
pub type CMP4IS_W<'a> = crate::BitWriter<'a, u32, COMP_INTSTS_SPEC, bool, 3>;
#[doc = "Field `CMP5IS` reader - CMP5IS"]
pub type CMP5IS_R = crate::BitReader<bool>;
#[doc = "Field `CMP5IS` writer - CMP5IS"]
pub type CMP5IS_W<'a> = crate::BitWriter<'a, u32, COMP_INTSTS_SPEC, bool, 4>;
#[doc = "Field `CMP6IS` reader - CMP6IS"]
pub type CMP6IS_R = crate::BitReader<bool>;
#[doc = "Field `CMP6IS` writer - CMP6IS"]
pub type CMP6IS_W<'a> = crate::BitWriter<'a, u32, COMP_INTSTS_SPEC, bool, 5>;
#[doc = "Field `CMP7IS` reader - CMP7IS"]
pub type CMP7IS_R = crate::BitReader<bool>;
#[doc = "Field `CMP7IS` writer - CMP7IS"]
pub type CMP7IS_W<'a> = crate::BitWriter<'a, u32, COMP_INTSTS_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 0 - CMP1IS"]
    #[inline(always)]
    pub fn cmp1is(&self) -> CMP1IS_R {
        CMP1IS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMP2IS"]
    #[inline(always)]
    pub fn cmp2is(&self) -> CMP2IS_R {
        CMP2IS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CMP3IS"]
    #[inline(always)]
    pub fn cmp3is(&self) -> CMP3IS_R {
        CMP3IS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMP4IS"]
    #[inline(always)]
    pub fn cmp4is(&self) -> CMP4IS_R {
        CMP4IS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CMP5IS"]
    #[inline(always)]
    pub fn cmp5is(&self) -> CMP5IS_R {
        CMP5IS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CMP6IS"]
    #[inline(always)]
    pub fn cmp6is(&self) -> CMP6IS_R {
        CMP6IS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMP7IS"]
    #[inline(always)]
    pub fn cmp7is(&self) -> CMP7IS_R {
        CMP7IS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMP1IS"]
    #[inline(always)]
    pub fn cmp1is(&mut self) -> CMP1IS_W {
        CMP1IS_W::new(self)
    }
    #[doc = "Bit 1 - CMP2IS"]
    #[inline(always)]
    pub fn cmp2is(&mut self) -> CMP2IS_W {
        CMP2IS_W::new(self)
    }
    #[doc = "Bit 2 - CMP3IS"]
    #[inline(always)]
    pub fn cmp3is(&mut self) -> CMP3IS_W {
        CMP3IS_W::new(self)
    }
    #[doc = "Bit 3 - CMP4IS"]
    #[inline(always)]
    pub fn cmp4is(&mut self) -> CMP4IS_W {
        CMP4IS_W::new(self)
    }
    #[doc = "Bit 4 - CMP5IS"]
    #[inline(always)]
    pub fn cmp5is(&mut self) -> CMP5IS_W {
        CMP5IS_W::new(self)
    }
    #[doc = "Bit 5 - CMP6IS"]
    #[inline(always)]
    pub fn cmp6is(&mut self) -> CMP6IS_W {
        CMP6IS_W::new(self)
    }
    #[doc = "Bit 6 - CMP7IS"]
    #[inline(always)]
    pub fn cmp7is(&mut self) -> CMP7IS_W {
        CMP7IS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "COMP_INTSTS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_intsts](index.html) module"]
pub struct COMP_INTSTS_SPEC;
impl crate::RegisterSpec for COMP_INTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp_intsts::R](R) reader structure"]
impl crate::Readable for COMP_INTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp_intsts::W](W) writer structure"]
impl crate::Writable for COMP_INTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP_INTSTS to value 0"]
impl crate::Resettable for COMP_INTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
