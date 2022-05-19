#[doc = "Reader of register CHCTL0_Input"]
pub type R = crate::R<u16, super::CHCTL0_INPUT>;
#[doc = "Writer for register CHCTL0_Input"]
pub type W = crate::W<u16, super::CHCTL0_INPUT>;
#[doc = "Register CHCTL0_Input `reset()`'s with value 0"]
impl crate::ResetValue for super::CHCTL0_INPUT {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH1CAPFLT`"]
pub type CH1CAPFLT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH1CAPFLT`"]
pub struct CH1CAPFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1CAPFLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u16) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `CH1CAPPSC`"]
pub type CH1CAPPSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH1CAPPSC`"]
pub struct CH1CAPPSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1CAPPSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u16) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `CH1MS`"]
pub type CH1MS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH1MS`"]
pub struct CH1MS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1MS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `CH0CAPFLT`"]
pub type CH0CAPFLT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH0CAPFLT`"]
pub struct CH0CAPFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0CAPFLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u16) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `CH0CAPPSC`"]
pub type CH0CAPPSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH0CAPPSC`"]
pub struct CH0CAPPSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0CAPPSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `CH0MS`"]
pub type CH0MS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH0MS`"]
pub struct CH0MS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0MS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - Channel 1 input capture filter control"]
    #[inline(always)]
    pub fn ch1capflt(&self) -> CH1CAPFLT_R {
        CH1CAPFLT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - Channel 1 input capture prescaler"]
    #[inline(always)]
    pub fn ch1cappsc(&self) -> CH1CAPPSC_R {
        CH1CAPPSC_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Channel 1 mode selection"]
    #[inline(always)]
    pub fn ch1ms(&self) -> CH1MS_R {
        CH1MS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - Channel 0 input capture filter control"]
    #[inline(always)]
    pub fn ch0capflt(&self) -> CH0CAPFLT_R {
        CH0CAPFLT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 2:3 - Channel 0 input capture prescaler"]
    #[inline(always)]
    pub fn ch0cappsc(&self) -> CH0CAPPSC_R {
        CH0CAPPSC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Channel 0 mode selection"]
    #[inline(always)]
    pub fn ch0ms(&self) -> CH0MS_R {
        CH0MS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - Channel 1 input capture filter control"]
    #[inline(always)]
    pub fn ch1capflt(&mut self) -> CH1CAPFLT_W {
        CH1CAPFLT_W { w: self }
    }
    #[doc = "Bits 10:11 - Channel 1 input capture prescaler"]
    #[inline(always)]
    pub fn ch1cappsc(&mut self) -> CH1CAPPSC_W {
        CH1CAPPSC_W { w: self }
    }
    #[doc = "Bits 8:9 - Channel 1 mode selection"]
    #[inline(always)]
    pub fn ch1ms(&mut self) -> CH1MS_W {
        CH1MS_W { w: self }
    }
    #[doc = "Bits 4:7 - Channel 0 input capture filter control"]
    #[inline(always)]
    pub fn ch0capflt(&mut self) -> CH0CAPFLT_W {
        CH0CAPFLT_W { w: self }
    }
    #[doc = "Bits 2:3 - Channel 0 input capture prescaler"]
    #[inline(always)]
    pub fn ch0cappsc(&mut self) -> CH0CAPPSC_W {
        CH0CAPPSC_W { w: self }
    }
    #[doc = "Bits 0:1 - Channel 0 mode selection"]
    #[inline(always)]
    pub fn ch0ms(&mut self) -> CH0MS_W {
        CH0MS_W { w: self }
    }
}
