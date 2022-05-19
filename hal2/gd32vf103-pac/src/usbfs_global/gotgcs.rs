#[doc = "Reader of register GOTGCS"]
pub type R = crate::R<u32, super::GOTGCS>;
#[doc = "Writer for register GOTGCS"]
pub type W = crate::W<u32, super::GOTGCS>;
#[doc = "Register GOTGCS `reset()`'s with value 0x0800"]
impl crate::ResetValue for super::GOTGCS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0800
    }
}
#[doc = "Reader of field `SRPS`"]
pub type SRPS_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPREQ`"]
pub type SRPREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRPREQ`"]
pub struct SRPREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SRPREQ_W<'a> {
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
#[doc = "Reader of field `HNPS`"]
pub type HNPS_R = crate::R<bool, bool>;
#[doc = "Reader of field `HNPREQ`"]
pub type HNPREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HNPREQ`"]
pub struct HNPREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HNPREQ_W<'a> {
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
#[doc = "Reader of field `HHNPEN`"]
pub type HHNPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HHNPEN`"]
pub struct HHNPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HHNPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `DHNPEN`"]
pub type DHNPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DHNPEN`"]
pub struct DHNPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DHNPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `IDPS`"]
pub type IDPS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DI`"]
pub type DI_R = crate::R<bool, bool>;
#[doc = "Reader of field `ASV`"]
pub type ASV_R = crate::R<bool, bool>;
#[doc = "Reader of field `BSV`"]
pub type BSV_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SRP success"]
    #[inline(always)]
    pub fn srps(&self) -> SRPS_R {
        SRPS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SRP request"]
    #[inline(always)]
    pub fn srpreq(&self) -> SRPREQ_R {
        SRPREQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Host success"]
    #[inline(always)]
    pub fn hnps(&self) -> HNPS_R {
        HNPS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    pub fn hnpreq(&self) -> HNPREQ_R {
        HNPREQ_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Host HNP enable"]
    #[inline(always)]
    pub fn hhnpen(&self) -> HHNPEN_R {
        HHNPEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    pub fn dhnpen(&self) -> DHNPEN_R {
        DHNPEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ID pin status"]
    #[inline(always)]
    pub fn idps(&self) -> IDPS_R {
        IDPS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Debounce interval"]
    #[inline(always)]
    pub fn di(&self) -> DI_R {
        DI_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - A-session valid"]
    #[inline(always)]
    pub fn asv(&self) -> ASV_R {
        ASV_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B-session valid"]
    #[inline(always)]
    pub fn bsv(&self) -> BSV_R {
        BSV_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SRP request"]
    #[inline(always)]
    pub fn srpreq(&mut self) -> SRPREQ_W {
        SRPREQ_W { w: self }
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    pub fn hnpreq(&mut self) -> HNPREQ_W {
        HNPREQ_W { w: self }
    }
    #[doc = "Bit 10 - Host HNP enable"]
    #[inline(always)]
    pub fn hhnpen(&mut self) -> HHNPEN_W {
        HHNPEN_W { w: self }
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    pub fn dhnpen(&mut self) -> DHNPEN_W {
        DHNPEN_W { w: self }
    }
}
