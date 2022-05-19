#[doc = "Reader of register DAEPINT"]
pub type R = crate::R<u32, super::DAEPINT>;
#[doc = "Reader of field `IEPITB`"]
pub type IEPITB_R = crate::R<u8, u8>;
#[doc = "Reader of field `OEPITB`"]
pub type OEPITB_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Device all IN endpoint interrupt bits"]
    #[inline(always)]
    pub fn iepitb(&self) -> IEPITB_R {
        IEPITB_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Device all OUT endpoint interrupt bits"]
    #[inline(always)]
    pub fn oepitb(&self) -> OEPITB_R {
        OEPITB_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
