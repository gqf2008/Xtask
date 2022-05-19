#[doc = "Reader of register HACHINT"]
pub type R = crate::R<u32, super::HACHINT>;
#[doc = "Reader of field `HACHINT`"]
pub type HACHINT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Host all channel interrupts"]
    #[inline(always)]
    pub fn hachint(&self) -> HACHINT_R {
        HACHINT_R::new((self.bits & 0xff) as u8)
    }
}
