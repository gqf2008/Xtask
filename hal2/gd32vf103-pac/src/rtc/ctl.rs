#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0x20"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Reader of field `LWOFF`"]
pub type LWOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LWOFF`"]
pub struct LWOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> LWOFF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CMF`"]
pub type CMF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMF`"]
pub struct CMF_W<'a> {
    w: &'a mut W,
}
impl<'a> CMF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RSYNF`"]
pub type RSYNF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSYNF`"]
pub struct RSYNF_W<'a> {
    w: &'a mut W,
}
impl<'a> RSYNF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `OVIF`"]
pub type OVIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVIF`"]
pub struct OVIF_W<'a> {
    w: &'a mut W,
}
impl<'a> OVIF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `ALRMIF`"]
pub type ALRMIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALRMIF`"]
pub struct ALRMIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRMIF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SCIF`"]
pub type SCIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCIF`"]
pub struct SCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SCIF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - Last write operation finished flag"]
    #[inline(always)]
    pub fn lwoff(&self) -> LWOFF_R {
        LWOFF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Configuration mode flag"]
    #[inline(always)]
    pub fn cmf(&self) -> CMF_R {
        CMF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Registers synchronized flag"]
    #[inline(always)]
    pub fn rsynf(&self) -> RSYNF_R {
        RSYNF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Overflow interrupt flag"]
    #[inline(always)]
    pub fn ovif(&self) -> OVIF_R {
        OVIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Alarm interrupt flag"]
    #[inline(always)]
    pub fn alrmif(&self) -> ALRMIF_R {
        ALRMIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Sencond interrupt flag"]
    #[inline(always)]
    pub fn scif(&self) -> SCIF_R {
        SCIF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Last write operation finished flag"]
    #[inline(always)]
    pub fn lwoff(&mut self) -> LWOFF_W {
        LWOFF_W { w: self }
    }
    #[doc = "Bit 4 - Configuration mode flag"]
    #[inline(always)]
    pub fn cmf(&mut self) -> CMF_W {
        CMF_W { w: self }
    }
    #[doc = "Bit 3 - Registers synchronized flag"]
    #[inline(always)]
    pub fn rsynf(&mut self) -> RSYNF_W {
        RSYNF_W { w: self }
    }
    #[doc = "Bit 2 - Overflow interrupt flag"]
    #[inline(always)]
    pub fn ovif(&mut self) -> OVIF_W {
        OVIF_W { w: self }
    }
    #[doc = "Bit 1 - Alarm interrupt flag"]
    #[inline(always)]
    pub fn alrmif(&mut self) -> ALRMIF_W {
        ALRMIF_W { w: self }
    }
    #[doc = "Bit 0 - Sencond interrupt flag"]
    #[inline(always)]
    pub fn scif(&mut self) -> SCIF_W {
        SCIF_W { w: self }
    }
}
