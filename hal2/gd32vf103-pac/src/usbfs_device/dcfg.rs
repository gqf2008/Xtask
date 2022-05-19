#[doc = "Reader of register DCFG"]
pub type R = crate::R<u32, super::DCFG>;
#[doc = "Writer for register DCFG"]
pub type W = crate::W<u32, super::DCFG>;
#[doc = "Register DCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::DCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DS`"]
pub type DS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DS`"]
pub struct DS_W<'a> {
    w: &'a mut W,
}
impl<'a> DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `NZLSOH`"]
pub type NZLSOH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NZLSOH`"]
pub struct NZLSOH_W<'a> {
    w: &'a mut W,
}
impl<'a> NZLSOH_W<'a> {
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
#[doc = "Reader of field `DAR`"]
pub type DAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAR`"]
pub struct DAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 4)) | (((value as u32) & 0x7f) << 4);
        self.w
    }
}
#[doc = "Reader of field `EOPFT`"]
pub type EOPFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EOPFT`"]
pub struct EOPFT_W<'a> {
    w: &'a mut W,
}
impl<'a> EOPFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Device speed"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Non-zero-length status OUT handshake"]
    #[inline(always)]
    pub fn nzlsoh(&self) -> NZLSOH_R {
        NZLSOH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:10 - Device address"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - end of periodic frame time"]
    #[inline(always)]
    pub fn eopft(&self) -> EOPFT_R {
        EOPFT_R::new(((self.bits >> 11) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Device speed"]
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W {
        DS_W { w: self }
    }
    #[doc = "Bit 2 - Non-zero-length status OUT handshake"]
    #[inline(always)]
    pub fn nzlsoh(&mut self) -> NZLSOH_W {
        NZLSOH_W { w: self }
    }
    #[doc = "Bits 4:10 - Device address"]
    #[inline(always)]
    pub fn dar(&mut self) -> DAR_W {
        DAR_W { w: self }
    }
    #[doc = "Bits 11:12 - end of periodic frame time"]
    #[inline(always)]
    pub fn eopft(&mut self) -> EOPFT_W {
        EOPFT_W { w: self }
    }
}
