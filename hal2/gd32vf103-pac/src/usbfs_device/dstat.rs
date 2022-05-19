#[doc = "Reader of register DSTAT"]
pub type R = crate::R<u32, super::DSTAT>;
#[doc = "Reader of field `SPST`"]
pub type SPST_R = crate::R<bool, bool>;
#[doc = "Reader of field `ES`"]
pub type ES_R = crate::R<u8, u8>;
#[doc = "Reader of field `FNRSOF`"]
pub type FNRSOF_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bit 0 - Suspend status"]
    #[inline(always)]
    pub fn spst(&self) -> SPST_R {
        SPST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Enumerated speed"]
    #[inline(always)]
    pub fn es(&self) -> ES_R {
        ES_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 8:21 - Frame number of the received SOF"]
    #[inline(always)]
    pub fn fnrsof(&self) -> FNRSOF_R {
        FNRSOF_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
