#[doc = "Register `USART_STS` reader"]
pub struct R(crate::R<USART_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USART_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USART_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USART_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USART_STS` writer"]
pub struct W(crate::W<USART_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USART_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<USART_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USART_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEF` reader - PEF"]
pub type PEF_R = crate::BitReader<bool>;
#[doc = "Field `PEF` writer - PEF"]
pub type PEF_W<'a> = crate::BitWriter<'a, u32, USART_STS_SPEC, bool, 0>;
#[doc = "Field `FEF` reader - FEF"]
pub type FEF_R = crate::BitReader<bool>;
#[doc = "Field `FEF` writer - FEF"]
pub type FEF_W<'a> = crate::BitWriter<'a, u32, USART_STS_SPEC, bool, 1>;
#[doc = "Field `NEF` reader - NEF"]
pub type NEF_R = crate::BitReader<bool>;
#[doc = "Field `NEF` writer - NEF"]
pub type NEF_W<'a> = crate::BitWriter<'a, u32, USART_STS_SPEC, bool, 2>;
#[doc = "Field `OREF` reader - OREF"]
pub type OREF_R = crate::BitReader<bool>;
#[doc = "Field `OREF` writer - OREF"]
pub type OREF_W<'a> = crate::BitWriter<'a, u32, USART_STS_SPEC, bool, 3>;
#[doc = "Field `IDLEF` reader - IDLEF"]
pub type IDLEF_R = crate::BitReader<bool>;
#[doc = "Field `IDLEF` writer - IDLEF"]
pub type IDLEF_W<'a> = crate::BitWriter<'a, u32, USART_STS_SPEC, bool, 4>;
#[doc = "Field `RXDNE` reader - RXDNE"]
pub type RXDNE_R = crate::BitReader<bool>;
#[doc = "Field `RXDNE` writer - RXDNE"]
pub type RXDNE_W<'a> = crate::BitWriter<'a, u32, USART_STS_SPEC, bool, 5>;
#[doc = "Field `TXC` reader - TXC"]
pub type TXC_R = crate::BitReader<bool>;
#[doc = "Field `TXC` writer - TXC"]
pub type TXC_W<'a> = crate::BitWriter<'a, u32, USART_STS_SPEC, bool, 6>;
#[doc = "Field `TXDE` reader - TXDE"]
pub type TXDE_R = crate::BitReader<bool>;
#[doc = "Field `TXDE` writer - TXDE"]
pub type TXDE_W<'a> = crate::BitWriter<'a, u32, USART_STS_SPEC, bool, 7>;
#[doc = "Field `LINBDF` reader - LINBDF"]
pub type LINBDF_R = crate::BitReader<bool>;
#[doc = "Field `LINBDF` writer - LINBDF"]
pub type LINBDF_W<'a> = crate::BitWriter<'a, u32, USART_STS_SPEC, bool, 8>;
#[doc = "Field `CTSF` reader - CTSF"]
pub type CTSF_R = crate::BitReader<bool>;
#[doc = "Field `CTSF` writer - CTSF"]
pub type CTSF_W<'a> = crate::BitWriter<'a, u32, USART_STS_SPEC, bool, 9>;
impl R {
    #[doc = "Bit 0 - PEF"]
    #[inline(always)]
    pub fn pef(&self) -> PEF_R {
        PEF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FEF"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NEF"]
    #[inline(always)]
    pub fn nef(&self) -> NEF_R {
        NEF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OREF"]
    #[inline(always)]
    pub fn oref(&self) -> OREF_R {
        OREF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLEF"]
    #[inline(always)]
    pub fn idlef(&self) -> IDLEF_R {
        IDLEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXDNE"]
    #[inline(always)]
    pub fn rxdne(&self) -> RXDNE_R {
        RXDNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXC"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXDE"]
    #[inline(always)]
    pub fn txde(&self) -> TXDE_R {
        TXDE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LINBDF"]
    #[inline(always)]
    pub fn linbdf(&self) -> LINBDF_R {
        LINBDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTSF"]
    #[inline(always)]
    pub fn ctsf(&self) -> CTSF_R {
        CTSF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PEF"]
    #[inline(always)]
    pub fn pef(&mut self) -> PEF_W {
        PEF_W::new(self)
    }
    #[doc = "Bit 1 - FEF"]
    #[inline(always)]
    pub fn fef(&mut self) -> FEF_W {
        FEF_W::new(self)
    }
    #[doc = "Bit 2 - NEF"]
    #[inline(always)]
    pub fn nef(&mut self) -> NEF_W {
        NEF_W::new(self)
    }
    #[doc = "Bit 3 - OREF"]
    #[inline(always)]
    pub fn oref(&mut self) -> OREF_W {
        OREF_W::new(self)
    }
    #[doc = "Bit 4 - IDLEF"]
    #[inline(always)]
    pub fn idlef(&mut self) -> IDLEF_W {
        IDLEF_W::new(self)
    }
    #[doc = "Bit 5 - RXDNE"]
    #[inline(always)]
    pub fn rxdne(&mut self) -> RXDNE_W {
        RXDNE_W::new(self)
    }
    #[doc = "Bit 6 - TXC"]
    #[inline(always)]
    pub fn txc(&mut self) -> TXC_W {
        TXC_W::new(self)
    }
    #[doc = "Bit 7 - TXDE"]
    #[inline(always)]
    pub fn txde(&mut self) -> TXDE_W {
        TXDE_W::new(self)
    }
    #[doc = "Bit 8 - LINBDF"]
    #[inline(always)]
    pub fn linbdf(&mut self) -> LINBDF_W {
        LINBDF_W::new(self)
    }
    #[doc = "Bit 9 - CTSF"]
    #[inline(always)]
    pub fn ctsf(&mut self) -> CTSF_W {
        CTSF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART_STS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_sts](index.html) module"]
pub struct USART_STS_SPEC;
impl crate::RegisterSpec for USART_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usart_sts::R](R) reader structure"]
impl crate::Readable for USART_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usart_sts::W](W) writer structure"]
impl crate::Writable for USART_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USART_STS to value 0xc0"]
impl crate::Resettable for USART_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc0
    }
}
