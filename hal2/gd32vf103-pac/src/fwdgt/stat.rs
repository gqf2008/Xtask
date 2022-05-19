#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Reader of field `PUD`"]
pub type PUD_R = crate::R<bool, bool>;
#[doc = "Reader of field `RUD`"]
pub type RUD_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Free watchdog timer prescaler value update"]
    #[inline(always)]
    pub fn pud(&self) -> PUD_R {
        PUD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Free watchdog timer counter reload value update"]
    #[inline(always)]
    pub fn rud(&self) -> RUD_R {
        RUD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
