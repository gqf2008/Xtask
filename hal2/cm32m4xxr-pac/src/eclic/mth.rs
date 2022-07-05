#[doc = "Register `mth` reader"]
pub struct R(crate::R<MTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mth` writer"]
pub struct W(crate::W<MTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTH_SPEC>;
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
impl From<crate::W<MTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mth` reader - Target threshold level register"]
pub type MTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `mth` writer - Target threshold level register"]
pub type MTH_W<'a> = crate::FieldWriter<'a, u32, MTH_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Target threshold level register"]
    #[inline(always)]
    pub fn mth(&self) -> MTH_R {
        MTH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Target threshold level register"]
    #[inline(always)]
    pub fn mth(&mut self) -> MTH_W {
        MTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "set the target interrupt threshold level\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mth](index.html) module"]
pub struct MTH_SPEC;
impl crate::RegisterSpec for MTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mth::R](R) reader structure"]
impl crate::Readable for MTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mth::W](W) writer structure"]
impl crate::Writable for MTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets mth to value 0"]
impl crate::Resettable for MTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
