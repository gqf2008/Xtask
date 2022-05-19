#[doc = "Reader of register EXTISS3"]
pub type R = crate::R<u32, super::EXTISS3>;
#[doc = "Writer for register EXTISS3"]
pub type W = crate::W<u32, super::EXTISS3>;
#[doc = "Register EXTISS3 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTISS3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTI15_SS`"]
pub type EXTI15_SS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI15_SS`"]
pub struct EXTI15_SS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI15_SS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `EXTI14_SS`"]
pub type EXTI14_SS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI14_SS`"]
pub struct EXTI14_SS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI14_SS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `EXTI13_SS`"]
pub type EXTI13_SS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI13_SS`"]
pub struct EXTI13_SS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI13_SS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `EXTI12_SS`"]
pub type EXTI12_SS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI12_SS`"]
pub struct EXTI12_SS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI12_SS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - EXTI 15 sources selection"]
    #[inline(always)]
    pub fn exti15_ss(&self) -> EXTI15_SS_R {
        EXTI15_SS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 14 sources selection"]
    #[inline(always)]
    pub fn exti14_ss(&self) -> EXTI14_SS_R {
        EXTI14_SS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 13 sources selection"]
    #[inline(always)]
    pub fn exti13_ss(&self) -> EXTI13_SS_R {
        EXTI13_SS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - EXTI 12 sources selection"]
    #[inline(always)]
    pub fn exti12_ss(&self) -> EXTI12_SS_R {
        EXTI12_SS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - EXTI 15 sources selection"]
    #[inline(always)]
    pub fn exti15_ss(&mut self) -> EXTI15_SS_W {
        EXTI15_SS_W { w: self }
    }
    #[doc = "Bits 8:11 - EXTI 14 sources selection"]
    #[inline(always)]
    pub fn exti14_ss(&mut self) -> EXTI14_SS_W {
        EXTI14_SS_W { w: self }
    }
    #[doc = "Bits 4:7 - EXTI 13 sources selection"]
    #[inline(always)]
    pub fn exti13_ss(&mut self) -> EXTI13_SS_W {
        EXTI13_SS_W { w: self }
    }
    #[doc = "Bits 0:3 - EXTI 12 sources selection"]
    #[inline(always)]
    pub fn exti12_ss(&mut self) -> EXTI12_SS_W {
        EXTI12_SS_W { w: self }
    }
}
