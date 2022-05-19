#[doc = "Reader of register RFIFOMI1"]
pub type R = crate::R<u32, super::RFIFOMI1>;
#[doc = "Reader of field `SFID_EFID`"]
pub type SFID_EFID_R = crate::R<u16, u16>;
#[doc = "Reader of field `EFID`"]
pub type EFID_R = crate::R<u32, u32>;
#[doc = "Reader of field `FF`"]
pub type FF_R = crate::R<bool, bool>;
#[doc = "Reader of field `FT`"]
pub type FT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 21:31 - The frame identifier"]
    #[inline(always)]
    pub fn sfid_efid(&self) -> SFID_EFID_R {
        SFID_EFID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
    #[doc = "Bits 3:20 - The frame identifier"]
    #[inline(always)]
    pub fn efid(&self) -> EFID_R {
        EFID_R::new(((self.bits >> 3) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 2 - Frame format"]
    #[inline(always)]
    pub fn ff(&self) -> FF_R {
        FF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Frame type"]
    #[inline(always)]
    pub fn ft(&self) -> FT_R {
        FT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
