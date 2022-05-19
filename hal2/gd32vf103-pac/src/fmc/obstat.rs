#[doc = "Reader of register OBSTAT"]
pub type R = crate::R<u32, super::OBSTAT>;
#[doc = "Reader of field `OBERR`"]
pub type OBERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPC`"]
pub type SPC_R = crate::R<bool, bool>;
#[doc = "Reader of field `USER`"]
pub type USER_R = crate::R<u8, u8>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bit 0 - Option bytes read error bit"]
    #[inline(always)]
    pub fn oberr(&self) -> OBERR_R {
        OBERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Option bytes security protection code"]
    #[inline(always)]
    pub fn spc(&self) -> SPC_R {
        SPC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:9 - Store USER of option bytes block after system reset"]
    #[inline(always)]
    pub fn user(&self) -> USER_R {
        USER_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bits 10:25 - Store DATA\\[15:0\\]
of option bytes block after system reset"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 10) & 0xffff) as u16)
    }
}
