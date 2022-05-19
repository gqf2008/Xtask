#[doc = "Reader of register DIEP0TFLEN"]
pub type R = crate::R<u32, super::DIEP0TFLEN>;
#[doc = "Writer for register DIEP0TFLEN"]
pub type W = crate::W<u32, super::DIEP0TFLEN>;
#[doc = "Register DIEP0TFLEN `reset()`'s with value 0x0200_0200"]
impl crate::ResetValue for super::DIEP0TFLEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200_0200
    }
}
#[doc = "Reader of field `IEP0TXFD`"]
pub type IEP0TXFD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IEP0TXFD`"]
pub struct IEP0TXFD_W<'a> {
    w: &'a mut W,
}
impl<'a> IEP0TXFD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `IEP0TXRSAR`"]
pub type IEP0TXRSAR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IEP0TXRSAR`"]
pub struct IEP0TXRSAR_W<'a> {
    w: &'a mut W,
}
impl<'a> IEP0TXRSAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - in endpoint 0 Tx FIFO depth"]
    #[inline(always)]
    pub fn iep0txfd(&self) -> IEP0TXFD_R {
        IEP0TXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - in endpoint 0 Tx RAM start address"]
    #[inline(always)]
    pub fn iep0txrsar(&self) -> IEP0TXRSAR_R {
        IEP0TXRSAR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - in endpoint 0 Tx FIFO depth"]
    #[inline(always)]
    pub fn iep0txfd(&mut self) -> IEP0TXFD_W {
        IEP0TXFD_W { w: self }
    }
    #[doc = "Bits 0:15 - in endpoint 0 Tx RAM start address"]
    #[inline(always)]
    pub fn iep0txrsar(&mut self) -> IEP0TXRSAR_W {
        IEP0TXRSAR_W { w: self }
    }
}
