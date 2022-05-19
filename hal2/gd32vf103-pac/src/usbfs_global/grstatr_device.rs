#[doc = "Reader of register GRSTATR_Device"]
pub type R = crate::R<u32, super::GRSTATR_DEVICE>;
#[doc = "Reader of field `EPNUM`"]
pub type EPNUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `BCOUNT`"]
pub type BCOUNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `DPID`"]
pub type DPID_R = crate::R<u8, u8>;
#[doc = "Reader of field `RPCKST`"]
pub type RPCKST_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Endpoint number"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14 - Byte count"]
    #[inline(always)]
    pub fn bcount(&self) -> BCOUNT_R {
        BCOUNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bits 17:20 - Recieve packet status"]
    #[inline(always)]
    pub fn rpckst(&self) -> RPCKST_R {
        RPCKST_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
}
