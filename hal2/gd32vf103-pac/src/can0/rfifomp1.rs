#[doc = "Reader of register RFIFOMP1"]
pub type R = crate::R<u32, super::RFIFOMP1>;
#[doc = "Reader of field `TS`"]
pub type TS_R = crate::R<u16, u16>;
#[doc = "Reader of field `FI`"]
pub type FI_R = crate::R<u8, u8>;
#[doc = "Reader of field `DLENC`"]
pub type DLENC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 16:31 - Time stamp"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - Filtering index"]
    #[inline(always)]
    pub fn fi(&self) -> FI_R {
        FI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:3 - Data length code"]
    #[inline(always)]
    pub fn dlenc(&self) -> DLENC_R {
        DLENC_R::new((self.bits & 0x0f) as u8)
    }
}
