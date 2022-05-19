#[doc = "Reader of register STAT"]
pub type R = crate::R<u16, super::STAT>;
#[doc = "Writer for register STAT"]
pub type W = crate::W<u16, super::STAT>;
#[doc = "Register STAT `reset()`'s with value 0x02"]
impl crate::ResetValue for super::STAT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `FERR`"]
pub type FERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRANS`"]
pub type TRANS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXORERR`"]
pub type RXORERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `CONFERR`"]
pub type CONFERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRCERR`"]
pub type CRCERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCERR`"]
pub struct CRCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TXURERR`"]
pub type TXURERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2SCH`"]
pub type I2SCH_R = crate::R<bool, bool>;
#[doc = "Reader of field `TBE`"]
pub type TBE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RBNE`"]
pub type RBNE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 8 - Format error"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmitting On-going Bit"]
    #[inline(always)]
    pub fn trans(&self) -> TRANS_R {
        TRANS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Reception Overrun Error Bit"]
    #[inline(always)]
    pub fn rxorerr(&self) -> RXORERR_R {
        RXORERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SPI Configuration error"]
    #[inline(always)]
    pub fn conferr(&self) -> CONFERR_R {
        CONFERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SPI CRC Error Bit"]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmission underrun error bit"]
    #[inline(always)]
    pub fn txurerr(&self) -> TXURERR_R {
        TXURERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2S channel side"]
    #[inline(always)]
    pub fn i2sch(&self) -> I2SCH_R {
        I2SCH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Buffer Empty"]
    #[inline(always)]
    pub fn tbe(&self) -> TBE_R {
        TBE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Receive Buffer Not Empty"]
    #[inline(always)]
    pub fn rbne(&self) -> RBNE_R {
        RBNE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - SPI CRC Error Bit"]
    #[inline(always)]
    pub fn crcerr(&mut self) -> CRCERR_W {
        CRCERR_W { w: self }
    }
}
