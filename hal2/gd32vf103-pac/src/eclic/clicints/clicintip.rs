#[doc = "Reader of register CLICINTIP"]
pub type R = crate::R<u8, super::CLICINTIP>;
#[doc = "Writer for register CLICINTIP"]
pub type W = crate::W<u8, super::CLICINTIP>;
#[doc = "Register CLICINTIP `reset()`'s with value 0"]
impl crate::ResetValue for super::CLICINTIP {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IP`"]
pub type IP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IP`"]
pub struct IP_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - IP"]
    #[inline(always)]
    pub fn ip(&self) -> IP_R {
        IP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IP"]
    #[inline(always)]
    pub fn ip(&mut self) -> IP_W {
        IP_W { w: self }
    }
}
