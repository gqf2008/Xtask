#[doc = "Reader of register SWIEV"]
pub type R = crate::R<u32, super::SWIEV>;
#[doc = "Writer for register SWIEV"]
pub type W = crate::W<u32, super::SWIEV>;
#[doc = "Register SWIEV `reset()`'s with value 0"]
impl crate::ResetValue for super::SWIEV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWIEV0`"]
pub type SWIEV0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWIEV0`"]
pub struct SWIEV0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIEV0_W<'a> {
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
#[doc = "Reader of field `SWIEV1`"]
pub type SWIEV1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWIEV1`"]
pub struct SWIEV1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIEV1_W<'a> {
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
#[doc = "Reader of field `SWIEV2`"]
pub type SWIEV2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWIEV2`"]
pub struct SWIEV2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIEV2_W<'a> {
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
#[doc = "Reader of field `SWIEV3`"]
pub type SWIEV3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWIEV3`"]
pub struct SWIEV3_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIEV3_W<'a> {
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
#[doc = "Reader of field `SWIEV4`"]
pub type SWIEV4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWIEV4`"]
pub struct SWIEV4_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIEV4_W<'a> {
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
#[doc = "Reader of field `SWIEV5`"]
pub type SWIEV5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWIEV5`"]
pub struct SWIEV5_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIEV5_W<'a> {
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
#[doc = "Reader of field `SWIEV6`"]
pub type SWIEV6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWIEV6`"]
pub struct SWIEV6_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIEV6_W<'a> {
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
#[doc = "Reader of field `SWIEV7`"]
pub type SWIEV7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWIEV7`"]
pub struct SWIEV7_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIEV7_W<'a> {
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
#[doc = "Reader of field `SWIEV8`"]
pub type SWIEV8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWIEV8`"]
pub struct SWIEV8_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIEV8_W<'a> {
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
#[doc = "Reader of field `SWIEV9`"]
pub type SWIEV9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWIEV9`"]
pub struct SWIEV9_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIEV9_W<'a> {
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
#[doc = "Reader of field `SWIEV10`"]
pub type SWIEV10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWIEV10`"]
pub struct SWIEV10_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIEV10_W<'a> {
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
#[doc = "Reader of field `SWIEV11`"]
pub type SWIEV11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWIEV11`"]
pub struct SWIEV11_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIEV11_W<'a> {
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
#[doc = "Reader of field `SWIEV12`"]
pub type SWIEV12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWIEV12`"]
pub struct SWIEV12_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIEV12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `SWIEV13`"]
pub type SWIEV13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWIEV13`"]
pub struct SWIEV13_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIEV13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `SWIEV14`"]
pub type SWIEV14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWIEV14`"]
pub struct SWIEV14_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIEV14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `SWIEV15`"]
pub type SWIEV15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWIEV15`"]
pub struct SWIEV15_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIEV15_W<'a> {
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
#[doc = "Reader of field `SWIEV16`"]
pub type SWIEV16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWIEV16`"]
pub struct SWIEV16_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIEV16_W<'a> {
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
#[doc = "Reader of field `SWIEV17`"]
pub type SWIEV17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWIEV17`"]
pub struct SWIEV17_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIEV17_W<'a> {
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
#[doc = "Reader of field `SWIEV18`"]
pub type SWIEV18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWIEV18`"]
pub struct SWIEV18_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIEV18_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Interrupt/Event software trigger on line 0"]
    #[inline(always)]
    pub fn swiev0(&self) -> SWIEV0_R {
        SWIEV0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt/Event software trigger on line 1"]
    #[inline(always)]
    pub fn swiev1(&self) -> SWIEV1_R {
        SWIEV1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt/Event software trigger on line 2"]
    #[inline(always)]
    pub fn swiev2(&self) -> SWIEV2_R {
        SWIEV2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt/Event software trigger on line 3"]
    #[inline(always)]
    pub fn swiev3(&self) -> SWIEV3_R {
        SWIEV3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt/Event software trigger on line 4"]
    #[inline(always)]
    pub fn swiev4(&self) -> SWIEV4_R {
        SWIEV4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt/Event software trigger on line 5"]
    #[inline(always)]
    pub fn swiev5(&self) -> SWIEV5_R {
        SWIEV5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt/Event software trigger on line 6"]
    #[inline(always)]
    pub fn swiev6(&self) -> SWIEV6_R {
        SWIEV6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt/Event software trigger on line 7"]
    #[inline(always)]
    pub fn swiev7(&self) -> SWIEV7_R {
        SWIEV7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt/Event software trigger on line 8"]
    #[inline(always)]
    pub fn swiev8(&self) -> SWIEV8_R {
        SWIEV8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt/Event software trigger on line 9"]
    #[inline(always)]
    pub fn swiev9(&self) -> SWIEV9_R {
        SWIEV9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt/Event software trigger on line 10"]
    #[inline(always)]
    pub fn swiev10(&self) -> SWIEV10_R {
        SWIEV10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Interrupt/Event software trigger on line 11"]
    #[inline(always)]
    pub fn swiev11(&self) -> SWIEV11_R {
        SWIEV11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Interrupt/Event software trigger on line 12"]
    #[inline(always)]
    pub fn swiev12(&self) -> SWIEV12_R {
        SWIEV12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Interrupt/Event software trigger on line 13"]
    #[inline(always)]
    pub fn swiev13(&self) -> SWIEV13_R {
        SWIEV13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Interrupt/Event software trigger on line 14"]
    #[inline(always)]
    pub fn swiev14(&self) -> SWIEV14_R {
        SWIEV14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Interrupt/Event software trigger on line 15"]
    #[inline(always)]
    pub fn swiev15(&self) -> SWIEV15_R {
        SWIEV15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Interrupt/Event software trigger on line 16"]
    #[inline(always)]
    pub fn swiev16(&self) -> SWIEV16_R {
        SWIEV16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Interrupt/Event software trigger on line 17"]
    #[inline(always)]
    pub fn swiev17(&self) -> SWIEV17_R {
        SWIEV17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Interrupt/Event software trigger on line 18"]
    #[inline(always)]
    pub fn swiev18(&self) -> SWIEV18_R {
        SWIEV18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt/Event software trigger on line 0"]
    #[inline(always)]
    pub fn swiev0(&mut self) -> SWIEV0_W {
        SWIEV0_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt/Event software trigger on line 1"]
    #[inline(always)]
    pub fn swiev1(&mut self) -> SWIEV1_W {
        SWIEV1_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt/Event software trigger on line 2"]
    #[inline(always)]
    pub fn swiev2(&mut self) -> SWIEV2_W {
        SWIEV2_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt/Event software trigger on line 3"]
    #[inline(always)]
    pub fn swiev3(&mut self) -> SWIEV3_W {
        SWIEV3_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt/Event software trigger on line 4"]
    #[inline(always)]
    pub fn swiev4(&mut self) -> SWIEV4_W {
        SWIEV4_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt/Event software trigger on line 5"]
    #[inline(always)]
    pub fn swiev5(&mut self) -> SWIEV5_W {
        SWIEV5_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt/Event software trigger on line 6"]
    #[inline(always)]
    pub fn swiev6(&mut self) -> SWIEV6_W {
        SWIEV6_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt/Event software trigger on line 7"]
    #[inline(always)]
    pub fn swiev7(&mut self) -> SWIEV7_W {
        SWIEV7_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt/Event software trigger on line 8"]
    #[inline(always)]
    pub fn swiev8(&mut self) -> SWIEV8_W {
        SWIEV8_W { w: self }
    }
    #[doc = "Bit 9 - Interrupt/Event software trigger on line 9"]
    #[inline(always)]
    pub fn swiev9(&mut self) -> SWIEV9_W {
        SWIEV9_W { w: self }
    }
    #[doc = "Bit 10 - Interrupt/Event software trigger on line 10"]
    #[inline(always)]
    pub fn swiev10(&mut self) -> SWIEV10_W {
        SWIEV10_W { w: self }
    }
    #[doc = "Bit 11 - Interrupt/Event software trigger on line 11"]
    #[inline(always)]
    pub fn swiev11(&mut self) -> SWIEV11_W {
        SWIEV11_W { w: self }
    }
    #[doc = "Bit 12 - Interrupt/Event software trigger on line 12"]
    #[inline(always)]
    pub fn swiev12(&mut self) -> SWIEV12_W {
        SWIEV12_W { w: self }
    }
    #[doc = "Bit 13 - Interrupt/Event software trigger on line 13"]
    #[inline(always)]
    pub fn swiev13(&mut self) -> SWIEV13_W {
        SWIEV13_W { w: self }
    }
    #[doc = "Bit 14 - Interrupt/Event software trigger on line 14"]
    #[inline(always)]
    pub fn swiev14(&mut self) -> SWIEV14_W {
        SWIEV14_W { w: self }
    }
    #[doc = "Bit 15 - Interrupt/Event software trigger on line 15"]
    #[inline(always)]
    pub fn swiev15(&mut self) -> SWIEV15_W {
        SWIEV15_W { w: self }
    }
    #[doc = "Bit 16 - Interrupt/Event software trigger on line 16"]
    #[inline(always)]
    pub fn swiev16(&mut self) -> SWIEV16_W {
        SWIEV16_W { w: self }
    }
    #[doc = "Bit 17 - Interrupt/Event software trigger on line 17"]
    #[inline(always)]
    pub fn swiev17(&mut self) -> SWIEV17_W {
        SWIEV17_W { w: self }
    }
    #[doc = "Bit 18 - Interrupt/Event software trigger on line 18"]
    #[inline(always)]
    pub fn swiev18(&mut self) -> SWIEV18_W {
        SWIEV18_W { w: self }
    }
}
