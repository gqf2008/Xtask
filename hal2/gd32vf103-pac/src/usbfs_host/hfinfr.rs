#[doc = "Reader of register HFINFR"]
pub type R = crate::R<u32, super::HFINFR>;
#[doc = "Reader of field `FRNUM`"]
pub type FRNUM_R = crate::R<u16, u16>;
#[doc = "Reader of field `FRT`"]
pub type FRT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Frame number"]
    #[inline(always)]
    pub fn frnum(&self) -> FRNUM_R {
        FRNUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Frame remaining time"]
    #[inline(always)]
    pub fn frt(&self) -> FRT_R {
        FRT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
