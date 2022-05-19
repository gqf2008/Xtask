#[doc = "Reader of register TSTAT"]
pub type R = crate::R<u32, super::TSTAT>;
#[doc = "Writer for register TSTAT"]
pub type W = crate::W<u32, super::TSTAT>;
#[doc = "Register TSTAT `reset()`'s with value 0x1c00_0000"]
impl crate::ResetValue for super::TSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1c00_0000
    }
}
#[doc = "Reader of field `TMLS2`"]
pub type TMLS2_R = crate::R<bool, bool>;
#[doc = "Reader of field `TMLS1`"]
pub type TMLS1_R = crate::R<bool, bool>;
#[doc = "Reader of field `TMLS0`"]
pub type TMLS0_R = crate::R<bool, bool>;
#[doc = "Reader of field `TME2`"]
pub type TME2_R = crate::R<bool, bool>;
#[doc = "Reader of field `TME1`"]
pub type TME1_R = crate::R<bool, bool>;
#[doc = "Reader of field `TME0`"]
pub type TME0_R = crate::R<bool, bool>;
#[doc = "Reader of field `NUM`"]
pub type NUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `MST2`"]
pub type MST2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MST2`"]
pub struct MST2_W<'a> {
    w: &'a mut W,
}
impl<'a> MST2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `MTE2`"]
pub type MTE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MTE2`"]
pub struct MTE2_W<'a> {
    w: &'a mut W,
}
impl<'a> MTE2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `MAL2`"]
pub type MAL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAL2`"]
pub struct MAL2_W<'a> {
    w: &'a mut W,
}
impl<'a> MAL2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `MTFNERR2`"]
pub type MTFNERR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MTFNERR2`"]
pub struct MTFNERR2_W<'a> {
    w: &'a mut W,
}
impl<'a> MTFNERR2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `MTF2`"]
pub type MTF2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MTF2`"]
pub struct MTF2_W<'a> {
    w: &'a mut W,
}
impl<'a> MTF2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `MST1`"]
pub type MST1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MST1`"]
pub struct MST1_W<'a> {
    w: &'a mut W,
}
impl<'a> MST1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `MTE1`"]
pub type MTE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MTE1`"]
pub struct MTE1_W<'a> {
    w: &'a mut W,
}
impl<'a> MTE1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `MAL1`"]
pub type MAL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAL1`"]
pub struct MAL1_W<'a> {
    w: &'a mut W,
}
impl<'a> MAL1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `MTFNERR1`"]
pub type MTFNERR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MTFNERR1`"]
pub struct MTFNERR1_W<'a> {
    w: &'a mut W,
}
impl<'a> MTFNERR1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `MTF1`"]
pub type MTF1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MTF1`"]
pub struct MTF1_W<'a> {
    w: &'a mut W,
}
impl<'a> MTF1_W<'a> {
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
#[doc = "Reader of field `MST0`"]
pub type MST0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MST0`"]
pub struct MST0_W<'a> {
    w: &'a mut W,
}
impl<'a> MST0_W<'a> {
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
#[doc = "Reader of field `MTE0`"]
pub type MTE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MTE0`"]
pub struct MTE0_W<'a> {
    w: &'a mut W,
}
impl<'a> MTE0_W<'a> {
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
#[doc = "Reader of field `MAL0`"]
pub type MAL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAL0`"]
pub struct MAL0_W<'a> {
    w: &'a mut W,
}
impl<'a> MAL0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `MTFNERR0`"]
pub type MTFNERR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MTFNERR0`"]
pub struct MTFNERR0_W<'a> {
    w: &'a mut W,
}
impl<'a> MTFNERR0_W<'a> {
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
#[doc = "Reader of field `MTF0`"]
pub type MTF0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MTF0`"]
pub struct MTF0_W<'a> {
    w: &'a mut W,
}
impl<'a> MTF0_W<'a> {
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
impl R {
    #[doc = "Bit 31 - Transmit mailbox 2 last sending in transmit FIFO"]
    #[inline(always)]
    pub fn tmls2(&self) -> TMLS2_R {
        TMLS2_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Transmit mailbox 1 last sending in transmit FIFO"]
    #[inline(always)]
    pub fn tmls1(&self) -> TMLS1_R {
        TMLS1_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Transmit mailbox 0 last sending in transmit FIFO"]
    #[inline(always)]
    pub fn tmls0(&self) -> TMLS0_R {
        TMLS0_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Transmit mailbox 2 empty"]
    #[inline(always)]
    pub fn tme2(&self) -> TME2_R {
        TME2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Transmit mailbox 1 empty"]
    #[inline(always)]
    pub fn tme1(&self) -> TME1_R {
        TME1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Transmit mailbox 0 empty"]
    #[inline(always)]
    pub fn tme0(&self) -> TME0_R {
        TME0_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - number of the transmit FIFO mailbox in which the frame will be transmitted if at least one mailbox is empty"]
    #[inline(always)]
    pub fn num(&self) -> NUM_R {
        NUM_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 23 - Mailbox 2 stop transmitting"]
    #[inline(always)]
    pub fn mst2(&self) -> MST2_R {
        MST2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Mailbox 2 transmit error"]
    #[inline(always)]
    pub fn mte2(&self) -> MTE2_R {
        MTE2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Mailbox 2 arbitration lost"]
    #[inline(always)]
    pub fn mal2(&self) -> MAL2_R {
        MAL2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Mailbox 2 transmit finished and no error"]
    #[inline(always)]
    pub fn mtfnerr2(&self) -> MTFNERR2_R {
        MTFNERR2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Mailbox 2 transmit finished"]
    #[inline(always)]
    pub fn mtf2(&self) -> MTF2_R {
        MTF2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Mailbox 1 stop transmitting"]
    #[inline(always)]
    pub fn mst1(&self) -> MST1_R {
        MST1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Mailbox 1 transmit error"]
    #[inline(always)]
    pub fn mte1(&self) -> MTE1_R {
        MTE1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Mailbox 1 arbitration lost"]
    #[inline(always)]
    pub fn mal1(&self) -> MAL1_R {
        MAL1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Mailbox 1 transmit finished and no error"]
    #[inline(always)]
    pub fn mtfnerr1(&self) -> MTFNERR1_R {
        MTFNERR1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Mailbox 1 transmit finished"]
    #[inline(always)]
    pub fn mtf1(&self) -> MTF1_R {
        MTF1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Mailbox 0 stop transmitting"]
    #[inline(always)]
    pub fn mst0(&self) -> MST0_R {
        MST0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Mailbox 0 transmit error"]
    #[inline(always)]
    pub fn mte0(&self) -> MTE0_R {
        MTE0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mailbox 0 arbitration lost"]
    #[inline(always)]
    pub fn mal0(&self) -> MAL0_R {
        MAL0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mailbox 0 transmit finished and no error"]
    #[inline(always)]
    pub fn mtfnerr0(&self) -> MTFNERR0_R {
        MTFNERR0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Mailbox 0 transmit finished"]
    #[inline(always)]
    pub fn mtf0(&self) -> MTF0_R {
        MTF0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - Mailbox 2 stop transmitting"]
    #[inline(always)]
    pub fn mst2(&mut self) -> MST2_W {
        MST2_W { w: self }
    }
    #[doc = "Bit 19 - Mailbox 2 transmit error"]
    #[inline(always)]
    pub fn mte2(&mut self) -> MTE2_W {
        MTE2_W { w: self }
    }
    #[doc = "Bit 18 - Mailbox 2 arbitration lost"]
    #[inline(always)]
    pub fn mal2(&mut self) -> MAL2_W {
        MAL2_W { w: self }
    }
    #[doc = "Bit 17 - Mailbox 2 transmit finished and no error"]
    #[inline(always)]
    pub fn mtfnerr2(&mut self) -> MTFNERR2_W {
        MTFNERR2_W { w: self }
    }
    #[doc = "Bit 16 - Mailbox 2 transmit finished"]
    #[inline(always)]
    pub fn mtf2(&mut self) -> MTF2_W {
        MTF2_W { w: self }
    }
    #[doc = "Bit 15 - Mailbox 1 stop transmitting"]
    #[inline(always)]
    pub fn mst1(&mut self) -> MST1_W {
        MST1_W { w: self }
    }
    #[doc = "Bit 11 - Mailbox 1 transmit error"]
    #[inline(always)]
    pub fn mte1(&mut self) -> MTE1_W {
        MTE1_W { w: self }
    }
    #[doc = "Bit 10 - Mailbox 1 arbitration lost"]
    #[inline(always)]
    pub fn mal1(&mut self) -> MAL1_W {
        MAL1_W { w: self }
    }
    #[doc = "Bit 9 - Mailbox 1 transmit finished and no error"]
    #[inline(always)]
    pub fn mtfnerr1(&mut self) -> MTFNERR1_W {
        MTFNERR1_W { w: self }
    }
    #[doc = "Bit 8 - Mailbox 1 transmit finished"]
    #[inline(always)]
    pub fn mtf1(&mut self) -> MTF1_W {
        MTF1_W { w: self }
    }
    #[doc = "Bit 7 - Mailbox 0 stop transmitting"]
    #[inline(always)]
    pub fn mst0(&mut self) -> MST0_W {
        MST0_W { w: self }
    }
    #[doc = "Bit 3 - Mailbox 0 transmit error"]
    #[inline(always)]
    pub fn mte0(&mut self) -> MTE0_W {
        MTE0_W { w: self }
    }
    #[doc = "Bit 2 - Mailbox 0 arbitration lost"]
    #[inline(always)]
    pub fn mal0(&mut self) -> MAL0_W {
        MAL0_W { w: self }
    }
    #[doc = "Bit 1 - Mailbox 0 transmit finished and no error"]
    #[inline(always)]
    pub fn mtfnerr0(&mut self) -> MTFNERR0_W {
        MTFNERR0_W { w: self }
    }
    #[doc = "Bit 0 - Mailbox 0 transmit finished"]
    #[inline(always)]
    pub fn mtf0(&mut self) -> MTF0_W {
        MTF0_W { w: self }
    }
}
