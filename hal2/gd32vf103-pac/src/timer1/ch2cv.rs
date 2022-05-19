#[doc = "Reader of register CH2CV"]
pub type R = crate::R<u16, super::CH2CV>;
#[doc = "Writer for register CH2CV"]
pub type W = crate::W<u16, super::CH2CV>;
#[doc = "Register CH2CV `reset()`'s with value 0"]
impl crate::ResetValue for super::CH2CV {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH2VAL`"]
pub type CH2VAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CH2VAL`"]
pub struct CH2VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Capture or compare value of channel 2"]
    #[inline(always)]
    pub fn ch2val(&self) -> CH2VAL_R {
        CH2VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture or compare value of channel 2"]
    #[inline(always)]
    pub fn ch2val(&mut self) -> CH2VAL_W {
        CH2VAL_W { w: self }
    }
}
