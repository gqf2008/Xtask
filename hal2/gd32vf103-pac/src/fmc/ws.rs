#[doc = "Reader of register WS"]
pub type R = crate::R<u32, super::WS>;
#[doc = "Writer for register WS"]
pub type W = crate::W<u32, super::WS>;
#[doc = "Register WS `reset()`'s with value 0"]
impl crate::ResetValue for super::WS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WSCNT`"]
pub type WSCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WSCNT`"]
pub struct WSCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> WSCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - wait state counter register"]
    #[inline(always)]
    pub fn wscnt(&self) -> WSCNT_R {
        WSCNT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - wait state counter register"]
    #[inline(always)]
    pub fn wscnt(&mut self) -> WSCNT_W {
        WSCNT_W { w: self }
    }
}
