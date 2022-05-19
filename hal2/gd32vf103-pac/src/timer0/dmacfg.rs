#[doc = "Reader of register DMACFG"]
pub type R = crate::R<u16, super::DMACFG>;
#[doc = "Writer for register DMACFG"]
pub type W = crate::W<u16, super::DMACFG>;
#[doc = "Register DMACFG `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACFG {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMATC`"]
pub type DMATC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMATC`"]
pub struct DMATC_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u16) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `DMATA`"]
pub type DMATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMATA`"]
pub struct DMATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u16) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:12 - DMA transfer count"]
    #[inline(always)]
    pub fn dmatc(&self) -> DMATC_R {
        DMATC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - DMA transfer access start address"]
    #[inline(always)]
    pub fn dmata(&self) -> DMATA_R {
        DMATA_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - DMA transfer count"]
    #[inline(always)]
    pub fn dmatc(&mut self) -> DMATC_W {
        DMATC_W { w: self }
    }
    #[doc = "Bits 0:4 - DMA transfer access start address"]
    #[inline(always)]
    pub fn dmata(&mut self) -> DMATA_W {
        DMATA_W { w: self }
    }
}
