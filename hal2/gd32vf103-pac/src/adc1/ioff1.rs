#[doc = "Reader of register IOFF1"]
pub type R = crate::R<u32, super::IOFF1>;
#[doc = "Writer for register IOFF1"]
pub type W = crate::W<u32, super::IOFF1>;
#[doc = "Register IOFF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::IOFF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IOFF`"]
pub type IOFF_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IOFF`"]
pub struct IOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> IOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Data offset for inserted channel 1"]
    #[inline(always)]
    pub fn ioff(&self) -> IOFF_R {
        IOFF_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for inserted channel 1"]
    #[inline(always)]
    pub fn ioff(&mut self) -> IOFF_W {
        IOFF_W { w: self }
    }
}
