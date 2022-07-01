#[doc = "Register `CAN_RFF0` reader"]
pub struct R(crate::R<CAN_RFF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_RFF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAN_RFF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAN_RFF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_RFF0` writer"]
pub struct W(crate::W<CAN_RFF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_RFF0_SPEC>;
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
impl From<crate::W<CAN_RFF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAN_RFF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FFMP0` reader - FFMP0"]
pub type FFMP0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FFMP0` writer - FFMP0"]
pub type FFMP0_W<'a> = crate::FieldWriter<'a, u32, CAN_RFF0_SPEC, u8, u8, 2, 0>;
#[doc = "Field `FFULL0` reader - FFULL0"]
pub type FFULL0_R = crate::BitReader<bool>;
#[doc = "Field `FFULL0` writer - FFULL0"]
pub type FFULL0_W<'a> = crate::BitWriter<'a, u32, CAN_RFF0_SPEC, bool, 3>;
#[doc = "Field `FFOVR0` reader - FFOVR0"]
pub type FFOVR0_R = crate::BitReader<bool>;
#[doc = "Field `FFOVR0` writer - FFOVR0"]
pub type FFOVR0_W<'a> = crate::BitWriter<'a, u32, CAN_RFF0_SPEC, bool, 4>;
#[doc = "Field `RFFOM0` reader - RFFOM0"]
pub type RFFOM0_R = crate::BitReader<bool>;
#[doc = "Field `RFFOM0` writer - RFFOM0"]
pub type RFFOM0_W<'a> = crate::BitWriter<'a, u32, CAN_RFF0_SPEC, bool, 5>;
impl R {
    #[doc = "Bits 0:1 - FFMP0"]
    #[inline(always)]
    pub fn ffmp0(&self) -> FFMP0_R {
        FFMP0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - FFULL0"]
    #[inline(always)]
    pub fn ffull0(&self) -> FFULL0_R {
        FFULL0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FFOVR0"]
    #[inline(always)]
    pub fn ffovr0(&self) -> FFOVR0_R {
        FFOVR0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RFFOM0"]
    #[inline(always)]
    pub fn rffom0(&self) -> RFFOM0_R {
        RFFOM0_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FFMP0"]
    #[inline(always)]
    pub fn ffmp0(&mut self) -> FFMP0_W {
        FFMP0_W::new(self)
    }
    #[doc = "Bit 3 - FFULL0"]
    #[inline(always)]
    pub fn ffull0(&mut self) -> FFULL0_W {
        FFULL0_W::new(self)
    }
    #[doc = "Bit 4 - FFOVR0"]
    #[inline(always)]
    pub fn ffovr0(&mut self) -> FFOVR0_W {
        FFOVR0_W::new(self)
    }
    #[doc = "Bit 5 - RFFOM0"]
    #[inline(always)]
    pub fn rffom0(&mut self) -> RFFOM0_W {
        RFFOM0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN_RFF0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_rff0](index.html) module"]
pub struct CAN_RFF0_SPEC;
impl crate::RegisterSpec for CAN_RFF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_rff0::R](R) reader structure"]
impl crate::Readable for CAN_RFF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_rff0::W](W) writer structure"]
impl crate::Writable for CAN_RFF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_RFF0 to value 0"]
impl crate::Resettable for CAN_RFF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
