#[doc = "Reader of register DIEP1TFSTAT"]
pub type R = crate::R<u32, super::DIEP1TFSTAT>;
#[doc = "Reader of field `IEPTFS`"]
pub type IEPTFS_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space remaining"]
    #[inline(always)]
    pub fn ieptfs(&self) -> IEPTFS_R {
        IEPTFS_R::new((self.bits & 0xffff) as u16)
    }
}
