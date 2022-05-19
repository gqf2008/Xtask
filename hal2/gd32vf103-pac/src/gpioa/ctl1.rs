#[doc = "Reader of register CTL1"]
pub type R = crate::R<u32, super::CTL1>;
#[doc = "Writer for register CTL1"]
pub type W = crate::W<u32, super::CTL1>;
#[doc = "Register CTL1 `reset()`'s with value 0x4444_4444"]
impl crate::ResetValue for super::CTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4444_4444
    }
}
#[doc = "Reader of field `CTL15`"]
pub type CTL15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTL15`"]
pub struct CTL15_W<'a> {
    w: &'a mut W,
}
impl<'a> CTL15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `MD15`"]
pub type MD15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MD15`"]
pub struct MD15_W<'a> {
    w: &'a mut W,
}
impl<'a> MD15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `CTL14`"]
pub type CTL14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTL14`"]
pub struct CTL14_W<'a> {
    w: &'a mut W,
}
impl<'a> CTL14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `MD14`"]
pub type MD14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MD14`"]
pub struct MD14_W<'a> {
    w: &'a mut W,
}
impl<'a> MD14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `CTL13`"]
pub type CTL13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTL13`"]
pub struct CTL13_W<'a> {
    w: &'a mut W,
}
impl<'a> CTL13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `MD13`"]
pub type MD13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MD13`"]
pub struct MD13_W<'a> {
    w: &'a mut W,
}
impl<'a> MD13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `CTL12`"]
pub type CTL12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTL12`"]
pub struct CTL12_W<'a> {
    w: &'a mut W,
}
impl<'a> CTL12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `MD12`"]
pub type MD12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MD12`"]
pub struct MD12_W<'a> {
    w: &'a mut W,
}
impl<'a> MD12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `CTL11`"]
pub type CTL11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTL11`"]
pub struct CTL11_W<'a> {
    w: &'a mut W,
}
impl<'a> CTL11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `MD11`"]
pub type MD11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MD11`"]
pub struct MD11_W<'a> {
    w: &'a mut W,
}
impl<'a> MD11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `CTL10`"]
pub type CTL10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTL10`"]
pub struct CTL10_W<'a> {
    w: &'a mut W,
}
impl<'a> CTL10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `MD10`"]
pub type MD10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MD10`"]
pub struct MD10_W<'a> {
    w: &'a mut W,
}
impl<'a> MD10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `CTL9`"]
pub type CTL9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTL9`"]
pub struct CTL9_W<'a> {
    w: &'a mut W,
}
impl<'a> CTL9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `MD9`"]
pub type MD9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MD9`"]
pub struct MD9_W<'a> {
    w: &'a mut W,
}
impl<'a> MD9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `CTL8`"]
pub type CTL8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTL8`"]
pub struct CTL8_W<'a> {
    w: &'a mut W,
}
impl<'a> CTL8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `MD8`"]
pub type MD8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MD8`"]
pub struct MD8_W<'a> {
    w: &'a mut W,
}
impl<'a> MD8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Port x configuration bits (x = 15)"]
    #[inline(always)]
    pub fn ctl15(&self) -> CTL15_R {
        CTL15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Port x mode bits (x = 15)"]
    #[inline(always)]
    pub fn md15(&self) -> MD15_R {
        MD15_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 14)"]
    #[inline(always)]
    pub fn ctl14(&self) -> CTL14_R {
        CTL14_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Port x mode bits (x = 14)"]
    #[inline(always)]
    pub fn md14(&self) -> MD14_R {
        MD14_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 13)"]
    #[inline(always)]
    pub fn ctl13(&self) -> CTL13_R {
        CTL13_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port x mode bits (x = 13)"]
    #[inline(always)]
    pub fn md13(&self) -> MD13_R {
        MD13_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 12)"]
    #[inline(always)]
    pub fn ctl12(&self) -> CTL12_R {
        CTL12_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Port x mode bits (x = 12)"]
    #[inline(always)]
    pub fn md12(&self) -> MD12_R {
        MD12_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 11)"]
    #[inline(always)]
    pub fn ctl11(&self) -> CTL11_R {
        CTL11_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Port x mode bits (x = 11 )"]
    #[inline(always)]
    pub fn md11(&self) -> MD11_R {
        MD11_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 10)"]
    #[inline(always)]
    pub fn ctl10(&self) -> CTL10_R {
        CTL10_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port x mode bits (x = 10 )"]
    #[inline(always)]
    pub fn md10(&self) -> MD10_R {
        MD10_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 9)"]
    #[inline(always)]
    pub fn ctl9(&self) -> CTL9_R {
        CTL9_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port x mode bits (x = 9)"]
    #[inline(always)]
    pub fn md9(&self) -> MD9_R {
        MD9_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 8)"]
    #[inline(always)]
    pub fn ctl8(&self) -> CTL8_R {
        CTL8_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Port x mode bits (x = 8)"]
    #[inline(always)]
    pub fn md8(&self) -> MD8_R {
        MD8_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Port x configuration bits (x = 15)"]
    #[inline(always)]
    pub fn ctl15(&mut self) -> CTL15_W {
        CTL15_W { w: self }
    }
    #[doc = "Bits 28:29 - Port x mode bits (x = 15)"]
    #[inline(always)]
    pub fn md15(&mut self) -> MD15_W {
        MD15_W { w: self }
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 14)"]
    #[inline(always)]
    pub fn ctl14(&mut self) -> CTL14_W {
        CTL14_W { w: self }
    }
    #[doc = "Bits 24:25 - Port x mode bits (x = 14)"]
    #[inline(always)]
    pub fn md14(&mut self) -> MD14_W {
        MD14_W { w: self }
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 13)"]
    #[inline(always)]
    pub fn ctl13(&mut self) -> CTL13_W {
        CTL13_W { w: self }
    }
    #[doc = "Bits 20:21 - Port x mode bits (x = 13)"]
    #[inline(always)]
    pub fn md13(&mut self) -> MD13_W {
        MD13_W { w: self }
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 12)"]
    #[inline(always)]
    pub fn ctl12(&mut self) -> CTL12_W {
        CTL12_W { w: self }
    }
    #[doc = "Bits 16:17 - Port x mode bits (x = 12)"]
    #[inline(always)]
    pub fn md12(&mut self) -> MD12_W {
        MD12_W { w: self }
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 11)"]
    #[inline(always)]
    pub fn ctl11(&mut self) -> CTL11_W {
        CTL11_W { w: self }
    }
    #[doc = "Bits 12:13 - Port x mode bits (x = 11 )"]
    #[inline(always)]
    pub fn md11(&mut self) -> MD11_W {
        MD11_W { w: self }
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 10)"]
    #[inline(always)]
    pub fn ctl10(&mut self) -> CTL10_W {
        CTL10_W { w: self }
    }
    #[doc = "Bits 8:9 - Port x mode bits (x = 10 )"]
    #[inline(always)]
    pub fn md10(&mut self) -> MD10_W {
        MD10_W { w: self }
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 9)"]
    #[inline(always)]
    pub fn ctl9(&mut self) -> CTL9_W {
        CTL9_W { w: self }
    }
    #[doc = "Bits 4:5 - Port x mode bits (x = 9)"]
    #[inline(always)]
    pub fn md9(&mut self) -> MD9_W {
        MD9_W { w: self }
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 8)"]
    #[inline(always)]
    pub fn ctl8(&mut self) -> CTL8_W {
        CTL8_W { w: self }
    }
    #[doc = "Bits 0:1 - Port x mode bits (x = 8)"]
    #[inline(always)]
    pub fn md8(&mut self) -> MD8_W {
        MD8_W { w: self }
    }
}
