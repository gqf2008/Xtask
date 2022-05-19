#[doc = "Reader of register DATA"]
pub type R = crate::R<u16, super::DATA>;
#[doc = "Writer for register DATA"]
pub type W = crate::W<u16, super::DATA>;
#[doc = "Register DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_DATA`"]
pub type SPI_DATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SPI_DATA`"]
pub struct SPI_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Data transfer register"]
    #[inline(always)]
    pub fn spi_data(&self) -> SPI_DATA_R {
        SPI_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data transfer register"]
    #[inline(always)]
    pub fn spi_data(&mut self) -> SPI_DATA_W {
        SPI_DATA_W { w: self }
    }
}
