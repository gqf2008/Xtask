#[doc = "Reader of register WDLT"]
pub type R = crate::R<u32, super::WDLT>;
#[doc = "Writer for register WDLT"]
pub type W = crate::W<u32, super::WDLT>;
#[doc = "Register WDLT `reset()`'s with value 0"]
impl crate::ResetValue for super::WDLT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDLT`"]
pub type WDLT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WDLT`"]
pub struct WDLT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Analog watchdog lower threshold"]
    #[inline(always)]
    pub fn wdlt(&self) -> WDLT_R {
        WDLT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog lower threshold"]
    #[inline(always)]
    pub fn wdlt(&mut self) -> WDLT_W {
        WDLT_W { w: self }
    }
}
