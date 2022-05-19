#[doc = "Reader of register SADDR0"]
pub type R = crate::R<u16, super::SADDR0>;
#[doc = "Writer for register SADDR0"]
pub type W = crate::W<u16, super::SADDR0>;
#[doc = "Register SADDR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SADDR0 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDFORMAT`"]
pub type ADDFORMAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDFORMAT`"]
pub struct ADDFORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDFORMAT_W<'a> {
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
#[doc = "Reader of field `ADDRESS9_8`"]
pub type ADDRESS9_8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRESS9_8`"]
pub struct ADDRESS9_8_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS9_8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADDRESS7_1`"]
pub type ADDRESS7_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRESS7_1`"]
pub struct ADDRESS7_1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS7_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u16) & 0x7f) << 1);
        self.w
    }
}
#[doc = "Reader of field `ADDRESS0`"]
pub type ADDRESS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDRESS0`"]
pub struct ADDRESS0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS0_W<'a> {
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
    #[doc = "Bit 15 - Address mode for the I2C slave"]
    #[inline(always)]
    pub fn addformat(&self) -> ADDFORMAT_R {
        ADDFORMAT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Highest two bits of a 10-bit address"]
    #[inline(always)]
    pub fn address9_8(&self) -> ADDRESS9_8_R {
        ADDRESS9_8_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 1:7 - 7-bit address or bits 7:1 of a 10-bit address"]
    #[inline(always)]
    pub fn address7_1(&self) -> ADDRESS7_1_R {
        ADDRESS7_1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 0 - Bit 0 of a 10-bit address"]
    #[inline(always)]
    pub fn address0(&self) -> ADDRESS0_R {
        ADDRESS0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Address mode for the I2C slave"]
    #[inline(always)]
    pub fn addformat(&mut self) -> ADDFORMAT_W {
        ADDFORMAT_W { w: self }
    }
    #[doc = "Bits 8:9 - Highest two bits of a 10-bit address"]
    #[inline(always)]
    pub fn address9_8(&mut self) -> ADDRESS9_8_W {
        ADDRESS9_8_W { w: self }
    }
    #[doc = "Bits 1:7 - 7-bit address or bits 7:1 of a 10-bit address"]
    #[inline(always)]
    pub fn address7_1(&mut self) -> ADDRESS7_1_W {
        ADDRESS7_1_W { w: self }
    }
    #[doc = "Bit 0 - Bit 0 of a 10-bit address"]
    #[inline(always)]
    pub fn address0(&mut self) -> ADDRESS0_W {
        ADDRESS0_W { w: self }
    }
}
