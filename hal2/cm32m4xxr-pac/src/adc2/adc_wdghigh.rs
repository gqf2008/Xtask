#[doc = "Register `ADC_WDGHIGH` reader"]
pub struct R(crate::R<ADC_WDGHIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_WDGHIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_WDGHIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_WDGHIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_WDGHIGH` writer"]
pub struct W(crate::W<ADC_WDGHIGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_WDGHIGH_SPEC>;
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
impl From<crate::W<ADC_WDGHIGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_WDGHIGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HTH` reader - HTH"]
pub type HTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HTH` writer - HTH"]
pub type HTH_W<'a> = crate::FieldWriter<'a, u32, ADC_WDGHIGH_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - HTH"]
    #[inline(always)]
    pub fn hth(&self) -> HTH_R {
        HTH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - HTH"]
    #[inline(always)]
    pub fn hth(&mut self) -> HTH_W {
        HTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_WDGHIGH\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_wdghigh](index.html) module"]
pub struct ADC_WDGHIGH_SPEC;
impl crate::RegisterSpec for ADC_WDGHIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_wdghigh::R](R) reader structure"]
impl crate::Readable for ADC_WDGHIGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_wdghigh::W](W) writer structure"]
impl crate::Writable for ADC_WDGHIGH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_WDGHIGH to value 0"]
impl crate::Resettable for ADC_WDGHIGH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
