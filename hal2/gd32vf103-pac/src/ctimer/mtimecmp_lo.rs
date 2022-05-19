#[doc = "Reader of register mtimecmp_lo"]
pub type R = crate::R<u32, super::MTIMECMP_LO>;
#[doc = "Writer for register mtimecmp_lo"]
pub type W = crate::W<u32, super::MTIMECMP_LO>;
#[doc = "Register mtimecmp_lo `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MTIMECMP_LO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
