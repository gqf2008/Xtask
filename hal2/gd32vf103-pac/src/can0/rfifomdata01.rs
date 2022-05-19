#[doc = "Reader of register RFIFOMDATA01"]
pub type R = crate::R<u32, super::RFIFOMDATA01>;
#[doc = "Reader of field `DB3`"]
pub type DB3_R = crate::R<u8, u8>;
#[doc = "Reader of field `DB2`"]
pub type DB2_R = crate::R<u8, u8>;
#[doc = "Reader of field `DB1`"]
pub type DB1_R = crate::R<u8, u8>;
#[doc = "Reader of field `DB0`"]
pub type DB0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:31 - Data byte 3"]
    #[inline(always)]
    pub fn db3(&self) -> DB3_R {
        DB3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data byte 2"]
    #[inline(always)]
    pub fn db2(&self) -> DB2_R {
        DB2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data byte 1"]
    #[inline(always)]
    pub fn db1(&self) -> DB1_R {
        DB1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Data byte 0"]
    #[inline(always)]
    pub fn db0(&self) -> DB0_R {
        DB0_R::new((self.bits & 0xff) as u8)
    }
}
