#[doc = "Register `ADC_SAMPT3` reader"]
pub struct R(crate::R<ADC_SAMPT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_SAMPT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_SAMPT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_SAMPT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_SAMPT3` writer"]
pub struct W(crate::W<ADC_SAMPT3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_SAMPT3_SPEC>;
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
impl From<crate::W<ADC_SAMPT3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_SAMPT3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAMP` reader - SAMP"]
pub type SAMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMP` writer - SAMP"]
pub type SAMP_W<'a> = crate::FieldWriter<'a, u32, ADC_SAMPT3_SPEC, u8, u8, 3, 0>;
#[doc = "Field `SAMPSEL` reader - SAMPSEL"]
pub type SAMPSEL_R = crate::BitReader<bool>;
#[doc = "Field `SAMPSEL` writer - SAMPSEL"]
pub type SAMPSEL_W<'a> = crate::BitWriter<'a, u32, ADC_SAMPT3_SPEC, bool, 3>;
impl R {
    #[doc = "Bits 0:2 - SAMP"]
    #[inline(always)]
    pub fn samp(&self) -> SAMP_R {
        SAMP_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - SAMPSEL"]
    #[inline(always)]
    pub fn sampsel(&self) -> SAMPSEL_R {
        SAMPSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SAMP"]
    #[inline(always)]
    pub fn samp(&mut self) -> SAMP_W {
        SAMP_W::new(self)
    }
    #[doc = "Bit 3 - SAMPSEL"]
    #[inline(always)]
    pub fn sampsel(&mut self) -> SAMPSEL_W {
        SAMPSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_SAMPT3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_sampt3](index.html) module"]
pub struct ADC_SAMPT3_SPEC;
impl crate::RegisterSpec for ADC_SAMPT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_sampt3::R](R) reader structure"]
impl crate::Readable for ADC_SAMPT3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_sampt3::W](W) writer structure"]
impl crate::Writable for ADC_SAMPT3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_SAMPT3 to value 0"]
impl crate::Resettable for ADC_SAMPT3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
