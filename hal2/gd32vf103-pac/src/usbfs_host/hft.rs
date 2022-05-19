#[doc = "Reader of register HFT"]
pub type R = crate::R<u32, super::HFT>;
#[doc = "Writer for register HFT"]
pub type W = crate::W<u32, super::HFT>;
#[doc = "Register HFT `reset()`'s with value 0xbb80"]
impl crate::ResetValue for super::HFT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xbb80
    }
}
#[doc = "Reader of field `FRI`"]
pub type FRI_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FRI`"]
pub struct FRI_W<'a> {
    w: &'a mut W,
}
impl<'a> FRI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Frame interval"]
    #[inline(always)]
    pub fn fri(&self) -> FRI_R {
        FRI_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame interval"]
    #[inline(always)]
    pub fn fri(&mut self) -> FRI_W {
        FRI_W { w: self }
    }
}
