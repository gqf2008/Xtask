#[doc = "Writer for register BOP"]
pub type W = crate::W<u32, super::BOP>;
#[doc = "Register BOP `reset()`'s with value 0"]
impl crate::ResetValue for super::BOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CR15`"]
pub struct CR15_W<'a> {
    w: &'a mut W,
}
impl<'a> CR15_W<'a> {
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
#[doc = "Write proxy for field `CR14`"]
pub struct CR14_W<'a> {
    w: &'a mut W,
}
impl<'a> CR14_W<'a> {
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
#[doc = "Write proxy for field `CR13`"]
pub struct CR13_W<'a> {
    w: &'a mut W,
}
impl<'a> CR13_W<'a> {
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
#[doc = "Write proxy for field `CR12`"]
pub struct CR12_W<'a> {
    w: &'a mut W,
}
impl<'a> CR12_W<'a> {
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
#[doc = "Write proxy for field `CR11`"]
pub struct CR11_W<'a> {
    w: &'a mut W,
}
impl<'a> CR11_W<'a> {
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
#[doc = "Write proxy for field `CR10`"]
pub struct CR10_W<'a> {
    w: &'a mut W,
}
impl<'a> CR10_W<'a> {
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
#[doc = "Write proxy for field `CR9`"]
pub struct CR9_W<'a> {
    w: &'a mut W,
}
impl<'a> CR9_W<'a> {
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
#[doc = "Write proxy for field `CR8`"]
pub struct CR8_W<'a> {
    w: &'a mut W,
}
impl<'a> CR8_W<'a> {
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
#[doc = "Write proxy for field `CR7`"]
pub struct CR7_W<'a> {
    w: &'a mut W,
}
impl<'a> CR7_W<'a> {
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
#[doc = "Write proxy for field `CR6`"]
pub struct CR6_W<'a> {
    w: &'a mut W,
}
impl<'a> CR6_W<'a> {
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
#[doc = "Write proxy for field `CR5`"]
pub struct CR5_W<'a> {
    w: &'a mut W,
}
impl<'a> CR5_W<'a> {
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
#[doc = "Write proxy for field `CR4`"]
pub struct CR4_W<'a> {
    w: &'a mut W,
}
impl<'a> CR4_W<'a> {
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
#[doc = "Write proxy for field `CR3`"]
pub struct CR3_W<'a> {
    w: &'a mut W,
}
impl<'a> CR3_W<'a> {
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
#[doc = "Write proxy for field `CR2`"]
pub struct CR2_W<'a> {
    w: &'a mut W,
}
impl<'a> CR2_W<'a> {
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
#[doc = "Write proxy for field `CR1`"]
pub struct CR1_W<'a> {
    w: &'a mut W,
}
impl<'a> CR1_W<'a> {
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
#[doc = "Write proxy for field `CR0`"]
pub struct CR0_W<'a> {
    w: &'a mut W,
}
impl<'a> CR0_W<'a> {
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
#[doc = "Write proxy for field `BOP15`"]
pub struct BOP15_W<'a> {
    w: &'a mut W,
}
impl<'a> BOP15_W<'a> {
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
#[doc = "Write proxy for field `BOP14`"]
pub struct BOP14_W<'a> {
    w: &'a mut W,
}
impl<'a> BOP14_W<'a> {
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
#[doc = "Write proxy for field `BOP13`"]
pub struct BOP13_W<'a> {
    w: &'a mut W,
}
impl<'a> BOP13_W<'a> {
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
#[doc = "Write proxy for field `BOP12`"]
pub struct BOP12_W<'a> {
    w: &'a mut W,
}
impl<'a> BOP12_W<'a> {
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
#[doc = "Write proxy for field `BOP11`"]
pub struct BOP11_W<'a> {
    w: &'a mut W,
}
impl<'a> BOP11_W<'a> {
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
#[doc = "Write proxy for field `BOP10`"]
pub struct BOP10_W<'a> {
    w: &'a mut W,
}
impl<'a> BOP10_W<'a> {
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
#[doc = "Write proxy for field `BOP9`"]
pub struct BOP9_W<'a> {
    w: &'a mut W,
}
impl<'a> BOP9_W<'a> {
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
#[doc = "Write proxy for field `BOP8`"]
pub struct BOP8_W<'a> {
    w: &'a mut W,
}
impl<'a> BOP8_W<'a> {
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
#[doc = "Write proxy for field `BOP7`"]
pub struct BOP7_W<'a> {
    w: &'a mut W,
}
impl<'a> BOP7_W<'a> {
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
#[doc = "Write proxy for field `BOP6`"]
pub struct BOP6_W<'a> {
    w: &'a mut W,
}
impl<'a> BOP6_W<'a> {
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
#[doc = "Write proxy for field `BOP5`"]
pub struct BOP5_W<'a> {
    w: &'a mut W,
}
impl<'a> BOP5_W<'a> {
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
#[doc = "Write proxy for field `BOP4`"]
pub struct BOP4_W<'a> {
    w: &'a mut W,
}
impl<'a> BOP4_W<'a> {
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
#[doc = "Write proxy for field `BOP3`"]
pub struct BOP3_W<'a> {
    w: &'a mut W,
}
impl<'a> BOP3_W<'a> {
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
#[doc = "Write proxy for field `BOP2`"]
pub struct BOP2_W<'a> {
    w: &'a mut W,
}
impl<'a> BOP2_W<'a> {
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
#[doc = "Write proxy for field `BOP1`"]
pub struct BOP1_W<'a> {
    w: &'a mut W,
}
impl<'a> BOP1_W<'a> {
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
#[doc = "Write proxy for field `BOP0`"]
pub struct BOP0_W<'a> {
    w: &'a mut W,
}
impl<'a> BOP0_W<'a> {
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
impl W {
    #[doc = "Bit 31 - Port 15 Clear bit"]
    #[inline(always)]
    pub fn cr15(&mut self) -> CR15_W {
        CR15_W { w: self }
    }
    #[doc = "Bit 30 - Port 14 Clear bit"]
    #[inline(always)]
    pub fn cr14(&mut self) -> CR14_W {
        CR14_W { w: self }
    }
    #[doc = "Bit 29 - Port 13 Clear bit"]
    #[inline(always)]
    pub fn cr13(&mut self) -> CR13_W {
        CR13_W { w: self }
    }
    #[doc = "Bit 28 - Port 12 Clear bit"]
    #[inline(always)]
    pub fn cr12(&mut self) -> CR12_W {
        CR12_W { w: self }
    }
    #[doc = "Bit 27 - Port 11 Clear bit"]
    #[inline(always)]
    pub fn cr11(&mut self) -> CR11_W {
        CR11_W { w: self }
    }
    #[doc = "Bit 26 - Port 10 Clear bit"]
    #[inline(always)]
    pub fn cr10(&mut self) -> CR10_W {
        CR10_W { w: self }
    }
    #[doc = "Bit 25 - Port 9 Clear bit"]
    #[inline(always)]
    pub fn cr9(&mut self) -> CR9_W {
        CR9_W { w: self }
    }
    #[doc = "Bit 24 - Port 8 Clear bit"]
    #[inline(always)]
    pub fn cr8(&mut self) -> CR8_W {
        CR8_W { w: self }
    }
    #[doc = "Bit 23 - Port 7 Clear bit"]
    #[inline(always)]
    pub fn cr7(&mut self) -> CR7_W {
        CR7_W { w: self }
    }
    #[doc = "Bit 22 - Port 6 Clear bit"]
    #[inline(always)]
    pub fn cr6(&mut self) -> CR6_W {
        CR6_W { w: self }
    }
    #[doc = "Bit 21 - Port 5 Clear bit"]
    #[inline(always)]
    pub fn cr5(&mut self) -> CR5_W {
        CR5_W { w: self }
    }
    #[doc = "Bit 20 - Port 4 Clear bit"]
    #[inline(always)]
    pub fn cr4(&mut self) -> CR4_W {
        CR4_W { w: self }
    }
    #[doc = "Bit 19 - Port 3 Clear bit"]
    #[inline(always)]
    pub fn cr3(&mut self) -> CR3_W {
        CR3_W { w: self }
    }
    #[doc = "Bit 18 - Port 2 Clear bit"]
    #[inline(always)]
    pub fn cr2(&mut self) -> CR2_W {
        CR2_W { w: self }
    }
    #[doc = "Bit 17 - Port 1 Clear bit"]
    #[inline(always)]
    pub fn cr1(&mut self) -> CR1_W {
        CR1_W { w: self }
    }
    #[doc = "Bit 16 - Port 0 Clear bit"]
    #[inline(always)]
    pub fn cr0(&mut self) -> CR0_W {
        CR0_W { w: self }
    }
    #[doc = "Bit 15 - Port 15 Set bit"]
    #[inline(always)]
    pub fn bop15(&mut self) -> BOP15_W {
        BOP15_W { w: self }
    }
    #[doc = "Bit 14 - Port 14 Set bit"]
    #[inline(always)]
    pub fn bop14(&mut self) -> BOP14_W {
        BOP14_W { w: self }
    }
    #[doc = "Bit 13 - Port 13 Set bit"]
    #[inline(always)]
    pub fn bop13(&mut self) -> BOP13_W {
        BOP13_W { w: self }
    }
    #[doc = "Bit 12 - Port 12 Set bit"]
    #[inline(always)]
    pub fn bop12(&mut self) -> BOP12_W {
        BOP12_W { w: self }
    }
    #[doc = "Bit 11 - Port 11 Set bit"]
    #[inline(always)]
    pub fn bop11(&mut self) -> BOP11_W {
        BOP11_W { w: self }
    }
    #[doc = "Bit 10 - Port 10 Set bit"]
    #[inline(always)]
    pub fn bop10(&mut self) -> BOP10_W {
        BOP10_W { w: self }
    }
    #[doc = "Bit 9 - Port 9 Set bit"]
    #[inline(always)]
    pub fn bop9(&mut self) -> BOP9_W {
        BOP9_W { w: self }
    }
    #[doc = "Bit 8 - Port 8 Set bit"]
    #[inline(always)]
    pub fn bop8(&mut self) -> BOP8_W {
        BOP8_W { w: self }
    }
    #[doc = "Bit 7 - Port 7 Set bit"]
    #[inline(always)]
    pub fn bop7(&mut self) -> BOP7_W {
        BOP7_W { w: self }
    }
    #[doc = "Bit 6 - Port 6 Set bit"]
    #[inline(always)]
    pub fn bop6(&mut self) -> BOP6_W {
        BOP6_W { w: self }
    }
    #[doc = "Bit 5 - Port 5 Set bit"]
    #[inline(always)]
    pub fn bop5(&mut self) -> BOP5_W {
        BOP5_W { w: self }
    }
    #[doc = "Bit 4 - Port 4 Set bit"]
    #[inline(always)]
    pub fn bop4(&mut self) -> BOP4_W {
        BOP4_W { w: self }
    }
    #[doc = "Bit 3 - Port 3 Set bit"]
    #[inline(always)]
    pub fn bop3(&mut self) -> BOP3_W {
        BOP3_W { w: self }
    }
    #[doc = "Bit 2 - Port 2 Set bit"]
    #[inline(always)]
    pub fn bop2(&mut self) -> BOP2_W {
        BOP2_W { w: self }
    }
    #[doc = "Bit 1 - Port 1 Set bit"]
    #[inline(always)]
    pub fn bop1(&mut self) -> BOP1_W {
        BOP1_W { w: self }
    }
    #[doc = "Bit 0 - Port 0 Set bit"]
    #[inline(always)]
    pub fn bop0(&mut self) -> BOP0_W {
        BOP0_W { w: self }
    }
}
