#[doc = "Reader of register FCTL"]
pub type R = crate::R<u32, super::FCTL>;
#[doc = "Writer for register FCTL"]
pub type W = crate::W<u32, super::FCTL>;
#[doc = "Register FCTL `reset()`'s with value 0x2a1c_0e01"]
impl crate::ResetValue for super::FCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2a1c_0e01
    }
}
#[doc = "Reader of field `HBC1F`"]
pub type HBC1F_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HBC1F`"]
pub struct HBC1F_W<'a> {
    w: &'a mut W,
}
impl<'a> HBC1F_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `FLD`"]
pub type FLD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLD`"]
pub struct FLD_W<'a> {
    w: &'a mut W,
}
impl<'a> FLD_W<'a> {
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
    #[doc = "Bits 8:13 - Header bank of CAN1 filter"]
    #[inline(always)]
    pub fn hbc1f(&self) -> HBC1F_R {
        HBC1F_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 0 - Filter lock disable"]
    #[inline(always)]
    pub fn fld(&self) -> FLD_R {
        FLD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:13 - Header bank of CAN1 filter"]
    #[inline(always)]
    pub fn hbc1f(&mut self) -> HBC1F_W {
        HBC1F_W { w: self }
    }
    #[doc = "Bit 0 - Filter lock disable"]
    #[inline(always)]
    pub fn fld(&mut self) -> FLD_W {
        FLD_W { w: self }
    }
}
