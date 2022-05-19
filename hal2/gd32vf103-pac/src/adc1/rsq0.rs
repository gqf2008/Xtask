#[doc = "Reader of register RSQ0"]
pub type R = crate::R<u32, super::RSQ0>;
#[doc = "Writer for register RSQ0"]
pub type W = crate::W<u32, super::RSQ0>;
#[doc = "Register RSQ0 `reset()`'s with value 0"]
impl crate::ResetValue for super::RSQ0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RL`"]
pub type RL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RL`"]
pub struct RL_W<'a> {
    w: &'a mut W,
}
impl<'a> RL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `RSQ15`"]
pub type RSQ15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSQ15`"]
pub struct RSQ15_W<'a> {
    w: &'a mut W,
}
impl<'a> RSQ15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
#[doc = "Reader of field `RSQ14`"]
pub type RSQ14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSQ14`"]
pub struct RSQ14_W<'a> {
    w: &'a mut W,
}
impl<'a> RSQ14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `RSQ13`"]
pub type RSQ13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSQ13`"]
pub struct RSQ13_W<'a> {
    w: &'a mut W,
}
impl<'a> RSQ13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `RSQ12`"]
pub type RSQ12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSQ12`"]
pub struct RSQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> RSQ12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:23 - Regular channel group length"]
    #[inline(always)]
    pub fn rl(&self) -> RL_R {
        RL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 15:19 - 16th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq15(&self) -> RSQ15_R {
        RSQ15_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 15th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq14(&self) -> RSQ14_R {
        RSQ14_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 14th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq13(&self) -> RSQ13_R {
        RSQ13_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - 13th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq12(&self) -> RSQ12_R {
        RSQ12_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:23 - Regular channel group length"]
    #[inline(always)]
    pub fn rl(&mut self) -> RL_W {
        RL_W { w: self }
    }
    #[doc = "Bits 15:19 - 16th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq15(&mut self) -> RSQ15_W {
        RSQ15_W { w: self }
    }
    #[doc = "Bits 10:14 - 15th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq14(&mut self) -> RSQ14_W {
        RSQ14_W { w: self }
    }
    #[doc = "Bits 5:9 - 14th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq13(&mut self) -> RSQ13_W {
        RSQ13_W { w: self }
    }
    #[doc = "Bits 0:4 - 13th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq12(&mut self) -> RSQ12_W {
        RSQ12_W { w: self }
    }
}
