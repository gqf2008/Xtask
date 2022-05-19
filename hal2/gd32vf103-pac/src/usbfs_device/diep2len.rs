#[doc = "Reader of register DIEP2LEN"]
pub type R = crate::R<u32, super::DIEP2LEN>;
#[doc = "Writer for register DIEP2LEN"]
pub type W = crate::W<u32, super::DIEP2LEN>;
#[doc = "Register DIEP2LEN `reset()`'s with value 0"]
impl crate::ResetValue for super::DIEP2LEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCPF`"]
pub type MCPF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCPF`"]
pub struct MCPF_W<'a> {
    w: &'a mut W,
}
impl<'a> MCPF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
#[doc = "Reader of field `PCNT`"]
pub type PCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PCNT`"]
pub struct PCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 19)) | (((value as u32) & 0x03ff) << 19);
        self.w
    }
}
#[doc = "Reader of field `TLEN`"]
pub type TLEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TLEN`"]
pub struct TLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0007_ffff) | ((value as u32) & 0x0007_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 29:30 - Multi packet count per frame"]
    #[inline(always)]
    pub fn mcpf(&self) -> MCPF_R {
        MCPF_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pcnt(&self) -> PCNT_R {
        PCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:18 - Transfer length"]
    #[inline(always)]
    pub fn tlen(&self) -> TLEN_R {
        TLEN_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 29:30 - Multi packet count per frame"]
    #[inline(always)]
    pub fn mcpf(&mut self) -> MCPF_W {
        MCPF_W { w: self }
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pcnt(&mut self) -> PCNT_W {
        PCNT_W { w: self }
    }
    #[doc = "Bits 0:18 - Transfer length"]
    #[inline(always)]
    pub fn tlen(&mut self) -> TLEN_W {
        TLEN_W { w: self }
    }
}
