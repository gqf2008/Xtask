#[doc = "Register `CAN_RMDT1` reader"]
pub struct R(crate::R<CAN_RMDT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_RMDT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAN_RMDT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAN_RMDT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_RMDT1` writer"]
pub struct W(crate::W<CAN_RMDT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_RMDT1_SPEC>;
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
impl From<crate::W<CAN_RMDT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAN_RMDT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLC` reader - DLC"]
pub type DLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLC` writer - DLC"]
pub type DLC_W<'a> = crate::FieldWriter<'a, u32, CAN_RMDT1_SPEC, u8, u8, 4, 0>;
#[doc = "Field `FMI` reader - FMI"]
pub type FMI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FMI` writer - FMI"]
pub type FMI_W<'a> = crate::FieldWriter<'a, u32, CAN_RMDT1_SPEC, u8, u8, 8, 8>;
#[doc = "Field `MTIM` reader - MTIM"]
pub type MTIM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MTIM` writer - MTIM"]
pub type MTIM_W<'a> = crate::FieldWriter<'a, u32, CAN_RMDT1_SPEC, u16, u16, 16, 16>;
impl R {
    #[doc = "Bits 0:3 - DLC"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - FMI"]
    #[inline(always)]
    pub fn fmi(&self) -> FMI_R {
        FMI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - MTIM"]
    #[inline(always)]
    pub fn mtim(&self) -> MTIM_R {
        MTIM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - DLC"]
    #[inline(always)]
    pub fn dlc(&mut self) -> DLC_W {
        DLC_W::new(self)
    }
    #[doc = "Bits 8:15 - FMI"]
    #[inline(always)]
    pub fn fmi(&mut self) -> FMI_W {
        FMI_W::new(self)
    }
    #[doc = "Bits 16:31 - MTIM"]
    #[inline(always)]
    pub fn mtim(&mut self) -> MTIM_W {
        MTIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN_RMDT1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_rmdt1](index.html) module"]
pub struct CAN_RMDT1_SPEC;
impl crate::RegisterSpec for CAN_RMDT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_rmdt1::R](R) reader structure"]
impl crate::Readable for CAN_RMDT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_rmdt1::W](W) writer structure"]
impl crate::Writable for CAN_RMDT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_RMDT1 to value 0"]
impl crate::Resettable for CAN_RMDT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
