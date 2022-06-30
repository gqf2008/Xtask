#[doc = "Register `BKP_DAT31` reader"]
pub struct R(crate::R<BKP_DAT31_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BKP_DAT31_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BKP_DAT31_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BKP_DAT31_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BKP_DAT31` writer"]
pub struct W(crate::W<BKP_DAT31_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BKP_DAT31_SPEC>;
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
impl From<crate::W<BKP_DAT31_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BKP_DAT31_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAT` reader - DAT"]
pub type DAT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DAT` writer - DAT"]
pub type DAT_W<'a> = crate::FieldWriter<'a, u32, BKP_DAT31_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - DAT"]
    #[inline(always)]
    pub fn dat(&self) -> DAT_R {
        DAT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DAT"]
    #[inline(always)]
    pub fn dat(&mut self) -> DAT_W {
        DAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BKP_DAT31\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp_dat31](index.html) module"]
pub struct BKP_DAT31_SPEC;
impl crate::RegisterSpec for BKP_DAT31_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bkp_dat31::R](R) reader structure"]
impl crate::Readable for BKP_DAT31_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bkp_dat31::W](W) writer structure"]
impl crate::Writable for BKP_DAT31_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BKP_DAT31 to value 0"]
impl crate::Resettable for BKP_DAT31_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
