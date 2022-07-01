#[doc = "Register `COMP6_FILP` reader"]
pub struct R(crate::R<COMP6_FILP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP6_FILP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP6_FILP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP6_FILP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP6_FILP` writer"]
pub struct W(crate::W<COMP6_FILP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP6_FILP_SPEC>;
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
impl From<crate::W<COMP6_FILP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP6_FILP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKPSC` reader - CLKPSC"]
pub type CLKPSC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLKPSC` writer - CLKPSC"]
pub type CLKPSC_W<'a> = crate::FieldWriter<'a, u32, COMP6_FILP_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - CLKPSC"]
    #[inline(always)]
    pub fn clkpsc(&self) -> CLKPSC_R {
        CLKPSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CLKPSC"]
    #[inline(always)]
    pub fn clkpsc(&mut self) -> CLKPSC_W {
        CLKPSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "COMP6_FILP\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp6_filp](index.html) module"]
pub struct COMP6_FILP_SPEC;
impl crate::RegisterSpec for COMP6_FILP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp6_filp::R](R) reader structure"]
impl crate::Readable for COMP6_FILP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp6_filp::W](W) writer structure"]
impl crate::Writable for COMP6_FILP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP6_FILP to value 0"]
impl crate::Resettable for COMP6_FILP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
