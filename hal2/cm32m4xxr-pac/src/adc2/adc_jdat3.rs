#[doc = "Register `ADC_JDAT3` reader"]
pub struct R(crate::R<ADC_JDAT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_JDAT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_JDAT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_JDAT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_JDAT3` writer"]
pub struct W(crate::W<ADC_JDAT3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_JDAT3_SPEC>;
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
impl From<crate::W<ADC_JDAT3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_JDAT3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JDAT3` reader - JDAT3"]
pub type JDAT3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `JDAT3` writer - JDAT3"]
pub type JDAT3_W<'a> = crate::FieldWriter<'a, u32, ADC_JDAT3_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - JDAT3"]
    #[inline(always)]
    pub fn jdat3(&self) -> JDAT3_R {
        JDAT3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - JDAT3"]
    #[inline(always)]
    pub fn jdat3(&mut self) -> JDAT3_W {
        JDAT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_JDAT3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_jdat3](index.html) module"]
pub struct ADC_JDAT3_SPEC;
impl crate::RegisterSpec for ADC_JDAT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_jdat3::R](R) reader structure"]
impl crate::Readable for ADC_JDAT3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_jdat3::W](W) writer structure"]
impl crate::Writable for ADC_JDAT3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_JDAT3 to value 0"]
impl crate::Resettable for ADC_JDAT3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
