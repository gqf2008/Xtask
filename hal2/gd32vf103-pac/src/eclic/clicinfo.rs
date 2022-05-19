#[doc = "Reader of register CLICINFO"]
pub type R = crate::R<u32, super::CLICINFO>;
#[doc = "Reader of field `NUM_INTERRUPT`"]
pub type NUM_INTERRUPT_R = crate::R<u16, u16>;
#[doc = "Reader of field `VERSION`"]
pub type VERSION_R = crate::R<u8, u8>;
#[doc = "Reader of field `CLICINTCTLBITS`"]
pub type CLICINTCTLBITS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:12 - NUM_INTERRUPT"]
    #[inline(always)]
    pub fn num_interrupt(&self) -> NUM_INTERRUPT_R {
        NUM_INTERRUPT_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:20 - VERSION"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bits 21:24 - CLICINTCTLBITS"]
    #[inline(always)]
    pub fn clicintctlbits(&self) -> CLICINTCTLBITS_R {
        CLICINTCTLBITS_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
}
