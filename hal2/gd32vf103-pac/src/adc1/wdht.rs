#[doc = "Reader of register WDHT"]
pub type R = crate::R<u32, super::WDHT>;
#[doc = "Writer for register WDHT"]
pub type W = crate::W<u32, super::WDHT>;
#[doc = "Register WDHT `reset()`'s with value 0x0fff"]
impl crate::ResetValue for super::WDHT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fff
    }
}
#[doc = "Reader of field `WDHT`"]
pub type WDHT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WDHT`"]
pub struct WDHT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDHT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Analog watchdog higher threshold"]
    #[inline(always)]
    pub fn wdht(&self) -> WDHT_R {
        WDHT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog higher threshold"]
    #[inline(always)]
    pub fn wdht(&mut self) -> WDHT_W {
        WDHT_W { w: self }
    }
}
