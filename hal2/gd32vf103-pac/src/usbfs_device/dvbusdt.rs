#[doc = "Reader of register DVBUSDT"]
pub type R = crate::R<u32, super::DVBUSDT>;
#[doc = "Writer for register DVBUSDT"]
pub type W = crate::W<u32, super::DVBUSDT>;
#[doc = "Register DVBUSDT `reset()`'s with value 0x17d7"]
impl crate::ResetValue for super::DVBUSDT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x17d7
    }
}
#[doc = "Reader of field `DVBUSDT`"]
pub type DVBUSDT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DVBUSDT`"]
pub struct DVBUSDT_W<'a> {
    w: &'a mut W,
}
impl<'a> DVBUSDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Device VBUS discharge time"]
    #[inline(always)]
    pub fn dvbusdt(&self) -> DVBUSDT_R {
        DVBUSDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Device VBUS discharge time"]
    #[inline(always)]
    pub fn dvbusdt(&mut self) -> DVBUSDT_W {
        DVBUSDT_W { w: self }
    }
}
