#[doc = "Reader of register RSQ1"]
pub type R = crate::R<u32, super::RSQ1>;
#[doc = "Writer for register RSQ1"]
pub type W = crate::W<u32, super::RSQ1>;
#[doc = "Register RSQ1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RSQ1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSQ11`"]
pub type RSQ11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSQ11`"]
pub struct RSQ11_W<'a> {
    w: &'a mut W,
}
impl<'a> RSQ11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | (((value as u32) & 0x1f) << 25);
        self.w
    }
}
#[doc = "Reader of field `RSQ10`"]
pub type RSQ10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSQ10`"]
pub struct RSQ10_W<'a> {
    w: &'a mut W,
}
impl<'a> RSQ10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | (((value as u32) & 0x1f) << 20);
        self.w
    }
}
#[doc = "Reader of field `RSQ9`"]
pub type RSQ9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSQ9`"]
pub struct RSQ9_W<'a> {
    w: &'a mut W,
}
impl<'a> RSQ9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
#[doc = "Reader of field `RSQ8`"]
pub type RSQ8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSQ8`"]
pub struct RSQ8_W<'a> {
    w: &'a mut W,
}
impl<'a> RSQ8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `RSQ7`"]
pub type RSQ7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSQ7`"]
pub struct RSQ7_W<'a> {
    w: &'a mut W,
}
impl<'a> RSQ7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `RSQ6`"]
pub type RSQ6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSQ6`"]
pub struct RSQ6_W<'a> {
    w: &'a mut W,
}
impl<'a> RSQ6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:29 - 12th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq11(&self) -> RSQ11_R {
        RSQ11_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 11th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq10(&self) -> RSQ10_R {
        RSQ10_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 10th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq9(&self) -> RSQ9_R {
        RSQ9_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 9th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq8(&self) -> RSQ8_R {
        RSQ8_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 8th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq7(&self) -> RSQ7_R {
        RSQ7_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - 7th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq6(&self) -> RSQ6_R {
        RSQ6_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:29 - 12th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq11(&mut self) -> RSQ11_W {
        RSQ11_W { w: self }
    }
    #[doc = "Bits 20:24 - 11th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq10(&mut self) -> RSQ10_W {
        RSQ10_W { w: self }
    }
    #[doc = "Bits 15:19 - 10th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq9(&mut self) -> RSQ9_W {
        RSQ9_W { w: self }
    }
    #[doc = "Bits 10:14 - 9th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq8(&mut self) -> RSQ8_W {
        RSQ8_W { w: self }
    }
    #[doc = "Bits 5:9 - 8th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq7(&mut self) -> RSQ7_W {
        RSQ7_W { w: self }
    }
    #[doc = "Bits 0:4 - 7th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq6(&mut self) -> RSQ6_W {
        RSQ6_W { w: self }
    }
}
