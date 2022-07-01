#[doc = "Register `XIP_RXFOI_CLR` reader"]
pub struct R(crate::R<XIP_RXFOI_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XIP_RXFOI_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XIP_RXFOI_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XIP_RXFOI_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XIP_RXFOI_CLR` writer"]
pub struct W(crate::W<XIP_RXFOI_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XIP_RXFOI_CLR_SPEC>;
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
impl From<crate::W<XIP_RXFOI_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XIP_RXFOI_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XRXFOIC` reader - XRXFOIC"]
pub type XRXFOIC_R = crate::BitReader<bool>;
#[doc = "Field `XRXFOIC` writer - XRXFOIC"]
pub type XRXFOIC_W<'a> = crate::BitWriter<'a, u32, XIP_RXFOI_CLR_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - XRXFOIC"]
    #[inline(always)]
    pub fn xrxfoic(&self) -> XRXFOIC_R {
        XRXFOIC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XRXFOIC"]
    #[inline(always)]
    pub fn xrxfoic(&mut self) -> XRXFOIC_W {
        XRXFOIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XIP_RXFOI_CLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xip_rxfoi_clr](index.html) module"]
pub struct XIP_RXFOI_CLR_SPEC;
impl crate::RegisterSpec for XIP_RXFOI_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xip_rxfoi_clr::R](R) reader structure"]
impl crate::Readable for XIP_RXFOI_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xip_rxfoi_clr::W](W) writer structure"]
impl crate::Writable for XIP_RXFOI_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XIP_RXFOI_CLR to value 0"]
impl crate::Resettable for XIP_RXFOI_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
