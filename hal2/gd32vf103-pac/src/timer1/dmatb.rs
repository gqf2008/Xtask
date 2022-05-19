#[doc = "Reader of register DMATB"]
pub type R = crate::R<u16, super::DMATB>;
#[doc = "Writer for register DMATB"]
pub type W = crate::W<u16, super::DMATB>;
#[doc = "Register DMATB `reset()`'s with value 0"]
impl crate::ResetValue for super::DMATB {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMATB`"]
pub type DMATB_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DMATB`"]
pub struct DMATB_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - DMA transfer buffer"]
    #[inline(always)]
    pub fn dmatb(&self) -> DMATB_R {
        DMATB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DMA transfer buffer"]
    #[inline(always)]
    pub fn dmatb(&mut self) -> DMATB_W {
        DMATB_W { w: self }
    }
}
