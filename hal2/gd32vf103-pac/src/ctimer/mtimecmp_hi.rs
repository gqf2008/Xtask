#[doc = "Reader of register mtimecmp_hi"]
pub type R = crate::R<u32, super::MTIMECMP_HI>;
#[doc = "Writer for register mtimecmp_hi"]
pub type W = crate::W<u32, super::MTIMECMP_HI>;
#[doc = "Register mtimecmp_hi `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MTIMECMP_HI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
