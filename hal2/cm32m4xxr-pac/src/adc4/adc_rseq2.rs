#[doc = "Register `ADC_RSEQ2` reader"]
pub struct R(crate::R<ADC_RSEQ2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_RSEQ2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_RSEQ2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_RSEQ2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_RSEQ2` writer"]
pub struct W(crate::W<ADC_RSEQ2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_RSEQ2_SPEC>;
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
impl From<crate::W<ADC_RSEQ2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_RSEQ2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEQ7` reader - SEQ7"]
pub type SEQ7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEQ7` writer - SEQ7"]
pub type SEQ7_W<'a> = crate::FieldWriter<'a, u32, ADC_RSEQ2_SPEC, u8, u8, 5, 0>;
#[doc = "Field `SEQ8` reader - SEQ8"]
pub type SEQ8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEQ8` writer - SEQ8"]
pub type SEQ8_W<'a> = crate::FieldWriter<'a, u32, ADC_RSEQ2_SPEC, u8, u8, 5, 5>;
#[doc = "Field `SEQ9` reader - SEQ9"]
pub type SEQ9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEQ9` writer - SEQ9"]
pub type SEQ9_W<'a> = crate::FieldWriter<'a, u32, ADC_RSEQ2_SPEC, u8, u8, 5, 10>;
#[doc = "Field `SEQ10` reader - SEQ10"]
pub type SEQ10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEQ10` writer - SEQ10"]
pub type SEQ10_W<'a> = crate::FieldWriter<'a, u32, ADC_RSEQ2_SPEC, u8, u8, 5, 15>;
#[doc = "Field `SEQ11` reader - SEQ11"]
pub type SEQ11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEQ11` writer - SEQ11"]
pub type SEQ11_W<'a> = crate::FieldWriter<'a, u32, ADC_RSEQ2_SPEC, u8, u8, 5, 20>;
#[doc = "Field `SEQ12` reader - SEQ12"]
pub type SEQ12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEQ12` writer - SEQ12"]
pub type SEQ12_W<'a> = crate::FieldWriter<'a, u32, ADC_RSEQ2_SPEC, u8, u8, 5, 25>;
impl R {
    #[doc = "Bits 0:4 - SEQ7"]
    #[inline(always)]
    pub fn seq7(&self) -> SEQ7_R {
        SEQ7_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - SEQ8"]
    #[inline(always)]
    pub fn seq8(&self) -> SEQ8_R {
        SEQ8_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - SEQ9"]
    #[inline(always)]
    pub fn seq9(&self) -> SEQ9_R {
        SEQ9_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - SEQ10"]
    #[inline(always)]
    pub fn seq10(&self) -> SEQ10_R {
        SEQ10_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - SEQ11"]
    #[inline(always)]
    pub fn seq11(&self) -> SEQ11_R {
        SEQ11_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:29 - SEQ12"]
    #[inline(always)]
    pub fn seq12(&self) -> SEQ12_R {
        SEQ12_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - SEQ7"]
    #[inline(always)]
    pub fn seq7(&mut self) -> SEQ7_W {
        SEQ7_W::new(self)
    }
    #[doc = "Bits 5:9 - SEQ8"]
    #[inline(always)]
    pub fn seq8(&mut self) -> SEQ8_W {
        SEQ8_W::new(self)
    }
    #[doc = "Bits 10:14 - SEQ9"]
    #[inline(always)]
    pub fn seq9(&mut self) -> SEQ9_W {
        SEQ9_W::new(self)
    }
    #[doc = "Bits 15:19 - SEQ10"]
    #[inline(always)]
    pub fn seq10(&mut self) -> SEQ10_W {
        SEQ10_W::new(self)
    }
    #[doc = "Bits 20:24 - SEQ11"]
    #[inline(always)]
    pub fn seq11(&mut self) -> SEQ11_W {
        SEQ11_W::new(self)
    }
    #[doc = "Bits 25:29 - SEQ12"]
    #[inline(always)]
    pub fn seq12(&mut self) -> SEQ12_W {
        SEQ12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_RSEQ2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_rseq2](index.html) module"]
pub struct ADC_RSEQ2_SPEC;
impl crate::RegisterSpec for ADC_RSEQ2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_rseq2::R](R) reader structure"]
impl crate::Readable for ADC_RSEQ2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_rseq2::W](W) writer structure"]
impl crate::Writable for ADC_RSEQ2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_RSEQ2 to value 0"]
impl crate::Resettable for ADC_RSEQ2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
