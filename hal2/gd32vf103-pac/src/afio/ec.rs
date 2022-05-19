#[doc = "Reader of register EC"]
pub type R = crate::R<u32, super::EC>;
#[doc = "Writer for register EC"]
pub type W = crate::W<u32, super::EC>;
#[doc = "Register EC `reset()`'s with value 0"]
impl crate::ResetValue for super::EC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EOE`"]
pub type EOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOE`"]
pub struct EOE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `PORT`"]
pub type PORT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PORT`"]
pub struct PORT_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `PIN`"]
pub type PIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PIN`"]
pub struct PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Event output enable"]
    #[inline(always)]
    pub fn eoe(&self) -> EOE_R {
        EOE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Event output port selection"]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:3 - Event output pin selection"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - Event output enable"]
    #[inline(always)]
    pub fn eoe(&mut self) -> EOE_W {
        EOE_W { w: self }
    }
    #[doc = "Bits 4:6 - Event output port selection"]
    #[inline(always)]
    pub fn port(&mut self) -> PORT_W {
        PORT_W { w: self }
    }
    #[doc = "Bits 0:3 - Event output pin selection"]
    #[inline(always)]
    pub fn pin(&mut self) -> PIN_W {
        PIN_W { w: self }
    }
}
