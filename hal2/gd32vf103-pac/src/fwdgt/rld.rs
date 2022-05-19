#[doc = "Reader of register RLD"]
pub type R = crate::R<u32, super::RLD>;
#[doc = "Writer for register RLD"]
pub type W = crate::W<u32, super::RLD>;
#[doc = "Register RLD `reset()`'s with value 0x0fff"]
impl crate::ResetValue for super::RLD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fff
    }
}
#[doc = "Reader of field `RLD`"]
pub type RLD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RLD`"]
pub struct RLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Free watchdog timer counter reload value"]
    #[inline(always)]
    pub fn rld(&self) -> RLD_R {
        RLD_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Free watchdog timer counter reload value"]
    #[inline(always)]
    pub fn rld(&mut self) -> RLD_W {
        RLD_W { w: self }
    }
}
