#[doc = "Reader of register CREP"]
pub type R = crate::R<u16, super::CREP>;
#[doc = "Writer for register CREP"]
pub type W = crate::W<u16, super::CREP>;
#[doc = "Register CREP `reset()`'s with value 0"]
impl crate::ResetValue for super::CREP {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CREP`"]
pub type CREP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CREP`"]
pub struct CREP_W<'a> {
    w: &'a mut W,
}
impl<'a> CREP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u16) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Counter repetition value"]
    #[inline(always)]
    pub fn crep(&self) -> CREP_R {
        CREP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter repetition value"]
    #[inline(always)]
    pub fn crep(&mut self) -> CREP_W {
        CREP_W { w: self }
    }
}
