#[doc = "Reader of register DIEPFEINTEN"]
pub type R = crate::R<u32, super::DIEPFEINTEN>;
#[doc = "Writer for register DIEPFEINTEN"]
pub type W = crate::W<u32, super::DIEPFEINTEN>;
#[doc = "Register DIEPFEINTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::DIEPFEINTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IEPTXFEIE`"]
pub type IEPTXFEIE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IEPTXFEIE`"]
pub struct IEPTXFEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IEPTXFEIE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - IN EP Tx FIFO empty interrupt enable bits"]
    #[inline(always)]
    pub fn ieptxfeie(&self) -> IEPTXFEIE_R {
        IEPTXFEIE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - IN EP Tx FIFO empty interrupt enable bits"]
    #[inline(always)]
    pub fn ieptxfeie(&mut self) -> IEPTXFEIE_W {
        IEPTXFEIE_W { w: self }
    }
}
