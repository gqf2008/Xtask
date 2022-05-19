#[doc = "Reader of register TPCTL"]
pub type R = crate::R<u16, super::TPCTL>;
#[doc = "Writer for register TPCTL"]
pub type W = crate::W<u16, super::TPCTL>;
#[doc = "Register TPCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::TPCTL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TPAL`"]
pub type TPAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TPAL`"]
pub struct TPAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TPAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u16) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TPEN`"]
pub type TPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TPEN`"]
pub struct TPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u16) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - TAMPER pin active level"]
    #[inline(always)]
    pub fn tpal(&self) -> TPAL_R {
        TPAL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - TAMPER detection enable"]
    #[inline(always)]
    pub fn tpen(&self) -> TPEN_R {
        TPEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TAMPER pin active level"]
    #[inline(always)]
    pub fn tpal(&mut self) -> TPAL_W {
        TPAL_W { w: self }
    }
    #[doc = "Bit 0 - TAMPER detection enable"]
    #[inline(always)]
    pub fn tpen(&mut self) -> TPEN_W {
        TPEN_W { w: self }
    }
}
