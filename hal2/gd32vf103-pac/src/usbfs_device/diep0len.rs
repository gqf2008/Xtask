#[doc = "Reader of register DIEP0LEN"]
pub type R = crate::R<u32, super::DIEP0LEN>;
#[doc = "Writer for register DIEP0LEN"]
pub type W = crate::W<u32, super::DIEP0LEN>;
#[doc = "Register DIEP0LEN `reset()`'s with value 0"]
impl crate::ResetValue for super::DIEP0LEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCNT`"]
pub type PCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCNT`"]
pub struct PCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "Reader of field `TLEN`"]
pub type TLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TLEN`"]
pub struct TLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 19:20 - Packet count"]
    #[inline(always)]
    pub fn pcnt(&self) -> PCNT_R {
        PCNT_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bits 0:6 - Transfer length"]
    #[inline(always)]
    pub fn tlen(&self) -> TLEN_R {
        TLEN_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 19:20 - Packet count"]
    #[inline(always)]
    pub fn pcnt(&mut self) -> PCNT_W {
        PCNT_W { w: self }
    }
    #[doc = "Bits 0:6 - Transfer length"]
    #[inline(always)]
    pub fn tlen(&mut self) -> TLEN_W {
        TLEN_W { w: self }
    }
}
