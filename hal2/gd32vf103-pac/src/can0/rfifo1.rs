#[doc = "Reader of register RFIFO1"]
pub type R = crate::R<u32, super::RFIFO1>;
#[doc = "Writer for register RFIFO1"]
pub type W = crate::W<u32, super::RFIFO1>;
#[doc = "Register RFIFO1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RFIFO1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RFD1`"]
pub type RFD1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFD1`"]
pub struct RFD1_W<'a> {
    w: &'a mut W,
}
impl<'a> RFD1_W<'a> {
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
#[doc = "Reader of field `RFO1`"]
pub type RFO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFO1`"]
pub struct RFO1_W<'a> {
    w: &'a mut W,
}
impl<'a> RFO1_W<'a> {
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
#[doc = "Reader of field `RFF1`"]
pub type RFF1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFF1`"]
pub struct RFF1_W<'a> {
    w: &'a mut W,
}
impl<'a> RFF1_W<'a> {
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
#[doc = "Reader of field `RFL1`"]
pub type RFL1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 5 - Receive FIFO1 dequeue"]
    #[inline(always)]
    pub fn rfd1(&self) -> RFD1_R {
        RFD1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO1 overfull"]
    #[inline(always)]
    pub fn rfo1(&self) -> RFO1_R {
        RFO1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO1 full"]
    #[inline(always)]
    pub fn rff1(&self) -> RFF1_R {
        RFF1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - Receive FIFO1 length"]
    #[inline(always)]
    pub fn rfl1(&self) -> RFL1_R {
        RFL1_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - Receive FIFO1 dequeue"]
    #[inline(always)]
    pub fn rfd1(&mut self) -> RFD1_W {
        RFD1_W { w: self }
    }
    #[doc = "Bit 4 - Receive FIFO1 overfull"]
    #[inline(always)]
    pub fn rfo1(&mut self) -> RFO1_W {
        RFO1_W { w: self }
    }
    #[doc = "Bit 3 - Receive FIFO1 full"]
    #[inline(always)]
    pub fn rff1(&mut self) -> RFF1_W {
        RFF1_W { w: self }
    }
}
