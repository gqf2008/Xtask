#[doc = "Reader of register IDATA0"]
pub type R = crate::R<u32, super::IDATA0>;
#[doc = "Reader of field `IDATAn`"]
pub type IDATAN_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Inserted number n conversion data"]
    #[inline(always)]
    pub fn idatan(&self) -> IDATAN_R {
        IDATAN_R::new((self.bits & 0xffff) as u16)
    }
}
