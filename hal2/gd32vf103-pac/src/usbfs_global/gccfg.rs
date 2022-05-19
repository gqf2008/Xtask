#[doc = "Reader of register GCCFG"]
pub type R = crate::R<u32, super::GCCFG>;
#[doc = "Writer for register GCCFG"]
pub type W = crate::W<u32, super::GCCFG>;
#[doc = "Register GCCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::GCCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWRON`"]
pub type PWRON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRON`"]
pub struct PWRON_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRON_W<'a> {
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
#[doc = "Reader of field `VBUSACEN`"]
pub type VBUSACEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUSACEN`"]
pub struct VBUSACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSACEN_W<'a> {
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
#[doc = "Reader of field `VBUSBCEN`"]
pub type VBUSBCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUSBCEN`"]
pub struct VBUSBCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSBCEN_W<'a> {
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
#[doc = "Reader of field `SOFOEN`"]
pub type SOFOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFOEN`"]
pub struct SOFOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFOEN_W<'a> {
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
#[doc = "Reader of field `VBUSIG`"]
pub type VBUSIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUSIG`"]
pub struct VBUSIG_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSIG_W<'a> {
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
impl R {
    #[doc = "Bit 16 - Power on"]
    #[inline(always)]
    pub fn pwron(&self) -> PWRON_R {
        PWRON_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - The VBUS A-device Comparer enable"]
    #[inline(always)]
    pub fn vbusacen(&self) -> VBUSACEN_R {
        VBUSACEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - The VBUS B-device Comparer enable"]
    #[inline(always)]
    pub fn vbusbcen(&self) -> VBUSBCEN_R {
        VBUSBCEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - SOF output enable"]
    #[inline(always)]
    pub fn sofoen(&self) -> SOFOEN_R {
        SOFOEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - VBUS ignored"]
    #[inline(always)]
    pub fn vbusig(&self) -> VBUSIG_R {
        VBUSIG_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Power on"]
    #[inline(always)]
    pub fn pwron(&mut self) -> PWRON_W {
        PWRON_W { w: self }
    }
    #[doc = "Bit 18 - The VBUS A-device Comparer enable"]
    #[inline(always)]
    pub fn vbusacen(&mut self) -> VBUSACEN_W {
        VBUSACEN_W { w: self }
    }
    #[doc = "Bit 19 - The VBUS B-device Comparer enable"]
    #[inline(always)]
    pub fn vbusbcen(&mut self) -> VBUSBCEN_W {
        VBUSBCEN_W { w: self }
    }
    #[doc = "Bit 20 - SOF output enable"]
    #[inline(always)]
    pub fn sofoen(&mut self) -> SOFOEN_W {
        SOFOEN_W { w: self }
    }
    #[doc = "Bit 21 - VBUS ignored"]
    #[inline(always)]
    pub fn vbusig(&mut self) -> VBUSIG_W {
        VBUSIG_W { w: self }
    }
}
