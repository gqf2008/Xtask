#[doc = "Reader of register STAT1"]
pub type R = crate::R<u16, super::STAT1>;
#[doc = "Reader of field `PECV`"]
pub type PECV_R = crate::R<u8, u8>;
#[doc = "Reader of field `DUMODF`"]
pub type DUMODF_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSTSMB`"]
pub type HSTSMB_R = crate::R<bool, bool>;
#[doc = "Reader of field `DEFSMB`"]
pub type DEFSMB_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXGC`"]
pub type RXGC_R = crate::R<bool, bool>;
#[doc = "Reader of field `TR`"]
pub type TR_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2CBSY`"]
pub type I2CBSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `MASTER`"]
pub type MASTER_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 8:15 - Packet Error Checking Value that calculated by hardware when PEC is enabled"]
    #[inline(always)]
    pub fn pecv(&self) -> PECV_R {
        PECV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 7 - Dual Flag in slave mode"]
    #[inline(always)]
    pub fn dumodf(&self) -> DUMODF_R {
        DUMODF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SMBus Host Header detected in slave mode"]
    #[inline(always)]
    pub fn hstsmb(&self) -> HSTSMB_R {
        HSTSMB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Default address of SMBusDevice"]
    #[inline(always)]
    pub fn defsmb(&self) -> DEFSMB_R {
        DEFSMB_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - General call address (00h) received"]
    #[inline(always)]
    pub fn rxgc(&self) -> RXGC_R {
        RXGC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Whether the I2C is a transmitter or a receiver"]
    #[inline(always)]
    pub fn tr(&self) -> TR_R {
        TR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Busy flag"]
    #[inline(always)]
    pub fn i2cbsy(&self) -> I2CBSY_R {
        I2CBSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - A flag indicating whether I2C block is in master or slave mode"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new((self.bits & 0x01) != 0)
    }
}
