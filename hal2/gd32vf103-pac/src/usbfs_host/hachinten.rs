#[doc = "Reader of register HACHINTEN"]
pub type R = crate::R<u32, super::HACHINTEN>;
#[doc = "Writer for register HACHINTEN"]
pub type W = crate::W<u32, super::HACHINTEN>;
#[doc = "Register HACHINTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::HACHINTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CINTEN`"]
pub type CINTEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CINTEN`"]
pub struct CINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CINTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Channel interrupt enable"]
    #[inline(always)]
    pub fn cinten(&self) -> CINTEN_R {
        CINTEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel interrupt enable"]
    #[inline(always)]
    pub fn cinten(&mut self) -> CINTEN_W {
        CINTEN_W { w: self }
    }
}
