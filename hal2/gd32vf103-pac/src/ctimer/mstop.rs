#[doc = "Reader of register mstop"]
pub type R = crate::R<u32, super::MSTOP>;
#[doc = "Writer for register mstop"]
pub type W = crate::W<u32, super::MSTOP>;
#[doc = "Register mstop `reset()`'s with value 0"]
impl crate::ResetValue for super::MSTOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TIMESTOP`"]
pub type TIMESTOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMESTOP`"]
pub struct TIMESTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMESTOP_W<'a> {
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
    #[doc = "Bit 0 - Pause (1) or run (0) the timer"]
    #[inline(always)]
    pub fn timestop(&self) -> TIMESTOP_R {
        TIMESTOP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pause (1) or run (0) the timer"]
    #[inline(always)]
    pub fn timestop(&mut self) -> TIMESTOP_W {
        TIMESTOP_W { w: self }
    }
}
