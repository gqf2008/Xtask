#[doc = "Reader of register DATA"]
pub type R = crate::R<u16, super::DATA>;
#[doc = "Writer for register DATA"]
pub type W = crate::W<u16, super::DATA>;
#[doc = "Register DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRB`"]
pub type TRB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRB`"]
pub struct TRB_W<'a> {
    w: &'a mut W,
}
impl<'a> TRB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Transmission or reception data buffer register"]
    #[inline(always)]
    pub fn trb(&self) -> TRB_R {
        TRB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmission or reception data buffer register"]
    #[inline(always)]
    pub fn trb(&mut self) -> TRB_W {
        TRB_W { w: self }
    }
}
