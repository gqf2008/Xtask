#[doc = "Reader of register STAT0"]
pub type R = crate::R<u16, super::STAT0>;
#[doc = "Writer for register STAT0"]
pub type W = crate::W<u16, super::STAT0>;
#[doc = "Register STAT0 `reset()`'s with value 0"]
impl crate::ResetValue for super::STAT0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SMBALT`"]
pub type SMBALT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMBALT`"]
pub struct SMBALT_W<'a> {
    w: &'a mut W,
}
impl<'a> SMBALT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `SMBTO`"]
pub type SMBTO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMBTO`"]
pub struct SMBTO_W<'a> {
    w: &'a mut W,
}
impl<'a> SMBTO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u16) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `PECERR`"]
pub type PECERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PECERR`"]
pub struct PECERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PECERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u16) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `OUERR`"]
pub type OUERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUERR`"]
pub struct OUERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `AERR`"]
pub type AERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AERR`"]
pub struct AERR_W<'a> {
    w: &'a mut W,
}
impl<'a> AERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `LOSTARB`"]
pub type LOSTARB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOSTARB`"]
pub struct LOSTARB_W<'a> {
    w: &'a mut W,
}
impl<'a> LOSTARB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `BERR`"]
pub type BERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BERR`"]
pub struct BERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TBE`"]
pub type TBE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RBNE`"]
pub type RBNE_R = crate::R<bool, bool>;
#[doc = "Reader of field `STPDET`"]
pub type STPDET_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADD10SEND`"]
pub type ADD10SEND_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTC`"]
pub type BTC_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADDSEND`"]
pub type ADDSEND_R = crate::R<bool, bool>;
#[doc = "Reader of field `SBSEND`"]
pub type SBSEND_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 15 - SMBus Alert status"]
    #[inline(always)]
    pub fn smbalt(&self) -> SMBALT_R {
        SMBALT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Timeout signal in SMBus mode"]
    #[inline(always)]
    pub fn smbto(&self) -> SMBTO_R {
        SMBTO_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PEC error when receiving data"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Over-run or under-run situation occurs in slave mode"]
    #[inline(always)]
    pub fn ouerr(&self) -> OUERR_R {
        OUERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Acknowledge error"]
    #[inline(always)]
    pub fn aerr(&self) -> AERR_R {
        AERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Arbitration Lost in master mode"]
    #[inline(always)]
    pub fn lostarb(&self) -> LOSTARB_R {
        LOSTARB_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - A bus error occurs indication a unexpected START or STOP condition on I2C bus"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C_DATA is Empty during transmitting"]
    #[inline(always)]
    pub fn tbe(&self) -> TBE_R {
        TBE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C_DATA is not Empty during receiving"]
    #[inline(always)]
    pub fn rbne(&self) -> RBNE_R {
        RBNE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 4 - STOP condition detected in slave mode"]
    #[inline(always)]
    pub fn stpdet(&self) -> STPDET_R {
        STPDET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Header of 10-bit address is sent in master mode"]
    #[inline(always)]
    pub fn add10send(&self) -> ADD10SEND_R {
        ADD10SEND_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Byte transmission completed"]
    #[inline(always)]
    pub fn btc(&self) -> BTC_R {
        BTC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Address is sent in master mode or received and matches in slave mode"]
    #[inline(always)]
    pub fn addsend(&self) -> ADDSEND_R {
        ADDSEND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - START condition sent out in master mode"]
    #[inline(always)]
    pub fn sbsend(&self) -> SBSEND_R {
        SBSEND_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - SMBus Alert status"]
    #[inline(always)]
    pub fn smbalt(&mut self) -> SMBALT_W {
        SMBALT_W { w: self }
    }
    #[doc = "Bit 14 - Timeout signal in SMBus mode"]
    #[inline(always)]
    pub fn smbto(&mut self) -> SMBTO_W {
        SMBTO_W { w: self }
    }
    #[doc = "Bit 12 - PEC error when receiving data"]
    #[inline(always)]
    pub fn pecerr(&mut self) -> PECERR_W {
        PECERR_W { w: self }
    }
    #[doc = "Bit 11 - Over-run or under-run situation occurs in slave mode"]
    #[inline(always)]
    pub fn ouerr(&mut self) -> OUERR_W {
        OUERR_W { w: self }
    }
    #[doc = "Bit 10 - Acknowledge error"]
    #[inline(always)]
    pub fn aerr(&mut self) -> AERR_W {
        AERR_W { w: self }
    }
    #[doc = "Bit 9 - Arbitration Lost in master mode"]
    #[inline(always)]
    pub fn lostarb(&mut self) -> LOSTARB_W {
        LOSTARB_W { w: self }
    }
    #[doc = "Bit 8 - A bus error occurs indication a unexpected START or STOP condition on I2C bus"]
    #[inline(always)]
    pub fn berr(&mut self) -> BERR_W {
        BERR_W { w: self }
    }
}
