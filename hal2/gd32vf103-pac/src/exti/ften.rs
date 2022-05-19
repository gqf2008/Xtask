#[doc = "Reader of register FTEN"]
pub type R = crate::R<u32, super::FTEN>;
#[doc = "Writer for register FTEN"]
pub type W = crate::W<u32, super::FTEN>;
#[doc = "Register FTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::FTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FTEN0`"]
pub type FTEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTEN0`"]
pub struct FTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> FTEN0_W<'a> {
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
#[doc = "Reader of field `FTEN1`"]
pub type FTEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTEN1`"]
pub struct FTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> FTEN1_W<'a> {
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
#[doc = "Reader of field `FTEN2`"]
pub type FTEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTEN2`"]
pub struct FTEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> FTEN2_W<'a> {
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
#[doc = "Reader of field `FTEN3`"]
pub type FTEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTEN3`"]
pub struct FTEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> FTEN3_W<'a> {
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
#[doc = "Reader of field `FTEN4`"]
pub type FTEN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTEN4`"]
pub struct FTEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> FTEN4_W<'a> {
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
#[doc = "Reader of field `FTEN5`"]
pub type FTEN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTEN5`"]
pub struct FTEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> FTEN5_W<'a> {
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
#[doc = "Reader of field `FTEN6`"]
pub type FTEN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTEN6`"]
pub struct FTEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> FTEN6_W<'a> {
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
#[doc = "Reader of field `FTEN7`"]
pub type FTEN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTEN7`"]
pub struct FTEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> FTEN7_W<'a> {
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
#[doc = "Reader of field `FTEN8`"]
pub type FTEN8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTEN8`"]
pub struct FTEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> FTEN8_W<'a> {
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
#[doc = "Reader of field `FTEN9`"]
pub type FTEN9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTEN9`"]
pub struct FTEN9_W<'a> {
    w: &'a mut W,
}
impl<'a> FTEN9_W<'a> {
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
#[doc = "Reader of field `FTEN10`"]
pub type FTEN10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTEN10`"]
pub struct FTEN10_W<'a> {
    w: &'a mut W,
}
impl<'a> FTEN10_W<'a> {
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
#[doc = "Reader of field `FTEN11`"]
pub type FTEN11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTEN11`"]
pub struct FTEN11_W<'a> {
    w: &'a mut W,
}
impl<'a> FTEN11_W<'a> {
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
#[doc = "Reader of field `FTEN12`"]
pub type FTEN12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTEN12`"]
pub struct FTEN12_W<'a> {
    w: &'a mut W,
}
impl<'a> FTEN12_W<'a> {
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
#[doc = "Reader of field `FTEN13`"]
pub type FTEN13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTEN13`"]
pub struct FTEN13_W<'a> {
    w: &'a mut W,
}
impl<'a> FTEN13_W<'a> {
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
#[doc = "Reader of field `FTEN14`"]
pub type FTEN14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTEN14`"]
pub struct FTEN14_W<'a> {
    w: &'a mut W,
}
impl<'a> FTEN14_W<'a> {
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
#[doc = "Reader of field `FTEN15`"]
pub type FTEN15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTEN15`"]
pub struct FTEN15_W<'a> {
    w: &'a mut W,
}
impl<'a> FTEN15_W<'a> {
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
#[doc = "Reader of field `FTEN16`"]
pub type FTEN16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTEN16`"]
pub struct FTEN16_W<'a> {
    w: &'a mut W,
}
impl<'a> FTEN16_W<'a> {
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
#[doc = "Reader of field `FTEN17`"]
pub type FTEN17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTEN17`"]
pub struct FTEN17_W<'a> {
    w: &'a mut W,
}
impl<'a> FTEN17_W<'a> {
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
#[doc = "Reader of field `FTEN18`"]
pub type FTEN18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTEN18`"]
pub struct FTEN18_W<'a> {
    w: &'a mut W,
}
impl<'a> FTEN18_W<'a> {
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
    #[doc = "Bit 0 - Falling edge trigger enable of line 0"]
    #[inline(always)]
    pub fn ften0(&self) -> FTEN0_R {
        FTEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Falling edge trigger enable of line 1"]
    #[inline(always)]
    pub fn ften1(&self) -> FTEN1_R {
        FTEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Falling edge trigger enable of line 2"]
    #[inline(always)]
    pub fn ften2(&self) -> FTEN2_R {
        FTEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Falling edge trigger enable of line 3"]
    #[inline(always)]
    pub fn ften3(&self) -> FTEN3_R {
        FTEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Falling edge trigger enable of line 4"]
    #[inline(always)]
    pub fn ften4(&self) -> FTEN4_R {
        FTEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Falling edge trigger enable of line 5"]
    #[inline(always)]
    pub fn ften5(&self) -> FTEN5_R {
        FTEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Falling edge trigger enable of line 6"]
    #[inline(always)]
    pub fn ften6(&self) -> FTEN6_R {
        FTEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Falling edge trigger enable of line 7"]
    #[inline(always)]
    pub fn ften7(&self) -> FTEN7_R {
        FTEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Falling edge trigger enable of line 8"]
    #[inline(always)]
    pub fn ften8(&self) -> FTEN8_R {
        FTEN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Falling edge trigger enable of line 9"]
    #[inline(always)]
    pub fn ften9(&self) -> FTEN9_R {
        FTEN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Falling edge trigger enable of line 10"]
    #[inline(always)]
    pub fn ften10(&self) -> FTEN10_R {
        FTEN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Falling edge trigger enable of line 11"]
    #[inline(always)]
    pub fn ften11(&self) -> FTEN11_R {
        FTEN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Falling edge trigger enable of line 12"]
    #[inline(always)]
    pub fn ften12(&self) -> FTEN12_R {
        FTEN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Falling edge trigger enable of line 13"]
    #[inline(always)]
    pub fn ften13(&self) -> FTEN13_R {
        FTEN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Falling edge trigger enable of line 14"]
    #[inline(always)]
    pub fn ften14(&self) -> FTEN14_R {
        FTEN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Falling edge trigger enable of line 15"]
    #[inline(always)]
    pub fn ften15(&self) -> FTEN15_R {
        FTEN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Falling edge trigger enable of line 16"]
    #[inline(always)]
    pub fn ften16(&self) -> FTEN16_R {
        FTEN16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Falling edge trigger enable of line 17"]
    #[inline(always)]
    pub fn ften17(&self) -> FTEN17_R {
        FTEN17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Falling edge trigger enable of line 18"]
    #[inline(always)]
    pub fn ften18(&self) -> FTEN18_R {
        FTEN18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Falling edge trigger enable of line 0"]
    #[inline(always)]
    pub fn ften0(&mut self) -> FTEN0_W {
        FTEN0_W { w: self }
    }
    #[doc = "Bit 1 - Falling edge trigger enable of line 1"]
    #[inline(always)]
    pub fn ften1(&mut self) -> FTEN1_W {
        FTEN1_W { w: self }
    }
    #[doc = "Bit 2 - Falling edge trigger enable of line 2"]
    #[inline(always)]
    pub fn ften2(&mut self) -> FTEN2_W {
        FTEN2_W { w: self }
    }
    #[doc = "Bit 3 - Falling edge trigger enable of line 3"]
    #[inline(always)]
    pub fn ften3(&mut self) -> FTEN3_W {
        FTEN3_W { w: self }
    }
    #[doc = "Bit 4 - Falling edge trigger enable of line 4"]
    #[inline(always)]
    pub fn ften4(&mut self) -> FTEN4_W {
        FTEN4_W { w: self }
    }
    #[doc = "Bit 5 - Falling edge trigger enable of line 5"]
    #[inline(always)]
    pub fn ften5(&mut self) -> FTEN5_W {
        FTEN5_W { w: self }
    }
    #[doc = "Bit 6 - Falling edge trigger enable of line 6"]
    #[inline(always)]
    pub fn ften6(&mut self) -> FTEN6_W {
        FTEN6_W { w: self }
    }
    #[doc = "Bit 7 - Falling edge trigger enable of line 7"]
    #[inline(always)]
    pub fn ften7(&mut self) -> FTEN7_W {
        FTEN7_W { w: self }
    }
    #[doc = "Bit 8 - Falling edge trigger enable of line 8"]
    #[inline(always)]
    pub fn ften8(&mut self) -> FTEN8_W {
        FTEN8_W { w: self }
    }
    #[doc = "Bit 9 - Falling edge trigger enable of line 9"]
    #[inline(always)]
    pub fn ften9(&mut self) -> FTEN9_W {
        FTEN9_W { w: self }
    }
    #[doc = "Bit 10 - Falling edge trigger enable of line 10"]
    #[inline(always)]
    pub fn ften10(&mut self) -> FTEN10_W {
        FTEN10_W { w: self }
    }
    #[doc = "Bit 11 - Falling edge trigger enable of line 11"]
    #[inline(always)]
    pub fn ften11(&mut self) -> FTEN11_W {
        FTEN11_W { w: self }
    }
    #[doc = "Bit 12 - Falling edge trigger enable of line 12"]
    #[inline(always)]
    pub fn ften12(&mut self) -> FTEN12_W {
        FTEN12_W { w: self }
    }
    #[doc = "Bit 13 - Falling edge trigger enable of line 13"]
    #[inline(always)]
    pub fn ften13(&mut self) -> FTEN13_W {
        FTEN13_W { w: self }
    }
    #[doc = "Bit 14 - Falling edge trigger enable of line 14"]
    #[inline(always)]
    pub fn ften14(&mut self) -> FTEN14_W {
        FTEN14_W { w: self }
    }
    #[doc = "Bit 15 - Falling edge trigger enable of line 15"]
    #[inline(always)]
    pub fn ften15(&mut self) -> FTEN15_W {
        FTEN15_W { w: self }
    }
    #[doc = "Bit 16 - Falling edge trigger enable of line 16"]
    #[inline(always)]
    pub fn ften16(&mut self) -> FTEN16_W {
        FTEN16_W { w: self }
    }
    #[doc = "Bit 17 - Falling edge trigger enable of line 17"]
    #[inline(always)]
    pub fn ften17(&mut self) -> FTEN17_W {
        FTEN17_W { w: self }
    }
    #[doc = "Bit 18 - Falling edge trigger enable of line 18"]
    #[inline(always)]
    pub fn ften18(&mut self) -> FTEN18_W {
        FTEN18_W { w: self }
    }
}
