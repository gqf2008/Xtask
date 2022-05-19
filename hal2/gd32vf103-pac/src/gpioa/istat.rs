#[doc = "Reader of register ISTAT"]
pub type R = crate::R<u32, super::ISTAT>;
#[doc = "Reader of field `ISTAT15`"]
pub type ISTAT15_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISTAT14`"]
pub type ISTAT14_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISTAT13`"]
pub type ISTAT13_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISTAT12`"]
pub type ISTAT12_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISTAT11`"]
pub type ISTAT11_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISTAT10`"]
pub type ISTAT10_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISTAT9`"]
pub type ISTAT9_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISTAT8`"]
pub type ISTAT8_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISTAT7`"]
pub type ISTAT7_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISTAT6`"]
pub type ISTAT6_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISTAT5`"]
pub type ISTAT5_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISTAT4`"]
pub type ISTAT4_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISTAT3`"]
pub type ISTAT3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISTAT2`"]
pub type ISTAT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISTAT1`"]
pub type ISTAT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ISTAT0`"]
pub type ISTAT0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 15 - Port input status"]
    #[inline(always)]
    pub fn istat15(&self) -> ISTAT15_R {
        ISTAT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port input status"]
    #[inline(always)]
    pub fn istat14(&self) -> ISTAT14_R {
        ISTAT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port input status"]
    #[inline(always)]
    pub fn istat13(&self) -> ISTAT13_R {
        ISTAT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port input status"]
    #[inline(always)]
    pub fn istat12(&self) -> ISTAT12_R {
        ISTAT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port input status"]
    #[inline(always)]
    pub fn istat11(&self) -> ISTAT11_R {
        ISTAT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port input status"]
    #[inline(always)]
    pub fn istat10(&self) -> ISTAT10_R {
        ISTAT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port input status"]
    #[inline(always)]
    pub fn istat9(&self) -> ISTAT9_R {
        ISTAT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port input status"]
    #[inline(always)]
    pub fn istat8(&self) -> ISTAT8_R {
        ISTAT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port input status"]
    #[inline(always)]
    pub fn istat7(&self) -> ISTAT7_R {
        ISTAT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port input status"]
    #[inline(always)]
    pub fn istat6(&self) -> ISTAT6_R {
        ISTAT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port input status"]
    #[inline(always)]
    pub fn istat5(&self) -> ISTAT5_R {
        ISTAT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port input status"]
    #[inline(always)]
    pub fn istat4(&self) -> ISTAT4_R {
        ISTAT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port input status"]
    #[inline(always)]
    pub fn istat3(&self) -> ISTAT3_R {
        ISTAT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port input status"]
    #[inline(always)]
    pub fn istat2(&self) -> ISTAT2_R {
        ISTAT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port input status"]
    #[inline(always)]
    pub fn istat1(&self) -> ISTAT1_R {
        ISTAT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Port input status"]
    #[inline(always)]
    pub fn istat0(&self) -> ISTAT0_R {
        ISTAT0_R::new((self.bits & 0x01) != 0)
    }
}
