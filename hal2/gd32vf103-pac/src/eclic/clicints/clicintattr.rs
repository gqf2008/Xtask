#[doc = "Reader of register CLICINTATTR"]
pub type R = crate::R<u8, super::CLICINTATTR>;
#[doc = "Writer for register CLICINTATTR"]
pub type W = crate::W<u8, super::CLICINTATTR>;
#[doc = "Register CLICINTATTR `reset()`'s with value 0"]
impl crate::ResetValue for super::CLICINTATTR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SHV`"]
pub type SHV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SHV`"]
pub struct SHV_W<'a> {
    w: &'a mut W,
}
impl<'a> SHV_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `TRIG`"]
pub type TRIG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIG`"]
pub struct TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u8) & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SHV"]
    #[inline(always)]
    pub fn shv(&self) -> SHV_R {
        SHV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - TRIG"]
    #[inline(always)]
    pub fn trig(&self) -> TRIG_R {
        TRIG_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SHV"]
    #[inline(always)]
    pub fn shv(&mut self) -> SHV_W {
        SHV_W { w: self }
    }
    #[doc = "Bits 1:2 - TRIG"]
    #[inline(always)]
    pub fn trig(&mut self) -> TRIG_W {
        TRIG_W { w: self }
    }
}
