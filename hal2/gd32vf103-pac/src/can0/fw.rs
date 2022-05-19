#[doc = "Reader of register FW"]
pub type R = crate::R<u32, super::FW>;
#[doc = "Writer for register FW"]
pub type W = crate::W<u32, super::FW>;
#[doc = "Register FW `reset()`'s with value 0"]
impl crate::ResetValue for super::FW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FW0`"]
pub type FW0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW0`"]
pub struct FW0_W<'a> {
    w: &'a mut W,
}
impl<'a> FW0_W<'a> {
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
#[doc = "Reader of field `FW1`"]
pub type FW1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW1`"]
pub struct FW1_W<'a> {
    w: &'a mut W,
}
impl<'a> FW1_W<'a> {
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
#[doc = "Reader of field `FW2`"]
pub type FW2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW2`"]
pub struct FW2_W<'a> {
    w: &'a mut W,
}
impl<'a> FW2_W<'a> {
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
#[doc = "Reader of field `FW3`"]
pub type FW3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW3`"]
pub struct FW3_W<'a> {
    w: &'a mut W,
}
impl<'a> FW3_W<'a> {
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
#[doc = "Reader of field `FW4`"]
pub type FW4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW4`"]
pub struct FW4_W<'a> {
    w: &'a mut W,
}
impl<'a> FW4_W<'a> {
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
#[doc = "Reader of field `FW5`"]
pub type FW5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW5`"]
pub struct FW5_W<'a> {
    w: &'a mut W,
}
impl<'a> FW5_W<'a> {
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
#[doc = "Reader of field `FW6`"]
pub type FW6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW6`"]
pub struct FW6_W<'a> {
    w: &'a mut W,
}
impl<'a> FW6_W<'a> {
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
#[doc = "Reader of field `FW7`"]
pub type FW7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW7`"]
pub struct FW7_W<'a> {
    w: &'a mut W,
}
impl<'a> FW7_W<'a> {
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
#[doc = "Reader of field `FW8`"]
pub type FW8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW8`"]
pub struct FW8_W<'a> {
    w: &'a mut W,
}
impl<'a> FW8_W<'a> {
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
#[doc = "Reader of field `FW9`"]
pub type FW9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW9`"]
pub struct FW9_W<'a> {
    w: &'a mut W,
}
impl<'a> FW9_W<'a> {
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
#[doc = "Reader of field `FW10`"]
pub type FW10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW10`"]
pub struct FW10_W<'a> {
    w: &'a mut W,
}
impl<'a> FW10_W<'a> {
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
#[doc = "Reader of field `FW11`"]
pub type FW11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW11`"]
pub struct FW11_W<'a> {
    w: &'a mut W,
}
impl<'a> FW11_W<'a> {
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
#[doc = "Reader of field `FW12`"]
pub type FW12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW12`"]
pub struct FW12_W<'a> {
    w: &'a mut W,
}
impl<'a> FW12_W<'a> {
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
#[doc = "Reader of field `FW13`"]
pub type FW13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW13`"]
pub struct FW13_W<'a> {
    w: &'a mut W,
}
impl<'a> FW13_W<'a> {
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
#[doc = "Reader of field `FW14`"]
pub type FW14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW14`"]
pub struct FW14_W<'a> {
    w: &'a mut W,
}
impl<'a> FW14_W<'a> {
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
#[doc = "Reader of field `FW15`"]
pub type FW15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW15`"]
pub struct FW15_W<'a> {
    w: &'a mut W,
}
impl<'a> FW15_W<'a> {
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
#[doc = "Reader of field `FW16`"]
pub type FW16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW16`"]
pub struct FW16_W<'a> {
    w: &'a mut W,
}
impl<'a> FW16_W<'a> {
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
#[doc = "Reader of field `FW17`"]
pub type FW17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW17`"]
pub struct FW17_W<'a> {
    w: &'a mut W,
}
impl<'a> FW17_W<'a> {
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
#[doc = "Reader of field `FW18`"]
pub type FW18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW18`"]
pub struct FW18_W<'a> {
    w: &'a mut W,
}
impl<'a> FW18_W<'a> {
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
#[doc = "Reader of field `FW19`"]
pub type FW19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW19`"]
pub struct FW19_W<'a> {
    w: &'a mut W,
}
impl<'a> FW19_W<'a> {
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
#[doc = "Reader of field `FW20`"]
pub type FW20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW20`"]
pub struct FW20_W<'a> {
    w: &'a mut W,
}
impl<'a> FW20_W<'a> {
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
#[doc = "Reader of field `FW21`"]
pub type FW21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW21`"]
pub struct FW21_W<'a> {
    w: &'a mut W,
}
impl<'a> FW21_W<'a> {
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
#[doc = "Reader of field `FW22`"]
pub type FW22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW22`"]
pub struct FW22_W<'a> {
    w: &'a mut W,
}
impl<'a> FW22_W<'a> {
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
#[doc = "Reader of field `FW23`"]
pub type FW23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW23`"]
pub struct FW23_W<'a> {
    w: &'a mut W,
}
impl<'a> FW23_W<'a> {
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
#[doc = "Reader of field `FW24`"]
pub type FW24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW24`"]
pub struct FW24_W<'a> {
    w: &'a mut W,
}
impl<'a> FW24_W<'a> {
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
#[doc = "Reader of field `FW25`"]
pub type FW25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW25`"]
pub struct FW25_W<'a> {
    w: &'a mut W,
}
impl<'a> FW25_W<'a> {
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
#[doc = "Reader of field `FW26`"]
pub type FW26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW26`"]
pub struct FW26_W<'a> {
    w: &'a mut W,
}
impl<'a> FW26_W<'a> {
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
#[doc = "Reader of field `FW27`"]
pub type FW27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW27`"]
pub struct FW27_W<'a> {
    w: &'a mut W,
}
impl<'a> FW27_W<'a> {
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
    #[doc = "Bit 0 - Filter working"]
    #[inline(always)]
    pub fn fw0(&self) -> FW0_R {
        FW0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Filter working"]
    #[inline(always)]
    pub fn fw1(&self) -> FW1_R {
        FW1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Filter working"]
    #[inline(always)]
    pub fn fw2(&self) -> FW2_R {
        FW2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Filter working"]
    #[inline(always)]
    pub fn fw3(&self) -> FW3_R {
        FW3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Filter working"]
    #[inline(always)]
    pub fn fw4(&self) -> FW4_R {
        FW4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Filter working"]
    #[inline(always)]
    pub fn fw5(&self) -> FW5_R {
        FW5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Filter working"]
    #[inline(always)]
    pub fn fw6(&self) -> FW6_R {
        FW6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Filter working"]
    #[inline(always)]
    pub fn fw7(&self) -> FW7_R {
        FW7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Filter working"]
    #[inline(always)]
    pub fn fw8(&self) -> FW8_R {
        FW8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Filter working"]
    #[inline(always)]
    pub fn fw9(&self) -> FW9_R {
        FW9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Filter working"]
    #[inline(always)]
    pub fn fw10(&self) -> FW10_R {
        FW10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Filter working"]
    #[inline(always)]
    pub fn fw11(&self) -> FW11_R {
        FW11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Filter working"]
    #[inline(always)]
    pub fn fw12(&self) -> FW12_R {
        FW12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Filter working"]
    #[inline(always)]
    pub fn fw13(&self) -> FW13_R {
        FW13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Filter working"]
    #[inline(always)]
    pub fn fw14(&self) -> FW14_R {
        FW14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Filter working"]
    #[inline(always)]
    pub fn fw15(&self) -> FW15_R {
        FW15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Filter working"]
    #[inline(always)]
    pub fn fw16(&self) -> FW16_R {
        FW16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Filter working"]
    #[inline(always)]
    pub fn fw17(&self) -> FW17_R {
        FW17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Filter working"]
    #[inline(always)]
    pub fn fw18(&self) -> FW18_R {
        FW18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Filter working"]
    #[inline(always)]
    pub fn fw19(&self) -> FW19_R {
        FW19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Filter working"]
    #[inline(always)]
    pub fn fw20(&self) -> FW20_R {
        FW20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Filter working"]
    #[inline(always)]
    pub fn fw21(&self) -> FW21_R {
        FW21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Filter working"]
    #[inline(always)]
    pub fn fw22(&self) -> FW22_R {
        FW22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Filter working"]
    #[inline(always)]
    pub fn fw23(&self) -> FW23_R {
        FW23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Filter working"]
    #[inline(always)]
    pub fn fw24(&self) -> FW24_R {
        FW24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Filter working"]
    #[inline(always)]
    pub fn fw25(&self) -> FW25_R {
        FW25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Filter working"]
    #[inline(always)]
    pub fn fw26(&self) -> FW26_R {
        FW26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Filter working"]
    #[inline(always)]
    pub fn fw27(&self) -> FW27_R {
        FW27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter working"]
    #[inline(always)]
    pub fn fw0(&mut self) -> FW0_W {
        FW0_W { w: self }
    }
    #[doc = "Bit 1 - Filter working"]
    #[inline(always)]
    pub fn fw1(&mut self) -> FW1_W {
        FW1_W { w: self }
    }
    #[doc = "Bit 2 - Filter working"]
    #[inline(always)]
    pub fn fw2(&mut self) -> FW2_W {
        FW2_W { w: self }
    }
    #[doc = "Bit 3 - Filter working"]
    #[inline(always)]
    pub fn fw3(&mut self) -> FW3_W {
        FW3_W { w: self }
    }
    #[doc = "Bit 4 - Filter working"]
    #[inline(always)]
    pub fn fw4(&mut self) -> FW4_W {
        FW4_W { w: self }
    }
    #[doc = "Bit 5 - Filter working"]
    #[inline(always)]
    pub fn fw5(&mut self) -> FW5_W {
        FW5_W { w: self }
    }
    #[doc = "Bit 6 - Filter working"]
    #[inline(always)]
    pub fn fw6(&mut self) -> FW6_W {
        FW6_W { w: self }
    }
    #[doc = "Bit 7 - Filter working"]
    #[inline(always)]
    pub fn fw7(&mut self) -> FW7_W {
        FW7_W { w: self }
    }
    #[doc = "Bit 8 - Filter working"]
    #[inline(always)]
    pub fn fw8(&mut self) -> FW8_W {
        FW8_W { w: self }
    }
    #[doc = "Bit 9 - Filter working"]
    #[inline(always)]
    pub fn fw9(&mut self) -> FW9_W {
        FW9_W { w: self }
    }
    #[doc = "Bit 10 - Filter working"]
    #[inline(always)]
    pub fn fw10(&mut self) -> FW10_W {
        FW10_W { w: self }
    }
    #[doc = "Bit 11 - Filter working"]
    #[inline(always)]
    pub fn fw11(&mut self) -> FW11_W {
        FW11_W { w: self }
    }
    #[doc = "Bit 12 - Filter working"]
    #[inline(always)]
    pub fn fw12(&mut self) -> FW12_W {
        FW12_W { w: self }
    }
    #[doc = "Bit 13 - Filter working"]
    #[inline(always)]
    pub fn fw13(&mut self) -> FW13_W {
        FW13_W { w: self }
    }
    #[doc = "Bit 14 - Filter working"]
    #[inline(always)]
    pub fn fw14(&mut self) -> FW14_W {
        FW14_W { w: self }
    }
    #[doc = "Bit 15 - Filter working"]
    #[inline(always)]
    pub fn fw15(&mut self) -> FW15_W {
        FW15_W { w: self }
    }
    #[doc = "Bit 16 - Filter working"]
    #[inline(always)]
    pub fn fw16(&mut self) -> FW16_W {
        FW16_W { w: self }
    }
    #[doc = "Bit 17 - Filter working"]
    #[inline(always)]
    pub fn fw17(&mut self) -> FW17_W {
        FW17_W { w: self }
    }
    #[doc = "Bit 18 - Filter working"]
    #[inline(always)]
    pub fn fw18(&mut self) -> FW18_W {
        FW18_W { w: self }
    }
    #[doc = "Bit 19 - Filter working"]
    #[inline(always)]
    pub fn fw19(&mut self) -> FW19_W {
        FW19_W { w: self }
    }
    #[doc = "Bit 20 - Filter working"]
    #[inline(always)]
    pub fn fw20(&mut self) -> FW20_W {
        FW20_W { w: self }
    }
    #[doc = "Bit 21 - Filter working"]
    #[inline(always)]
    pub fn fw21(&mut self) -> FW21_W {
        FW21_W { w: self }
    }
    #[doc = "Bit 22 - Filter working"]
    #[inline(always)]
    pub fn fw22(&mut self) -> FW22_W {
        FW22_W { w: self }
    }
    #[doc = "Bit 23 - Filter working"]
    #[inline(always)]
    pub fn fw23(&mut self) -> FW23_W {
        FW23_W { w: self }
    }
    #[doc = "Bit 24 - Filter working"]
    #[inline(always)]
    pub fn fw24(&mut self) -> FW24_W {
        FW24_W { w: self }
    }
    #[doc = "Bit 25 - Filter working"]
    #[inline(always)]
    pub fn fw25(&mut self) -> FW25_W {
        FW25_W { w: self }
    }
    #[doc = "Bit 26 - Filter working"]
    #[inline(always)]
    pub fn fw26(&mut self) -> FW26_W {
        FW26_W { w: self }
    }
    #[doc = "Bit 27 - Filter working"]
    #[inline(always)]
    pub fn fw27(&mut self) -> FW27_W {
        FW27_W { w: self }
    }
}
