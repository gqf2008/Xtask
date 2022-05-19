#[doc = "Reader of register INT"]
pub type R = crate::R<u32, super::INT>;
#[doc = "Writer for register INT"]
pub type W = crate::W<u32, super::INT>;
#[doc = "Register INT `reset()`'s with value 0"]
impl crate::ResetValue for super::INT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IRC40KSTBIF`"]
pub type IRC40KSTBIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `LXTALSTBIF`"]
pub type LXTALSTBIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `IRC8MSTBIF`"]
pub type IRC8MSTBIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `HXTALSTBIF`"]
pub type HXTALSTBIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLLSTBIF`"]
pub type PLLSTBIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLL1STBIF`"]
pub type PLL1STBIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PLL2STBIF`"]
pub type PLL2STBIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CKMIF`"]
pub type CKMIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `IRC40KSTBIE`"]
pub type IRC40KSTBIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRC40KSTBIE`"]
pub struct IRC40KSTBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IRC40KSTBIE_W<'a> {
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
#[doc = "Reader of field `LXTALSTBIE`"]
pub type LXTALSTBIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LXTALSTBIE`"]
pub struct LXTALSTBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LXTALSTBIE_W<'a> {
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
#[doc = "Reader of field `IRC8MSTBIE`"]
pub type IRC8MSTBIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRC8MSTBIE`"]
pub struct IRC8MSTBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IRC8MSTBIE_W<'a> {
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
#[doc = "Reader of field `HXTALSTBIE`"]
pub type HXTALSTBIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HXTALSTBIE`"]
pub struct HXTALSTBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HXTALSTBIE_W<'a> {
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
#[doc = "Reader of field `PLLSTBIE`"]
pub type PLLSTBIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLSTBIE`"]
pub struct PLLSTBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSTBIE_W<'a> {
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
#[doc = "Reader of field `PLL1STBIE`"]
pub type PLL1STBIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL1STBIE`"]
pub struct PLL1STBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL1STBIE_W<'a> {
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
#[doc = "Reader of field `PLL2STBIE`"]
pub type PLL2STBIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLL2STBIE`"]
pub struct PLL2STBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL2STBIE_W<'a> {
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
#[doc = "Write proxy for field `IRC40KSTBIC`"]
pub struct IRC40KSTBIC_W<'a> {
    w: &'a mut W,
}
impl<'a> IRC40KSTBIC_W<'a> {
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
#[doc = "Write proxy for field `LXTALSTBIC`"]
pub struct LXTALSTBIC_W<'a> {
    w: &'a mut W,
}
impl<'a> LXTALSTBIC_W<'a> {
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
#[doc = "Write proxy for field `IRC8MSTBIC`"]
pub struct IRC8MSTBIC_W<'a> {
    w: &'a mut W,
}
impl<'a> IRC8MSTBIC_W<'a> {
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
#[doc = "Write proxy for field `HXTALSTBIC`"]
pub struct HXTALSTBIC_W<'a> {
    w: &'a mut W,
}
impl<'a> HXTALSTBIC_W<'a> {
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
#[doc = "Write proxy for field `PLLSTBIC`"]
pub struct PLLSTBIC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSTBIC_W<'a> {
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
#[doc = "Write proxy for field `PLL1STBIC`"]
pub struct PLL1STBIC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL1STBIC_W<'a> {
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
#[doc = "Write proxy for field `PLL2STBIC`"]
pub struct PLL2STBIC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL2STBIC_W<'a> {
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
#[doc = "Write proxy for field `CKMIC`"]
pub struct CKMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> CKMIC_W<'a> {
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
impl R {
    #[doc = "Bit 0 - IRC40K stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc40kstbif(&self) -> IRC40KSTBIF_R {
        IRC40KSTBIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LXTAL stabilization interrupt flag"]
    #[inline(always)]
    pub fn lxtalstbif(&self) -> LXTALSTBIF_R {
        LXTALSTBIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IRC8M stabilization interrupt flag"]
    #[inline(always)]
    pub fn irc8mstbif(&self) -> IRC8MSTBIF_R {
        IRC8MSTBIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HXTAL stabilization interrupt flag"]
    #[inline(always)]
    pub fn hxtalstbif(&self) -> HXTALSTBIF_R {
        HXTALSTBIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PLL stabilization interrupt flag"]
    #[inline(always)]
    pub fn pllstbif(&self) -> PLLSTBIF_R {
        PLLSTBIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PLL1 stabilization interrupt flag"]
    #[inline(always)]
    pub fn pll1stbif(&self) -> PLL1STBIF_R {
        PLL1STBIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLL2 stabilization interrupt flag"]
    #[inline(always)]
    pub fn pll2stbif(&self) -> PLL2STBIF_R {
        PLL2STBIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - HXTAL Clock Stuck Interrupt Flag"]
    #[inline(always)]
    pub fn ckmif(&self) -> CKMIF_R {
        CKMIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - IRC40K Stabilization interrupt enable"]
    #[inline(always)]
    pub fn irc40kstbie(&self) -> IRC40KSTBIE_R {
        IRC40KSTBIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn lxtalstbie(&self) -> LXTALSTBIE_R {
        LXTALSTBIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - IRC8M Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn irc8mstbie(&self) -> IRC8MSTBIE_R {
        IRC8MSTBIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn hxtalstbie(&self) -> HXTALSTBIE_R {
        HXTALSTBIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PLL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pllstbie(&self) -> PLLSTBIE_R {
        PLLSTBIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PLL1 Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pll1stbie(&self) -> PLL1STBIE_R {
        PLL1STBIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PLL2 Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pll2stbie(&self) -> PLL2STBIE_R {
        PLL2STBIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - IRC40K Stabilization interrupt enable"]
    #[inline(always)]
    pub fn irc40kstbie(&mut self) -> IRC40KSTBIE_W {
        IRC40KSTBIE_W { w: self }
    }
    #[doc = "Bit 9 - LXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn lxtalstbie(&mut self) -> LXTALSTBIE_W {
        LXTALSTBIE_W { w: self }
    }
    #[doc = "Bit 10 - IRC8M Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn irc8mstbie(&mut self) -> IRC8MSTBIE_W {
        IRC8MSTBIE_W { w: self }
    }
    #[doc = "Bit 11 - HXTAL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn hxtalstbie(&mut self) -> HXTALSTBIE_W {
        HXTALSTBIE_W { w: self }
    }
    #[doc = "Bit 12 - PLL Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pllstbie(&mut self) -> PLLSTBIE_W {
        PLLSTBIE_W { w: self }
    }
    #[doc = "Bit 13 - PLL1 Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pll1stbie(&mut self) -> PLL1STBIE_W {
        PLL1STBIE_W { w: self }
    }
    #[doc = "Bit 14 - PLL2 Stabilization Interrupt Enable"]
    #[inline(always)]
    pub fn pll2stbie(&mut self) -> PLL2STBIE_W {
        PLL2STBIE_W { w: self }
    }
    #[doc = "Bit 16 - IRC40K Stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn irc40kstbic(&mut self) -> IRC40KSTBIC_W {
        IRC40KSTBIC_W { w: self }
    }
    #[doc = "Bit 17 - LXTAL Stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn lxtalstbic(&mut self) -> LXTALSTBIC_W {
        LXTALSTBIC_W { w: self }
    }
    #[doc = "Bit 18 - IRC8M Stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn irc8mstbic(&mut self) -> IRC8MSTBIC_W {
        IRC8MSTBIC_W { w: self }
    }
    #[doc = "Bit 19 - HXTAL Stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn hxtalstbic(&mut self) -> HXTALSTBIC_W {
        HXTALSTBIC_W { w: self }
    }
    #[doc = "Bit 20 - PLL stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn pllstbic(&mut self) -> PLLSTBIC_W {
        PLLSTBIC_W { w: self }
    }
    #[doc = "Bit 21 - PLL1 stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn pll1stbic(&mut self) -> PLL1STBIC_W {
        PLL1STBIC_W { w: self }
    }
    #[doc = "Bit 22 - PLL2 stabilization Interrupt Clear"]
    #[inline(always)]
    pub fn pll2stbic(&mut self) -> PLL2STBIC_W {
        PLL2STBIC_W { w: self }
    }
    #[doc = "Bit 23 - HXTAL Clock Stuck Interrupt Clear"]
    #[inline(always)]
    pub fn ckmic(&mut self) -> CKMIC_W {
        CKMIC_W { w: self }
    }
}
