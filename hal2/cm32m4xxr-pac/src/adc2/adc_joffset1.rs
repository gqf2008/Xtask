#[doc = "Register `ADC_JOFFSET1` reader"]
pub struct R(crate::R<ADC_JOFFSET1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_JOFFSET1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_JOFFSET1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_JOFFSET1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_JOFFSET1` writer"]
pub struct W(crate::W<ADC_JOFFSET1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_JOFFSET1_SPEC>;
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
impl From<crate::W<ADC_JOFFSET1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_JOFFSET1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSETJCH2` reader - OFFSETJCH2"]
pub type OFFSETJCH2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OFFSETJCH2` writer - OFFSETJCH2"]
pub type OFFSETJCH2_W<'a> = crate::FieldWriter<'a, u32, ADC_JOFFSET1_SPEC, u16, u16, 12, 0>;
impl R {
    #[doc = "Bits 0:11 - OFFSETJCH2"]
    #[inline(always)]
    pub fn offsetjch2(&self) -> OFFSETJCH2_R {
        OFFSETJCH2_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - OFFSETJCH2"]
    #[inline(always)]
    pub fn offsetjch2(&mut self) -> OFFSETJCH2_W {
        OFFSETJCH2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_JOFFSET1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_joffset1](index.html) module"]
pub struct ADC_JOFFSET1_SPEC;
impl crate::RegisterSpec for ADC_JOFFSET1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_joffset1::R](R) reader structure"]
impl crate::Readable for ADC_JOFFSET1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_joffset1::W](W) writer structure"]
impl crate::Writable for ADC_JOFFSET1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_JOFFSET1 to value 0"]
impl crate::Resettable for ADC_JOFFSET1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
