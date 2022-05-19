#[doc = "Reader of register OCTL"]
pub type R = crate::R<u16, super::OCTL>;
#[doc = "Writer for register OCTL"]
pub type W = crate::W<u16, super::OCTL>;
#[doc = "Register OCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::OCTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ROSEL`"]
pub type ROSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ROSEL`"]
pub struct ROSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u16) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `ASOEN`"]
pub type ASOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASOEN`"]
pub struct ASOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ASOEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u16) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `COEN`"]
pub type COEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COEN`"]
pub struct COEN_W<'a> {
    w: &'a mut W,
}
impl<'a> COEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u16) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RCCV`"]
pub type RCCV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCCV`"]
pub struct RCCV_W<'a> {
    w: &'a mut W,
}
impl<'a> RCCV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bit 9 - RTC output selection"]
    #[inline(always)]
    pub fn rosel(&self) -> ROSEL_R {
        ROSEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RTC alarm or second signal output enable"]
    #[inline(always)]
    pub fn asoen(&self) -> ASOEN_R {
        ASOEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RTC clock calibration output enable"]
    #[inline(always)]
    pub fn coen(&self) -> COEN_R {
        COEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 0:6 - RTC clock calibration value"]
    #[inline(always)]
    pub fn rccv(&self) -> RCCV_R {
        RCCV_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 9 - RTC output selection"]
    #[inline(always)]
    pub fn rosel(&mut self) -> ROSEL_W {
        ROSEL_W { w: self }
    }
    #[doc = "Bit 8 - RTC alarm or second signal output enable"]
    #[inline(always)]
    pub fn asoen(&mut self) -> ASOEN_W {
        ASOEN_W { w: self }
    }
    #[doc = "Bit 7 - RTC clock calibration output enable"]
    #[inline(always)]
    pub fn coen(&mut self) -> COEN_W {
        COEN_W { w: self }
    }
    #[doc = "Bits 0:6 - RTC clock calibration value"]
    #[inline(always)]
    pub fn rccv(&mut self) -> RCCV_W {
        RCCV_W { w: self }
    }
}
