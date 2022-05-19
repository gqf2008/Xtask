#[doc = "Reader of register DAC1_R8DH"]
pub type R = crate::R<u32, super::DAC1_R8DH>;
#[doc = "Writer for register DAC1_R8DH"]
pub type W = crate::W<u32, super::DAC1_R8DH>;
#[doc = "Register DAC1_R8DH `reset()`'s with value 0"]
impl crate::ResetValue for super::DAC1_R8DH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DAC1_DH`"]
pub type DAC1_DH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAC1_DH`"]
pub struct DAC1_DH_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC1_DH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DAC1 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dac1_dh(&self) -> DAC1_DH_R {
        DAC1_DH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC1 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dac1_dh(&mut self) -> DAC1_DH_W {
        DAC1_DH_W { w: self }
    }
}
