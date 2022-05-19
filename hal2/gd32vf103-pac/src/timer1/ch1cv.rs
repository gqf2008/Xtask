#[doc = "Reader of register CH1CV"]
pub type R = crate::R<u16, super::CH1CV>;
#[doc = "Writer for register CH1CV"]
pub type W = crate::W<u16, super::CH1CV>;
#[doc = "Register CH1CV `reset()`'s with value 0"]
impl crate::ResetValue for super::CH1CV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH1VAL`"]
pub type CH1VAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CH1VAL`"]
pub struct CH1VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Capture or compare value of channel1"]
    #[inline(always)]
    pub fn ch1val(&self) -> CH1VAL_R {
        CH1VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture or compare value of channel1"]
    #[inline(always)]
    pub fn ch1val(&mut self) -> CH1VAL_W {
        CH1VAL_W { w: self }
    }
}
