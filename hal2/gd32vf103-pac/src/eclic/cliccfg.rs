#[doc = "Reader of register CLICCFG"]
pub type R = crate::R<u8, super::CLICCFG>;
#[doc = "Writer for register CLICCFG"]
pub type W = crate::W<u8, super::CLICCFG>;
#[doc = "Register CLICCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CLICCFG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NLBITS`"]
pub type NLBITS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NLBITS`"]
pub struct NLBITS_W<'a> {
    w: &'a mut W,
}
impl<'a> NLBITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u8) & 0x0f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:4 - NLBITS"]
    #[inline(always)]
    pub fn nlbits(&self) -> NLBITS_R {
        NLBITS_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:4 - NLBITS"]
    #[inline(always)]
    pub fn nlbits(&mut self) -> NLBITS_W {
        NLBITS_W { w: self }
    }
}
