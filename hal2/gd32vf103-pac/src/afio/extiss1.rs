#[doc = "Reader of register EXTISS1"]
pub type R = crate::R<u32, super::EXTISS1>;
#[doc = "Writer for register EXTISS1"]
pub type W = crate::W<u32, super::EXTISS1>;
#[doc = "Register EXTISS1 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTISS1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTI7_SS`"]
pub type EXTI7_SS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI7_SS`"]
pub struct EXTI7_SS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI7_SS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `EXTI6_SS`"]
pub type EXTI6_SS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI6_SS`"]
pub struct EXTI6_SS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI6_SS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `EXTI5_SS`"]
pub type EXTI5_SS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI5_SS`"]
pub struct EXTI5_SS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI5_SS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `EXTI4_SS`"]
pub type EXTI4_SS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI4_SS`"]
pub struct EXTI4_SS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI4_SS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - EXTI 7 sources selection"]
    #[inline(always)]
    pub fn exti7_ss(&self) -> EXTI7_SS_R {
        EXTI7_SS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 6 sources selection"]
    #[inline(always)]
    pub fn exti6_ss(&self) -> EXTI6_SS_R {
        EXTI6_SS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 5 sources selection"]
    #[inline(always)]
    pub fn exti5_ss(&self) -> EXTI5_SS_R {
        EXTI5_SS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - EXTI 4 sources selection"]
    #[inline(always)]
    pub fn exti4_ss(&self) -> EXTI4_SS_R {
        EXTI4_SS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - EXTI 7 sources selection"]
    #[inline(always)]
    pub fn exti7_ss(&mut self) -> EXTI7_SS_W {
        EXTI7_SS_W { w: self }
    }
    #[doc = "Bits 8:11 - EXTI 6 sources selection"]
    #[inline(always)]
    pub fn exti6_ss(&mut self) -> EXTI6_SS_W {
        EXTI6_SS_W { w: self }
    }
    #[doc = "Bits 4:7 - EXTI 5 sources selection"]
    #[inline(always)]
    pub fn exti5_ss(&mut self) -> EXTI5_SS_W {
        EXTI5_SS_W { w: self }
    }
    #[doc = "Bits 0:3 - EXTI 4 sources selection"]
    #[inline(always)]
    pub fn exti4_ss(&mut self) -> EXTI4_SS_W {
        EXTI4_SS_W { w: self }
    }
}
