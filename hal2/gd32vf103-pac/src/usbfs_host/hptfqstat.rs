#[doc = "Reader of register HPTFQSTAT"]
pub type R = crate::R<u32, super::HPTFQSTAT>;
#[doc = "Reader of field `PTXFS`"]
pub type PTXFS_R = crate::R<u16, u16>;
#[doc = "Reader of field `PTXREQS`"]
pub type PTXREQS_R = crate::R<u8, u8>;
#[doc = "Reader of field `PTXREQT`"]
pub type PTXREQT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Periodic transmit data FIFO space available"]
    #[inline(always)]
    pub fn ptxfs(&self) -> PTXFS_R {
        PTXFS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Periodic transmit request queue space available"]
    #[inline(always)]
    pub fn ptxreqs(&self) -> PTXREQS_R {
        PTXREQS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Top of the periodic transmit request queue"]
    #[inline(always)]
    pub fn ptxreqt(&self) -> PTXREQT_R {
        PTXREQT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
