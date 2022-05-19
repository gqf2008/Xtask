#[doc = "Reader of register DIVL"]
pub type R = crate::R<u32, super::DIVL>;
#[doc = "Reader of field `DIV`"]
pub type DIV_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC divider value low"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xffff) as u16)
    }
}
