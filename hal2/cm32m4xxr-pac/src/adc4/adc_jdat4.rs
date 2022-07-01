#[doc = "Register `ADC_JDAT4` reader"]
pub struct R(crate::R<ADC_JDAT4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_JDAT4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_JDAT4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_JDAT4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_JDAT4` writer"]
pub struct W(crate::W<ADC_JDAT4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_JDAT4_SPEC>;
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
impl From<crate::W<ADC_JDAT4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_JDAT4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JDAT4` reader - JDAT4"]
pub type JDAT4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `JDAT4` writer - JDAT4"]
pub type JDAT4_W<'a> = crate::FieldWriter<'a, u32, ADC_JDAT4_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - JDAT4"]
    #[inline(always)]
    pub fn jdat4(&self) -> JDAT4_R {
        JDAT4_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - JDAT4"]
    #[inline(always)]
    pub fn jdat4(&mut self) -> JDAT4_W {
        JDAT4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_JDAT4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_jdat4](index.html) module"]
pub struct ADC_JDAT4_SPEC;
impl crate::RegisterSpec for ADC_JDAT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_jdat4::R](R) reader structure"]
impl crate::Readable for ADC_JDAT4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_jdat4::W](W) writer structure"]
impl crate::Writable for ADC_JDAT4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_JDAT4 to value 0"]
impl crate::Resettable for ADC_JDAT4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
