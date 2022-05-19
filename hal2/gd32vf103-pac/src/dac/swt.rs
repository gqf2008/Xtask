#[doc = "Writer for register SWT"]
pub type W = crate::W<u32, super::SWT>;
#[doc = "Register SWT `reset()`'s with value 0"]
impl crate::ResetValue for super::SWT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SWTR0`"]
pub struct SWTR0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTR0_W<'a> {
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
#[doc = "Write proxy for field `SWTR1`"]
pub struct SWTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTR1_W<'a> {
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
impl W {
    #[doc = "Bit 0 - DAC0 software trigger"]
    #[inline(always)]
    pub fn swtr0(&mut self) -> SWTR0_W {
        SWTR0_W { w: self }
    }
    #[doc = "Bit 1 - DAC1 software trigger"]
    #[inline(always)]
    pub fn swtr1(&mut self) -> SWTR1_W {
        SWTR1_W { w: self }
    }
}
