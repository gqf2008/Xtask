#[doc = "Reader of register FAFIFO"]
pub type R = crate::R<u32, super::FAFIFO>;
#[doc = "Writer for register FAFIFO"]
pub type W = crate::W<u32, super::FAFIFO>;
#[doc = "Register FAFIFO `reset()`'s with value 0"]
impl crate::ResetValue for super::FAFIFO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FAF0`"]
pub type FAF0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF0`"]
pub struct FAF0_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF0_W<'a> {
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
#[doc = "Reader of field `FAF1`"]
pub type FAF1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF1`"]
pub struct FAF1_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF1_W<'a> {
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
#[doc = "Reader of field `FAF2`"]
pub type FAF2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF2`"]
pub struct FAF2_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF2_W<'a> {
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
#[doc = "Reader of field `FAF3`"]
pub type FAF3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF3`"]
pub struct FAF3_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF3_W<'a> {
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
#[doc = "Reader of field `FAF4`"]
pub type FAF4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF4`"]
pub struct FAF4_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF4_W<'a> {
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
#[doc = "Reader of field `FAF5`"]
pub type FAF5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF5`"]
pub struct FAF5_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF5_W<'a> {
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
#[doc = "Reader of field `FAF6`"]
pub type FAF6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF6`"]
pub struct FAF6_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF6_W<'a> {
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
#[doc = "Reader of field `FAF7`"]
pub type FAF7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF7`"]
pub struct FAF7_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF7_W<'a> {
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
#[doc = "Reader of field `FAF8`"]
pub type FAF8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF8`"]
pub struct FAF8_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF8_W<'a> {
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
#[doc = "Reader of field `FAF9`"]
pub type FAF9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF9`"]
pub struct FAF9_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF9_W<'a> {
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
#[doc = "Reader of field `FAF10`"]
pub type FAF10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF10`"]
pub struct FAF10_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF10_W<'a> {
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
#[doc = "Reader of field `FAF11`"]
pub type FAF11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF11`"]
pub struct FAF11_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF11_W<'a> {
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
#[doc = "Reader of field `FAF12`"]
pub type FAF12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF12`"]
pub struct FAF12_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF12_W<'a> {
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
#[doc = "Reader of field `FAF13`"]
pub type FAF13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF13`"]
pub struct FAF13_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF13_W<'a> {
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
#[doc = "Reader of field `FAF14`"]
pub type FAF14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF14`"]
pub struct FAF14_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF14_W<'a> {
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
#[doc = "Reader of field `FAF15`"]
pub type FAF15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF15`"]
pub struct FAF15_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF15_W<'a> {
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
#[doc = "Reader of field `FAF16`"]
pub type FAF16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF16`"]
pub struct FAF16_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF16_W<'a> {
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
#[doc = "Reader of field `FAF17`"]
pub type FAF17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF17`"]
pub struct FAF17_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF17_W<'a> {
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
#[doc = "Reader of field `FAF18`"]
pub type FAF18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF18`"]
pub struct FAF18_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF18_W<'a> {
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
#[doc = "Reader of field `FAF19`"]
pub type FAF19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF19`"]
pub struct FAF19_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF19_W<'a> {
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
#[doc = "Reader of field `FAF20`"]
pub type FAF20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF20`"]
pub struct FAF20_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF20_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `FAF21`"]
pub type FAF21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF21`"]
pub struct FAF21_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF21_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `FAF22`"]
pub type FAF22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF22`"]
pub struct FAF22_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `FAF23`"]
pub type FAF23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF23`"]
pub struct FAF23_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF23_W<'a> {
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
#[doc = "Reader of field `FAF24`"]
pub type FAF24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF24`"]
pub struct FAF24_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF24_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `FAF25`"]
pub type FAF25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF25`"]
pub struct FAF25_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF25_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `FAF26`"]
pub type FAF26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF26`"]
pub struct FAF26_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF26_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `FAF27`"]
pub type FAF27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAF27`"]
pub struct FAF27_W<'a> {
    w: &'a mut W,
}
impl<'a> FAF27_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Filter 0 associated with FIFO"]
    #[inline(always)]
    pub fn faf0(&self) -> FAF0_R {
        FAF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Filter 1 associated with FIFO"]
    #[inline(always)]
    pub fn faf1(&self) -> FAF1_R {
        FAF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Filter 2 associated with FIFO"]
    #[inline(always)]
    pub fn faf2(&self) -> FAF2_R {
        FAF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Filter 3 associated with FIFO"]
    #[inline(always)]
    pub fn faf3(&self) -> FAF3_R {
        FAF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Filter 4 associated with FIFO"]
    #[inline(always)]
    pub fn faf4(&self) -> FAF4_R {
        FAF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Filter 5 associated with FIFO"]
    #[inline(always)]
    pub fn faf5(&self) -> FAF5_R {
        FAF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Filter 6 associated with FIFO"]
    #[inline(always)]
    pub fn faf6(&self) -> FAF6_R {
        FAF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Filter 7 associated with FIFO"]
    #[inline(always)]
    pub fn faf7(&self) -> FAF7_R {
        FAF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Filter 8 associated with FIFO"]
    #[inline(always)]
    pub fn faf8(&self) -> FAF8_R {
        FAF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Filter 9 associated with FIFO"]
    #[inline(always)]
    pub fn faf9(&self) -> FAF9_R {
        FAF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Filter 10 associated with FIFO"]
    #[inline(always)]
    pub fn faf10(&self) -> FAF10_R {
        FAF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Filter 11 associated with FIFO"]
    #[inline(always)]
    pub fn faf11(&self) -> FAF11_R {
        FAF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Filter 12 associated with FIFO"]
    #[inline(always)]
    pub fn faf12(&self) -> FAF12_R {
        FAF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Filter 13 associated with FIFO"]
    #[inline(always)]
    pub fn faf13(&self) -> FAF13_R {
        FAF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Filter 14 associated with FIFO"]
    #[inline(always)]
    pub fn faf14(&self) -> FAF14_R {
        FAF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Filter 15 associated with FIFO"]
    #[inline(always)]
    pub fn faf15(&self) -> FAF15_R {
        FAF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Filter 16 associated with FIFO"]
    #[inline(always)]
    pub fn faf16(&self) -> FAF16_R {
        FAF16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Filter 17 associated with FIFO"]
    #[inline(always)]
    pub fn faf17(&self) -> FAF17_R {
        FAF17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Filter 18 associated with FIFO"]
    #[inline(always)]
    pub fn faf18(&self) -> FAF18_R {
        FAF18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Filter 19 associated with FIFO"]
    #[inline(always)]
    pub fn faf19(&self) -> FAF19_R {
        FAF19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Filter 20 associated with FIFO"]
    #[inline(always)]
    pub fn faf20(&self) -> FAF20_R {
        FAF20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Filter 21 associated with FIFO"]
    #[inline(always)]
    pub fn faf21(&self) -> FAF21_R {
        FAF21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Filter 22 associated with FIFO"]
    #[inline(always)]
    pub fn faf22(&self) -> FAF22_R {
        FAF22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Filter 23 associated with FIFO"]
    #[inline(always)]
    pub fn faf23(&self) -> FAF23_R {
        FAF23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Filter 24 associated with FIFO"]
    #[inline(always)]
    pub fn faf24(&self) -> FAF24_R {
        FAF24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Filter 25 associated with FIFO"]
    #[inline(always)]
    pub fn faf25(&self) -> FAF25_R {
        FAF25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Filter 26 associated with FIFO"]
    #[inline(always)]
    pub fn faf26(&self) -> FAF26_R {
        FAF26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Filter 27 associated with FIFO"]
    #[inline(always)]
    pub fn faf27(&self) -> FAF27_R {
        FAF27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter 0 associated with FIFO"]
    #[inline(always)]
    pub fn faf0(&mut self) -> FAF0_W {
        FAF0_W { w: self }
    }
    #[doc = "Bit 1 - Filter 1 associated with FIFO"]
    #[inline(always)]
    pub fn faf1(&mut self) -> FAF1_W {
        FAF1_W { w: self }
    }
    #[doc = "Bit 2 - Filter 2 associated with FIFO"]
    #[inline(always)]
    pub fn faf2(&mut self) -> FAF2_W {
        FAF2_W { w: self }
    }
    #[doc = "Bit 3 - Filter 3 associated with FIFO"]
    #[inline(always)]
    pub fn faf3(&mut self) -> FAF3_W {
        FAF3_W { w: self }
    }
    #[doc = "Bit 4 - Filter 4 associated with FIFO"]
    #[inline(always)]
    pub fn faf4(&mut self) -> FAF4_W {
        FAF4_W { w: self }
    }
    #[doc = "Bit 5 - Filter 5 associated with FIFO"]
    #[inline(always)]
    pub fn faf5(&mut self) -> FAF5_W {
        FAF5_W { w: self }
    }
    #[doc = "Bit 6 - Filter 6 associated with FIFO"]
    #[inline(always)]
    pub fn faf6(&mut self) -> FAF6_W {
        FAF6_W { w: self }
    }
    #[doc = "Bit 7 - Filter 7 associated with FIFO"]
    #[inline(always)]
    pub fn faf7(&mut self) -> FAF7_W {
        FAF7_W { w: self }
    }
    #[doc = "Bit 8 - Filter 8 associated with FIFO"]
    #[inline(always)]
    pub fn faf8(&mut self) -> FAF8_W {
        FAF8_W { w: self }
    }
    #[doc = "Bit 9 - Filter 9 associated with FIFO"]
    #[inline(always)]
    pub fn faf9(&mut self) -> FAF9_W {
        FAF9_W { w: self }
    }
    #[doc = "Bit 10 - Filter 10 associated with FIFO"]
    #[inline(always)]
    pub fn faf10(&mut self) -> FAF10_W {
        FAF10_W { w: self }
    }
    #[doc = "Bit 11 - Filter 11 associated with FIFO"]
    #[inline(always)]
    pub fn faf11(&mut self) -> FAF11_W {
        FAF11_W { w: self }
    }
    #[doc = "Bit 12 - Filter 12 associated with FIFO"]
    #[inline(always)]
    pub fn faf12(&mut self) -> FAF12_W {
        FAF12_W { w: self }
    }
    #[doc = "Bit 13 - Filter 13 associated with FIFO"]
    #[inline(always)]
    pub fn faf13(&mut self) -> FAF13_W {
        FAF13_W { w: self }
    }
    #[doc = "Bit 14 - Filter 14 associated with FIFO"]
    #[inline(always)]
    pub fn faf14(&mut self) -> FAF14_W {
        FAF14_W { w: self }
    }
    #[doc = "Bit 15 - Filter 15 associated with FIFO"]
    #[inline(always)]
    pub fn faf15(&mut self) -> FAF15_W {
        FAF15_W { w: self }
    }
    #[doc = "Bit 16 - Filter 16 associated with FIFO"]
    #[inline(always)]
    pub fn faf16(&mut self) -> FAF16_W {
        FAF16_W { w: self }
    }
    #[doc = "Bit 17 - Filter 17 associated with FIFO"]
    #[inline(always)]
    pub fn faf17(&mut self) -> FAF17_W {
        FAF17_W { w: self }
    }
    #[doc = "Bit 18 - Filter 18 associated with FIFO"]
    #[inline(always)]
    pub fn faf18(&mut self) -> FAF18_W {
        FAF18_W { w: self }
    }
    #[doc = "Bit 19 - Filter 19 associated with FIFO"]
    #[inline(always)]
    pub fn faf19(&mut self) -> FAF19_W {
        FAF19_W { w: self }
    }
    #[doc = "Bit 20 - Filter 20 associated with FIFO"]
    #[inline(always)]
    pub fn faf20(&mut self) -> FAF20_W {
        FAF20_W { w: self }
    }
    #[doc = "Bit 21 - Filter 21 associated with FIFO"]
    #[inline(always)]
    pub fn faf21(&mut self) -> FAF21_W {
        FAF21_W { w: self }
    }
    #[doc = "Bit 22 - Filter 22 associated with FIFO"]
    #[inline(always)]
    pub fn faf22(&mut self) -> FAF22_W {
        FAF22_W { w: self }
    }
    #[doc = "Bit 23 - Filter 23 associated with FIFO"]
    #[inline(always)]
    pub fn faf23(&mut self) -> FAF23_W {
        FAF23_W { w: self }
    }
    #[doc = "Bit 24 - Filter 24 associated with FIFO"]
    #[inline(always)]
    pub fn faf24(&mut self) -> FAF24_W {
        FAF24_W { w: self }
    }
    #[doc = "Bit 25 - Filter 25 associated with FIFO"]
    #[inline(always)]
    pub fn faf25(&mut self) -> FAF25_W {
        FAF25_W { w: self }
    }
    #[doc = "Bit 26 - Filter 26 associated with FIFO"]
    #[inline(always)]
    pub fn faf26(&mut self) -> FAF26_W {
        FAF26_W { w: self }
    }
    #[doc = "Bit 27 - Filter 27 associated with FIFO"]
    #[inline(always)]
    pub fn faf27(&mut self) -> FAF27_W {
        FAF27_W { w: self }
    }
}
