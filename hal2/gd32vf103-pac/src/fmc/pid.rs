#[doc = "Reader of register PID"]
pub type R = crate::R<u32, super::PID>;
#[doc = "Reader of field `PID`"]
pub type PID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Product reserved ID code register"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
