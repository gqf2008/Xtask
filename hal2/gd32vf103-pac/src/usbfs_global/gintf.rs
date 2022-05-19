#[doc = "Reader of register GINTF"]
pub type R = crate::R<u32, super::GINTF>;
#[doc = "Writer for register GINTF"]
pub type W = crate::W<u32, super::GINTF>;
#[doc = "Register GINTF `reset()`'s with value 0x0400_0021"]
impl crate::ResetValue for super::GINTF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0400_0021
    }
}
#[doc = "Reader of field `COPM`"]
pub type COPM_R = crate::R<bool, bool>;
#[doc = "Reader of field `MFIF`"]
pub type MFIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MFIF`"]
pub struct MFIF_W<'a> {
    w: &'a mut W,
}
impl<'a> MFIF_W<'a> {
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
#[doc = "Reader of field `OTGIF`"]
pub type OTGIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SOF`"]
pub type SOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOF`"]
pub struct SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_W<'a> {
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
#[doc = "Reader of field `RXFNEIF`"]
pub type RXFNEIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `NPTXFEIF`"]
pub type NPTXFEIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `GNPINAK`"]
pub type GNPINAK_R = crate::R<bool, bool>;
#[doc = "Reader of field `GONAK`"]
pub type GONAK_R = crate::R<bool, bool>;
#[doc = "Reader of field `ESP`"]
pub type ESP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ESP`"]
pub struct ESP_W<'a> {
    w: &'a mut W,
}
impl<'a> ESP_W<'a> {
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
#[doc = "Reader of field `SP`"]
pub type SP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SP`"]
pub struct SP_W<'a> {
    w: &'a mut W,
}
impl<'a> SP_W<'a> {
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
#[doc = "Reader of field `RST`"]
pub type RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST`"]
pub struct RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_W<'a> {
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
#[doc = "Reader of field `ENUMF`"]
pub type ENUMF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENUMF`"]
pub struct ENUMF_W<'a> {
    w: &'a mut W,
}
impl<'a> ENUMF_W<'a> {
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
#[doc = "Reader of field `ISOOPDIF`"]
pub type ISOOPDIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISOOPDIF`"]
pub struct ISOOPDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOOPDIF_W<'a> {
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
#[doc = "Reader of field `EOPFIF`"]
pub type EOPFIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOPFIF`"]
pub struct EOPFIF_W<'a> {
    w: &'a mut W,
}
impl<'a> EOPFIF_W<'a> {
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
#[doc = "Reader of field `IEPIF`"]
pub type IEPIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `OEPIF`"]
pub type OEPIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISOINCIF`"]
pub type ISOINCIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISOINCIF`"]
pub struct ISOINCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOINCIF_W<'a> {
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
#[doc = "Reader of field `PXNCIF_ISOONCIF`"]
pub type PXNCIF_ISOONCIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PXNCIF_ISOONCIF`"]
pub struct PXNCIF_ISOONCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> PXNCIF_ISOONCIF_W<'a> {
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
#[doc = "Reader of field `HPIF`"]
pub type HPIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `HCIF`"]
pub type HCIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PTXFEIF`"]
pub type PTXFEIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `IDPSC`"]
pub type IDPSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDPSC`"]
pub struct IDPSC_W<'a> {
    w: &'a mut W,
}
impl<'a> IDPSC_W<'a> {
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
#[doc = "Reader of field `DISCIF`"]
pub type DISCIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISCIF`"]
pub struct DISCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCIF_W<'a> {
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
#[doc = "Reader of field `SESIF`"]
pub type SESIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SESIF`"]
pub struct SESIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SESIF_W<'a> {
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
#[doc = "Reader of field `WKUPIF`"]
pub type WKUPIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WKUPIF`"]
pub struct WKUPIF_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPIF_W<'a> {
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
    #[doc = "Bit 0 - Current operation mode"]
    #[inline(always)]
    pub fn copm(&self) -> COPM_R {
        COPM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mode fault interrupt flag"]
    #[inline(always)]
    pub fn mfif(&self) -> MFIF_R {
        MFIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OTG interrupt flag"]
    #[inline(always)]
    pub fn otgif(&self) -> OTGIF_R {
        OTGIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start of frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RxFIFO non-empty interrupt flag"]
    #[inline(always)]
    pub fn rxfneif(&self) -> RXFNEIF_R {
        RXFNEIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Non-periodic TxFIFO empty interrupt flag"]
    #[inline(always)]
    pub fn nptxfeif(&self) -> NPTXFEIF_R {
        NPTXFEIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Global Non-Periodic IN NAK effective"]
    #[inline(always)]
    pub fn gnpinak(&self) -> GNPINAK_R {
        GNPINAK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK effective"]
    #[inline(always)]
    pub fn gonak(&self) -> GONAK_R {
        GONAK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Early suspend"]
    #[inline(always)]
    pub fn esp(&self) -> ESP_R {
        ESP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USB suspend"]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - USB reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enumeration finished"]
    #[inline(always)]
    pub fn enumf(&self) -> ENUMF_R {
        ENUMF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt"]
    #[inline(always)]
    pub fn isoopdif(&self) -> ISOOPDIF_R {
        ISOOPDIF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt flag"]
    #[inline(always)]
    pub fn eopfif(&self) -> EOPFIF_R {
        EOPFIF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IN endpoint interrupt flag"]
    #[inline(always)]
    pub fn iepif(&self) -> IEPIF_R {
        IEPIF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OUT endpoint interrupt flag"]
    #[inline(always)]
    pub fn oepif(&self) -> OEPIF_R {
        OEPIF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Isochronous IN transfer Not Complete Interrupt Flag"]
    #[inline(always)]
    pub fn isoincif(&self) -> ISOINCIF_R {
        ISOINCIF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - periodic transfer not complete interrupt flag(Host mode)/isochronous OUT transfer not complete interrupt flag(Device mode)"]
    #[inline(always)]
    pub fn pxncif_isooncif(&self) -> PXNCIF_ISOONCIF_R {
        PXNCIF_ISOONCIF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Host port interrupt flag"]
    #[inline(always)]
    pub fn hpif(&self) -> HPIF_R {
        HPIF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Host channels interrupt flag"]
    #[inline(always)]
    pub fn hcif(&self) -> HCIF_R {
        HCIF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty interrupt flag"]
    #[inline(always)]
    pub fn ptxfeif(&self) -> PTXFEIF_R {
        PTXFEIF_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - ID pin status change"]
    #[inline(always)]
    pub fn idpsc(&self) -> IDPSC_R {
        IDPSC_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Disconnect interrupt flag"]
    #[inline(always)]
    pub fn discif(&self) -> DISCIF_R {
        DISCIF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Session interrupt flag"]
    #[inline(always)]
    pub fn sesif(&self) -> SESIF_R {
        SESIF_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Wakeup interrupt flag"]
    #[inline(always)]
    pub fn wkupif(&self) -> WKUPIF_R {
        WKUPIF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode fault interrupt flag"]
    #[inline(always)]
    pub fn mfif(&mut self) -> MFIF_W {
        MFIF_W { w: self }
    }
    #[doc = "Bit 3 - Start of frame"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W {
        SOF_W { w: self }
    }
    #[doc = "Bit 10 - Early suspend"]
    #[inline(always)]
    pub fn esp(&mut self) -> ESP_W {
        ESP_W { w: self }
    }
    #[doc = "Bit 11 - USB suspend"]
    #[inline(always)]
    pub fn sp(&mut self) -> SP_W {
        SP_W { w: self }
    }
    #[doc = "Bit 12 - USB reset"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W {
        RST_W { w: self }
    }
    #[doc = "Bit 13 - Enumeration finished"]
    #[inline(always)]
    pub fn enumf(&mut self) -> ENUMF_W {
        ENUMF_W { w: self }
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt"]
    #[inline(always)]
    pub fn isoopdif(&mut self) -> ISOOPDIF_W {
        ISOOPDIF_W { w: self }
    }
    #[doc = "Bit 15 - End of periodic frame interrupt flag"]
    #[inline(always)]
    pub fn eopfif(&mut self) -> EOPFIF_W {
        EOPFIF_W { w: self }
    }
    #[doc = "Bit 20 - Isochronous IN transfer Not Complete Interrupt Flag"]
    #[inline(always)]
    pub fn isoincif(&mut self) -> ISOINCIF_W {
        ISOINCIF_W { w: self }
    }
    #[doc = "Bit 21 - periodic transfer not complete interrupt flag(Host mode)/isochronous OUT transfer not complete interrupt flag(Device mode)"]
    #[inline(always)]
    pub fn pxncif_isooncif(&mut self) -> PXNCIF_ISOONCIF_W {
        PXNCIF_ISOONCIF_W { w: self }
    }
    #[doc = "Bit 28 - ID pin status change"]
    #[inline(always)]
    pub fn idpsc(&mut self) -> IDPSC_W {
        IDPSC_W { w: self }
    }
    #[doc = "Bit 29 - Disconnect interrupt flag"]
    #[inline(always)]
    pub fn discif(&mut self) -> DISCIF_W {
        DISCIF_W { w: self }
    }
    #[doc = "Bit 30 - Session interrupt flag"]
    #[inline(always)]
    pub fn sesif(&mut self) -> SESIF_W {
        SESIF_W { w: self }
    }
    #[doc = "Bit 31 - Wakeup interrupt flag"]
    #[inline(always)]
    pub fn wkupif(&mut self) -> WKUPIF_W {
        WKUPIF_W { w: self }
    }
}
