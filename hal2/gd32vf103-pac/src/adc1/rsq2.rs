#[doc = "Reader of register RSQ2"]
pub type R = crate::R<u32, super::RSQ2>;
#[doc = "Writer for register RSQ2"]
pub type W = crate::W<u32, super::RSQ2>;
#[doc = "Register RSQ2 `reset()`'s with value 0"]
impl crate::ResetValue for super::RSQ2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSQ5`"]
pub type RSQ5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSQ5`"]
pub struct RSQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> RSQ5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | (((value as u32) & 0x1f) << 25);
        self.w
    }
}
#[doc = "Reader of field `RSQ4`"]
pub type RSQ4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSQ4`"]
pub struct RSQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> RSQ4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | (((value as u32) & 0x1f) << 20);
        self.w
    }
}
#[doc = "Reader of field `RSQ3`"]
pub type RSQ3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSQ3`"]
pub struct RSQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> RSQ3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
#[doc = "Reader of field `RSQ2`"]
pub type RSQ2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSQ2`"]
pub struct RSQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSQ2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `RSQ1`"]
pub type RSQ1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSQ1`"]
pub struct RSQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSQ1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `RSQ0`"]
pub type RSQ0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSQ0`"]
pub struct RSQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> RSQ0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:29 - 6th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq5(&self) -> RSQ5_R {
        RSQ5_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 5th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq4(&self) -> RSQ4_R {
        RSQ4_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 4th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq3(&self) -> RSQ3_R {
        RSQ3_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 3rd conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq2(&self) -> RSQ2_R {
        RSQ2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 2nd conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq1(&self) -> RSQ1_R {
        RSQ1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - 1st conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq0(&self) -> RSQ0_R {
        RSQ0_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:29 - 6th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq5(&mut self) -> RSQ5_W {
        RSQ5_W { w: self }
    }
    #[doc = "Bits 20:24 - 5th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq4(&mut self) -> RSQ4_W {
        RSQ4_W { w: self }
    }
    #[doc = "Bits 15:19 - 4th conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq3(&mut self) -> RSQ3_W {
        RSQ3_W { w: self }
    }
    #[doc = "Bits 10:14 - 3rd conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq2(&mut self) -> RSQ2_W {
        RSQ2_W { w: self }
    }
    #[doc = "Bits 5:9 - 2nd conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq1(&mut self) -> RSQ1_W {
        RSQ1_W { w: self }
    }
    #[doc = "Bits 0:4 - 1st conversion in regular sequence"]
    #[inline(always)]
    pub fn rsq0(&mut self) -> RSQ0_W {
        RSQ0_W { w: self }
    }
}
