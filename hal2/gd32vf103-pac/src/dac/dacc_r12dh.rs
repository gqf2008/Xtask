#[doc = "Reader of register DACC_R12DH"]
pub type R = crate::R<u32, super::DACC_R12DH>;
#[doc = "Writer for register DACC_R12DH"]
pub type W = crate::W<u32, super::DACC_R12DH>;
#[doc = "Register DACC_R12DH `reset()`'s with value 0"]
impl crate::ResetValue for super::DACC_R12DH {
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
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - DAC0 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dac0_dh(&self) -> DAC0_DH_R {
        DAC0_DH_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - DAC1 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dac1_dh(&self) -> DAC1_DH_R {
        DAC1_DH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC0 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dac0_dh(&mut self) -> DAC0_DH_W {
        DAC0_DH_W { w: self }
    }
    #[doc = "Bits 16:27 - DAC1 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dac1_dh(&mut self) -> DAC1_DH_W {
        DAC1_DH_W { w: self }
    }
}
