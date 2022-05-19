#[doc = "Writer for register ALRML"]
pub type W = crate::W<u32, super::ALRML>;
#[doc = "Register ALRML `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::ALRML {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Write proxy for field `ALRM`"]
pub struct ALRM_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - alarm value low"]
    #[inline(always)]
    pub fn alrm(&mut self) -> ALRM_W {
        ALRM_W { w: self }
    }
}
