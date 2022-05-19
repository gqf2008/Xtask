#[doc = "Reader of register OCTL"]
pub type R = crate::R<u32, super::OCTL>;
#[doc = "Writer for register OCTL"]
pub type W = crate::W<u32, super::OCTL>;
#[doc = "Register OCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::OCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OCTL15`"]
pub type OCTL15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCTL15`"]
pub struct OCTL15_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTL15_W<'a> {
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
#[doc = "Reader of field `OCTL14`"]
pub type OCTL14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCTL14`"]
pub struct OCTL14_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTL14_W<'a> {
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
#[doc = "Reader of field `OCTL13`"]
pub type OCTL13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCTL13`"]
pub struct OCTL13_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTL13_W<'a> {
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
#[doc = "Reader of field `OCTL12`"]
pub type OCTL12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCTL12`"]
pub struct OCTL12_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTL12_W<'a> {
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
#[doc = "Reader of field `OCTL11`"]
pub type OCTL11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCTL11`"]
pub struct OCTL11_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTL11_W<'a> {
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
#[doc = "Reader of field `OCTL10`"]
pub type OCTL10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCTL10`"]
pub struct OCTL10_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTL10_W<'a> {
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
#[doc = "Reader of field `OCTL9`"]
pub type OCTL9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCTL9`"]
pub struct OCTL9_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTL9_W<'a> {
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
#[doc = "Reader of field `OCTL8`"]
pub type OCTL8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCTL8`"]
pub struct OCTL8_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTL8_W<'a> {
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
#[doc = "Reader of field `OCTL7`"]
pub type OCTL7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCTL7`"]
pub struct OCTL7_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTL7_W<'a> {
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
#[doc = "Reader of field `OCTL6`"]
pub type OCTL6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCTL6`"]
pub struct OCTL6_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTL6_W<'a> {
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
#[doc = "Reader of field `OCTL5`"]
pub type OCTL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCTL5`"]
pub struct OCTL5_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTL5_W<'a> {
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
#[doc = "Reader of field `OCTL4`"]
pub type OCTL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCTL4`"]
pub struct OCTL4_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTL4_W<'a> {
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
#[doc = "Reader of field `OCTL3`"]
pub type OCTL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCTL3`"]
pub struct OCTL3_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTL3_W<'a> {
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
#[doc = "Reader of field `OCTL2`"]
pub type OCTL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCTL2`"]
pub struct OCTL2_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTL2_W<'a> {
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
#[doc = "Reader of field `OCTL1`"]
pub type OCTL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCTL1`"]
pub struct OCTL1_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTL1_W<'a> {
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
#[doc = "Reader of field `OCTL0`"]
pub type OCTL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCTL0`"]
pub struct OCTL0_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTL0_W<'a> {
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
    #[doc = "Bit 15 - Port output control"]
    #[inline(always)]
    pub fn octl15(&self) -> OCTL15_R {
        OCTL15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port output control"]
    #[inline(always)]
    pub fn octl14(&self) -> OCTL14_R {
        OCTL14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port output control"]
    #[inline(always)]
    pub fn octl13(&self) -> OCTL13_R {
        OCTL13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port output control"]
    #[inline(always)]
    pub fn octl12(&self) -> OCTL12_R {
        OCTL12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port output control"]
    #[inline(always)]
    pub fn octl11(&self) -> OCTL11_R {
        OCTL11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port output control"]
    #[inline(always)]
    pub fn octl10(&self) -> OCTL10_R {
        OCTL10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port output control"]
    #[inline(always)]
    pub fn octl9(&self) -> OCTL9_R {
        OCTL9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port output control"]
    #[inline(always)]
    pub fn octl8(&self) -> OCTL8_R {
        OCTL8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port output control"]
    #[inline(always)]
    pub fn octl7(&self) -> OCTL7_R {
        OCTL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port output control"]
    #[inline(always)]
    pub fn octl6(&self) -> OCTL6_R {
        OCTL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port output control"]
    #[inline(always)]
    pub fn octl5(&self) -> OCTL5_R {
        OCTL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port output control"]
    #[inline(always)]
    pub fn octl4(&self) -> OCTL4_R {
        OCTL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port output control"]
    #[inline(always)]
    pub fn octl3(&self) -> OCTL3_R {
        OCTL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port output control"]
    #[inline(always)]
    pub fn octl2(&self) -> OCTL2_R {
        OCTL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port output control"]
    #[inline(always)]
    pub fn octl1(&self) -> OCTL1_R {
        OCTL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Port output control"]
    #[inline(always)]
    pub fn octl0(&self) -> OCTL0_R {
        OCTL0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Port output control"]
    #[inline(always)]
    pub fn octl15(&mut self) -> OCTL15_W {
        OCTL15_W { w: self }
    }
    #[doc = "Bit 14 - Port output control"]
    #[inline(always)]
    pub fn octl14(&mut self) -> OCTL14_W {
        OCTL14_W { w: self }
    }
    #[doc = "Bit 13 - Port output control"]
    #[inline(always)]
    pub fn octl13(&mut self) -> OCTL13_W {
        OCTL13_W { w: self }
    }
    #[doc = "Bit 12 - Port output control"]
    #[inline(always)]
    pub fn octl12(&mut self) -> OCTL12_W {
        OCTL12_W { w: self }
    }
    #[doc = "Bit 11 - Port output control"]
    #[inline(always)]
    pub fn octl11(&mut self) -> OCTL11_W {
        OCTL11_W { w: self }
    }
    #[doc = "Bit 10 - Port output control"]
    #[inline(always)]
    pub fn octl10(&mut self) -> OCTL10_W {
        OCTL10_W { w: self }
    }
    #[doc = "Bit 9 - Port output control"]
    #[inline(always)]
    pub fn octl9(&mut self) -> OCTL9_W {
        OCTL9_W { w: self }
    }
    #[doc = "Bit 8 - Port output control"]
    #[inline(always)]
    pub fn octl8(&mut self) -> OCTL8_W {
        OCTL8_W { w: self }
    }
    #[doc = "Bit 7 - Port output control"]
    #[inline(always)]
    pub fn octl7(&mut self) -> OCTL7_W {
        OCTL7_W { w: self }
    }
    #[doc = "Bit 6 - Port output control"]
    #[inline(always)]
    pub fn octl6(&mut self) -> OCTL6_W {
        OCTL6_W { w: self }
    }
    #[doc = "Bit 5 - Port output control"]
    #[inline(always)]
    pub fn octl5(&mut self) -> OCTL5_W {
        OCTL5_W { w: self }
    }
    #[doc = "Bit 4 - Port output control"]
    #[inline(always)]
    pub fn octl4(&mut self) -> OCTL4_W {
        OCTL4_W { w: self }
    }
    #[doc = "Bit 3 - Port output control"]
    #[inline(always)]
    pub fn octl3(&mut self) -> OCTL3_W {
        OCTL3_W { w: self }
    }
    #[doc = "Bit 2 - Port output control"]
    #[inline(always)]
    pub fn octl2(&mut self) -> OCTL2_W {
        OCTL2_W { w: self }
    }
    #[doc = "Bit 1 - Port output control"]
    #[inline(always)]
    pub fn octl1(&mut self) -> OCTL1_W {
        OCTL1_W { w: self }
    }
    #[doc = "Bit 0 - Port output control"]
    #[inline(always)]
    pub fn octl0(&mut self) -> OCTL0_W {
        OCTL0_W { w: self }
    }
}
