#[doc = "Reader of register DAC1_L12DH"]
pub type R = crate::R<u32, super::DAC1_L12DH>;
#[doc = "Writer for register DAC1_L12DH"]
pub type W = crate::W<u32, super::DAC1_L12DH>;
#[doc = "Register DAC1_L12DH `reset()`'s with value 0"]
impl crate::ResetValue for super::DAC1_L12DH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DAC1_DH`"]
pub type DAC1_DH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DAC1_DH`"]
pub struct DAC1_DH_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC1_DH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | (((value as u32) & 0x0fff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:15 - DAC1 12-bit left-aligned data"]
    #[inline(always)]
    pub fn dac1_dh(&self) -> DAC1_DH_R {
        DAC1_DH_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - DAC1 12-bit left-aligned data"]
    #[inline(always)]
    pub fn dac1_dh(&mut self) -> DAC1_DH_W {
        DAC1_DH_W { w: self }
    }
}
