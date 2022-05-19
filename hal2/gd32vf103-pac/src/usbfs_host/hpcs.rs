#[doc = "Reader of register HPCS"]
pub type R = crate::R<u32, super::HPCS>;
#[doc = "Writer for register HPCS"]
pub type W = crate::W<u32, super::HPCS>;
#[doc = "Register HPCS `reset()`'s with value 0"]
impl crate::ResetValue for super::HPCS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCST`"]
pub type PCST_R = crate::R<bool, bool>;
#[doc = "Reader of field `PCD`"]
pub type PCD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCD`"]
pub struct PCD_W<'a> {
    w: &'a mut W,
}
impl<'a> PCD_W<'a> {
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
#[doc = "Reader of field `PE`"]
pub type PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PE`"]
pub struct PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE_W<'a> {
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
#[doc = "Reader of field `PEDC`"]
pub type PEDC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEDC`"]
pub struct PEDC_W<'a> {
    w: &'a mut W,
}
impl<'a> PEDC_W<'a> {
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
#[doc = "Reader of field `PREM`"]
pub type PREM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PREM`"]
pub struct PREM_W<'a> {
    w: &'a mut W,
}
impl<'a> PREM_W<'a> {
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
#[doc = "Reader of field `PSP`"]
pub type PSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PSP`"]
pub struct PSP_W<'a> {
    w: &'a mut W,
}
impl<'a> PSP_W<'a> {
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
#[doc = "Reader of field `PRST`"]
pub type PRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRST`"]
pub struct PRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PRST_W<'a> {
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
#[doc = "Reader of field `PLST`"]
pub type PLST_R = crate::R<u8, u8>;
#[doc = "Reader of field `PP`"]
pub type PP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PP`"]
pub struct PP_W<'a> {
    w: &'a mut W,
}
impl<'a> PP_W<'a> {
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
#[doc = "Reader of field `PS`"]
pub type PS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Port connect status"]
    #[inline(always)]
    pub fn pcst(&self) -> PCST_R {
        PCST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port connect detected"]
    #[inline(always)]
    pub fn pcd(&self) -> PCD_R {
        PCD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port enable/disable change"]
    #[inline(always)]
    pub fn pedc(&self) -> PEDC_R {
        PEDC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port resume"]
    #[inline(always)]
    pub fn prem(&self) -> PREM_R {
        PREM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port suspend"]
    #[inline(always)]
    pub fn psp(&self) -> PSP_R {
        PSP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port reset"]
    #[inline(always)]
    pub fn prst(&self) -> PRST_R {
        PRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Port line status"]
    #[inline(always)]
    pub fn plst(&self) -> PLST_R {
        PLST_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Port power"]
    #[inline(always)]
    pub fn pp(&self) -> PP_R {
        PP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - Port speed"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 17) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Port connect detected"]
    #[inline(always)]
    pub fn pcd(&mut self) -> PCD_W {
        PCD_W { w: self }
    }
    #[doc = "Bit 2 - Port enable"]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W {
        PE_W { w: self }
    }
    #[doc = "Bit 3 - Port enable/disable change"]
    #[inline(always)]
    pub fn pedc(&mut self) -> PEDC_W {
        PEDC_W { w: self }
    }
    #[doc = "Bit 6 - Port resume"]
    #[inline(always)]
    pub fn prem(&mut self) -> PREM_W {
        PREM_W { w: self }
    }
    #[doc = "Bit 7 - Port suspend"]
    #[inline(always)]
    pub fn psp(&mut self) -> PSP_W {
        PSP_W { w: self }
    }
    #[doc = "Bit 8 - Port reset"]
    #[inline(always)]
    pub fn prst(&mut self) -> PRST_W {
        PRST_W { w: self }
    }
    #[doc = "Bit 12 - Port power"]
    #[inline(always)]
    pub fn pp(&mut self) -> PP_W {
        PP_W { w: self }
    }
}
