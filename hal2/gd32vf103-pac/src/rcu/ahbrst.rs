#[doc = "Reader of register AHBRST"]
pub type R = crate::R<u32, super::AHBRST>;
#[doc = "Writer for register AHBRST"]
pub type W = crate::W<u32, super::AHBRST>;
#[doc = "Register AHBRST `reset()`'s with value 0"]
impl crate::ResetValue for super::AHBRST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USBFSRST`"]
pub type USBFSRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBFSRST`"]
pub struct USBFSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBFSRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 12 - USBFS reset"]
    #[inline(always)]
    pub fn usbfsrst(&self) -> USBFSRST_R {
        USBFSRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - USBFS reset"]
    #[inline(always)]
    pub fn usbfsrst(&mut self) -> USBFSRST_W {
        USBFSRST_W { w: self }
    }
}
