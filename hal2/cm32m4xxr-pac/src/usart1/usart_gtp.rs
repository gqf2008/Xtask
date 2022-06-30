#[doc = "Register `USART_GTP` reader"]
pub struct R(crate::R<USART_GTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USART_GTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USART_GTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USART_GTP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USART_GTP` writer"]
pub struct W(crate::W<USART_GTP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USART_GTP_SPEC>;
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
impl From<crate::W<USART_GTP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USART_GTP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSCV` reader - PSCV"]
pub type PSCV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSCV` writer - PSCV"]
pub type PSCV_W<'a> = crate::FieldWriter<'a, u32, USART_GTP_SPEC, u8, u8, 8, 0>;
#[doc = "Field `GTV` reader - GTV"]
pub type GTV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GTV` writer - GTV"]
pub type GTV_W<'a> = crate::FieldWriter<'a, u32, USART_GTP_SPEC, u8, u8, 8, 8>;
impl R {
    #[doc = "Bits 0:7 - PSCV"]
    #[inline(always)]
    pub fn pscv(&self) -> PSCV_R {
        PSCV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GTV"]
    #[inline(always)]
    pub fn gtv(&self) -> GTV_R {
        GTV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PSCV"]
    #[inline(always)]
    pub fn pscv(&mut self) -> PSCV_W {
        PSCV_W::new(self)
    }
    #[doc = "Bits 8:15 - GTV"]
    #[inline(always)]
    pub fn gtv(&mut self) -> GTV_W {
        GTV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART_GTP\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_gtp](index.html) module"]
pub struct USART_GTP_SPEC;
impl crate::RegisterSpec for USART_GTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usart_gtp::R](R) reader structure"]
impl crate::Readable for USART_GTP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usart_gtp::W](W) writer structure"]
impl crate::Writable for USART_GTP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USART_GTP to value 0"]
impl crate::Resettable for USART_GTP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
