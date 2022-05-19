#[doc = "Reader of register ERR"]
pub type R = crate::R<u32, super::ERR>;
#[doc = "Writer for register ERR"]
pub type W = crate::W<u32, super::ERR>;
#[doc = "Register ERR `reset()`'s with value 0"]
impl crate::ResetValue for super::ERR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RECNT`"]
pub type RECNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `TECNT`"]
pub type TECNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `ERRN`"]
pub type ERRN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ERRN`"]
pub struct ERRN_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `BOERR`"]
pub type BOERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PERR`"]
pub type PERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `WERR`"]
pub type WERR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 24:31 - Receive Error Count defined by the CAN standard"]
    #[inline(always)]
    pub fn recnt(&self) -> RECNT_R {
        RECNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Transmit Error Count defined by the CAN standard"]
    #[inline(always)]
    pub fn tecnt(&self) -> TECNT_R {
        TECNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 4:6 - Error number"]
    #[inline(always)]
    pub fn errn(&self) -> ERRN_R {
        ERRN_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Bus-off error"]
    #[inline(always)]
    pub fn boerr(&self) -> BOERR_R {
        BOERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Passive error"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Warning error"]
    #[inline(always)]
    pub fn werr(&self) -> WERR_R {
        WERR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - Error number"]
    #[inline(always)]
    pub fn errn(&mut self) -> ERRN_W {
        ERRN_W { w: self }
    }
}
