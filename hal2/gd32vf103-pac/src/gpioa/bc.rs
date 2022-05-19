#[doc = "Writer for register BC"]
pub type W = crate::W<u32, super::BC>;
#[doc = "Register BC `reset()`'s with value 0"]
impl crate::ResetValue for super::BC {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 15 - Port 15 Clear bit"]
    #[inline(always)]
    pub fn cr15(&mut self) -> CR15_W {
        CR15_W { w: self }
    }
    #[doc = "Bit 14 - Port 14 Clear bit"]
    #[inline(always)]
    pub fn cr14(&mut self) -> CR14_W {
        CR14_W { w: self }
    }
    #[doc = "Bit 13 - Port 13 Clear bit"]
    #[inline(always)]
    pub fn cr13(&mut self) -> CR13_W {
        CR13_W { w: self }
    }
    #[doc = "Bit 12 - Port 12 Clear bit"]
    #[inline(always)]
    pub fn cr12(&mut self) -> CR12_W {
        CR12_W { w: self }
    }
    #[doc = "Bit 11 - Port 11 Clear bit"]
    #[inline(always)]
    pub fn cr11(&mut self) -> CR11_W {
        CR11_W { w: self }
    }
    #[doc = "Bit 10 - Port 10 Clear bit"]
    #[inline(always)]
    pub fn cr10(&mut self) -> CR10_W {
        CR10_W { w: self }
    }
    #[doc = "Bit 9 - Port 9 Clear bit"]
    #[inline(always)]
    pub fn cr9(&mut self) -> CR9_W {
        CR9_W { w: self }
    }
    #[doc = "Bit 8 - Port 8 Clear bit"]
    #[inline(always)]
    pub fn cr8(&mut self) -> CR8_W {
        CR8_W { w: self }
    }
    #[doc = "Bit 7 - Port 7 Clear bit"]
    #[inline(always)]
    pub fn cr7(&mut self) -> CR7_W {
        CR7_W { w: self }
    }
    #[doc = "Bit 6 - Port 6 Clear bit"]
    #[inline(always)]
    pub fn cr6(&mut self) -> CR6_W {
        CR6_W { w: self }
    }
    #[doc = "Bit 5 - Port 5 Clear bit"]
    #[inline(always)]
    pub fn cr5(&mut self) -> CR5_W {
        CR5_W { w: self }
    }
    #[doc = "Bit 4 - Port 4 Clear bit"]
    #[inline(always)]
    pub fn cr4(&mut self) -> CR4_W {
        CR4_W { w: self }
    }
    #[doc = "Bit 3 - Port 3 Clear bit"]
    #[inline(always)]
    pub fn cr3(&mut self) -> CR3_W {
        CR3_W { w: self }
    }
    #[doc = "Bit 2 - Port 2 Clear bit"]
    #[inline(always)]
    pub fn cr2(&mut self) -> CR2_W {
        CR2_W { w: self }
    }
    #[doc = "Bit 1 - Port 1 Clear bit"]
    #[inline(always)]
    pub fn cr1(&mut self) -> CR1_W {
        CR1_W { w: self }
    }
    #[doc = "Bit 0 - Port 0 Clear bit"]
    #[inline(always)]
    pub fn cr0(&mut self) -> CR0_W {
        CR0_W { w: self }
    }
}
