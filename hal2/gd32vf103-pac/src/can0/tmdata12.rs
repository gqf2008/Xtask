#[doc = "Reader of register TMDATA12"]
pub type R = crate::R<u32, super::TMDATA12>;
#[doc = "Writer for register TMDATA12"]
pub type W = crate::W<u32, super::TMDATA12>;
#[doc = "Register TMDATA12 `reset()`'s with value 0"]
impl crate::ResetValue for super::TMDATA12 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DB7`"]
pub type DB7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DB7`"]
pub struct DB7_W<'a> {
    w: &'a mut W,
}
impl<'a> DB7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `DB6`"]
pub type DB6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DB6`"]
pub struct DB6_W<'a> {
    w: &'a mut W,
}
impl<'a> DB6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DB5`"]
pub type DB5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DB5`"]
pub struct DB5_W<'a> {
    w: &'a mut W,
}
impl<'a> DB5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DB4`"]
pub type DB4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DB4`"]
pub struct DB4_W<'a> {
    w: &'a mut W,
}
impl<'a> DB4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - Data byte 7"]
    #[inline(always)]
    pub fn db7(&self) -> DB7_R {
        DB7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 6"]
    #[inline(always)]
    pub fn db6(&self) -> DB6_R {
        DB6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 5"]
    #[inline(always)]
    pub fn db5(&self) -> DB5_R {
        DB5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Data byte 4"]
    #[inline(always)]
    pub fn db4(&self) -> DB4_R {
        DB4_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Data byte 7"]
    #[inline(always)]
    pub fn db7(&mut self) -> DB7_W {
        DB7_W { w: self }
    }
    #[doc = "Bits 16:23 - Data byte 6"]
    #[inline(always)]
    pub fn db6(&mut self) -> DB6_W {
        DB6_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 5"]
    #[inline(always)]
    pub fn db5(&mut self) -> DB5_W {
        DB5_W { w: self }
    }
    #[doc = "Bits 0:7 - Data byte 4"]
    #[inline(always)]
    pub fn db4(&mut self) -> DB4_W {
        DB4_W { w: self }
    }
}
