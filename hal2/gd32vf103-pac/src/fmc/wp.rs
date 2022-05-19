#[doc = "Reader of register WP"]
pub type R = crate::R<u32, super::WP>;
#[doc = "Reader of field `WP`"]
pub type WP_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Store WP\\[31:0\\]
of option bytes block after system reset"]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
