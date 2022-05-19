#[doc = "Reader of register TMDATA02"]
pub type R = crate::R<u32, super::TMDATA02>;
#[doc = "Writer for register TMDATA02"]
pub type W = crate::W<u32, super::TMDATA02>;
#[doc = "Register TMDATA02 `reset()`'s with value 0"]
impl crate::ResetValue for super::TMDATA02 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DB3`"]
pub type DB3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DB3`"]
pub struct DB3_W<'a> {
    w: &'a mut W,
}
impl<'a> DB3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `DB2`"]
pub type DB2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DB2`"]
pub struct DB2_W<'a> {
    w: &'a mut W,
}
impl<'a> DB2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DB1`"]
pub type DB1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DB1`"]
pub struct DB1_W<'a> {
    w: &'a mut W,
}
impl<'a> DB1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DB0`"]
pub type DB0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DB0`"]
pub struct DB0_W<'a> {
    w: &'a mut W,
}
impl<'a> DB0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - Data byte 3"]
    #[inline(always)]
    pub fn db3(&self) -> DB3_R {
        DB3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 2"]
    #[inline(always)]
    pub fn db2(&self) -> DB2_R {
        DB2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1"]
    #[inline(always)]
    pub fn db1(&self) -> DB1_R {
        DB1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Data byte 0"]
    #[inline(always)]
    pub fn db0(&self) -> DB0_R {
        DB0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Data byte 3"]
    #[inline(always)]
    pub fn db3(&mut self) -> DB3_W {
        DB3_W { w: self }
    }
    #[doc = "Bits 16:23 - Data byte 2"]
    #[inline(always)]
    pub fn db2(&mut self) -> DB2_W {
        DB2_W { w: self }
    }
    #[doc = "Bits 8:15 - Data byte 1"]
    #[inline(always)]
    pub fn db1(&mut self) -> DB1_W {
        DB1_W { w: self }
    }
    #[doc = "Bits 0:7 - Data byte 0"]
    #[inline(always)]
    pub fn db0(&mut self) -> DB0_W {
        DB0_W { w: self }
    }
}
