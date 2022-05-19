#[doc = "Reader of register DIEP3TFLEN"]
pub type R = crate::R<u32, super::DIEP3TFLEN>;
#[doc = "Writer for register DIEP3TFLEN"]
pub type W = crate::W<u32, super::DIEP3TFLEN>;
#[doc = "Register DIEP3TFLEN `reset()`'s with value 0x0200_0400"]
impl crate::ResetValue for super::DIEP3TFLEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200_0400
    }
}
#[doc = "Reader of field `IEPTXRSAR`"]
pub type IEPTXRSAR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IEPTXRSAR`"]
pub struct IEPTXRSAR_W<'a> {
    w: &'a mut W,
}
impl<'a> IEPTXRSAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `IEPTXFD`"]
pub type IEPTXFD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IEPTXFD`"]
pub struct IEPTXFD_W<'a> {
    w: &'a mut W,
}
impl<'a> IEPTXFD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - IN endpoint FIFO4 transmit RAM start address"]
    #[inline(always)]
    pub fn ieptxrsar(&self) -> IEPTXRSAR_R {
        IEPTXRSAR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ieptxfd(&self) -> IEPTXFD_R {
        IEPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN endpoint FIFO4 transmit RAM start address"]
    #[inline(always)]
    pub fn ieptxrsar(&mut self) -> IEPTXRSAR_W {
        IEPTXRSAR_W { w: self }
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ieptxfd(&mut self) -> IEPTXFD_W {
        IEPTXFD_W { w: self }
    }
}
