#[doc = "Reader of register SAMPT1"]
pub type R = crate::R<u32, super::SAMPT1>;
#[doc = "Writer for register SAMPT1"]
pub type W = crate::W<u32, super::SAMPT1>;
#[doc = "Register SAMPT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SAMPT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPT0`"]
pub type SPT0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPT0`"]
pub struct SPT0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `SPT1`"]
pub type SPT1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPT1`"]
pub struct SPT1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `SPT2`"]
pub type SPT2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPT2`"]
pub struct SPT2_W<'a> {
    w: &'a mut W,
}
impl<'a> SPT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Reader of field `SPT3`"]
pub type SPT3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPT3`"]
pub struct SPT3_W<'a> {
    w: &'a mut W,
}
impl<'a> SPT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Reader of field `SPT4`"]
pub type SPT4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPT4`"]
pub struct SPT4_W<'a> {
    w: &'a mut W,
}
impl<'a> SPT4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `SPT5`"]
pub type SPT5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPT5`"]
pub struct SPT5_W<'a> {
    w: &'a mut W,
}
impl<'a> SPT5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | (((value as u32) & 0x07) << 15);
        self.w
    }
}
#[doc = "Reader of field `SPT6`"]
pub type SPT6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPT6`"]
pub struct SPT6_W<'a> {
    w: &'a mut W,
}
impl<'a> SPT6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `SPT7`"]
pub type SPT7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPT7`"]
pub struct SPT7_W<'a> {
    w: &'a mut W,
}
impl<'a> SPT7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
#[doc = "Reader of field `SPT8`"]
pub type SPT8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPT8`"]
pub struct SPT8_W<'a> {
    w: &'a mut W,
}
impl<'a> SPT8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `SPT9`"]
pub type SPT9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPT9`"]
pub struct SPT9_W<'a> {
    w: &'a mut W,
}
impl<'a> SPT9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Channel 0 sample time selection"]
    #[inline(always)]
    pub fn spt0(&self) -> SPT0_R {
        SPT0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Channel 1 sample time selection"]
    #[inline(always)]
    pub fn spt1(&self) -> SPT1_R {
        SPT1_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - Channel 2 sample time selection"]
    #[inline(always)]
    pub fn spt2(&self) -> SPT2_R {
        SPT2_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - Channel 3 sample time selection"]
    #[inline(always)]
    pub fn spt3(&self) -> SPT3_R {
        SPT3_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Channel 4 sample time selection"]
    #[inline(always)]
    pub fn spt4(&self) -> SPT4_R {
        SPT4_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 15:17 - Channel 5 sample time selection"]
    #[inline(always)]
    pub fn spt5(&self) -> SPT5_R {
        SPT5_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 18:20 - Channel 6 sample time selection"]
    #[inline(always)]
    pub fn spt6(&self) -> SPT6_R {
        SPT6_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 21:23 - Channel 7 sample time selection"]
    #[inline(always)]
    pub fn spt7(&self) -> SPT7_R {
        SPT7_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - Channel 8 sample time selection"]
    #[inline(always)]
    pub fn spt8(&self) -> SPT8_R {
        SPT8_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 27:29 - Channel 9 sample time selection"]
    #[inline(always)]
    pub fn spt9(&self) -> SPT9_R {
        SPT9_R::new(((self.bits >> 27) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel 0 sample time selection"]
    #[inline(always)]
    pub fn spt0(&mut self) -> SPT0_W {
        SPT0_W { w: self }
    }
    #[doc = "Bits 3:5 - Channel 1 sample time selection"]
    #[inline(always)]
    pub fn spt1(&mut self) -> SPT1_W {
        SPT1_W { w: self }
    }
    #[doc = "Bits 6:8 - Channel 2 sample time selection"]
    #[inline(always)]
    pub fn spt2(&mut self) -> SPT2_W {
        SPT2_W { w: self }
    }
    #[doc = "Bits 9:11 - Channel 3 sample time selection"]
    #[inline(always)]
    pub fn spt3(&mut self) -> SPT3_W {
        SPT3_W { w: self }
    }
    #[doc = "Bits 12:14 - Channel 4 sample time selection"]
    #[inline(always)]
    pub fn spt4(&mut self) -> SPT4_W {
        SPT4_W { w: self }
    }
    #[doc = "Bits 15:17 - Channel 5 sample time selection"]
    #[inline(always)]
    pub fn spt5(&mut self) -> SPT5_W {
        SPT5_W { w: self }
    }
    #[doc = "Bits 18:20 - Channel 6 sample time selection"]
    #[inline(always)]
    pub fn spt6(&mut self) -> SPT6_W {
        SPT6_W { w: self }
    }
    #[doc = "Bits 21:23 - Channel 7 sample time selection"]
    #[inline(always)]
    pub fn spt7(&mut self) -> SPT7_W {
        SPT7_W { w: self }
    }
    #[doc = "Bits 24:26 - Channel 8 sample time selection"]
    #[inline(always)]
    pub fn spt8(&mut self) -> SPT8_W {
        SPT8_W { w: self }
    }
    #[doc = "Bits 27:29 - Channel 9 sample time selection"]
    #[inline(always)]
    pub fn spt9(&mut self) -> SPT9_W {
        SPT9_W { w: self }
    }
}
