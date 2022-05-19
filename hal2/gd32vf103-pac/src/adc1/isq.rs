#[doc = "Reader of register ISQ"]
pub type R = crate::R<u32, super::ISQ>;
#[doc = "Writer for register ISQ"]
pub type W = crate::W<u32, super::ISQ>;
#[doc = "Register ISQ `reset()`'s with value 0"]
impl crate::ResetValue for super::ISQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IL`"]
pub type IL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IL`"]
pub struct IL_W<'a> {
    w: &'a mut W,
}
impl<'a> IL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `ISQ3`"]
pub type ISQ3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ISQ3`"]
pub struct ISQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> ISQ3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
#[doc = "Reader of field `ISQ2`"]
pub type ISQ2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ISQ2`"]
pub struct ISQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> ISQ2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `ISQ1`"]
pub type ISQ1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ISQ1`"]
pub struct ISQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> ISQ1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `ISQ0`"]
pub type ISQ0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ISQ0`"]
pub struct ISQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> ISQ0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:21 - Inserted channel group length"]
    #[inline(always)]
    pub fn il(&self) -> IL_R {
        IL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 15:19 - 4th conversion in inserted sequence"]
    #[inline(always)]
    pub fn isq3(&self) -> ISQ3_R {
        ISQ3_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 3rd conversion in inserted sequence"]
    #[inline(always)]
    pub fn isq2(&self) -> ISQ2_R {
        ISQ2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 2nd conversion in inserted sequence"]
    #[inline(always)]
    pub fn isq1(&self) -> ISQ1_R {
        ISQ1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - 1st conversion in inserted sequence"]
    #[inline(always)]
    pub fn isq0(&self) -> ISQ0_R {
        ISQ0_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:21 - Inserted channel group length"]
    #[inline(always)]
    pub fn il(&mut self) -> IL_W {
        IL_W { w: self }
    }
    #[doc = "Bits 15:19 - 4th conversion in inserted sequence"]
    #[inline(always)]
    pub fn isq3(&mut self) -> ISQ3_W {
        ISQ3_W { w: self }
    }
    #[doc = "Bits 10:14 - 3rd conversion in inserted sequence"]
    #[inline(always)]
    pub fn isq2(&mut self) -> ISQ2_W {
        ISQ2_W { w: self }
    }
    #[doc = "Bits 5:9 - 2nd conversion in inserted sequence"]
    #[inline(always)]
    pub fn isq1(&mut self) -> ISQ1_W {
        ISQ1_W { w: self }
    }
    #[doc = "Bits 0:4 - 1st conversion in inserted sequence"]
    #[inline(always)]
    pub fn isq0(&mut self) -> ISQ0_W {
        ISQ0_W { w: self }
    }
}
