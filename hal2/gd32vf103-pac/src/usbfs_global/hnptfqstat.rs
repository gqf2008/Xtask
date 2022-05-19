#[doc = "Reader of register HNPTFQSTAT"]
pub type R = crate::R<u32, super::HNPTFQSTAT>;
#[doc = "Reader of field `NPTXFS`"]
pub type NPTXFS_R = crate::R<u16, u16>;
#[doc = "Reader of field `NPTXRQS`"]
pub type NPTXRQS_R = crate::R<u8, u8>;
#[doc = "Reader of field `NPTXRQTOP`"]
pub type NPTXRQTOP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Non-periodic TxFIFO space"]
    #[inline(always)]
    pub fn nptxfs(&self) -> NPTXFS_R {
        NPTXFS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Non-periodic transmit request queue space"]
    #[inline(always)]
    pub fn nptxrqs(&self) -> NPTXRQS_R {
        NPTXRQS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Top of the non-periodic transmit request queue"]
    #[inline(always)]
    pub fn nptxrqtop(&self) -> NPTXRQTOP_R {
        NPTXRQTOP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
