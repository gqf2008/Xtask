#[doc = "Reader of register FMPCFG"]
pub type R = crate::R<u16, super::FMPCFG>;
#[doc = "Writer for register FMPCFG"]
pub type W = crate::W<u16, super::FMPCFG>;
#[doc = "Register FMPCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::FMPCFG {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FMPEN`"]
pub type FMPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMPEN`"]
pub struct FMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FMPEN_W<'a> {
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
    #[doc = "Bit 0 - Fast mode plus enable"]
    #[inline(always)]
    pub fn fmpen(&self) -> FMPEN_R {
        FMPEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast mode plus enable"]
    #[inline(always)]
    pub fn fmpen(&mut self) -> FMPEN_W {
        FMPEN_W { w: self }
    }
}
