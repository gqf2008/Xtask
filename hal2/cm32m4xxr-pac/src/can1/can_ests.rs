#[doc = "Register `CAN_ESTS` reader"]
pub struct R(crate::R<CAN_ESTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_ESTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAN_ESTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAN_ESTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_ESTS` writer"]
pub struct W(crate::W<CAN_ESTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_ESTS_SPEC>;
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
impl From<crate::W<CAN_ESTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAN_ESTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EWGFL` reader - EWGFL"]
pub type EWGFL_R = crate::BitReader<bool>;
#[doc = "Field `EWGFL` writer - EWGFL"]
pub type EWGFL_W<'a> = crate::BitWriter<'a, u32, CAN_ESTS_SPEC, bool, 0>;
#[doc = "Field `EPVFL` reader - EPVFL"]
pub type EPVFL_R = crate::BitReader<bool>;
#[doc = "Field `EPVFL` writer - EPVFL"]
pub type EPVFL_W<'a> = crate::BitWriter<'a, u32, CAN_ESTS_SPEC, bool, 1>;
#[doc = "Field `BOFFL` reader - BOFFL"]
pub type BOFFL_R = crate::BitReader<bool>;
#[doc = "Field `BOFFL` writer - BOFFL"]
pub type BOFFL_W<'a> = crate::BitWriter<'a, u32, CAN_ESTS_SPEC, bool, 2>;
#[doc = "Field `LEC` reader - LEC"]
pub type LEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEC` writer - LEC"]
pub type LEC_W<'a> = crate::FieldWriter<'a, u32, CAN_ESTS_SPEC, u8, u8, 3, 4>;
#[doc = "Field `TXEC` reader - TXEC"]
pub type TXEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXEC` writer - TXEC"]
pub type TXEC_W<'a> = crate::FieldWriter<'a, u32, CAN_ESTS_SPEC, u8, u8, 8, 16>;
#[doc = "Field `RXEC` reader - RXEC"]
pub type RXEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXEC` writer - RXEC"]
pub type RXEC_W<'a> = crate::FieldWriter<'a, u32, CAN_ESTS_SPEC, u8, u8, 8, 24>;
impl R {
    #[doc = "Bit 0 - EWGFL"]
    #[inline(always)]
    pub fn ewgfl(&self) -> EWGFL_R {
        EWGFL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EPVFL"]
    #[inline(always)]
    pub fn epvfl(&self) -> EPVFL_R {
        EPVFL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BOFFL"]
    #[inline(always)]
    pub fn boffl(&self) -> BOFFL_R {
        BOFFL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - LEC"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:23 - TXEC"]
    #[inline(always)]
    pub fn txec(&self) -> TXEC_R {
        TXEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - RXEC"]
    #[inline(always)]
    pub fn rxec(&self) -> RXEC_R {
        RXEC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EWGFL"]
    #[inline(always)]
    pub fn ewgfl(&mut self) -> EWGFL_W {
        EWGFL_W::new(self)
    }
    #[doc = "Bit 1 - EPVFL"]
    #[inline(always)]
    pub fn epvfl(&mut self) -> EPVFL_W {
        EPVFL_W::new(self)
    }
    #[doc = "Bit 2 - BOFFL"]
    #[inline(always)]
    pub fn boffl(&mut self) -> BOFFL_W {
        BOFFL_W::new(self)
    }
    #[doc = "Bits 4:6 - LEC"]
    #[inline(always)]
    pub fn lec(&mut self) -> LEC_W {
        LEC_W::new(self)
    }
    #[doc = "Bits 16:23 - TXEC"]
    #[inline(always)]
    pub fn txec(&mut self) -> TXEC_W {
        TXEC_W::new(self)
    }
    #[doc = "Bits 24:31 - RXEC"]
    #[inline(always)]
    pub fn rxec(&mut self) -> RXEC_W {
        RXEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN_ESTS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_ests](index.html) module"]
pub struct CAN_ESTS_SPEC;
impl crate::RegisterSpec for CAN_ESTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_ests::R](R) reader structure"]
impl crate::Readable for CAN_ESTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_ests::W](W) writer structure"]
impl crate::Writable for CAN_ESTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_ESTS to value 0"]
impl crate::Resettable for CAN_ESTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
