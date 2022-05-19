#[doc = "Reader of register CTL0"]
pub type R = crate::R<u32, super::CTL0>;
#[doc = "Writer for register CTL0"]
pub type W = crate::W<u32, super::CTL0>;
#[doc = "Register CTL0 `reset()`'s with value 0x4444_4444"]
impl crate::ResetValue for super::CTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4444_4444
    }
}
#[doc = "Reader of field `CTL7`"]
pub type CTL7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTL7`"]
pub struct CTL7_W<'a> {
    w: &'a mut W,
}
impl<'a> CTL7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `MD7`"]
pub type MD7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MD7`"]
pub struct MD7_W<'a> {
    w: &'a mut W,
}
impl<'a> MD7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `CTL6`"]
pub type CTL6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTL6`"]
pub struct CTL6_W<'a> {
    w: &'a mut W,
}
impl<'a> CTL6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `MD6`"]
pub type MD6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MD6`"]
pub struct MD6_W<'a> {
    w: &'a mut W,
}
impl<'a> MD6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `CTL5`"]
pub type CTL5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTL5`"]
pub struct CTL5_W<'a> {
    w: &'a mut W,
}
impl<'a> CTL5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `MD5`"]
pub type MD5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MD5`"]
pub struct MD5_W<'a> {
    w: &'a mut W,
}
impl<'a> MD5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `CTL4`"]
pub type CTL4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTL4`"]
pub struct CTL4_W<'a> {
    w: &'a mut W,
}
impl<'a> CTL4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `MD4`"]
pub type MD4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MD4`"]
pub struct MD4_W<'a> {
    w: &'a mut W,
}
impl<'a> MD4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `CTL3`"]
pub type CTL3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTL3`"]
pub struct CTL3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTL3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `MD3`"]
pub type MD3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MD3`"]
pub struct MD3_W<'a> {
    w: &'a mut W,
}
impl<'a> MD3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `CTL2`"]
pub type CTL2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTL2`"]
pub struct CTL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `MD2`"]
pub type MD2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MD2`"]
pub struct MD2_W<'a> {
    w: &'a mut W,
}
impl<'a> MD2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `CTL1`"]
pub type CTL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTL1`"]
pub struct CTL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `MD1`"]
pub type MD1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MD1`"]
pub struct MD1_W<'a> {
    w: &'a mut W,
}
impl<'a> MD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `CTL0`"]
pub type CTL0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTL0`"]
pub struct CTL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `MD0`"]
pub type MD0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MD0`"]
pub struct MD0_W<'a> {
    w: &'a mut W,
}
impl<'a> MD0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Port x configuration bits (x = 7)"]
    #[inline(always)]
    pub fn ctl7(&self) -> CTL7_R {
        CTL7_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Port x mode bits (x = 7)"]
    #[inline(always)]
    pub fn md7(&self) -> MD7_R {
        MD7_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 6)"]
    #[inline(always)]
    pub fn ctl6(&self) -> CTL6_R {
        CTL6_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Port x mode bits (x = 6)"]
    #[inline(always)]
    pub fn md6(&self) -> MD6_R {
        MD6_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 5)"]
    #[inline(always)]
    pub fn ctl5(&self) -> CTL5_R {
        CTL5_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Port x mode bits (x = 5)"]
    #[inline(always)]
    pub fn md5(&self) -> MD5_R {
        MD5_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 4)"]
    #[inline(always)]
    pub fn ctl4(&self) -> CTL4_R {
        CTL4_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Port x mode bits (x = 4)"]
    #[inline(always)]
    pub fn md4(&self) -> MD4_R {
        MD4_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 3)"]
    #[inline(always)]
    pub fn ctl3(&self) -> CTL3_R {
        CTL3_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Port x mode bits (x = 3 )"]
    #[inline(always)]
    pub fn md3(&self) -> MD3_R {
        MD3_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 2)"]
    #[inline(always)]
    pub fn ctl2(&self) -> CTL2_R {
        CTL2_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Port x mode bits (x = 2 )"]
    #[inline(always)]
    pub fn md2(&self) -> MD2_R {
        MD2_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 1)"]
    #[inline(always)]
    pub fn ctl1(&self) -> CTL1_R {
        CTL1_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Port x mode bits (x = 1)"]
    #[inline(always)]
    pub fn md1(&self) -> MD1_R {
        MD1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 0)"]
    #[inline(always)]
    pub fn ctl0(&self) -> CTL0_R {
        CTL0_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Port x mode bits (x = 0)"]
    #[inline(always)]
    pub fn md0(&self) -> MD0_R {
        MD0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Port x configuration bits (x = 7)"]
    #[inline(always)]
    pub fn ctl7(&mut self) -> CTL7_W {
        CTL7_W { w: self }
    }
    #[doc = "Bits 28:29 - Port x mode bits (x = 7)"]
    #[inline(always)]
    pub fn md7(&mut self) -> MD7_W {
        MD7_W { w: self }
    }
    #[doc = "Bits 26:27 - Port x configuration bits (x = 6)"]
    #[inline(always)]
    pub fn ctl6(&mut self) -> CTL6_W {
        CTL6_W { w: self }
    }
    #[doc = "Bits 24:25 - Port x mode bits (x = 6)"]
    #[inline(always)]
    pub fn md6(&mut self) -> MD6_W {
        MD6_W { w: self }
    }
    #[doc = "Bits 22:23 - Port x configuration bits (x = 5)"]
    #[inline(always)]
    pub fn ctl5(&mut self) -> CTL5_W {
        CTL5_W { w: self }
    }
    #[doc = "Bits 20:21 - Port x mode bits (x = 5)"]
    #[inline(always)]
    pub fn md5(&mut self) -> MD5_W {
        MD5_W { w: self }
    }
    #[doc = "Bits 18:19 - Port x configuration bits (x = 4)"]
    #[inline(always)]
    pub fn ctl4(&mut self) -> CTL4_W {
        CTL4_W { w: self }
    }
    #[doc = "Bits 16:17 - Port x mode bits (x = 4)"]
    #[inline(always)]
    pub fn md4(&mut self) -> MD4_W {
        MD4_W { w: self }
    }
    #[doc = "Bits 14:15 - Port x configuration bits (x = 3)"]
    #[inline(always)]
    pub fn ctl3(&mut self) -> CTL3_W {
        CTL3_W { w: self }
    }
    #[doc = "Bits 12:13 - Port x mode bits (x = 3 )"]
    #[inline(always)]
    pub fn md3(&mut self) -> MD3_W {
        MD3_W { w: self }
    }
    #[doc = "Bits 10:11 - Port x configuration bits (x = 2)"]
    #[inline(always)]
    pub fn ctl2(&mut self) -> CTL2_W {
        CTL2_W { w: self }
    }
    #[doc = "Bits 8:9 - Port x mode bits (x = 2 )"]
    #[inline(always)]
    pub fn md2(&mut self) -> MD2_W {
        MD2_W { w: self }
    }
    #[doc = "Bits 6:7 - Port x configuration bits (x = 1)"]
    #[inline(always)]
    pub fn ctl1(&mut self) -> CTL1_W {
        CTL1_W { w: self }
    }
    #[doc = "Bits 4:5 - Port x mode bits (x = 1)"]
    #[inline(always)]
    pub fn md1(&mut self) -> MD1_W {
        MD1_W { w: self }
    }
    #[doc = "Bits 2:3 - Port x configuration bits (x = 0)"]
    #[inline(always)]
    pub fn ctl0(&mut self) -> CTL0_W {
        CTL0_W { w: self }
    }
    #[doc = "Bits 0:1 - Port x mode bits (x = 0)"]
    #[inline(always)]
    pub fn md0(&mut self) -> MD0_W {
        MD0_W { w: self }
    }
}
