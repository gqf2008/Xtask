#[doc = "Reader of register GP"]
pub type R = crate::R<u32, super::GP>;
#[doc = "Writer for register GP"]
pub type W = crate::W<u32, super::GP>;
#[doc = "Register GP `reset()`'s with value 0"]
impl crate::ResetValue for super::GP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GUAT`"]
pub type GUAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GUAT`"]
pub struct GUAT_W<'a> {
    w: &'a mut W,
}
impl<'a> GUAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `PSC`"]
pub type PSC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PSC`"]
pub struct PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Guard time value in Smartcard mode"]
    #[inline(always)]
    pub fn guat(&self) -> GUAT_R {
        GUAT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Prescaler value"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Guard time value in Smartcard mode"]
    #[inline(always)]
    pub fn guat(&mut self) -> GUAT_W {
        GUAT_W { w: self }
    }
    #[doc = "Bits 0:7 - Prescaler value"]
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W {
        PSC_W { w: self }
    }
}
