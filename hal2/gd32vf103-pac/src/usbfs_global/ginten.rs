#[doc = "Reader of register GINTEN"]
pub type R = crate::R<u32, super::GINTEN>;
#[doc = "Writer for register GINTEN"]
pub type W = crate::W<u32, super::GINTEN>;
#[doc = "Register GINTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::GINTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MFIE`"]
pub type MFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MFIE`"]
pub struct MFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MFIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `OTGIE`"]
pub type OTGIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTGIE`"]
pub struct OTGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SOFIE`"]
pub type SOFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFIE`"]
pub struct SOFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `RXFNEIE`"]
pub type RXFNEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFNEIE`"]
pub struct RXFNEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFNEIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `NPTXFEIE`"]
pub type NPTXFEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NPTXFEIE`"]
pub struct NPTXFEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> NPTXFEIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `GNPINAKIE`"]
pub type GNPINAKIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GNPINAKIE`"]
pub struct GNPINAKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> GNPINAKIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `GONAKIE`"]
pub type GONAKIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GONAKIE`"]
pub struct GONAKIE_W<'a> {
    w: &'a mut W,
}
impl<'a> GONAKIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `ESPIE`"]
pub type ESPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ESPIE`"]
pub struct ESPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ESPIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SPIE`"]
pub type SPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPIE`"]
pub struct SPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `RSTIE`"]
pub type RSTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTIE`"]
pub struct RSTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `ENUMFIE`"]
pub type ENUMFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENUMFIE`"]
pub struct ENUMFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUMFIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `ISOOPDIE`"]
pub type ISOOPDIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISOOPDIE`"]
pub struct ISOOPDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOOPDIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `EOPFIE`"]
pub type EOPFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOPFIE`"]
pub struct EOPFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOPFIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `IEPIE`"]
pub type IEPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEPIE`"]
pub struct IEPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IEPIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `OEPIE`"]
pub type OEPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OEPIE`"]
pub struct OEPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OEPIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `ISOINCIE`"]
pub type ISOINCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISOINCIE`"]
pub struct ISOINCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOINCIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `PXNCIE_ISOONCIE`"]
pub type PXNCIE_ISOONCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PXNCIE_ISOONCIE`"]
pub struct PXNCIE_ISOONCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PXNCIE_ISOONCIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `HPIE`"]
pub type HPIE_R = crate::R<bool, bool>;
#[doc = "Reader of field `HCIE`"]
pub type HCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HCIE`"]
pub struct HCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HCIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `PTXFEIE`"]
pub type PTXFEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTXFEIE`"]
pub struct PTXFEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFEIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `IDPSCIE`"]
pub type IDPSCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDPSCIE`"]
pub struct IDPSCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDPSCIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `DISCIE`"]
pub type DISCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISCIE`"]
pub struct DISCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `SESIE`"]
pub type SESIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SESIE`"]
pub struct SESIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SESIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `WKUPIE`"]
pub type WKUPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKUPIE`"]
pub struct WKUPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPIE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Mode fault interrupt enable"]
    #[inline(always)]
    pub fn mfie(&self) -> MFIE_R {
        MFIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OTG interrupt enable"]
    #[inline(always)]
    pub fn otgie(&self) -> OTGIE_R {
        OTGIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start of frame interrupt enable"]
    #[inline(always)]
    pub fn sofie(&self) -> SOFIE_R {
        SOFIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO non-empty interrupt enable"]
    #[inline(always)]
    pub fn rxfneie(&self) -> RXFNEIE_R {
        RXFNEIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Non-periodic TxFIFO empty interrupt enable"]
    #[inline(always)]
    pub fn nptxfeie(&self) -> NPTXFEIE_R {
        NPTXFEIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Global non-periodic IN NAK effective interrupt enable"]
    #[inline(always)]
    pub fn gnpinakie(&self) -> GNPINAKIE_R {
        GNPINAKIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK effective interrupt enable"]
    #[inline(always)]
    pub fn gonakie(&self) -> GONAKIE_R {
        GONAKIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Early suspend interrupt enable"]
    #[inline(always)]
    pub fn espie(&self) -> ESPIE_R {
        ESPIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USB suspend interrupt enable"]
    #[inline(always)]
    pub fn spie(&self) -> SPIE_R {
        SPIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - USB reset interrupt enable"]
    #[inline(always)]
    pub fn rstie(&self) -> RSTIE_R {
        RSTIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enumeration finish interrupt enable"]
    #[inline(always)]
    pub fn enumfie(&self) -> ENUMFIE_R {
        ENUMFIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt enable"]
    #[inline(always)]
    pub fn isoopdie(&self) -> ISOOPDIE_R {
        ISOOPDIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt enable"]
    #[inline(always)]
    pub fn eopfie(&self) -> EOPFIE_R {
        EOPFIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IN endpoints interrupt enable"]
    #[inline(always)]
    pub fn iepie(&self) -> IEPIE_R {
        IEPIE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OUT endpoints interrupt enable"]
    #[inline(always)]
    pub fn oepie(&self) -> OEPIE_R {
        OEPIE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - isochronous IN transfer not complete interrupt enable"]
    #[inline(always)]
    pub fn isoincie(&self) -> ISOINCIE_R {
        ISOINCIE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - periodic transfer not compelete Interrupt enable(Host mode)/isochronous OUT transfer not complete interrupt enable(Device mode)"]
    #[inline(always)]
    pub fn pxncie_isooncie(&self) -> PXNCIE_ISOONCIE_R {
        PXNCIE_ISOONCIE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Host port interrupt enable"]
    #[inline(always)]
    pub fn hpie(&self) -> HPIE_R {
        HPIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Host channels interrupt enable"]
    #[inline(always)]
    pub fn hcie(&self) -> HCIE_R {
        HCIE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty interrupt enable"]
    #[inline(always)]
    pub fn ptxfeie(&self) -> PTXFEIE_R {
        PTXFEIE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - ID pin status change interrupt enable"]
    #[inline(always)]
    pub fn idpscie(&self) -> IDPSCIE_R {
        IDPSCIE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Disconnect interrupt enable"]
    #[inline(always)]
    pub fn discie(&self) -> DISCIE_R {
        DISCIE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Session interrupt enable"]
    #[inline(always)]
    pub fn sesie(&self) -> SESIE_R {
        SESIE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Wakeup interrupt enable"]
    #[inline(always)]
    pub fn wkupie(&self) -> WKUPIE_R {
        WKUPIE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode fault interrupt enable"]
    #[inline(always)]
    pub fn mfie(&mut self) -> MFIE_W {
        MFIE_W { w: self }
    }
    #[doc = "Bit 2 - OTG interrupt enable"]
    #[inline(always)]
    pub fn otgie(&mut self) -> OTGIE_W {
        OTGIE_W { w: self }
    }
    #[doc = "Bit 3 - Start of frame interrupt enable"]
    #[inline(always)]
    pub fn sofie(&mut self) -> SOFIE_W {
        SOFIE_W { w: self }
    }
    #[doc = "Bit 4 - Receive FIFO non-empty interrupt enable"]
    #[inline(always)]
    pub fn rxfneie(&mut self) -> RXFNEIE_W {
        RXFNEIE_W { w: self }
    }
    #[doc = "Bit 5 - Non-periodic TxFIFO empty interrupt enable"]
    #[inline(always)]
    pub fn nptxfeie(&mut self) -> NPTXFEIE_W {
        NPTXFEIE_W { w: self }
    }
    #[doc = "Bit 6 - Global non-periodic IN NAK effective interrupt enable"]
    #[inline(always)]
    pub fn gnpinakie(&mut self) -> GNPINAKIE_W {
        GNPINAKIE_W { w: self }
    }
    #[doc = "Bit 7 - Global OUT NAK effective interrupt enable"]
    #[inline(always)]
    pub fn gonakie(&mut self) -> GONAKIE_W {
        GONAKIE_W { w: self }
    }
    #[doc = "Bit 10 - Early suspend interrupt enable"]
    #[inline(always)]
    pub fn espie(&mut self) -> ESPIE_W {
        ESPIE_W { w: self }
    }
    #[doc = "Bit 11 - USB suspend interrupt enable"]
    #[inline(always)]
    pub fn spie(&mut self) -> SPIE_W {
        SPIE_W { w: self }
    }
    #[doc = "Bit 12 - USB reset interrupt enable"]
    #[inline(always)]
    pub fn rstie(&mut self) -> RSTIE_W {
        RSTIE_W { w: self }
    }
    #[doc = "Bit 13 - Enumeration finish interrupt enable"]
    #[inline(always)]
    pub fn enumfie(&mut self) -> ENUMFIE_W {
        ENUMFIE_W { w: self }
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt enable"]
    #[inline(always)]
    pub fn isoopdie(&mut self) -> ISOOPDIE_W {
        ISOOPDIE_W { w: self }
    }
    #[doc = "Bit 15 - End of periodic frame interrupt enable"]
    #[inline(always)]
    pub fn eopfie(&mut self) -> EOPFIE_W {
        EOPFIE_W { w: self }
    }
    #[doc = "Bit 18 - IN endpoints interrupt enable"]
    #[inline(always)]
    pub fn iepie(&mut self) -> IEPIE_W {
        IEPIE_W { w: self }
    }
    #[doc = "Bit 19 - OUT endpoints interrupt enable"]
    #[inline(always)]
    pub fn oepie(&mut self) -> OEPIE_W {
        OEPIE_W { w: self }
    }
    #[doc = "Bit 20 - isochronous IN transfer not complete interrupt enable"]
    #[inline(always)]
    pub fn isoincie(&mut self) -> ISOINCIE_W {
        ISOINCIE_W { w: self }
    }
    #[doc = "Bit 21 - periodic transfer not compelete Interrupt enable(Host mode)/isochronous OUT transfer not complete interrupt enable(Device mode)"]
    #[inline(always)]
    pub fn pxncie_isooncie(&mut self) -> PXNCIE_ISOONCIE_W {
        PXNCIE_ISOONCIE_W { w: self }
    }
    #[doc = "Bit 25 - Host channels interrupt enable"]
    #[inline(always)]
    pub fn hcie(&mut self) -> HCIE_W {
        HCIE_W { w: self }
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty interrupt enable"]
    #[inline(always)]
    pub fn ptxfeie(&mut self) -> PTXFEIE_W {
        PTXFEIE_W { w: self }
    }
    #[doc = "Bit 28 - ID pin status change interrupt enable"]
    #[inline(always)]
    pub fn idpscie(&mut self) -> IDPSCIE_W {
        IDPSCIE_W { w: self }
    }
    #[doc = "Bit 29 - Disconnect interrupt enable"]
    #[inline(always)]
    pub fn discie(&mut self) -> DISCIE_W {
        DISCIE_W { w: self }
    }
    #[doc = "Bit 30 - Session interrupt enable"]
    #[inline(always)]
    pub fn sesie(&mut self) -> SESIE_W {
        SESIE_W { w: self }
    }
    #[doc = "Bit 31 - Wakeup interrupt enable"]
    #[inline(always)]
    pub fn wkupie(&mut self) -> WKUPIE_W {
        WKUPIE_W { w: self }
    }
}
