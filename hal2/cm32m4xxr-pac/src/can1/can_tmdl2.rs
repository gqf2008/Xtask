#[doc = "Register `CAN_TMDL2` reader"]
pub struct R(crate::R<CAN_TMDL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_TMDL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAN_TMDL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAN_TMDL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_TMDL2` writer"]
pub struct W(crate::W<CAN_TMDL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_TMDL2_SPEC>;
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
impl From<crate::W<CAN_TMDL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAN_TMDL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA0` reader - DATA0"]
pub type DATA0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA0` writer - DATA0"]
pub type DATA0_W<'a> = crate::FieldWriter<'a, u32, CAN_TMDL2_SPEC, u8, u8, 8, 0>;
#[doc = "Field `DATA1` reader - DATA1"]
pub type DATA1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA1` writer - DATA1"]
pub type DATA1_W<'a> = crate::FieldWriter<'a, u32, CAN_TMDL2_SPEC, u8, u8, 8, 8>;
#[doc = "Field `DATA2` reader - DATA2"]
pub type DATA2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA2` writer - DATA2"]
pub type DATA2_W<'a> = crate::FieldWriter<'a, u32, CAN_TMDL2_SPEC, u8, u8, 8, 16>;
#[doc = "Field `DATA3` reader - DATA3"]
pub type DATA3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA3` writer - DATA3"]
pub type DATA3_W<'a> = crate::FieldWriter<'a, u32, CAN_TMDL2_SPEC, u8, u8, 8, 24>;
impl R {
    #[doc = "Bits 0:7 - DATA0"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DATA1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DATA2"]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DATA3"]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA0"]
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W {
        DATA0_W::new(self)
    }
    #[doc = "Bits 8:15 - DATA1"]
    #[inline(always)]
    pub fn data1(&mut self) -> DATA1_W {
        DATA1_W::new(self)
    }
    #[doc = "Bits 16:23 - DATA2"]
    #[inline(always)]
    pub fn data2(&mut self) -> DATA2_W {
        DATA2_W::new(self)
    }
    #[doc = "Bits 24:31 - DATA3"]
    #[inline(always)]
    pub fn data3(&mut self) -> DATA3_W {
        DATA3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN_TMDL2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_tmdl2](index.html) module"]
pub struct CAN_TMDL2_SPEC;
impl crate::RegisterSpec for CAN_TMDL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_tmdl2::R](R) reader structure"]
impl crate::Readable for CAN_TMDL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_tmdl2::W](W) writer structure"]
impl crate::Writable for CAN_TMDL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_TMDL2 to value 0"]
impl crate::Resettable for CAN_TMDL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
