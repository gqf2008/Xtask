#[doc = "Reader of register BT"]
pub type R = crate::R<u32, super::BT>;
#[doc = "Writer for register BT"]
pub type W = crate::W<u32, super::BT>;
#[doc = "Register BT `reset()`'s with value 0x0123_0000"]
impl crate::ResetValue for super::BT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0123_0000
    }
}
#[doc = "Reader of field `SCMOD`"]
pub type SCMOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCMOD`"]
pub struct SCMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SCMOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `LCMOD`"]
pub type LCMOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCMOD`"]
pub struct LCMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> LCMOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `SJW`"]
pub type SJW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SJW`"]
pub struct SJW_W<'a> {
    w: &'a mut W,
}
impl<'a> SJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `BS2`"]
pub type BS2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BS2`"]
pub struct BS2_W<'a> {
    w: &'a mut W,
}
impl<'a> BS2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `BS1`"]
pub type BS1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BS1`"]
pub struct BS1_W<'a> {
    w: &'a mut W,
}
impl<'a> BS1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `BAUDPSC`"]
pub type BAUDPSC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BAUDPSC`"]
pub struct BAUDPSC_W<'a> {
    w: &'a mut W,
}
impl<'a> BAUDPSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Silent communication mode"]
    #[inline(always)]
    pub fn scmod(&self) -> SCMOD_R {
        SCMOD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Loopback communication mode"]
    #[inline(always)]
    pub fn lcmod(&self) -> LCMOD_R {
        LCMOD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Resynchronization jump width"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 20:22 - Bit segment 2"]
    #[inline(always)]
    pub fn bs2(&self) -> BS2_R {
        BS2_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 16:19 - Bit segment 1"]
    #[inline(always)]
    pub fn bs1(&self) -> BS1_R {
        BS1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:9 - Baud rate prescaler"]
    #[inline(always)]
    pub fn baudpsc(&self) -> BAUDPSC_R {
        BAUDPSC_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Silent communication mode"]
    #[inline(always)]
    pub fn scmod(&mut self) -> SCMOD_W {
        SCMOD_W { w: self }
    }
    #[doc = "Bit 30 - Loopback communication mode"]
    #[inline(always)]
    pub fn lcmod(&mut self) -> LCMOD_W {
        LCMOD_W { w: self }
    }
    #[doc = "Bits 24:25 - Resynchronization jump width"]
    #[inline(always)]
    pub fn sjw(&mut self) -> SJW_W {
        SJW_W { w: self }
    }
    #[doc = "Bits 20:22 - Bit segment 2"]
    #[inline(always)]
    pub fn bs2(&mut self) -> BS2_W {
        BS2_W { w: self }
    }
    #[doc = "Bits 16:19 - Bit segment 1"]
    #[inline(always)]
    pub fn bs1(&mut self) -> BS1_W {
        BS1_W { w: self }
    }
    #[doc = "Bits 0:9 - Baud rate prescaler"]
    #[inline(always)]
    pub fn baudpsc(&mut self) -> BAUDPSC_W {
        BAUDPSC_W { w: self }
    }
}
