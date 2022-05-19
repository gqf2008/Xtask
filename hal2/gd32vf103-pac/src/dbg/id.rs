#[doc = "Reader of register ID"]
pub type R = crate::R<u32, super::ID>;
#[doc = "Reader of field `ID_CODE`"]
pub type ID_CODE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - DBG ID code register"]
    #[inline(always)]
    pub fn id_code(&self) -> ID_CODE_R {
        ID_CODE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
