#[doc = "Reader of register F0DATA0"]
pub type R = crate::R<u32, super::F0DATA0>;
#[doc = "Writer for register F0DATA0"]
pub type W = crate::W<u32, super::F0DATA0>;
#[doc = "Register F0DATA0 `reset()`'s with value 0"]
impl crate::ResetValue for super::F0DATA0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FD0`"]
pub type FD0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD0`"]
pub struct FD0_W<'a> {
    w: &'a mut W,
}
impl<'a> FD0_W<'a> {
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
#[doc = "Reader of field `FD1`"]
pub type FD1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD1`"]
pub struct FD1_W<'a> {
    w: &'a mut W,
}
impl<'a> FD1_W<'a> {
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
#[doc = "Reader of field `FD2`"]
pub type FD2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD2`"]
pub struct FD2_W<'a> {
    w: &'a mut W,
}
impl<'a> FD2_W<'a> {
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
#[doc = "Reader of field `FD3`"]
pub type FD3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD3`"]
pub struct FD3_W<'a> {
    w: &'a mut W,
}
impl<'a> FD3_W<'a> {
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
#[doc = "Reader of field `FD4`"]
pub type FD4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD4`"]
pub struct FD4_W<'a> {
    w: &'a mut W,
}
impl<'a> FD4_W<'a> {
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
#[doc = "Reader of field `FD5`"]
pub type FD5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD5`"]
pub struct FD5_W<'a> {
    w: &'a mut W,
}
impl<'a> FD5_W<'a> {
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
#[doc = "Reader of field `FD6`"]
pub type FD6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD6`"]
pub struct FD6_W<'a> {
    w: &'a mut W,
}
impl<'a> FD6_W<'a> {
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
#[doc = "Reader of field `FD7`"]
pub type FD7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD7`"]
pub struct FD7_W<'a> {
    w: &'a mut W,
}
impl<'a> FD7_W<'a> {
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
#[doc = "Reader of field `FD8`"]
pub type FD8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD8`"]
pub struct FD8_W<'a> {
    w: &'a mut W,
}
impl<'a> FD8_W<'a> {
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
#[doc = "Reader of field `FD9`"]
pub type FD9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD9`"]
pub struct FD9_W<'a> {
    w: &'a mut W,
}
impl<'a> FD9_W<'a> {
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
#[doc = "Reader of field `FD10`"]
pub type FD10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD10`"]
pub struct FD10_W<'a> {
    w: &'a mut W,
}
impl<'a> FD10_W<'a> {
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
#[doc = "Reader of field `FD11`"]
pub type FD11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD11`"]
pub struct FD11_W<'a> {
    w: &'a mut W,
}
impl<'a> FD11_W<'a> {
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
#[doc = "Reader of field `FD12`"]
pub type FD12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD12`"]
pub struct FD12_W<'a> {
    w: &'a mut W,
}
impl<'a> FD12_W<'a> {
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
#[doc = "Reader of field `FD13`"]
pub type FD13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD13`"]
pub struct FD13_W<'a> {
    w: &'a mut W,
}
impl<'a> FD13_W<'a> {
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
#[doc = "Reader of field `FD14`"]
pub type FD14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD14`"]
pub struct FD14_W<'a> {
    w: &'a mut W,
}
impl<'a> FD14_W<'a> {
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
#[doc = "Reader of field `FD15`"]
pub type FD15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD15`"]
pub struct FD15_W<'a> {
    w: &'a mut W,
}
impl<'a> FD15_W<'a> {
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
#[doc = "Reader of field `FD16`"]
pub type FD16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD16`"]
pub struct FD16_W<'a> {
    w: &'a mut W,
}
impl<'a> FD16_W<'a> {
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
#[doc = "Reader of field `FD17`"]
pub type FD17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD17`"]
pub struct FD17_W<'a> {
    w: &'a mut W,
}
impl<'a> FD17_W<'a> {
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
#[doc = "Reader of field `FD18`"]
pub type FD18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD18`"]
pub struct FD18_W<'a> {
    w: &'a mut W,
}
impl<'a> FD18_W<'a> {
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
#[doc = "Reader of field `FD19`"]
pub type FD19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD19`"]
pub struct FD19_W<'a> {
    w: &'a mut W,
}
impl<'a> FD19_W<'a> {
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
#[doc = "Reader of field `FD20`"]
pub type FD20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD20`"]
pub struct FD20_W<'a> {
    w: &'a mut W,
}
impl<'a> FD20_W<'a> {
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
#[doc = "Reader of field `FD21`"]
pub type FD21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD21`"]
pub struct FD21_W<'a> {
    w: &'a mut W,
}
impl<'a> FD21_W<'a> {
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
#[doc = "Reader of field `FD22`"]
pub type FD22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD22`"]
pub struct FD22_W<'a> {
    w: &'a mut W,
}
impl<'a> FD22_W<'a> {
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
#[doc = "Reader of field `FD23`"]
pub type FD23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD23`"]
pub struct FD23_W<'a> {
    w: &'a mut W,
}
impl<'a> FD23_W<'a> {
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
#[doc = "Reader of field `FD24`"]
pub type FD24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD24`"]
pub struct FD24_W<'a> {
    w: &'a mut W,
}
impl<'a> FD24_W<'a> {
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
#[doc = "Reader of field `FD25`"]
pub type FD25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD25`"]
pub struct FD25_W<'a> {
    w: &'a mut W,
}
impl<'a> FD25_W<'a> {
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
#[doc = "Reader of field `FD26`"]
pub type FD26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD26`"]
pub struct FD26_W<'a> {
    w: &'a mut W,
}
impl<'a> FD26_W<'a> {
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
#[doc = "Reader of field `FD27`"]
pub type FD27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD27`"]
pub struct FD27_W<'a> {
    w: &'a mut W,
}
impl<'a> FD27_W<'a> {
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
#[doc = "Reader of field `FD28`"]
pub type FD28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD28`"]
pub struct FD28_W<'a> {
    w: &'a mut W,
}
impl<'a> FD28_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `FD29`"]
pub type FD29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD29`"]
pub struct FD29_W<'a> {
    w: &'a mut W,
}
impl<'a> FD29_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `FD30`"]
pub type FD30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD30`"]
pub struct FD30_W<'a> {
    w: &'a mut W,
}
impl<'a> FD30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `FD31`"]
pub type FD31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD31`"]
pub struct FD31_W<'a> {
    w: &'a mut W,
}
impl<'a> FD31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Filter bits"]
    #[inline(always)]
    pub fn fd0(&self) -> FD0_R {
        FD0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Filter bits"]
    #[inline(always)]
    pub fn fd1(&self) -> FD1_R {
        FD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Filter bits"]
    #[inline(always)]
    pub fn fd2(&self) -> FD2_R {
        FD2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Filter bits"]
    #[inline(always)]
    pub fn fd3(&self) -> FD3_R {
        FD3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Filter bits"]
    #[inline(always)]
    pub fn fd4(&self) -> FD4_R {
        FD4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Filter bits"]
    #[inline(always)]
    pub fn fd5(&self) -> FD5_R {
        FD5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Filter bits"]
    #[inline(always)]
    pub fn fd6(&self) -> FD6_R {
        FD6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Filter bits"]
    #[inline(always)]
    pub fn fd7(&self) -> FD7_R {
        FD7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Filter bits"]
    #[inline(always)]
    pub fn fd8(&self) -> FD8_R {
        FD8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Filter bits"]
    #[inline(always)]
    pub fn fd9(&self) -> FD9_R {
        FD9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Filter bits"]
    #[inline(always)]
    pub fn fd10(&self) -> FD10_R {
        FD10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Filter bits"]
    #[inline(always)]
    pub fn fd11(&self) -> FD11_R {
        FD11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Filter bits"]
    #[inline(always)]
    pub fn fd12(&self) -> FD12_R {
        FD12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Filter bits"]
    #[inline(always)]
    pub fn fd13(&self) -> FD13_R {
        FD13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Filter bits"]
    #[inline(always)]
    pub fn fd14(&self) -> FD14_R {
        FD14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Filter bits"]
    #[inline(always)]
    pub fn fd15(&self) -> FD15_R {
        FD15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Filter bits"]
    #[inline(always)]
    pub fn fd16(&self) -> FD16_R {
        FD16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Filter bits"]
    #[inline(always)]
    pub fn fd17(&self) -> FD17_R {
        FD17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Filter bits"]
    #[inline(always)]
    pub fn fd18(&self) -> FD18_R {
        FD18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Filter bits"]
    #[inline(always)]
    pub fn fd19(&self) -> FD19_R {
        FD19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Filter bits"]
    #[inline(always)]
    pub fn fd20(&self) -> FD20_R {
        FD20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Filter bits"]
    #[inline(always)]
    pub fn fd21(&self) -> FD21_R {
        FD21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Filter bits"]
    #[inline(always)]
    pub fn fd22(&self) -> FD22_R {
        FD22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Filter bits"]
    #[inline(always)]
    pub fn fd23(&self) -> FD23_R {
        FD23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Filter bits"]
    #[inline(always)]
    pub fn fd24(&self) -> FD24_R {
        FD24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Filter bits"]
    #[inline(always)]
    pub fn fd25(&self) -> FD25_R {
        FD25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Filter bits"]
    #[inline(always)]
    pub fn fd26(&self) -> FD26_R {
        FD26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Filter bits"]
    #[inline(always)]
    pub fn fd27(&self) -> FD27_R {
        FD27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Filter bits"]
    #[inline(always)]
    pub fn fd28(&self) -> FD28_R {
        FD28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Filter bits"]
    #[inline(always)]
    pub fn fd29(&self) -> FD29_R {
        FD29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Filter bits"]
    #[inline(always)]
    pub fn fd30(&self) -> FD30_R {
        FD30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Filter bits"]
    #[inline(always)]
    pub fn fd31(&self) -> FD31_R {
        FD31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter bits"]
    #[inline(always)]
    pub fn fd0(&mut self) -> FD0_W {
        FD0_W { w: self }
    }
    #[doc = "Bit 1 - Filter bits"]
    #[inline(always)]
    pub fn fd1(&mut self) -> FD1_W {
        FD1_W { w: self }
    }
    #[doc = "Bit 2 - Filter bits"]
    #[inline(always)]
    pub fn fd2(&mut self) -> FD2_W {
        FD2_W { w: self }
    }
    #[doc = "Bit 3 - Filter bits"]
    #[inline(always)]
    pub fn fd3(&mut self) -> FD3_W {
        FD3_W { w: self }
    }
    #[doc = "Bit 4 - Filter bits"]
    #[inline(always)]
    pub fn fd4(&mut self) -> FD4_W {
        FD4_W { w: self }
    }
    #[doc = "Bit 5 - Filter bits"]
    #[inline(always)]
    pub fn fd5(&mut self) -> FD5_W {
        FD5_W { w: self }
    }
    #[doc = "Bit 6 - Filter bits"]
    #[inline(always)]
    pub fn fd6(&mut self) -> FD6_W {
        FD6_W { w: self }
    }
    #[doc = "Bit 7 - Filter bits"]
    #[inline(always)]
    pub fn fd7(&mut self) -> FD7_W {
        FD7_W { w: self }
    }
    #[doc = "Bit 8 - Filter bits"]
    #[inline(always)]
    pub fn fd8(&mut self) -> FD8_W {
        FD8_W { w: self }
    }
    #[doc = "Bit 9 - Filter bits"]
    #[inline(always)]
    pub fn fd9(&mut self) -> FD9_W {
        FD9_W { w: self }
    }
    #[doc = "Bit 10 - Filter bits"]
    #[inline(always)]
    pub fn fd10(&mut self) -> FD10_W {
        FD10_W { w: self }
    }
    #[doc = "Bit 11 - Filter bits"]
    #[inline(always)]
    pub fn fd11(&mut self) -> FD11_W {
        FD11_W { w: self }
    }
    #[doc = "Bit 12 - Filter bits"]
    #[inline(always)]
    pub fn fd12(&mut self) -> FD12_W {
        FD12_W { w: self }
    }
    #[doc = "Bit 13 - Filter bits"]
    #[inline(always)]
    pub fn fd13(&mut self) -> FD13_W {
        FD13_W { w: self }
    }
    #[doc = "Bit 14 - Filter bits"]
    #[inline(always)]
    pub fn fd14(&mut self) -> FD14_W {
        FD14_W { w: self }
    }
    #[doc = "Bit 15 - Filter bits"]
    #[inline(always)]
    pub fn fd15(&mut self) -> FD15_W {
        FD15_W { w: self }
    }
    #[doc = "Bit 16 - Filter bits"]
    #[inline(always)]
    pub fn fd16(&mut self) -> FD16_W {
        FD16_W { w: self }
    }
    #[doc = "Bit 17 - Filter bits"]
    #[inline(always)]
    pub fn fd17(&mut self) -> FD17_W {
        FD17_W { w: self }
    }
    #[doc = "Bit 18 - Filter bits"]
    #[inline(always)]
    pub fn fd18(&mut self) -> FD18_W {
        FD18_W { w: self }
    }
    #[doc = "Bit 19 - Filter bits"]
    #[inline(always)]
    pub fn fd19(&mut self) -> FD19_W {
        FD19_W { w: self }
    }
    #[doc = "Bit 20 - Filter bits"]
    #[inline(always)]
    pub fn fd20(&mut self) -> FD20_W {
        FD20_W { w: self }
    }
    #[doc = "Bit 21 - Filter bits"]
    #[inline(always)]
    pub fn fd21(&mut self) -> FD21_W {
        FD21_W { w: self }
    }
    #[doc = "Bit 22 - Filter bits"]
    #[inline(always)]
    pub fn fd22(&mut self) -> FD22_W {
        FD22_W { w: self }
    }
    #[doc = "Bit 23 - Filter bits"]
    #[inline(always)]
    pub fn fd23(&mut self) -> FD23_W {
        FD23_W { w: self }
    }
    #[doc = "Bit 24 - Filter bits"]
    #[inline(always)]
    pub fn fd24(&mut self) -> FD24_W {
        FD24_W { w: self }
    }
    #[doc = "Bit 25 - Filter bits"]
    #[inline(always)]
    pub fn fd25(&mut self) -> FD25_W {
        FD25_W { w: self }
    }
    #[doc = "Bit 26 - Filter bits"]
    #[inline(always)]
    pub fn fd26(&mut self) -> FD26_W {
        FD26_W { w: self }
    }
    #[doc = "Bit 27 - Filter bits"]
    #[inline(always)]
    pub fn fd27(&mut self) -> FD27_W {
        FD27_W { w: self }
    }
    #[doc = "Bit 28 - Filter bits"]
    #[inline(always)]
    pub fn fd28(&mut self) -> FD28_W {
        FD28_W { w: self }
    }
    #[doc = "Bit 29 - Filter bits"]
    #[inline(always)]
    pub fn fd29(&mut self) -> FD29_W {
        FD29_W { w: self }
    }
    #[doc = "Bit 30 - Filter bits"]
    #[inline(always)]
    pub fn fd30(&mut self) -> FD30_W {
        FD30_W { w: self }
    }
    #[doc = "Bit 31 - Filter bits"]
    #[inline(always)]
    pub fn fd31(&mut self) -> FD31_W {
        FD31_W { w: self }
    }
}
