#[doc = "Reader of register CHCTL1_Input"]
pub type R = crate::R<u16, super::CHCTL1_INPUT>;
#[doc = "Writer for register CHCTL1_Input"]
pub type W = crate::W<u16, super::CHCTL1_INPUT>;
#[doc = "Register CHCTL1_Input `reset()`'s with value 0"]
impl crate::ResetValue for super::CHCTL1_INPUT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH3CAPFLT`"]
pub type CH3CAPFLT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH3CAPFLT`"]
pub struct CH3CAPFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3CAPFLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u16) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `CH3CAPPSC`"]
pub type CH3CAPPSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH3CAPPSC`"]
pub struct CH3CAPPSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3CAPPSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u16) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `CH3MS`"]
pub type CH3MS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH3MS`"]
pub struct CH3MS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3MS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `CH2CAPFLT`"]
pub type CH2CAPFLT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH2CAPFLT`"]
pub struct CH2CAPFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2CAPFLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u16) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `CH2CAPPSC`"]
pub type CH2CAPPSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH2CAPPSC`"]
pub struct CH2CAPPSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2CAPPSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `CH2MS`"]
pub type CH2MS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH2MS`"]
pub struct CH2MS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2MS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - Channel 3 input capture filter control"]
    #[inline(always)]
    pub fn ch3capflt(&self) -> CH3CAPFLT_R {
        CH3CAPFLT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - Channel 3 input capture prescaler"]
    #[inline(always)]
    pub fn ch3cappsc(&self) -> CH3CAPPSC_R {
        CH3CAPPSC_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Channel 3 mode selection"]
    #[inline(always)]
    pub fn ch3ms(&self) -> CH3MS_R {
        CH3MS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - Channel 2 input capture filter control"]
    #[inline(always)]
    pub fn ch2capflt(&self) -> CH2CAPFLT_R {
        CH2CAPFLT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 2:3 - Channel 2 input capture prescaler"]
    #[inline(always)]
    pub fn ch2cappsc(&self) -> CH2CAPPSC_R {
        CH2CAPPSC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Channel 2 mode selection"]
    #[inline(always)]
    pub fn ch2ms(&self) -> CH2MS_R {
        CH2MS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - Channel 3 input capture filter control"]
    #[inline(always)]
    pub fn ch3capflt(&mut self) -> CH3CAPFLT_W {
        CH3CAPFLT_W { w: self }
    }
    #[doc = "Bits 10:11 - Channel 3 input capture prescaler"]
    #[inline(always)]
    pub fn ch3cappsc(&mut self) -> CH3CAPPSC_W {
        CH3CAPPSC_W { w: self }
    }
    #[doc = "Bits 8:9 - Channel 3 mode selection"]
    #[inline(always)]
    pub fn ch3ms(&mut self) -> CH3MS_W {
        CH3MS_W { w: self }
    }
    #[doc = "Bits 4:7 - Channel 2 input capture filter control"]
    #[inline(always)]
    pub fn ch2capflt(&mut self) -> CH2CAPFLT_W {
        CH2CAPFLT_W { w: self }
    }
    #[doc = "Bits 2:3 - Channel 2 input capture prescaler"]
    #[inline(always)]
    pub fn ch2cappsc(&mut self) -> CH2CAPPSC_W {
        CH2CAPPSC_W { w: self }
    }
    #[doc = "Bits 0:1 - Channel 2 mode selection"]
    #[inline(always)]
    pub fn ch2ms(&mut self) -> CH2MS_W {
        CH2MS_W { w: self }
    }
}
