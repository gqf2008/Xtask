#[doc = "Reader of register CH5CTL"]
pub type R = crate::R<u32, super::CH5CTL>;
#[doc = "Writer for register CH5CTL"]
pub type W = crate::W<u32, super::CH5CTL>;
#[doc = "Register CH5CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CH5CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHEN`"]
pub type CHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHEN`"]
pub struct CHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN_W<'a> {
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
#[doc = "Reader of field `FTFIE`"]
pub type FTFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTFIE`"]
pub struct FTFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FTFIE_W<'a> {
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
#[doc = "Reader of field `HTFIE`"]
pub type HTFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HTFIE`"]
pub struct HTFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HTFIE_W<'a> {
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
#[doc = "Reader of field `ERRIE`"]
pub type ERRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ERRIE`"]
pub struct ERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIE_W<'a> {
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
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR`"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
#[doc = "Reader of field `CMEN`"]
pub type CMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMEN`"]
pub struct CMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMEN_W<'a> {
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
#[doc = "Reader of field `PNAGA`"]
pub type PNAGA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PNAGA`"]
pub struct PNAGA_W<'a> {
    w: &'a mut W,
}
impl<'a> PNAGA_W<'a> {
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
#[doc = "Reader of field `MNAGA`"]
pub type MNAGA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MNAGA`"]
pub struct MNAGA_W<'a> {
    w: &'a mut W,
}
impl<'a> MNAGA_W<'a> {
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
#[doc = "Reader of field `PWIDTH`"]
pub type PWIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PWIDTH`"]
pub struct PWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> PWIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `MWIDTH`"]
pub type MWIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MWIDTH`"]
pub struct MWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> MWIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `PRIO`"]
pub type PRIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRIO`"]
pub struct PRIO_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `M2M`"]
pub type M2M_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `M2M`"]
pub struct M2M_W<'a> {
    w: &'a mut W,
}
impl<'a> M2M_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable bit for channel full transfer finish interrupt"]
    #[inline(always)]
    pub fn ftfie(&self) -> FTFIE_R {
        FTFIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable bit for channel half transfer finish interrupt"]
    #[inline(always)]
    pub fn htfie(&self) -> HTFIE_R {
        HTFIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable bit for channel error interrupt"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transfer direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Circular mode enable"]
    #[inline(always)]
    pub fn cmen(&self) -> CMEN_R {
        CMEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Next address generation algorithm of peripheral"]
    #[inline(always)]
    pub fn pnaga(&self) -> PNAGA_R {
        PNAGA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Next address generation algorithm of memory"]
    #[inline(always)]
    pub fn mnaga(&self) -> MNAGA_R {
        MNAGA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Transfer data size of peripheral"]
    #[inline(always)]
    pub fn pwidth(&self) -> PWIDTH_R {
        PWIDTH_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Transfer data size of memory"]
    #[inline(always)]
    pub fn mwidth(&self) -> MWIDTH_R {
        MWIDTH_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Priority level"]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - Memory to Memory Mode"]
    #[inline(always)]
    pub fn m2m(&self) -> M2M_R {
        M2M_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    pub fn chen(&mut self) -> CHEN_W {
        CHEN_W { w: self }
    }
    #[doc = "Bit 1 - Enable bit for channel full transfer finish interrupt"]
    #[inline(always)]
    pub fn ftfie(&mut self) -> FTFIE_W {
        FTFIE_W { w: self }
    }
    #[doc = "Bit 2 - Enable bit for channel half transfer finish interrupt"]
    #[inline(always)]
    pub fn htfie(&mut self) -> HTFIE_W {
        HTFIE_W { w: self }
    }
    #[doc = "Bit 3 - Enable bit for channel error interrupt"]
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W {
        ERRIE_W { w: self }
    }
    #[doc = "Bit 4 - Transfer direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 5 - Circular mode enable"]
    #[inline(always)]
    pub fn cmen(&mut self) -> CMEN_W {
        CMEN_W { w: self }
    }
    #[doc = "Bit 6 - Next address generation algorithm of peripheral"]
    #[inline(always)]
    pub fn pnaga(&mut self) -> PNAGA_W {
        PNAGA_W { w: self }
    }
    #[doc = "Bit 7 - Next address generation algorithm of memory"]
    #[inline(always)]
    pub fn mnaga(&mut self) -> MNAGA_W {
        MNAGA_W { w: self }
    }
    #[doc = "Bits 8:9 - Transfer data size of peripheral"]
    #[inline(always)]
    pub fn pwidth(&mut self) -> PWIDTH_W {
        PWIDTH_W { w: self }
    }
    #[doc = "Bits 10:11 - Transfer data size of memory"]
    #[inline(always)]
    pub fn mwidth(&mut self) -> MWIDTH_W {
        MWIDTH_W { w: self }
    }
    #[doc = "Bits 12:13 - Priority level"]
    #[inline(always)]
    pub fn prio(&mut self) -> PRIO_W {
        PRIO_W { w: self }
    }
    #[doc = "Bit 14 - Memory to Memory Mode"]
    #[inline(always)]
    pub fn m2m(&mut self) -> M2M_W {
        M2M_W { w: self }
    }
}
