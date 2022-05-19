#[doc = "Reader of register CLICINTCTL"]
pub type R = crate::R<u8, super::CLICINTCTL>;
#[doc = "Writer for register CLICINTCTL"]
pub type W = crate::W<u8, super::CLICINTCTL>;
#[doc = "Register CLICINTCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CLICINTCTL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEVEL_PRIORITY`"]
pub type LEVEL_PRIORITY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LEVEL_PRIORITY`"]
pub struct LEVEL_PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> LEVEL_PRIORITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - LEVEL_PRIORITY"]
    #[inline(always)]
    pub fn level_priority(&self) -> LEVEL_PRIORITY_R {
        LEVEL_PRIORITY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LEVEL_PRIORITY"]
    #[inline(always)]
    pub fn level_priority(&mut self) -> LEVEL_PRIORITY_W {
        LEVEL_PRIORITY_W { w: self }
    }
}
