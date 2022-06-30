#[doc = "Register `CAN_TMI0` reader"]
pub struct R(crate::R<CAN_TMI0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_TMI0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAN_TMI0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAN_TMI0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_TMI0` writer"]
pub struct W(crate::W<CAN_TMI0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_TMI0_SPEC>;
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
impl From<crate::W<CAN_TMI0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAN_TMI0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXRQ` reader - TXRQ"]
pub type TXRQ_R = crate::BitReader<bool>;
#[doc = "Field `TXRQ` writer - TXRQ"]
pub type TXRQ_W<'a> = crate::BitWriter<'a, u32, CAN_TMI0_SPEC, bool, 0>;
#[doc = "Field `RTRQ` reader - RTRQ"]
pub type RTRQ_R = crate::BitReader<bool>;
#[doc = "Field `RTRQ` writer - RTRQ"]
pub type RTRQ_W<'a> = crate::BitWriter<'a, u32, CAN_TMI0_SPEC, bool, 1>;
#[doc = "Field `IDE` reader - IDE"]
pub type IDE_R = crate::BitReader<bool>;
#[doc = "Field `IDE` writer - IDE"]
pub type IDE_W<'a> = crate::BitWriter<'a, u32, CAN_TMI0_SPEC, bool, 2>;
#[doc = "Field `EXTID` reader - EXTID"]
pub type EXTID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EXTID` writer - EXTID"]
pub type EXTID_W<'a> = crate::FieldWriter<'a, u32, CAN_TMI0_SPEC, u32, u32, 18, 3>;
#[doc = "Field `STDID` reader - STDID"]
pub type STDID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STDID` writer - STDID"]
pub type STDID_W<'a> = crate::FieldWriter<'a, u32, CAN_TMI0_SPEC, u16, u16, 11, 21>;
impl R {
    #[doc = "Bit 0 - TXRQ"]
    #[inline(always)]
    pub fn txrq(&self) -> TXRQ_R {
        TXRQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTRQ"]
    #[inline(always)]
    pub fn rtrq(&self) -> RTRQ_R {
        RTRQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IDE"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - EXTID"]
    #[inline(always)]
    pub fn extid(&self) -> EXTID_R {
        EXTID_R::new(((self.bits >> 3) & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 21:31 - STDID"]
    #[inline(always)]
    pub fn stdid(&self) -> STDID_R {
        STDID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - TXRQ"]
    #[inline(always)]
    pub fn txrq(&mut self) -> TXRQ_W {
        TXRQ_W::new(self)
    }
    #[doc = "Bit 1 - RTRQ"]
    #[inline(always)]
    pub fn rtrq(&mut self) -> RTRQ_W {
        RTRQ_W::new(self)
    }
    #[doc = "Bit 2 - IDE"]
    #[inline(always)]
    pub fn ide(&mut self) -> IDE_W {
        IDE_W::new(self)
    }
    #[doc = "Bits 3:20 - EXTID"]
    #[inline(always)]
    pub fn extid(&mut self) -> EXTID_W {
        EXTID_W::new(self)
    }
    #[doc = "Bits 21:31 - STDID"]
    #[inline(always)]
    pub fn stdid(&mut self) -> STDID_W {
        STDID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN_TMI0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_tmi0](index.html) module"]
pub struct CAN_TMI0_SPEC;
impl crate::RegisterSpec for CAN_TMI0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_tmi0::R](R) reader structure"]
impl crate::Readable for CAN_TMI0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_tmi0::W](W) writer structure"]
impl crate::Writable for CAN_TMI0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_TMI0 to value 0"]
impl crate::Resettable for CAN_TMI0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
