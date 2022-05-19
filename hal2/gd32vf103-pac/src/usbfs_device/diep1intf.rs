#[doc = "Reader of register DIEP1INTF"]
pub type R = crate::R<u32, super::DIEP1INTF>;
#[doc = "Writer for register DIEP1INTF"]
pub type W = crate::W<u32, super::DIEP1INTF>;
#[doc = "Register DIEP1INTF `reset()`'s with value 0x80"]
impl crate::ResetValue for super::DIEP1INTF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x80
    }
}
#[doc = "Reader of field `TXFE`"]
pub type TXFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `IEPNE`"]
pub type IEPNE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEPNE`"]
pub struct IEPNE_W<'a> {
    w: &'a mut W,
}
impl<'a> IEPNE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `EPTXFUD`"]
pub type EPTXFUD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPTXFUD`"]
pub struct EPTXFUD_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXFUD_W<'a> {
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
#[doc = "Reader of field `CITO`"]
pub type CITO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CITO`"]
pub struct CITO_W<'a> {
    w: &'a mut W,
}
impl<'a> CITO_W<'a> {
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
#[doc = "Reader of field `EPDIS`"]
pub type EPDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPDIS`"]
pub struct EPDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDIS_W<'a> {
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
#[doc = "Reader of field `TF`"]
pub type TF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TF`"]
pub struct TF_W<'a> {
    w: &'a mut W,
}
impl<'a> TF_W<'a> {
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
    #[doc = "Bit 7 - Transmit FIFO empty"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective"]
    #[inline(always)]
    pub fn iepne(&self) -> IEPNE_R {
        IEPNE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint Tx FIFO underrun"]
    #[inline(always)]
    pub fn eptxfud(&self) -> EPTXFUD_R {
        EPTXFUD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Control in timeout interrupt"]
    #[inline(always)]
    pub fn cito(&self) -> CITO_R {
        CITO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint finished"]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Transfer finished"]
    #[inline(always)]
    pub fn tf(&self) -> TF_R {
        TF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - IN endpoint NAK effective"]
    #[inline(always)]
    pub fn iepne(&mut self) -> IEPNE_W {
        IEPNE_W { w: self }
    }
    #[doc = "Bit 4 - Endpoint Tx FIFO underrun"]
    #[inline(always)]
    pub fn eptxfud(&mut self) -> EPTXFUD_W {
        EPTXFUD_W { w: self }
    }
    #[doc = "Bit 3 - Control in timeout interrupt"]
    #[inline(always)]
    pub fn cito(&mut self) -> CITO_W {
        CITO_W { w: self }
    }
    #[doc = "Bit 1 - Endpoint finished"]
    #[inline(always)]
    pub fn epdis(&mut self) -> EPDIS_W {
        EPDIS_W { w: self }
    }
    #[doc = "Bit 0 - Transfer finished"]
    #[inline(always)]
    pub fn tf(&mut self) -> TF_W {
        TF_W { w: self }
    }
}
