#[doc = "Reader of register SADDR1"]
pub type R = crate::R<u16, super::SADDR1>;
#[doc = "Writer for register SADDR1"]
pub type W = crate::W<u16, super::SADDR1>;
#[doc = "Register SADDR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SADDR1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDRESS2`"]
pub type ADDRESS2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRESS2`"]
pub struct ADDRESS2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u16) & 0x7f) << 1);
        self.w
    }
}
#[doc = "Reader of field `DUADEN`"]
pub type DUADEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUADEN`"]
pub struct DUADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DUADEN_W<'a> {
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
    #[doc = "Bits 1:7 - Second I2C address for the slave in Dual-Address mode"]
    #[inline(always)]
    pub fn address2(&self) -> ADDRESS2_R {
        ADDRESS2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 0 - Dual-Address mode switch"]
    #[inline(always)]
    pub fn duaden(&self) -> DUADEN_R {
        DUADEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:7 - Second I2C address for the slave in Dual-Address mode"]
    #[inline(always)]
    pub fn address2(&mut self) -> ADDRESS2_W {
        ADDRESS2_W { w: self }
    }
    #[doc = "Bit 0 - Dual-Address mode switch"]
    #[inline(always)]
    pub fn duaden(&mut self) -> DUADEN_W {
        DUADEN_W { w: self }
    }
}
