#[doc = "Reader of register GAHBCS"]
pub type R = crate::R<u32, super::GAHBCS>;
#[doc = "Writer for register GAHBCS"]
pub type W = crate::W<u32, super::GAHBCS>;
#[doc = "Register GAHBCS `reset()`'s with value 0"]
impl crate::ResetValue for super::GAHBCS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GINTEN`"]
pub type GINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GINTEN`"]
pub struct GINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GINTEN_W<'a> {
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
#[doc = "Reader of field `TXFTH`"]
pub type TXFTH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFTH`"]
pub struct TXFTH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFTH_W<'a> {
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
#[doc = "Reader of field `PTXFTH`"]
pub type PTXFTH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTXFTH`"]
pub struct PTXFTH_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFTH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Global interrupt enable"]
    #[inline(always)]
    pub fn ginten(&self) -> GINTEN_R {
        GINTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 7 - Tx FIFO threshold"]
    #[inline(always)]
    pub fn txfth(&self) -> TXFTH_R {
        TXFTH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Periodic Tx FIFO threshold"]
    #[inline(always)]
    pub fn ptxfth(&self) -> PTXFTH_R {
        PTXFTH_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global interrupt enable"]
    #[inline(always)]
    pub fn ginten(&mut self) -> GINTEN_W {
        GINTEN_W { w: self }
    }
    #[doc = "Bit 7 - Tx FIFO threshold"]
    #[inline(always)]
    pub fn txfth(&mut self) -> TXFTH_W {
        TXFTH_W { w: self }
    }
    #[doc = "Bit 8 - Periodic Tx FIFO threshold"]
    #[inline(always)]
    pub fn ptxfth(&mut self) -> PTXFTH_W {
        PTXFTH_W { w: self }
    }
}
