#[doc = "Register `CAN_RFF1` reader"]
pub struct R(crate::R<CAN_RFF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_RFF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAN_RFF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAN_RFF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_RFF1` writer"]
pub struct W(crate::W<CAN_RFF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_RFF1_SPEC>;
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
impl From<crate::W<CAN_RFF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAN_RFF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FFMP1` reader - FFMP1"]
pub type FFMP1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FFMP1` writer - FFMP1"]
pub type FFMP1_W<'a> = crate::FieldWriter<'a, u32, CAN_RFF1_SPEC, u8, u8, 2, 0>;
#[doc = "Field `FFULL1` reader - FFULL1"]
pub type FFULL1_R = crate::BitReader<bool>;
#[doc = "Field `FFULL1` writer - FFULL1"]
pub type FFULL1_W<'a> = crate::BitWriter<'a, u32, CAN_RFF1_SPEC, bool, 3>;
#[doc = "Field `FFOVR1` reader - FFOVR1"]
pub type FFOVR1_R = crate::BitReader<bool>;
#[doc = "Field `FFOVR1` writer - FFOVR1"]
pub type FFOVR1_W<'a> = crate::BitWriter<'a, u32, CAN_RFF1_SPEC, bool, 4>;
#[doc = "Field `RFFOM1` reader - RFFOM1"]
pub type RFFOM1_R = crate::BitReader<bool>;
#[doc = "Field `RFFOM1` writer - RFFOM1"]
pub type RFFOM1_W<'a> = crate::BitWriter<'a, u32, CAN_RFF1_SPEC, bool, 5>;
impl R {
    #[doc = "Bits 0:1 - FFMP1"]
    #[inline(always)]
    pub fn ffmp1(&self) -> FFMP1_R {
        FFMP1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - FFULL1"]
    #[inline(always)]
    pub fn ffull1(&self) -> FFULL1_R {
        FFULL1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FFOVR1"]
    #[inline(always)]
    pub fn ffovr1(&self) -> FFOVR1_R {
        FFOVR1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RFFOM1"]
    #[inline(always)]
    pub fn rffom1(&self) -> RFFOM1_R {
        RFFOM1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FFMP1"]
    #[inline(always)]
    pub fn ffmp1(&mut self) -> FFMP1_W {
        FFMP1_W::new(self)
    }
    #[doc = "Bit 3 - FFULL1"]
    #[inline(always)]
    pub fn ffull1(&mut self) -> FFULL1_W {
        FFULL1_W::new(self)
    }
    #[doc = "Bit 4 - FFOVR1"]
    #[inline(always)]
    pub fn ffovr1(&mut self) -> FFOVR1_W {
        FFOVR1_W::new(self)
    }
    #[doc = "Bit 5 - RFFOM1"]
    #[inline(always)]
    pub fn rffom1(&mut self) -> RFFOM1_W {
        RFFOM1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN_RFF1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_rff1](index.html) module"]
pub struct CAN_RFF1_SPEC;
impl crate::RegisterSpec for CAN_RFF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_rff1::R](R) reader structure"]
impl crate::Readable for CAN_RFF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_rff1::W](W) writer structure"]
impl crate::Writable for CAN_RFF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_RFF1 to value 0"]
impl crate::Resettable for CAN_RFF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
