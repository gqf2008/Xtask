#[doc = "Reader of register RFIFO0"]
pub type R = crate::R<u32, super::RFIFO0>;
#[doc = "Writer for register RFIFO0"]
pub type W = crate::W<u32, super::RFIFO0>;
#[doc = "Register RFIFO0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RFIFO0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RFD0`"]
pub type RFD0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFD0`"]
pub struct RFD0_W<'a> {
    w: &'a mut W,
}
impl<'a> RFD0_W<'a> {
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
#[doc = "Reader of field `RFO0`"]
pub type RFO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFO0`"]
pub struct RFO0_W<'a> {
    w: &'a mut W,
}
impl<'a> RFO0_W<'a> {
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
#[doc = "Reader of field `RFF0`"]
pub type RFF0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFF0`"]
pub struct RFF0_W<'a> {
    w: &'a mut W,
}
impl<'a> RFF0_W<'a> {
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
#[doc = "Reader of field `RFL0`"]
pub type RFL0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 5 - Receive FIFO0 dequeue"]
    #[inline(always)]
    pub fn rfd0(&self) -> RFD0_R {
        RFD0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO0 overfull"]
    #[inline(always)]
    pub fn rfo0(&self) -> RFO0_R {
        RFO0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO0 full"]
    #[inline(always)]
    pub fn rff0(&self) -> RFF0_R {
        RFF0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - Receive FIFO0 length"]
    #[inline(always)]
    pub fn rfl0(&self) -> RFL0_R {
        RFL0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - Receive FIFO0 dequeue"]
    #[inline(always)]
    pub fn rfd0(&mut self) -> RFD0_W {
        RFD0_W { w: self }
    }
    #[doc = "Bit 4 - Receive FIFO0 overfull"]
    #[inline(always)]
    pub fn rfo0(&mut self) -> RFO0_W {
        RFO0_W { w: self }
    }
    #[doc = "Bit 3 - Receive FIFO0 full"]
    #[inline(always)]
    pub fn rff0(&mut self) -> RFF0_W {
        RFF0_W { w: self }
    }
}
