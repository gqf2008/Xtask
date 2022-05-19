#[doc = "Reader of register DOEPINTEN"]
pub type R = crate::R<u32, super::DOEPINTEN>;
#[doc = "Writer for register DOEPINTEN"]
pub type W = crate::W<u32, super::DOEPINTEN>;
#[doc = "Register DOEPINTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::DOEPINTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TFEN`"]
pub type TFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFEN`"]
pub struct TFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TFEN_W<'a> {
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
#[doc = "Reader of field `EPDISEN`"]
pub type EPDISEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPDISEN`"]
pub struct EPDISEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDISEN_W<'a> {
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
#[doc = "Reader of field `STPFEN`"]
pub type STPFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STPFEN`"]
pub struct STPFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STPFEN_W<'a> {
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
#[doc = "Reader of field `EPRXFOVREN`"]
pub type EPRXFOVREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPRXFOVREN`"]
pub struct EPRXFOVREN_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRXFOVREN_W<'a> {
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
#[doc = "Reader of field `BTBSTPEN`"]
pub type BTBSTPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BTBSTPEN`"]
pub struct BTBSTPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BTBSTPEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Transfer finished interrupt enable"]
    #[inline(always)]
    pub fn tfen(&self) -> TFEN_R {
        TFEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt enable"]
    #[inline(always)]
    pub fn epdisen(&self) -> EPDISEN_R {
        EPDISEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SETUP phase finished interrupt enable"]
    #[inline(always)]
    pub fn stpfen(&self) -> STPFEN_R {
        STPFEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint Rx FIFO overrun interrupt enable"]
    #[inline(always)]
    pub fn eprxfovren(&self) -> EPRXFOVREN_R {
        EPRXFOVREN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets interrupt enable"]
    #[inline(always)]
    pub fn btbstpen(&self) -> BTBSTPEN_R {
        BTBSTPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer finished interrupt enable"]
    #[inline(always)]
    pub fn tfen(&mut self) -> TFEN_W {
        TFEN_W { w: self }
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt enable"]
    #[inline(always)]
    pub fn epdisen(&mut self) -> EPDISEN_W {
        EPDISEN_W { w: self }
    }
    #[doc = "Bit 3 - SETUP phase finished interrupt enable"]
    #[inline(always)]
    pub fn stpfen(&mut self) -> STPFEN_W {
        STPFEN_W { w: self }
    }
    #[doc = "Bit 4 - Endpoint Rx FIFO overrun interrupt enable"]
    #[inline(always)]
    pub fn eprxfovren(&mut self) -> EPRXFOVREN_W {
        EPRXFOVREN_W { w: self }
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets interrupt enable"]
    #[inline(always)]
    pub fn btbstpen(&mut self) -> BTBSTPEN_W {
        BTBSTPEN_W { w: self }
    }
}
