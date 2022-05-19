#[doc = "Reader of register TMI1"]
pub type R = crate::R<u32, super::TMI1>;
#[doc = "Writer for register TMI1"]
pub type W = crate::W<u32, super::TMI1>;
#[doc = "Register TMI1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TMI1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SFID_EFID`"]
pub type SFID_EFID_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SFID_EFID`"]
pub struct SFID_EFID_W<'a> {
    w: &'a mut W,
}
impl<'a> SFID_EFID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 21)) | (((value as u32) & 0x07ff) << 21);
        self.w
    }
}
#[doc = "Reader of field `EFID`"]
pub type EFID_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EFID`"]
pub struct EFID_W<'a> {
    w: &'a mut W,
}
impl<'a> EFID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 3)) | (((value as u32) & 0x0003_ffff) << 3);
        self.w
    }
}
#[doc = "Reader of field `FF`"]
pub type FF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FF`"]
pub struct FF_W<'a> {
    w: &'a mut W,
}
impl<'a> FF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `FT`"]
pub type FT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT`"]
pub struct FT_W<'a> {
    w: &'a mut W,
}
impl<'a> FT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TEN`"]
pub type TEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEN`"]
pub struct TEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 21:31 - The frame identifier"]
    #[inline(always)]
    pub fn sfid_efid(&self) -> SFID_EFID_R {
        SFID_EFID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
    #[doc = "Bits 3:20 - The frame identifier"]
    #[inline(always)]
    pub fn efid(&self) -> EFID_R {
        EFID_R::new(((self.bits >> 3) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 2 - Frame format"]
    #[inline(always)]
    pub fn ff(&self) -> FF_R {
        FF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Frame type"]
    #[inline(always)]
    pub fn ft(&self) -> FT_R {
        FT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Transmit enable"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 21:31 - The frame identifier"]
    #[inline(always)]
    pub fn sfid_efid(&mut self) -> SFID_EFID_W {
        SFID_EFID_W { w: self }
    }
    #[doc = "Bits 3:20 - The frame identifier"]
    #[inline(always)]
    pub fn efid(&mut self) -> EFID_W {
        EFID_W { w: self }
    }
    #[doc = "Bit 2 - Frame format"]
    #[inline(always)]
    pub fn ff(&mut self) -> FF_W {
        FF_W { w: self }
    }
    #[doc = "Bit 1 - Frame type"]
    #[inline(always)]
    pub fn ft(&mut self) -> FT_W {
        FT_W { w: self }
    }
    #[doc = "Bit 0 - Transmit enable"]
    #[inline(always)]
    pub fn ten(&mut self) -> TEN_W {
        TEN_W { w: self }
    }
}
