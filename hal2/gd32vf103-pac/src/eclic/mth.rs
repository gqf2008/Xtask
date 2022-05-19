#[doc = "Reader of register MTH"]
pub type R = crate::R<u8, super::MTH>;
#[doc = "Writer for register MTH"]
pub type W = crate::W<u8, super::MTH>;
#[doc = "Register MTH `reset()`'s with value 0"]
impl crate::ResetValue for super::MTH {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MTH`"]
pub type MTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MTH`"]
pub struct MTH_W<'a> {
    w: &'a mut W,
}
impl<'a> MTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - MTH"]
    #[inline(always)]
    pub fn mth(&self) -> MTH_R {
        MTH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - MTH"]
    #[inline(always)]
    pub fn mth(&mut self) -> MTH_W {
        MTH_W { w: self }
    }
}
