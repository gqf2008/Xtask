#[doc = "Reader of register RSTSCK"]
pub type R = crate::R<u32, super::RSTSCK>;
#[doc = "Writer for register RSTSCK"]
pub type W = crate::W<u32, super::RSTSCK>;
#[doc = "Register RSTSCK `reset()`'s with value 0x0c00_0000"]
impl crate::ResetValue for super::RSTSCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0c00_0000
    }
}
#[doc = "Reader of field `IRC40KEN`"]
pub type IRC40KEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRC40KEN`"]
pub struct IRC40KEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRC40KEN_W<'a> {
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
#[doc = "Reader of field `IRC40KSTB`"]
pub type IRC40KSTB_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSTFC`"]
pub type RSTFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTFC`"]
pub struct RSTFC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `EPRSTF`"]
pub type EPRSTF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PORRSTF`"]
pub type PORRSTF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRSTF`"]
pub type SWRSTF_R = crate::R<bool, bool>;
#[doc = "Reader of field `FWDGTRSTF`"]
pub type FWDGTRSTF_R = crate::R<bool, bool>;
#[doc = "Reader of field `WWDGTRSTF`"]
pub type WWDGTRSTF_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPRSTF`"]
pub type LPRSTF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - IRC40K enable"]
    #[inline(always)]
    pub fn irc40ken(&self) -> IRC40KEN_R {
        IRC40KEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IRC40K stabilization"]
    #[inline(always)]
    pub fn irc40kstb(&self) -> IRC40KSTB_R {
        IRC40KSTB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Reset flag clear"]
    #[inline(always)]
    pub fn rstfc(&self) -> RSTFC_R {
        RSTFC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - External PIN reset flag"]
    #[inline(always)]
    pub fn eprstf(&self) -> EPRSTF_R {
        EPRSTF_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Power reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn swrstf(&self) -> SWRSTF_R {
        SWRSTF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Free Watchdog timer reset flag"]
    #[inline(always)]
    pub fn fwdgtrstf(&self) -> FWDGTRSTF_R {
        FWDGTRSTF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Window watchdog timer reset flag"]
    #[inline(always)]
    pub fn wwdgtrstf(&self) -> WWDGTRSTF_R {
        WWDGTRSTF_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lprstf(&self) -> LPRSTF_R {
        LPRSTF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IRC40K enable"]
    #[inline(always)]
    pub fn irc40ken(&mut self) -> IRC40KEN_W {
        IRC40KEN_W { w: self }
    }
    #[doc = "Bit 24 - Reset flag clear"]
    #[inline(always)]
    pub fn rstfc(&mut self) -> RSTFC_W {
        RSTFC_W { w: self }
    }
}
