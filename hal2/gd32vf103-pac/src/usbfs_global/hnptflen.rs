#[doc = "Reader of register HNPTFLEN"]
pub type R = crate::R<u32, super::HNPTFLEN>;
#[doc = "Writer for register HNPTFLEN"]
pub type W = crate::W<u32, super::HNPTFLEN>;
#[doc = "Register HNPTFLEN `reset()`'s with value 0x0200_0200"]
impl crate::ResetValue for super::HNPTFLEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200_0200
    }
}
#[doc = "Reader of field `HNPTXRSAR`"]
pub type HNPTXRSAR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HNPTXRSAR`"]
pub struct HNPTXRSAR_W<'a> {
    w: &'a mut W,
}
impl<'a> HNPTXRSAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `HNPTXFD`"]
pub type HNPTXFD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HNPTXFD`"]
pub struct HNPTXFD_W<'a> {
    w: &'a mut W,
}
impl<'a> HNPTXFD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - host non-periodic transmit Tx RAM start address"]
    #[inline(always)]
    pub fn hnptxrsar(&self) -> HNPTXRSAR_R {
        HNPTXRSAR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - host non-periodic TxFIFO depth"]
    #[inline(always)]
    pub fn hnptxfd(&self) -> HNPTXFD_R {
        HNPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - host non-periodic transmit Tx RAM start address"]
    #[inline(always)]
    pub fn hnptxrsar(&mut self) -> HNPTXRSAR_W {
        HNPTXRSAR_W { w: self }
    }
    #[doc = "Bits 16:31 - host non-periodic TxFIFO depth"]
    #[inline(always)]
    pub fn hnptxfd(&mut self) -> HNPTXFD_W {
        HNPTXFD_W { w: self }
    }
}
