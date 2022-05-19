#[doc = "Reader of register DAC1_DO"]
pub type R = crate::R<u32, super::DAC1_DO>;
#[doc = "Reader of field `DAC1_DO`"]
pub type DAC1_DO_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC1 data output"]
    #[inline(always)]
    pub fn dac1_do(&self) -> DAC1_DO_R {
        DAC1_DO_R::new((self.bits & 0x0fff) as u16)
    }
}
