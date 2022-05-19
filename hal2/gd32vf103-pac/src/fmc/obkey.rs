#[doc = "Writer for register OBKEY"]
pub type W = crate::W<u32, super::OBKEY>;
#[doc = "Register OBKEY `reset()`'s with value 0"]
impl crate::ResetValue for super::OBKEY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `OBKEY`"]
pub struct OBKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> OBKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - FMC_ CTL0 option byte operation unlock register"]
    #[inline(always)]
    pub fn obkey(&mut self) -> OBKEY_W {
        OBKEY_W { w: self }
    }
}
