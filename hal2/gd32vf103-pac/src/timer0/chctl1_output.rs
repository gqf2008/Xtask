#[doc = "Reader of register CHCTL1_Output"]
pub type R = crate::R<u16, super::CHCTL1_OUTPUT>;
#[doc = "Writer for register CHCTL1_Output"]
pub type W = crate::W<u16, super::CHCTL1_OUTPUT>;
#[doc = "Register CHCTL1_Output `reset()`'s with value 0"]
impl crate::ResetValue for super::CHCTL1_OUTPUT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH3COMCEN`"]
pub type CH3COMCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3COMCEN`"]
pub struct CH3COMCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3COMCEN_W<'a> {
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
#[doc = "Reader of field `CH3COMCTL`"]
pub type CH3COMCTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH3COMCTL`"]
pub struct CH3COMCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3COMCTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u16) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `CH3COMSEN`"]
pub type CH3COMSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3COMSEN`"]
pub struct CH3COMSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3COMSEN_W<'a> {
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
#[doc = "Reader of field `CH3COMFEN`"]
pub type CH3COMFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3COMFEN`"]
pub struct CH3COMFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3COMFEN_W<'a> {
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
#[doc = "Reader of field `CH3MS`"]
pub type CH3MS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH3MS`"]
pub struct CH3MS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3MS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `CH2COMCEN`"]
pub type CH2COMCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2COMCEN`"]
pub struct CH2COMCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2COMCEN_W<'a> {
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
#[doc = "Reader of field `CH2COMCTL`"]
pub type CH2COMCTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH2COMCTL`"]
pub struct CH2COMCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2COMCTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u16) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `CH2COMSEN`"]
pub type CH2COMSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2COMSEN`"]
pub struct CH2COMSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2COMSEN_W<'a> {
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
#[doc = "Reader of field `CH2COMFEN`"]
pub type CH2COMFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2COMFEN`"]
pub struct CH2COMFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2COMFEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u16) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CH2MS`"]
pub type CH2MS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH2MS`"]
pub struct CH2MS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2MS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Channel 3 output compare clear enable"]
    #[inline(always)]
    pub fn ch3comcen(&self) -> CH3COMCEN_R {
        CH3COMCEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Channel 3 compare output control"]
    #[inline(always)]
    pub fn ch3comctl(&self) -> CH3COMCTL_R {
        CH3COMCTL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Channel 3 output compare shadow enable"]
    #[inline(always)]
    pub fn ch3comsen(&self) -> CH3COMSEN_R {
        CH3COMSEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel 3 output compare fast enable"]
    #[inline(always)]
    pub fn ch3comfen(&self) -> CH3COMFEN_R {
        CH3COMFEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Channel 3 mode selection"]
    #[inline(always)]
    pub fn ch3ms(&self) -> CH3MS_R {
        CH3MS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Channel 2 output compare clear enable"]
    #[inline(always)]
    pub fn ch2comcen(&self) -> CH2COMCEN_R {
        CH2COMCEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Channel 2 compare output control"]
    #[inline(always)]
    pub fn ch2comctl(&self) -> CH2COMCTL_R {
        CH2COMCTL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3 - Channel 2 compare output shadow enable"]
    #[inline(always)]
    pub fn ch2comsen(&self) -> CH2COMSEN_R {
        CH2COMSEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 output compare fast enable"]
    #[inline(always)]
    pub fn ch2comfen(&self) -> CH2COMFEN_R {
        CH2COMFEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - Channel 2 I/O mode selection"]
    #[inline(always)]
    pub fn ch2ms(&self) -> CH2MS_R {
        CH2MS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - Channel 3 output compare clear enable"]
    #[inline(always)]
    pub fn ch3comcen(&mut self) -> CH3COMCEN_W {
        CH3COMCEN_W { w: self }
    }
    #[doc = "Bits 12:14 - Channel 3 compare output control"]
    #[inline(always)]
    pub fn ch3comctl(&mut self) -> CH3COMCTL_W {
        CH3COMCTL_W { w: self }
    }
    #[doc = "Bit 11 - Channel 3 output compare shadow enable"]
    #[inline(always)]
    pub fn ch3comsen(&mut self) -> CH3COMSEN_W {
        CH3COMSEN_W { w: self }
    }
    #[doc = "Bit 10 - Channel 3 output compare fast enable"]
    #[inline(always)]
    pub fn ch3comfen(&mut self) -> CH3COMFEN_W {
        CH3COMFEN_W { w: self }
    }
    #[doc = "Bits 8:9 - Channel 3 mode selection"]
    #[inline(always)]
    pub fn ch3ms(&mut self) -> CH3MS_W {
        CH3MS_W { w: self }
    }
    #[doc = "Bit 7 - Channel 2 output compare clear enable"]
    #[inline(always)]
    pub fn ch2comcen(&mut self) -> CH2COMCEN_W {
        CH2COMCEN_W { w: self }
    }
    #[doc = "Bits 4:6 - Channel 2 compare output control"]
    #[inline(always)]
    pub fn ch2comctl(&mut self) -> CH2COMCTL_W {
        CH2COMCTL_W { w: self }
    }
    #[doc = "Bit 3 - Channel 2 compare output shadow enable"]
    #[inline(always)]
    pub fn ch2comsen(&mut self) -> CH2COMSEN_W {
        CH2COMSEN_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 output compare fast enable"]
    #[inline(always)]
    pub fn ch2comfen(&mut self) -> CH2COMFEN_W {
        CH2COMFEN_W { w: self }
    }
    #[doc = "Bits 0:1 - Channel 2 I/O mode selection"]
    #[inline(always)]
    pub fn ch2ms(&mut self) -> CH2MS_W {
        CH2MS_W { w: self }
    }
}
