#[doc = "Register `ADC_RSEQ1` reader"]
pub struct R(crate::R<ADC_RSEQ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_RSEQ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_RSEQ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_RSEQ1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_RSEQ1` writer"]
pub struct W(crate::W<ADC_RSEQ1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_RSEQ1_SPEC>;
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
impl From<crate::W<ADC_RSEQ1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_RSEQ1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEQ13` reader - SEQ13"]
pub type SEQ13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEQ13` writer - SEQ13"]
pub type SEQ13_W<'a> = crate::FieldWriter<'a, u32, ADC_RSEQ1_SPEC, u8, u8, 5, 0>;
#[doc = "Field `SEQ14` reader - SEQ14"]
pub type SEQ14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEQ14` writer - SEQ14"]
pub type SEQ14_W<'a> = crate::FieldWriter<'a, u32, ADC_RSEQ1_SPEC, u8, u8, 5, 5>;
#[doc = "Field `SEQ15` reader - SEQ15"]
pub type SEQ15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEQ15` writer - SEQ15"]
pub type SEQ15_W<'a> = crate::FieldWriter<'a, u32, ADC_RSEQ1_SPEC, u8, u8, 5, 10>;
#[doc = "Field `SEQ16` reader - SEQ16"]
pub type SEQ16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEQ16` writer - SEQ16"]
pub type SEQ16_W<'a> = crate::FieldWriter<'a, u32, ADC_RSEQ1_SPEC, u8, u8, 5, 15>;
#[doc = "Field `LEN` reader - LEN"]
pub type LEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEN` writer - LEN"]
pub type LEN_W<'a> = crate::FieldWriter<'a, u32, ADC_RSEQ1_SPEC, u8, u8, 4, 20>;
impl R {
    #[doc = "Bits 0:4 - SEQ13"]
    #[inline(always)]
    pub fn seq13(&self) -> SEQ13_R {
        SEQ13_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - SEQ14"]
    #[inline(always)]
    pub fn seq14(&self) -> SEQ14_R {
        SEQ14_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - SEQ15"]
    #[inline(always)]
    pub fn seq15(&self) -> SEQ15_R {
        SEQ15_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - SEQ16"]
    #[inline(always)]
    pub fn seq16(&self) -> SEQ16_R {
        SEQ16_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:23 - LEN"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - SEQ13"]
    #[inline(always)]
    pub fn seq13(&mut self) -> SEQ13_W {
        SEQ13_W::new(self)
    }
    #[doc = "Bits 5:9 - SEQ14"]
    #[inline(always)]
    pub fn seq14(&mut self) -> SEQ14_W {
        SEQ14_W::new(self)
    }
    #[doc = "Bits 10:14 - SEQ15"]
    #[inline(always)]
    pub fn seq15(&mut self) -> SEQ15_W {
        SEQ15_W::new(self)
    }
    #[doc = "Bits 15:19 - SEQ16"]
    #[inline(always)]
    pub fn seq16(&mut self) -> SEQ16_W {
        SEQ16_W::new(self)
    }
    #[doc = "Bits 20:23 - LEN"]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W {
        LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_RSEQ1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_rseq1](index.html) module"]
pub struct ADC_RSEQ1_SPEC;
impl crate::RegisterSpec for ADC_RSEQ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_rseq1::R](R) reader structure"]
impl crate::Readable for ADC_RSEQ1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_rseq1::W](W) writer structure"]
impl crate::Writable for ADC_RSEQ1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_RSEQ1 to value 0"]
impl crate::Resettable for ADC_RSEQ1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
