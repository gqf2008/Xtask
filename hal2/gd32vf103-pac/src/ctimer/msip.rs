#[doc = "Reader of register msip"]
pub type R = crate::R<u32, super::MSIP>;
#[doc = "Writer for register msip"]
pub type W = crate::W<u32, super::MSIP>;
#[doc = "Register msip `reset()`'s with value 0"]
impl crate::ResetValue for super::MSIP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MSIP`"]
pub type MSIP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSIP`"]
pub struct MSIP_W<'a> {
    w: &'a mut W,
}
impl<'a> MSIP_W<'a> {
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
    #[doc = "Bit 0 - Generate software interrupts"]
    #[inline(always)]
    pub fn msip(&self) -> MSIP_R {
        MSIP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Generate software interrupts"]
    #[inline(always)]
    pub fn msip(&mut self) -> MSIP_W {
        MSIP_W { w: self }
    }
}
