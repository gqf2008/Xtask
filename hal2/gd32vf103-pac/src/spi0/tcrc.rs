#[doc = "Reader of register TCRC"]
pub type R = crate::R<u16, super::TCRC>;
#[doc = "Reader of field `TCRC`"]
pub type TCRC_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Tx CRC value"]
    #[inline(always)]
    pub fn tcrc(&self) -> TCRC_R {
        TCRC_R::new((self.bits & 0xffff) as u16)
    }
}
