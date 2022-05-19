#[doc = "Reader of register CTL0"]
pub type R = crate::R<u16, super::CTL0>;
#[doc = "Writer for register CTL0"]
pub type W = crate::W<u16, super::CTL0>;
#[doc = "Register CTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SRESET`"]
pub type SRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRESET`"]
pub struct SRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SRESET_W<'a> {
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
#[doc = "Reader of field `SALT`"]
pub type SALT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SALT`"]
pub struct SALT_W<'a> {
    w: &'a mut W,
}
impl<'a> SALT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u16) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `PECTRANS`"]
pub type PECTRANS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PECTRANS`"]
pub struct PECTRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> PECTRANS_W<'a> {
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
#[doc = "Reader of field `POAP`"]
pub type POAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POAP`"]
pub struct POAP_W<'a> {
    w: &'a mut W,
}
impl<'a> POAP_W<'a> {
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
#[doc = "Reader of field `ACKEN`"]
pub type ACKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACKEN`"]
pub struct ACKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKEN_W<'a> {
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
#[doc = "Reader of field `STOP`"]
pub type STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOP`"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
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
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
#[doc = "Reader of field `SS`"]
pub type SS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SS`"]
pub struct SS_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `GCEN`"]
pub type GCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCEN`"]
pub struct GCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u16) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PECEN`"]
pub type PECEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PECEN`"]
pub struct PECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PECEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u16) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `ARPEN`"]
pub type ARPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARPEN`"]
pub struct ARPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ARPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SMBSEL`"]
pub type SMBSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMBSEL`"]
pub struct SMBSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SMBSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SMBEN`"]
pub type SMBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMBEN`"]
pub struct SMBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMBEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `I2CEN`"]
pub type I2CEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2CEN`"]
pub struct I2CEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    pub fn sreset(&self) -> SRESET_R {
        SRESET_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline(always)]
    pub fn salt(&self) -> SALT_R {
        SALT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PEC Transfer"]
    #[inline(always)]
    pub fn pectrans(&self) -> PECTRANS_R {
        PECTRANS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Position of ACK and PEC when receiving"]
    #[inline(always)]
    pub fn poap(&self) -> POAP_R {
        POAP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Whether or not to send an ACK"]
    #[inline(always)]
    pub fn acken(&self) -> ACKEN_R {
        ACKEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Generate a STOP condition on I2C bus"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Generate a START condition on I2C bus"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Whether to stretch SCL low when data is not ready in slave mode"]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Whether or not to response to a General Call (0x00)"]
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PEC Calculation Switch"]
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ARP protocol in SMBus switch"]
    #[inline(always)]
    pub fn arpen(&self) -> ARPEN_R {
        ARPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SMBusType Selection"]
    #[inline(always)]
    pub fn smbsel(&self) -> SMBSEL_R {
        SMBSEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SMBus/I2C mode switch"]
    #[inline(always)]
    pub fn smben(&self) -> SMBEN_R {
        SMBEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - I2C peripheral enable"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2CEN_R {
        I2CEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    pub fn sreset(&mut self) -> SRESET_W {
        SRESET_W { w: self }
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline(always)]
    pub fn salt(&mut self) -> SALT_W {
        SALT_W { w: self }
    }
    #[doc = "Bit 12 - PEC Transfer"]
    #[inline(always)]
    pub fn pectrans(&mut self) -> PECTRANS_W {
        PECTRANS_W { w: self }
    }
    #[doc = "Bit 11 - Position of ACK and PEC when receiving"]
    #[inline(always)]
    pub fn poap(&mut self) -> POAP_W {
        POAP_W { w: self }
    }
    #[doc = "Bit 10 - Whether or not to send an ACK"]
    #[inline(always)]
    pub fn acken(&mut self) -> ACKEN_W {
        ACKEN_W { w: self }
    }
    #[doc = "Bit 9 - Generate a STOP condition on I2C bus"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 8 - Generate a START condition on I2C bus"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 7 - Whether to stretch SCL low when data is not ready in slave mode"]
    #[inline(always)]
    pub fn ss(&mut self) -> SS_W {
        SS_W { w: self }
    }
    #[doc = "Bit 6 - Whether or not to response to a General Call (0x00)"]
    #[inline(always)]
    pub fn gcen(&mut self) -> GCEN_W {
        GCEN_W { w: self }
    }
    #[doc = "Bit 5 - PEC Calculation Switch"]
    #[inline(always)]
    pub fn pecen(&mut self) -> PECEN_W {
        PECEN_W { w: self }
    }
    #[doc = "Bit 4 - ARP protocol in SMBus switch"]
    #[inline(always)]
    pub fn arpen(&mut self) -> ARPEN_W {
        ARPEN_W { w: self }
    }
    #[doc = "Bit 3 - SMBusType Selection"]
    #[inline(always)]
    pub fn smbsel(&mut self) -> SMBSEL_W {
        SMBSEL_W { w: self }
    }
    #[doc = "Bit 1 - SMBus/I2C mode switch"]
    #[inline(always)]
    pub fn smben(&mut self) -> SMBEN_W {
        SMBEN_W { w: self }
    }
    #[doc = "Bit 0 - I2C peripheral enable"]
    #[inline(always)]
    pub fn i2cen(&mut self) -> I2CEN_W {
        I2CEN_W { w: self }
    }
}
