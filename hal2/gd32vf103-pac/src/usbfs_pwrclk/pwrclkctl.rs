#[doc = "Reader of register PWRCLKCTL"]
pub type R = crate::R<u32, super::PWRCLKCTL>;
#[doc = "Writer for register PWRCLKCTL"]
pub type W = crate::W<u32, super::PWRCLKCTL>;
#[doc = "Register PWRCLKCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::PWRCLKCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SUCLK`"]
pub type SUCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUCLK`"]
pub struct SUCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SUCLK_W<'a> {
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
#[doc = "Reader of field `SHCLK`"]
pub type SHCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SHCLK`"]
pub struct SHCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SHCLK_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Stop the USB clock"]
    #[inline(always)]
    pub fn suclk(&self) -> SUCLK_R {
        SUCLK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Stop HCLK"]
    #[inline(always)]
    pub fn shclk(&self) -> SHCLK_R {
        SHCLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop the USB clock"]
    #[inline(always)]
    pub fn suclk(&mut self) -> SUCLK_W {
        SUCLK_W { w: self }
    }
    #[doc = "Bit 1 - Stop HCLK"]
    #[inline(always)]
    pub fn shclk(&mut self) -> SHCLK_W {
        SHCLK_W { w: self }
    }
}
