#[doc = "Reader of register FMCFG"]
pub type R = crate::R<u32, super::FMCFG>;
#[doc = "Writer for register FMCFG"]
pub type W = crate::W<u32, super::FMCFG>;
#[doc = "Register FMCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::FMCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FMOD27`"]
pub type FMOD27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD27`"]
pub struct FMOD27_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD27_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `FMOD26`"]
pub type FMOD26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD26`"]
pub struct FMOD26_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD26_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `FMOD25`"]
pub type FMOD25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD25`"]
pub struct FMOD25_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD25_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `FMOD24`"]
pub type FMOD24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD24`"]
pub struct FMOD24_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD24_W<'a> {
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
#[doc = "Reader of field `FMOD23`"]
pub type FMOD23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD23`"]
pub struct FMOD23_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `FMOD22`"]
pub type FMOD22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD22`"]
pub struct FMOD22_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `FMOD21`"]
pub type FMOD21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD21`"]
pub struct FMOD21_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD21_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `FMOD20`"]
pub type FMOD20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD20`"]
pub struct FMOD20_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD20_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `FMOD19`"]
pub type FMOD19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD19`"]
pub struct FMOD19_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD19_W<'a> {
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
#[doc = "Reader of field `FMOD18`"]
pub type FMOD18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD18`"]
pub struct FMOD18_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD18_W<'a> {
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
#[doc = "Reader of field `FMOD17`"]
pub type FMOD17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD17`"]
pub struct FMOD17_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD17_W<'a> {
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
#[doc = "Reader of field `FMOD16`"]
pub type FMOD16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD16`"]
pub struct FMOD16_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD16_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `FMOD15`"]
pub type FMOD15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD15`"]
pub struct FMOD15_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `FMOD14`"]
pub type FMOD14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD14`"]
pub struct FMOD14_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `FMOD13`"]
pub type FMOD13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD13`"]
pub struct FMOD13_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `FMOD12`"]
pub type FMOD12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD12`"]
pub struct FMOD12_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD12_W<'a> {
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
#[doc = "Reader of field `FMOD11`"]
pub type FMOD11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD11`"]
pub struct FMOD11_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD11_W<'a> {
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
#[doc = "Reader of field `FMOD10`"]
pub type FMOD10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD10`"]
pub struct FMOD10_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD10_W<'a> {
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
#[doc = "Reader of field `FMOD9`"]
pub type FMOD9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD9`"]
pub struct FMOD9_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD9_W<'a> {
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
#[doc = "Reader of field `FMOD8`"]
pub type FMOD8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD8`"]
pub struct FMOD8_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD8_W<'a> {
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
#[doc = "Reader of field `FMOD7`"]
pub type FMOD7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD7`"]
pub struct FMOD7_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `FMOD6`"]
pub type FMOD6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD6`"]
pub struct FMOD6_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `FMOD5`"]
pub type FMOD5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD5`"]
pub struct FMOD5_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `FMOD4`"]
pub type FMOD4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD4`"]
pub struct FMOD4_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `FMOD3`"]
pub type FMOD3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD3`"]
pub struct FMOD3_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `FMOD2`"]
pub type FMOD2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD2`"]
pub struct FMOD2_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD2_W<'a> {
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
#[doc = "Reader of field `FMOD1`"]
pub type FMOD1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD1`"]
pub struct FMOD1_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD1_W<'a> {
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
#[doc = "Reader of field `FMOD0`"]
pub type FMOD0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FMOD0`"]
pub struct FMOD0_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD0_W<'a> {
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
impl R {
    #[doc = "Bit 27 - Filter mode"]
    #[inline(always)]
    pub fn fmod27(&self) -> FMOD27_R {
        FMOD27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Filter mode"]
    #[inline(always)]
    pub fn fmod26(&self) -> FMOD26_R {
        FMOD26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Filter mode"]
    #[inline(always)]
    pub fn fmod25(&self) -> FMOD25_R {
        FMOD25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Filter mode"]
    #[inline(always)]
    pub fn fmod24(&self) -> FMOD24_R {
        FMOD24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Filter mode"]
    #[inline(always)]
    pub fn fmod23(&self) -> FMOD23_R {
        FMOD23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Filter mode"]
    #[inline(always)]
    pub fn fmod22(&self) -> FMOD22_R {
        FMOD22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Filter mode"]
    #[inline(always)]
    pub fn fmod21(&self) -> FMOD21_R {
        FMOD21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Filter mode"]
    #[inline(always)]
    pub fn fmod20(&self) -> FMOD20_R {
        FMOD20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Filter mode"]
    #[inline(always)]
    pub fn fmod19(&self) -> FMOD19_R {
        FMOD19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Filter mode"]
    #[inline(always)]
    pub fn fmod18(&self) -> FMOD18_R {
        FMOD18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Filter mode"]
    #[inline(always)]
    pub fn fmod17(&self) -> FMOD17_R {
        FMOD17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Filter mode"]
    #[inline(always)]
    pub fn fmod16(&self) -> FMOD16_R {
        FMOD16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Filter mode"]
    #[inline(always)]
    pub fn fmod15(&self) -> FMOD15_R {
        FMOD15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Filter mode"]
    #[inline(always)]
    pub fn fmod14(&self) -> FMOD14_R {
        FMOD14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Filter mode"]
    #[inline(always)]
    pub fn fmod13(&self) -> FMOD13_R {
        FMOD13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Filter mode"]
    #[inline(always)]
    pub fn fmod12(&self) -> FMOD12_R {
        FMOD12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Filter mode"]
    #[inline(always)]
    pub fn fmod11(&self) -> FMOD11_R {
        FMOD11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Filter mode"]
    #[inline(always)]
    pub fn fmod10(&self) -> FMOD10_R {
        FMOD10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Filter mode"]
    #[inline(always)]
    pub fn fmod9(&self) -> FMOD9_R {
        FMOD9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Filter mode"]
    #[inline(always)]
    pub fn fmod8(&self) -> FMOD8_R {
        FMOD8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Filter mode"]
    #[inline(always)]
    pub fn fmod7(&self) -> FMOD7_R {
        FMOD7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Filter mode"]
    #[inline(always)]
    pub fn fmod6(&self) -> FMOD6_R {
        FMOD6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Filter mode"]
    #[inline(always)]
    pub fn fmod5(&self) -> FMOD5_R {
        FMOD5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Filter mode"]
    #[inline(always)]
    pub fn fmod4(&self) -> FMOD4_R {
        FMOD4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Filter mode"]
    #[inline(always)]
    pub fn fmod3(&self) -> FMOD3_R {
        FMOD3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Filter mode"]
    #[inline(always)]
    pub fn fmod2(&self) -> FMOD2_R {
        FMOD2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Filter mode"]
    #[inline(always)]
    pub fn fmod1(&self) -> FMOD1_R {
        FMOD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Filter mode"]
    #[inline(always)]
    pub fn fmod0(&self) -> FMOD0_R {
        FMOD0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - Filter mode"]
    #[inline(always)]
    pub fn fmod27(&mut self) -> FMOD27_W {
        FMOD27_W { w: self }
    }
    #[doc = "Bit 26 - Filter mode"]
    #[inline(always)]
    pub fn fmod26(&mut self) -> FMOD26_W {
        FMOD26_W { w: self }
    }
    #[doc = "Bit 25 - Filter mode"]
    #[inline(always)]
    pub fn fmod25(&mut self) -> FMOD25_W {
        FMOD25_W { w: self }
    }
    #[doc = "Bit 24 - Filter mode"]
    #[inline(always)]
    pub fn fmod24(&mut self) -> FMOD24_W {
        FMOD24_W { w: self }
    }
    #[doc = "Bit 23 - Filter mode"]
    #[inline(always)]
    pub fn fmod23(&mut self) -> FMOD23_W {
        FMOD23_W { w: self }
    }
    #[doc = "Bit 22 - Filter mode"]
    #[inline(always)]
    pub fn fmod22(&mut self) -> FMOD22_W {
        FMOD22_W { w: self }
    }
    #[doc = "Bit 21 - Filter mode"]
    #[inline(always)]
    pub fn fmod21(&mut self) -> FMOD21_W {
        FMOD21_W { w: self }
    }
    #[doc = "Bit 20 - Filter mode"]
    #[inline(always)]
    pub fn fmod20(&mut self) -> FMOD20_W {
        FMOD20_W { w: self }
    }
    #[doc = "Bit 19 - Filter mode"]
    #[inline(always)]
    pub fn fmod19(&mut self) -> FMOD19_W {
        FMOD19_W { w: self }
    }
    #[doc = "Bit 18 - Filter mode"]
    #[inline(always)]
    pub fn fmod18(&mut self) -> FMOD18_W {
        FMOD18_W { w: self }
    }
    #[doc = "Bit 17 - Filter mode"]
    #[inline(always)]
    pub fn fmod17(&mut self) -> FMOD17_W {
        FMOD17_W { w: self }
    }
    #[doc = "Bit 16 - Filter mode"]
    #[inline(always)]
    pub fn fmod16(&mut self) -> FMOD16_W {
        FMOD16_W { w: self }
    }
    #[doc = "Bit 15 - Filter mode"]
    #[inline(always)]
    pub fn fmod15(&mut self) -> FMOD15_W {
        FMOD15_W { w: self }
    }
    #[doc = "Bit 14 - Filter mode"]
    #[inline(always)]
    pub fn fmod14(&mut self) -> FMOD14_W {
        FMOD14_W { w: self }
    }
    #[doc = "Bit 13 - Filter mode"]
    #[inline(always)]
    pub fn fmod13(&mut self) -> FMOD13_W {
        FMOD13_W { w: self }
    }
    #[doc = "Bit 12 - Filter mode"]
    #[inline(always)]
    pub fn fmod12(&mut self) -> FMOD12_W {
        FMOD12_W { w: self }
    }
    #[doc = "Bit 11 - Filter mode"]
    #[inline(always)]
    pub fn fmod11(&mut self) -> FMOD11_W {
        FMOD11_W { w: self }
    }
    #[doc = "Bit 10 - Filter mode"]
    #[inline(always)]
    pub fn fmod10(&mut self) -> FMOD10_W {
        FMOD10_W { w: self }
    }
    #[doc = "Bit 9 - Filter mode"]
    #[inline(always)]
    pub fn fmod9(&mut self) -> FMOD9_W {
        FMOD9_W { w: self }
    }
    #[doc = "Bit 8 - Filter mode"]
    #[inline(always)]
    pub fn fmod8(&mut self) -> FMOD8_W {
        FMOD8_W { w: self }
    }
    #[doc = "Bit 7 - Filter mode"]
    #[inline(always)]
    pub fn fmod7(&mut self) -> FMOD7_W {
        FMOD7_W { w: self }
    }
    #[doc = "Bit 6 - Filter mode"]
    #[inline(always)]
    pub fn fmod6(&mut self) -> FMOD6_W {
        FMOD6_W { w: self }
    }
    #[doc = "Bit 5 - Filter mode"]
    #[inline(always)]
    pub fn fmod5(&mut self) -> FMOD5_W {
        FMOD5_W { w: self }
    }
    #[doc = "Bit 4 - Filter mode"]
    #[inline(always)]
    pub fn fmod4(&mut self) -> FMOD4_W {
        FMOD4_W { w: self }
    }
    #[doc = "Bit 3 - Filter mode"]
    #[inline(always)]
    pub fn fmod3(&mut self) -> FMOD3_W {
        FMOD3_W { w: self }
    }
    #[doc = "Bit 2 - Filter mode"]
    #[inline(always)]
    pub fn fmod2(&mut self) -> FMOD2_W {
        FMOD2_W { w: self }
    }
    #[doc = "Bit 1 - Filter mode"]
    #[inline(always)]
    pub fn fmod1(&mut self) -> FMOD1_W {
        FMOD1_W { w: self }
    }
    #[doc = "Bit 0 - Filter mode"]
    #[inline(always)]
    pub fn fmod0(&mut self) -> FMOD0_W {
        FMOD0_W { w: self }
    }
}
