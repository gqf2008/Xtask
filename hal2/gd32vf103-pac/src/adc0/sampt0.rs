#[doc = "Reader of register SAMPT0"]
pub type R = crate::R<u32, super::SAMPT0>;
#[doc = "Writer for register SAMPT0"]
pub type W = crate::W<u32, super::SAMPT0>;
#[doc = "Register SAMPT0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SAMPT0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPT10`"]
pub type SPT10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPT10`"]
pub struct SPT10_W<'a> {
    w: &'a mut W,
}
impl<'a> SPT10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `SPT11`"]
pub type SPT11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPT11`"]
pub struct SPT11_W<'a> {
    w: &'a mut W,
}
impl<'a> SPT11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `SPT12`"]
pub type SPT12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPT12`"]
pub struct SPT12_W<'a> {
    w: &'a mut W,
}
impl<'a> SPT12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Reader of field `SPT13`"]
pub type SPT13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPT13`"]
pub struct SPT13_W<'a> {
    w: &'a mut W,
}
impl<'a> SPT13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Reader of field `SPT14`"]
pub type SPT14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPT14`"]
pub struct SPT14_W<'a> {
    w: &'a mut W,
}
impl<'a> SPT14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `SPT15`"]
pub type SPT15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPT15`"]
pub struct SPT15_W<'a> {
    w: &'a mut W,
}
impl<'a> SPT15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | (((value as u32) & 0x07) << 15);
        self.w
    }
}
#[doc = "Reader of field `SPT16`"]
pub type SPT16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPT16`"]
pub struct SPT16_W<'a> {
    w: &'a mut W,
}
impl<'a> SPT16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `SPT17`"]
pub type SPT17_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPT17`"]
pub struct SPT17_W<'a> {
    w: &'a mut W,
}
impl<'a> SPT17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Channel 10 sample time selection"]
    #[inline(always)]
    pub fn spt10(&self) -> SPT10_R {
        SPT10_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Channel 11 sample time selection"]
    #[inline(always)]
    pub fn spt11(&self) -> SPT11_R {
        SPT11_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - Channel 12 sample time selection"]
    #[inline(always)]
    pub fn spt12(&self) -> SPT12_R {
        SPT12_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - Channel 13 sample time selection"]
    #[inline(always)]
    pub fn spt13(&self) -> SPT13_R {
        SPT13_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Channel 14 sample time selection"]
    #[inline(always)]
    pub fn spt14(&self) -> SPT14_R {
        SPT14_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 15:17 - Channel 15 sample time selection"]
    #[inline(always)]
    pub fn spt15(&self) -> SPT15_R {
        SPT15_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 18:20 - Channel 16 sample time selection"]
    #[inline(always)]
    pub fn spt16(&self) -> SPT16_R {
        SPT16_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 21:23 - Channel 17 sample time selection"]
    #[inline(always)]
    pub fn spt17(&self) -> SPT17_R {
        SPT17_R::new(((self.bits >> 21) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel 10 sample time selection"]
    #[inline(always)]
    pub fn spt10(&mut self) -> SPT10_W {
        SPT10_W { w: self }
    }
    #[doc = "Bits 3:5 - Channel 11 sample time selection"]
    #[inline(always)]
    pub fn spt11(&mut self) -> SPT11_W {
        SPT11_W { w: self }
    }
    #[doc = "Bits 6:8 - Channel 12 sample time selection"]
    #[inline(always)]
    pub fn spt12(&mut self) -> SPT12_W {
        SPT12_W { w: self }
    }
    #[doc = "Bits 9:11 - Channel 13 sample time selection"]
    #[inline(always)]
    pub fn spt13(&mut self) -> SPT13_W {
        SPT13_W { w: self }
    }
    #[doc = "Bits 12:14 - Channel 14 sample time selection"]
    #[inline(always)]
    pub fn spt14(&mut self) -> SPT14_W {
        SPT14_W { w: self }
    }
    #[doc = "Bits 15:17 - Channel 15 sample time selection"]
    #[inline(always)]
    pub fn spt15(&mut self) -> SPT15_W {
        SPT15_W { w: self }
    }
    #[doc = "Bits 18:20 - Channel 16 sample time selection"]
    #[inline(always)]
    pub fn spt16(&mut self) -> SPT16_W {
        SPT16_W { w: self }
    }
    #[doc = "Bits 21:23 - Channel 17 sample time selection"]
    #[inline(always)]
    pub fn spt17(&mut self) -> SPT17_W {
        SPT17_W { w: self }
    }
}
