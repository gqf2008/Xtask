#[doc = "Reader of register DIVH"]
pub type R = crate::R<u32, super::DIVH>;
#[doc = "Reader of field `DIV`"]
pub type DIV_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - RTC divider value high"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0x0f) as u8)
    }
}
