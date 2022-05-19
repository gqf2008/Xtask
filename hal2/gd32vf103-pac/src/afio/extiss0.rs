#[doc = "Reader of register EXTISS0"]
pub type R = crate::R<u32, super::EXTISS0>;
#[doc = "Writer for register EXTISS0"]
pub type W = crate::W<u32, super::EXTISS0>;
#[doc = "Register EXTISS0 `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTISS0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXTI3_SS`"]
pub type EXTI3_SS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI3_SS`"]
pub struct EXTI3_SS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI3_SS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `EXTI2_SS`"]
pub type EXTI2_SS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI2_SS`"]
pub struct EXTI2_SS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI2_SS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `EXTI1_SS`"]
pub type EXTI1_SS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI1_SS`"]
pub struct EXTI1_SS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI1_SS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `EXTI0_SS`"]
pub type EXTI0_SS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTI0_SS`"]
pub struct EXTI0_SS_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI0_SS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - EXTI 3 sources selection"]
    #[inline(always)]
    pub fn exti3_ss(&self) -> EXTI3_SS_R {
        EXTI3_SS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 2 sources selection"]
    #[inline(always)]
    pub fn exti2_ss(&self) -> EXTI2_SS_R {
        EXTI2_SS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 1 sources selection"]
    #[inline(always)]
    pub fn exti1_ss(&self) -> EXTI1_SS_R {
        EXTI1_SS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - EXTI 0 sources selection"]
    #[inline(always)]
    pub fn exti0_ss(&self) -> EXTI0_SS_R {
        EXTI0_SS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - EXTI 3 sources selection"]
    #[inline(always)]
    pub fn exti3_ss(&mut self) -> EXTI3_SS_W {
        EXTI3_SS_W { w: self }
    }
    #[doc = "Bits 8:11 - EXTI 2 sources selection"]
    #[inline(always)]
    pub fn exti2_ss(&mut self) -> EXTI2_SS_W {
        EXTI2_SS_W { w: self }
    }
    #[doc = "Bits 4:7 - EXTI 1 sources selection"]
    #[inline(always)]
    pub fn exti1_ss(&mut self) -> EXTI1_SS_W {
        EXTI1_SS_W { w: self }
    }
    #[doc = "Bits 0:3 - EXTI 0 sources selection"]
    #[inline(always)]
    pub fn exti0_ss(&mut self) -> EXTI0_SS_W {
        EXTI0_SS_W { w: self }
    }
}
