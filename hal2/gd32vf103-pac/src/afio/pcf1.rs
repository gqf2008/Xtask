#[doc = "Reader of register PCF1"]
pub type R = crate::R<u32, super::PCF1>;
#[doc = "Writer for register PCF1"]
pub type W = crate::W<u32, super::PCF1>;
#[doc = "Register PCF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PCF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXMC_NADV`"]
pub type EXMC_NADV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXMC_NADV`"]
pub struct EXMC_NADV_W<'a> {
    w: &'a mut W,
}
impl<'a> EXMC_NADV_W<'a> {
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
impl R {
    #[doc = "Bit 10 - EXMC_NADV connect/disconnect"]
    #[inline(always)]
    pub fn exmc_nadv(&self) -> EXMC_NADV_R {
        EXMC_NADV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - EXMC_NADV connect/disconnect"]
    #[inline(always)]
    pub fn exmc_nadv(&mut self) -> EXMC_NADV_W {
        EXMC_NADV_W { w: self }
    }
}
