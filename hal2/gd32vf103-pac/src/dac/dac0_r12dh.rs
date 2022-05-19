#[doc = "Reader of register DAC0_R12DH"]
pub type R = crate::R<u32, super::DAC0_R12DH>;
#[doc = "Writer for register DAC0_R12DH"]
pub type W = crate::W<u32, super::DAC0_R12DH>;
#[doc = "Register DAC0_R12DH `reset()`'s with value 0"]
impl crate::ResetValue for super::DAC0_R12DH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DAC0_DH`"]
pub type DAC0_DH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DAC0_DH`"]
pub struct DAC0_DH_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC0_DH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - DAC0 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dac0_dh(&self) -> DAC0_DH_R {
        DAC0_DH_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC0 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dac0_dh(&mut self) -> DAC0_DH_W {
        DAC0_DH_W { w: self }
    }
}
