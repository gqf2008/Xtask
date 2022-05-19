#[doc = "Reader of register DAEPINTEN"]
pub type R = crate::R<u32, super::DAEPINTEN>;
#[doc = "Writer for register DAEPINTEN"]
pub type W = crate::W<u32, super::DAEPINTEN>;
#[doc = "Register DAEPINTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::DAEPINTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IEPIE`"]
pub type IEPIE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IEPIE`"]
pub struct IEPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IEPIE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `OEPIE`"]
pub type OEPIE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OEPIE`"]
pub struct OEPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OEPIE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - IN EP interrupt interrupt enable bits"]
    #[inline(always)]
    pub fn iepie(&self) -> IEPIE_R {
        IEPIE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - OUT endpoint interrupt enable bits"]
    #[inline(always)]
    pub fn oepie(&self) -> OEPIE_R {
        OEPIE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - IN EP interrupt interrupt enable bits"]
    #[inline(always)]
    pub fn iepie(&mut self) -> IEPIE_W {
        IEPIE_W { w: self }
    }
    #[doc = "Bits 16:19 - OUT endpoint interrupt enable bits"]
    #[inline(always)]
    pub fn oepie(&mut self) -> OEPIE_W {
        OEPIE_W { w: self }
    }
}
