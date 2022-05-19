#[doc = "Reader of register DAC0_R8DH"]
pub type R = crate::R<u32, super::DAC0_R8DH>;
#[doc = "Writer for register DAC0_R8DH"]
pub type W = crate::W<u32, super::DAC0_R8DH>;
#[doc = "Register DAC0_R8DH `reset()`'s with value 0"]
impl crate::ResetValue for super::DAC0_R8DH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DAC0_DH`"]
pub type DAC0_DH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAC0_DH`"]
pub struct DAC0_DH_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC0_DH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DAC0 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dac0_dh(&self) -> DAC0_DH_R {
        DAC0_DH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC0 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dac0_dh(&mut self) -> DAC0_DH_W {
        DAC0_DH_W { w: self }
    }
}
