#[doc = "Reader of register RDATA"]
pub type R = crate::R<u32, super::RDATA>;
#[doc = "Reader of field `ADC1RDTR`"]
pub type ADC1RDTR_R = crate::R<u16, u16>;
#[doc = "Reader of field `RDATA`"]
pub type RDATA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - ADC regular channel data"]
    #[inline(always)]
    pub fn adc1rdtr(&self) -> ADC1RDTR_R {
        ADC1RDTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Regular channel data"]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits & 0xffff) as u16)
    }
}
