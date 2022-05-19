#[doc = "Reader of register HPTFLEN"]
pub type R = crate::R<u32, super::HPTFLEN>;
#[doc = "Writer for register HPTFLEN"]
pub type W = crate::W<u32, super::HPTFLEN>;
#[doc = "Register HPTFLEN `reset()`'s with value 0x0200_0600"]
impl crate::ResetValue for super::HPTFLEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200_0600
    }
}
#[doc = "Reader of field `HPTXFSAR`"]
pub type HPTXFSAR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HPTXFSAR`"]
pub struct HPTXFSAR_W<'a> {
    w: &'a mut W,
}
impl<'a> HPTXFSAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `HPTXFD`"]
pub type HPTXFD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HPTXFD`"]
pub struct HPTXFD_W<'a> {
    w: &'a mut W,
}
impl<'a> HPTXFD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    pub fn hptxfsar(&self) -> HPTXFSAR_R {
        HPTXFSAR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    pub fn hptxfd(&self) -> HPTXFD_R {
        HPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    pub fn hptxfsar(&mut self) -> HPTXFSAR_W {
        HPTXFSAR_W { w: self }
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    pub fn hptxfd(&mut self) -> HPTXFD_W {
        HPTXFD_W { w: self }
    }
}
