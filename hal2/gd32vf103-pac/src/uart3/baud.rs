#[doc = "Reader of register BAUD"]
pub type R = crate::R<u32, super::BAUD>;
#[doc = "Writer for register BAUD"]
pub type W = crate::W<u32, super::BAUD>;
#[doc = "Register BAUD `reset()`'s with value 0"]
impl crate::ResetValue for super::BAUD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTDIV`"]
pub type INTDIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INTDIV`"]
pub struct INTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> INTDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | (((value as u32) & 0x0fff) << 4);
        self.w
    }
}
#[doc = "Reader of field `FRADIV`"]
pub type FRADIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRADIV`"]
pub struct FRADIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FRADIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:15 - Integer part of baud-rate divider"]
    #[inline(always)]
    pub fn intdiv(&self) -> INTDIV_R {
        INTDIV_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - Fraction part of baud-rate divider"]
    #[inline(always)]
    pub fn fradiv(&self) -> FRADIV_R {
        FRADIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:15 - Integer part of baud-rate divider"]
    #[inline(always)]
    pub fn intdiv(&mut self) -> INTDIV_W {
        INTDIV_W { w: self }
    }
    #[doc = "Bits 0:3 - Fraction part of baud-rate divider"]
    #[inline(always)]
    pub fn fradiv(&mut self) -> FRADIV_W {
        FRADIV_W { w: self }
    }
}
