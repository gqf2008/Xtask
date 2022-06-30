#[doc = "Register `CAN_RMI1` reader"]
pub struct R(crate::R<CAN_RMI1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_RMI1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAN_RMI1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAN_RMI1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_RMI1` writer"]
pub struct W(crate::W<CAN_RMI1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_RMI1_SPEC>;
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
impl From<crate::W<CAN_RMI1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAN_RMI1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTRQ` reader - RTRQ"]
pub type RTRQ_R = crate::BitReader<bool>;
#[doc = "Field `RTRQ` writer - RTRQ"]
pub type RTRQ_W<'a> = crate::BitWriter<'a, u32, CAN_RMI1_SPEC, bool, 1>;
#[doc = "Field `IDE` reader - IDE"]
pub type IDE_R = crate::BitReader<bool>;
#[doc = "Field `IDE` writer - IDE"]
pub type IDE_W<'a> = crate::BitWriter<'a, u32, CAN_RMI1_SPEC, bool, 2>;
#[doc = "Field `EXTID` reader - EXTID"]
pub type EXTID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EXTID` writer - EXTID"]
pub type EXTID_W<'a> = crate::FieldWriter<'a, u32, CAN_RMI1_SPEC, u32, u32, 18, 3>;
#[doc = "Field `STDID` reader - STDID"]
pub type STDID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STDID` writer - STDID"]
pub type STDID_W<'a> = crate::FieldWriter<'a, u32, CAN_RMI1_SPEC, u16, u16, 11, 21>;
impl R {
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
#[doc = "CAN_RMI1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_rmi1](index.html) module"]
pub struct CAN_RMI1_SPEC;
impl crate::RegisterSpec for CAN_RMI1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_rmi1::R](R) reader structure"]
impl crate::Readable for CAN_RMI1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_rmi1::W](W) writer structure"]
impl crate::Writable for CAN_RMI1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_RMI1 to value 0"]
impl crate::Resettable for CAN_RMI1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
