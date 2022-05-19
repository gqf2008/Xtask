#[doc = "Reader of register DVBUSPT"]
pub type R = crate::R<u32, super::DVBUSPT>;
#[doc = "Writer for register DVBUSPT"]
pub type W = crate::W<u32, super::DVBUSPT>;
#[doc = "Register DVBUSPT `reset()`'s with value 0x05b8"]
impl crate::ResetValue for super::DVBUSPT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x05b8
    }
}
#[doc = "Reader of field `DVBUSPT`"]
pub type DVBUSPT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DVBUSPT`"]
pub struct DVBUSPT_W<'a> {
    w: &'a mut W,
}
impl<'a> DVBUSPT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Device VBUS pulsing time"]
    #[inline(always)]
    pub fn dvbuspt(&self) -> DVBUSPT_R {
        DVBUSPT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Device VBUS pulsing time"]
    #[inline(always)]
    pub fn dvbuspt(&mut self) -> DVBUSPT_W {
        DVBUSPT_W { w: self }
    }
}
