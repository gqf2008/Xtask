#[doc = "Register `ADC_DIFSEL` reader"]
pub struct R(crate::R<ADC_DIFSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_DIFSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_DIFSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_DIFSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_DIFSEL` writer"]
pub struct W(crate::W<ADC_DIFSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_DIFSEL_SPEC>;
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
impl From<crate::W<ADC_DIFSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_DIFSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIFSEL` reader - DIFSEL"]
pub type DIFSEL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DIFSEL` writer - DIFSEL"]
pub type DIFSEL_W<'a> = crate::FieldWriter<'a, u32, ADC_DIFSEL_SPEC, u32, u32, 18, 1>;
impl R {
    #[doc = "Bits 1:18 - DIFSEL"]
    #[inline(always)]
    pub fn difsel(&self) -> DIFSEL_R {
        DIFSEL_R::new(((self.bits >> 1) & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 1:18 - DIFSEL"]
    #[inline(always)]
    pub fn difsel(&mut self) -> DIFSEL_W {
        DIFSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_DIFSEL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_difsel](index.html) module"]
pub struct ADC_DIFSEL_SPEC;
impl crate::RegisterSpec for ADC_DIFSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_difsel::R](R) reader structure"]
impl crate::Readable for ADC_DIFSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_difsel::W](W) writer structure"]
impl crate::Writable for ADC_DIFSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_DIFSEL to value 0"]
impl crate::Resettable for ADC_DIFSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
