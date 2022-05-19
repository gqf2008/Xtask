#[doc = "Reader of register INTF"]
pub type R = crate::R<u32, super::INTF>;
#[doc = "Reader of field `GIF0`"]
pub type GIF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `FTFIF0`"]
pub type FTFIF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTFIF0`"]
pub type HTFIF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERRIF0`"]
pub type ERRIF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF1`"]
pub type GIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `FTFIF1`"]
pub type FTFIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTFIF1`"]
pub type HTFIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERRIF1`"]
pub type ERRIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF2`"]
pub type GIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `FTFIF2`"]
pub type FTFIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTFIF2`"]
pub type HTFIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERRIF2`"]
pub type ERRIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF3`"]
pub type GIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `FTFIF3`"]
pub type FTFIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTFIF3`"]
pub type HTFIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERRIF3`"]
pub type ERRIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF4`"]
pub type GIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `FTFIF4`"]
pub type FTFIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTFIF4`"]
pub type HTFIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERRIF4`"]
pub type ERRIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF5`"]
pub type GIF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `FTFIF5`"]
pub type FTFIF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTFIF5`"]
pub type HTFIF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERRIF5`"]
pub type ERRIF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF6`"]
pub type GIF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `FTFIF6`"]
pub type FTFIF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `HTFIF6`"]
pub type HTFIF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERRIF6`"]
pub type ERRIF6_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Global interrupt flag of channel 0"]
    #[inline(always)]
    pub fn gif0(&self) -> GIF0_R {
        GIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Full Transfer finish flag of channe 0"]
    #[inline(always)]
    pub fn ftfif0(&self) -> FTFIF0_R {
        FTFIF0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Half transfer finish flag of channel 0"]
    #[inline(always)]
    pub fn htfif0(&self) -> HTFIF0_R {
        HTFIF0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Error flag of channel 0"]
    #[inline(always)]
    pub fn errif0(&self) -> ERRIF0_R {
        ERRIF0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Global interrupt flag of channel 1"]
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Full Transfer finish flag of channe 1"]
    #[inline(always)]
    pub fn ftfif1(&self) -> FTFIF1_R {
        FTFIF1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Half transfer finish flag of channel 1"]
    #[inline(always)]
    pub fn htfif1(&self) -> HTFIF1_R {
        HTFIF1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Error flag of channel 1"]
    #[inline(always)]
    pub fn errif1(&self) -> ERRIF1_R {
        ERRIF1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Global interrupt flag of channel 2"]
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Full Transfer finish flag of channe 2"]
    #[inline(always)]
    pub fn ftfif2(&self) -> FTFIF2_R {
        FTFIF2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Half transfer finish flag of channel 2"]
    #[inline(always)]
    pub fn htfif2(&self) -> HTFIF2_R {
        HTFIF2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Error flag of channel 2"]
    #[inline(always)]
    pub fn errif2(&self) -> ERRIF2_R {
        ERRIF2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Global interrupt flag of channel 3"]
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Full Transfer finish flag of channe 3"]
    #[inline(always)]
    pub fn ftfif3(&self) -> FTFIF3_R {
        FTFIF3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Half transfer finish flag of channel 3"]
    #[inline(always)]
    pub fn htfif3(&self) -> HTFIF3_R {
        HTFIF3_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Error flag of channel 3"]
    #[inline(always)]
    pub fn errif3(&self) -> ERRIF3_R {
        ERRIF3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Global interrupt flag of channel 4"]
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Full Transfer finish flag of channe 4"]
    #[inline(always)]
    pub fn ftfif4(&self) -> FTFIF4_R {
        FTFIF4_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Half transfer finish flag of channel 4"]
    #[inline(always)]
    pub fn htfif4(&self) -> HTFIF4_R {
        HTFIF4_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Error flag of channel 4"]
    #[inline(always)]
    pub fn errif4(&self) -> ERRIF4_R {
        ERRIF4_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Global interrupt flag of channel 5"]
    #[inline(always)]
    pub fn gif5(&self) -> GIF5_R {
        GIF5_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Full Transfer finish flag of channe 5"]
    #[inline(always)]
    pub fn ftfif5(&self) -> FTFIF5_R {
        FTFIF5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Half transfer finish flag of channel 5"]
    #[inline(always)]
    pub fn htfif5(&self) -> HTFIF5_R {
        HTFIF5_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Error flag of channel 5"]
    #[inline(always)]
    pub fn errif5(&self) -> ERRIF5_R {
        ERRIF5_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Global interrupt flag of channel 6"]
    #[inline(always)]
    pub fn gif6(&self) -> GIF6_R {
        GIF6_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Full Transfer finish flag of channe 6"]
    #[inline(always)]
    pub fn ftfif6(&self) -> FTFIF6_R {
        FTFIF6_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Half transfer finish flag of channel 6"]
    #[inline(always)]
    pub fn htfif6(&self) -> HTFIF6_R {
        HTFIF6_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Error flag of channel 6"]
    #[inline(always)]
    pub fn errif6(&self) -> ERRIF6_R {
        ERRIF6_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
