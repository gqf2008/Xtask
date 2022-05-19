#[doc = "Reader of register EXTISS2"]
pub type R = crate::R<u32, super::EXTISS2>;
#[doc = "Writer for register EXTISS2"]
pub type W = crate::W<u32, super::EXTISS2>;
#[doc = "Register EXTISS2 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTISS2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTI11_SS`"]
pub type EXTI11_SS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI11_SS`"]
pub struct EXTI11_SS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI11_SS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `EXTI10_SS`"]
pub type EXTI10_SS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI10_SS`"]
pub struct EXTI10_SS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI10_SS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `EXTI9_SS`"]
pub type EXTI9_SS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI9_SS`"]
pub struct EXTI9_SS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI9_SS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `EXTI8_SS`"]
pub type EXTI8_SS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI8_SS`"]
pub struct EXTI8_SS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI8_SS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - EXTI 11 sources selection"]
    #[inline(always)]
    pub fn exti11_ss(&self) -> EXTI11_SS_R {
        EXTI11_SS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 10 sources selection"]
    #[inline(always)]
    pub fn exti10_ss(&self) -> EXTI10_SS_R {
        EXTI10_SS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 9 sources selection"]
    #[inline(always)]
    pub fn exti9_ss(&self) -> EXTI9_SS_R {
        EXTI9_SS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - EXTI 8 sources selection"]
    #[inline(always)]
    pub fn exti8_ss(&self) -> EXTI8_SS_R {
        EXTI8_SS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - EXTI 11 sources selection"]
    #[inline(always)]
    pub fn exti11_ss(&mut self) -> EXTI11_SS_W {
        EXTI11_SS_W { w: self }
    }
    #[doc = "Bits 8:11 - EXTI 10 sources selection"]
    #[inline(always)]
    pub fn exti10_ss(&mut self) -> EXTI10_SS_W {
        EXTI10_SS_W { w: self }
    }
    #[doc = "Bits 4:7 - EXTI 9 sources selection"]
    #[inline(always)]
    pub fn exti9_ss(&mut self) -> EXTI9_SS_W {
        EXTI9_SS_W { w: self }
    }
    #[doc = "Bits 0:3 - EXTI 8 sources selection"]
    #[inline(always)]
    pub fn exti8_ss(&mut self) -> EXTI8_SS_W {
        EXTI8_SS_W { w: self }
    }
}
