#[doc = "Register `ADC_JSEQ` reader"]
pub struct R(crate::R<ADC_JSEQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_JSEQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_JSEQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_JSEQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_JSEQ` writer"]
pub struct W(crate::W<ADC_JSEQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_JSEQ_SPEC>;
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
impl From<crate::W<ADC_JSEQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_JSEQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JSEQ1` reader - JSEQ1"]
pub type JSEQ1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JSEQ1` writer - JSEQ1"]
pub type JSEQ1_W<'a> = crate::FieldWriter<'a, u32, ADC_JSEQ_SPEC, u8, u8, 5, 0>;
#[doc = "Field `JSEQ2` reader - JSEQ2"]
pub type JSEQ2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JSEQ2` writer - JSEQ2"]
pub type JSEQ2_W<'a> = crate::FieldWriter<'a, u32, ADC_JSEQ_SPEC, u8, u8, 5, 5>;
#[doc = "Field `JSEQ3` reader - JSEQ3"]
pub type JSEQ3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JSEQ3` writer - JSEQ3"]
pub type JSEQ3_W<'a> = crate::FieldWriter<'a, u32, ADC_JSEQ_SPEC, u8, u8, 5, 10>;
#[doc = "Field `JSEQ4` reader - JSEQ4"]
pub type JSEQ4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JSEQ4` writer - JSEQ4"]
pub type JSEQ4_W<'a> = crate::FieldWriter<'a, u32, ADC_JSEQ_SPEC, u8, u8, 5, 15>;
#[doc = "Field `JLEN` reader - JLEN"]
pub type JLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JLEN` writer - JLEN"]
pub type JLEN_W<'a> = crate::FieldWriter<'a, u32, ADC_JSEQ_SPEC, u8, u8, 2, 20>;
impl R {
    #[doc = "Bits 0:4 - JSEQ1"]
    #[inline(always)]
    pub fn jseq1(&self) -> JSEQ1_R {
        JSEQ1_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - JSEQ2"]
    #[inline(always)]
    pub fn jseq2(&self) -> JSEQ2_R {
        JSEQ2_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - JSEQ3"]
    #[inline(always)]
    pub fn jseq3(&self) -> JSEQ3_R {
        JSEQ3_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - JSEQ4"]
    #[inline(always)]
    pub fn jseq4(&self) -> JSEQ4_R {
        JSEQ4_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:21 - JLEN"]
    #[inline(always)]
    pub fn jlen(&self) -> JLEN_R {
        JLEN_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - JSEQ1"]
    #[inline(always)]
    pub fn jseq1(&mut self) -> JSEQ1_W {
        JSEQ1_W::new(self)
    }
    #[doc = "Bits 5:9 - JSEQ2"]
    #[inline(always)]
    pub fn jseq2(&mut self) -> JSEQ2_W {
        JSEQ2_W::new(self)
    }
    #[doc = "Bits 10:14 - JSEQ3"]
    #[inline(always)]
    pub fn jseq3(&mut self) -> JSEQ3_W {
        JSEQ3_W::new(self)
    }
    #[doc = "Bits 15:19 - JSEQ4"]
    #[inline(always)]
    pub fn jseq4(&mut self) -> JSEQ4_W {
        JSEQ4_W::new(self)
    }
    #[doc = "Bits 20:21 - JLEN"]
    #[inline(always)]
    pub fn jlen(&mut self) -> JLEN_W {
        JLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_JSEQ\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_jseq](index.html) module"]
pub struct ADC_JSEQ_SPEC;
impl crate::RegisterSpec for ADC_JSEQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_jseq::R](R) reader structure"]
impl crate::Readable for ADC_JSEQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_jseq::W](W) writer structure"]
impl crate::Writable for ADC_JSEQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_JSEQ to value 0"]
impl crate::Resettable for ADC_JSEQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
