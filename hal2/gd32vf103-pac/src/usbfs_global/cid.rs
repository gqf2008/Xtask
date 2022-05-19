#[doc = "Reader of register CID"]
pub type R = crate::R<u32, super::CID>;
#[doc = "Writer for register CID"]
pub type W = crate::W<u32, super::CID>;
#[doc = "Register CID `reset()`'s with value 0x1000"]
impl crate::ResetValue for super::CID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000
    }
}
#[doc = "Reader of field `CID`"]
pub type CID_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CID`"]
pub struct CID_W<'a> {
    w: &'a mut W,
}
impl<'a> CID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Core ID"]
    #[inline(always)]
    pub fn cid(&self) -> CID_R {
        CID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Core ID"]
    #[inline(always)]
    pub fn cid(&mut self) -> CID_W {
        CID_W { w: self }
    }
}
