#[doc = "Reader of register DAC0_DO"]
pub type R = crate::R<u32, super::DAC0_DO>;
#[doc = "Reader of field `DAC0_DO`"]
pub type DAC0_DO_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC0 data output"]
    #[inline(always)]
    pub fn dac0_do(&self) -> DAC0_DO_R {
        DAC0_DO_R::new((self.bits & 0x0fff) as u16)
    }
}
