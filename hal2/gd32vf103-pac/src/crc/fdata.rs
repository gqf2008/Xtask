#[doc = "Reader of register FDATA"]
pub type R = crate::R<u32, super::FDATA>;
#[doc = "Writer for register FDATA"]
pub type W = crate::W<u32, super::FDATA>;
#[doc = "Register FDATA `reset()`'s with value 0"]
impl crate::ResetValue for super::FDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FDATA`"]
pub type FDATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FDATA`"]
pub struct FDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> FDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Free Data Register bits"]
    #[inline(always)]
    pub fn fdata(&self) -> FDATA_R {
        FDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Free Data Register bits"]
    #[inline(always)]
    pub fn fdata(&mut self) -> FDATA_W {
        FDATA_W { w: self }
    }
}
