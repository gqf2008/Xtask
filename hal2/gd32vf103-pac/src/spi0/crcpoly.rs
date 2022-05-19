#[doc = "Reader of register CRCPOLY"]
pub type R = crate::R<u16, super::CRCPOLY>;
#[doc = "Writer for register CRCPOLY"]
pub type W = crate::W<u16, super::CRCPOLY>;
#[doc = "Register CRCPOLY `reset()`'s with value 0x07"]
impl crate::ResetValue for super::CRCPOLY {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "Reader of field `CRCPOLY`"]
pub type CRCPOLY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CRCPOLY`"]
pub struct CRCPOLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCPOLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CRC polynomial value"]
    #[inline(always)]
    pub fn crcpoly(&self) -> CRCPOLY_R {
        CRCPOLY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC polynomial value"]
    #[inline(always)]
    pub fn crcpoly(&mut self) -> CRCPOLY_W {
        CRCPOLY_W { w: self }
    }
}
