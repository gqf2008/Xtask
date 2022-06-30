#[doc = "Register `ADC_JDAT` reader"]
pub struct R(crate::R<ADC_JDAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_JDAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_JDAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_JDAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_JDAT` writer"]
pub struct W(crate::W<ADC_JDAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_JDAT_SPEC>;
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
impl From<crate::W<ADC_JDAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_JDAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JDAT` reader - JDAT"]
pub type JDAT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `JDAT` writer - JDAT"]
pub type JDAT_W<'a> = crate::FieldWriter<'a, u32, ADC_JDAT_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - JDAT"]
    #[inline(always)]
    pub fn jdat(&self) -> JDAT_R {
        JDAT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - JDAT"]
    #[inline(always)]
    pub fn jdat(&mut self) -> JDAT_W {
        JDAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_JDAT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_jdat](index.html) module"]
pub struct ADC_JDAT_SPEC;
impl crate::RegisterSpec for ADC_JDAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_jdat::R](R) reader structure"]
impl crate::Readable for ADC_JDAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_jdat::W](W) writer structure"]
impl crate::Writable for ADC_JDAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_JDAT to value 0"]
impl crate::Resettable for ADC_JDAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
