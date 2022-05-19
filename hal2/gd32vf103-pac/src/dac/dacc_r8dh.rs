#[doc = "Reader of register DACC_R8DH"]
pub type R = crate::R<u32, super::DACC_R8DH>;
#[doc = "Writer for register DACC_R8DH"]
pub type W = crate::W<u32, super::DACC_R8DH>;
#[doc = "Register DACC_R8DH `reset()`'s with value 0"]
impl crate::ResetValue for super::DACC_R8DH {
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
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DAC0 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dac0_dh(&self) -> DAC0_DH_R {
        DAC0_DH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DAC1 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dac1_dh(&self) -> DAC1_DH_R {
        DAC1_DH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC0 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dac0_dh(&mut self) -> DAC0_DH_W {
        DAC0_DH_W { w: self }
    }
    #[doc = "Bits 8:15 - DAC1 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dac1_dh(&mut self) -> DAC1_DH_W {
        DAC1_DH_W { w: self }
    }
}
