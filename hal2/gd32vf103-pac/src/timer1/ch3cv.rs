#[doc = "Reader of register CH3CV"]
pub type R = crate::R<u16, super::CH3CV>;
#[doc = "Writer for register CH3CV"]
pub type W = crate::W<u16, super::CH3CV>;
#[doc = "Register CH3CV `reset()`'s with value 0"]
impl crate::ResetValue for super::CH3CV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH3VAL`"]
pub type CH3VAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CH3VAL`"]
pub struct CH3VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Capture or compare value of channel 3"]
    #[inline(always)]
    pub fn ch3val(&self) -> CH3VAL_R {
        CH3VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture or compare value of channel 3"]
    #[inline(always)]
    pub fn ch3val(&mut self) -> CH3VAL_W {
        CH3VAL_W { w: self }
    }
}
