#[doc = "Register `CAN_FFA1` reader"]
pub struct R(crate::R<CAN_FFA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_FFA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAN_FFA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAN_FFA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_FFA1` writer"]
pub struct W(crate::W<CAN_FFA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_FFA1_SPEC>;
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
impl From<crate::W<CAN_FFA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAN_FFA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FAF` reader - FAF"]
pub type FAF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FAF` writer - FAF"]
pub type FAF_W<'a> = crate::FieldWriter<'a, u32, CAN_FFA1_SPEC, u16, u16, 14, 0>;
impl R {
    #[doc = "Bits 0:13 - FAF"]
    #[inline(always)]
    pub fn faf(&self) -> FAF_R {
        FAF_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - FAF"]
    #[inline(always)]
    pub fn faf(&mut self) -> FAF_W {
        FAF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN_FFA1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_ffa1](index.html) module"]
pub struct CAN_FFA1_SPEC;
impl crate::RegisterSpec for CAN_FFA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_ffa1::R](R) reader structure"]
impl crate::Readable for CAN_FFA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_ffa1::W](W) writer structure"]
impl crate::Writable for CAN_FFA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_FFA1 to value 0"]
impl crate::Resettable for CAN_FFA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
