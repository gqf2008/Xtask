#[doc = "Register `USART_DAT` reader"]
pub struct R(crate::R<USART_DAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USART_DAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USART_DAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USART_DAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USART_DAT` writer"]
pub struct W(crate::W<USART_DAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USART_DAT_SPEC>;
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
impl From<crate::W<USART_DAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USART_DAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATV` reader - DATV"]
pub type DATV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATV` writer - DATV"]
pub type DATV_W<'a> = crate::FieldWriter<'a, u32, USART_DAT_SPEC, u16, u16, 9, 0>;
impl R {
    #[doc = "Bits 0:8 - DATV"]
    #[inline(always)]
    pub fn datv(&self) -> DATV_R {
        DATV_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - DATV"]
    #[inline(always)]
    pub fn datv(&mut self) -> DATV_W {
        DATV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART_DAT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_dat](index.html) module"]
pub struct USART_DAT_SPEC;
impl crate::RegisterSpec for USART_DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usart_dat::R](R) reader structure"]
impl crate::Readable for USART_DAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usart_dat::W](W) writer structure"]
impl crate::Writable for USART_DAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USART_DAT to value 0"]
impl crate::Resettable for USART_DAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
