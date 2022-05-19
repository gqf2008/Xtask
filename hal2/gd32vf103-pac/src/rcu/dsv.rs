#[doc = "Reader of register DSV"]
pub type R = crate::R<u32, super::DSV>;
#[doc = "Writer for register DSV"]
pub type W = crate::W<u32, super::DSV>;
#[doc = "Register DSV `reset()`'s with value 0"]
impl crate::ResetValue for super::DSV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DSLPVS`"]
pub type DSLPVS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DSLPVS`"]
pub struct DSLPVS_W<'a> {
    w: &'a mut W,
}
impl<'a> DSLPVS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Deep-sleep mode voltage select"]
    #[inline(always)]
    pub fn dslpvs(&self) -> DSLPVS_R {
        DSLPVS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Deep-sleep mode voltage select"]
    #[inline(always)]
    pub fn dslpvs(&mut self) -> DSLPVS_W {
        DSLPVS_W { w: self }
    }
}
