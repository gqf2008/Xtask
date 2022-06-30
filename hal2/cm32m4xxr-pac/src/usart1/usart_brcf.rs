#[doc = "Register `USART_BRCF` reader"]
pub struct R(crate::R<USART_BRCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USART_BRCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USART_BRCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USART_BRCF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USART_BRCF` writer"]
pub struct W(crate::W<USART_BRCF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USART_BRCF_SPEC>;
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
impl From<crate::W<USART_BRCF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USART_BRCF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV_Decimal` reader - DIV_Decimal"]
pub type DIV_DECIMAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV_Decimal` writer - DIV_Decimal"]
pub type DIV_DECIMAL_W<'a> = crate::FieldWriter<'a, u32, USART_BRCF_SPEC, u8, u8, 4, 0>;
#[doc = "Field `DIV_Integer` reader - DIV_Integer"]
pub type DIV_INTEGER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIV_Integer` writer - DIV_Integer"]
pub type DIV_INTEGER_W<'a> = crate::FieldWriter<'a, u32, USART_BRCF_SPEC, u16, u16, 12, 4>;
impl R {
    #[doc = "Bits 0:3 - DIV_Decimal"]
    #[inline(always)]
    pub fn div_decimal(&self) -> DIV_DECIMAL_R {
        DIV_DECIMAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - DIV_Integer"]
    #[inline(always)]
    pub fn div_integer(&self) -> DIV_INTEGER_R {
        DIV_INTEGER_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - DIV_Decimal"]
    #[inline(always)]
    pub fn div_decimal(&mut self) -> DIV_DECIMAL_W {
        DIV_DECIMAL_W::new(self)
    }
    #[doc = "Bits 4:15 - DIV_Integer"]
    #[inline(always)]
    pub fn div_integer(&mut self) -> DIV_INTEGER_W {
        DIV_INTEGER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART_BRCF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_brcf](index.html) module"]
pub struct USART_BRCF_SPEC;
impl crate::RegisterSpec for USART_BRCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usart_brcf::R](R) reader structure"]
impl crate::Readable for USART_BRCF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usart_brcf::W](W) writer structure"]
impl crate::Writable for USART_BRCF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USART_BRCF to value 0"]
impl crate::Resettable for USART_BRCF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
