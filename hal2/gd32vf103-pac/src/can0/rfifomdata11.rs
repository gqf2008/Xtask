#[doc = "Reader of register RFIFOMDATA11"]
pub type R = crate::R<u32, super::RFIFOMDATA11>;
#[doc = "Reader of field `DB7`"]
pub type DB7_R = crate::R<u8, u8>;
#[doc = "Reader of field `DB6`"]
pub type DB6_R = crate::R<u8, u8>;
#[doc = "Reader of field `DB5`"]
pub type DB5_R = crate::R<u8, u8>;
#[doc = "Reader of field `DB4`"]
pub type DB4_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:31 - Data byte 7"]
    #[inline(always)]
    pub fn db7(&self) -> DB7_R {
        DB7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 6"]
    #[inline(always)]
    pub fn db6(&self) -> DB6_R {
        DB6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 5"]
    #[inline(always)]
    pub fn db5(&self) -> DB5_R {
        DB5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Data byte 4"]
    #[inline(always)]
    pub fn db4(&self) -> DB4_R {
        DB4_R::new((self.bits & 0xff) as u8)
    }
}
