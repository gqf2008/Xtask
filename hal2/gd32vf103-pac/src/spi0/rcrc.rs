#[doc = "Reader of register RCRC"]
pub type R = crate::R<u16, super::RCRC>;
#[doc = "Reader of field `RCRC`"]
pub type RCRC_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - RX CRC value"]
    #[inline(always)]
    pub fn rcrc(&self) -> RCRC_R {
        RCRC_R::new((self.bits & 0xffff) as u16)
    }
}
