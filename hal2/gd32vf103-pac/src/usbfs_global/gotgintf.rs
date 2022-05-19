#[doc = "Reader of register GOTGINTF"]
pub type R = crate::R<u32, super::GOTGINTF>;
#[doc = "Writer for register GOTGINTF"]
pub type W = crate::W<u32, super::GOTGINTF>;
#[doc = "Register GOTGINTF `reset()`'s with value 0"]
impl crate::ResetValue for super::GOTGINTF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SESEND`"]
pub type SESEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SESEND`"]
pub struct SESEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SESEND_W<'a> {
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
#[doc = "Reader of field `SRPEND`"]
pub type SRPEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRPEND`"]
pub struct SRPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SRPEND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `HNPEND`"]
pub type HNPEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HNPEND`"]
pub struct HNPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> HNPEND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `HNPDET`"]
pub type HNPDET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HNPDET`"]
pub struct HNPDET_W<'a> {
    w: &'a mut W,
}
impl<'a> HNPDET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `ADTO`"]
pub type ADTO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADTO`"]
pub struct ADTO_W<'a> {
    w: &'a mut W,
}
impl<'a> ADTO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `DF`"]
pub type DF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DF`"]
pub struct DF_W<'a> {
    w: &'a mut W,
}
impl<'a> DF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Session end"]
    #[inline(always)]
    pub fn sesend(&self) -> SESEND_R {
        SESEND_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Session request success status change"]
    #[inline(always)]
    pub fn srpend(&self) -> SRPEND_R {
        SRPEND_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - HNP end"]
    #[inline(always)]
    pub fn hnpend(&self) -> HNPEND_R {
        HNPEND_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Host negotiation request detected"]
    #[inline(always)]
    pub fn hnpdet(&self) -> HNPDET_R {
        HNPDET_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - A-device timeout"]
    #[inline(always)]
    pub fn adto(&self) -> ADTO_R {
        ADTO_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Debounce finish"]
    #[inline(always)]
    pub fn df(&self) -> DF_R {
        DF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Session end"]
    #[inline(always)]
    pub fn sesend(&mut self) -> SESEND_W {
        SESEND_W { w: self }
    }
    #[doc = "Bit 8 - Session request success status change"]
    #[inline(always)]
    pub fn srpend(&mut self) -> SRPEND_W {
        SRPEND_W { w: self }
    }
    #[doc = "Bit 9 - HNP end"]
    #[inline(always)]
    pub fn hnpend(&mut self) -> HNPEND_W {
        HNPEND_W { w: self }
    }
    #[doc = "Bit 17 - Host negotiation request detected"]
    #[inline(always)]
    pub fn hnpdet(&mut self) -> HNPDET_W {
        HNPDET_W { w: self }
    }
    #[doc = "Bit 18 - A-device timeout"]
    #[inline(always)]
    pub fn adto(&mut self) -> ADTO_W {
        ADTO_W { w: self }
    }
    #[doc = "Bit 19 - Debounce finish"]
    #[inline(always)]
    pub fn df(&mut self) -> DF_W {
        DF_W { w: self }
    }
}
