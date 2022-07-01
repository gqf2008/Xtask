#[doc = "Register `ADC_SMPR1` reader"]
pub struct R(crate::R<ADC_SMPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_SMPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_SMPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_SMPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_SMPR1` writer"]
pub struct W(crate::W<ADC_SMPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_SMPR1_SPEC>;
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
impl From<crate::W<ADC_SMPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_SMPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAMP10` reader - SAMP10"]
pub type SAMP10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMP10` writer - SAMP10"]
pub type SAMP10_W<'a> = crate::FieldWriter<'a, u32, ADC_SMPR1_SPEC, u8, u8, 3, 0>;
#[doc = "Field `SAMP11` reader - SAMP11"]
pub type SAMP11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMP11` writer - SAMP11"]
pub type SAMP11_W<'a> = crate::FieldWriter<'a, u32, ADC_SMPR1_SPEC, u8, u8, 3, 3>;
#[doc = "Field `SAMP12` reader - SAMP12"]
pub type SAMP12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMP12` writer - SAMP12"]
pub type SAMP12_W<'a> = crate::FieldWriter<'a, u32, ADC_SMPR1_SPEC, u8, u8, 3, 6>;
#[doc = "Field `SAMP13` reader - SAMP13"]
pub type SAMP13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMP13` writer - SAMP13"]
pub type SAMP13_W<'a> = crate::FieldWriter<'a, u32, ADC_SMPR1_SPEC, u8, u8, 3, 9>;
#[doc = "Field `SAMP14` reader - SAMP14"]
pub type SAMP14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMP14` writer - SAMP14"]
pub type SAMP14_W<'a> = crate::FieldWriter<'a, u32, ADC_SMPR1_SPEC, u8, u8, 3, 12>;
#[doc = "Field `SAMP15` reader - SAMP15"]
pub type SAMP15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMP15` writer - SAMP15"]
pub type SAMP15_W<'a> = crate::FieldWriter<'a, u32, ADC_SMPR1_SPEC, u8, u8, 3, 15>;
#[doc = "Field `SAMP16` reader - SAMP16"]
pub type SAMP16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMP16` writer - SAMP16"]
pub type SAMP16_W<'a> = crate::FieldWriter<'a, u32, ADC_SMPR1_SPEC, u8, u8, 3, 18>;
#[doc = "Field `SAMP17` reader - SAMP17"]
pub type SAMP17_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMP17` writer - SAMP17"]
pub type SAMP17_W<'a> = crate::FieldWriter<'a, u32, ADC_SMPR1_SPEC, u8, u8, 3, 21>;
impl R {
    #[doc = "Bits 0:2 - SAMP10"]
    #[inline(always)]
    pub fn samp10(&self) -> SAMP10_R {
        SAMP10_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - SAMP11"]
    #[inline(always)]
    pub fn samp11(&self) -> SAMP11_R {
        SAMP11_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - SAMP12"]
    #[inline(always)]
    pub fn samp12(&self) -> SAMP12_R {
        SAMP12_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - SAMP13"]
    #[inline(always)]
    pub fn samp13(&self) -> SAMP13_R {
        SAMP13_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - SAMP14"]
    #[inline(always)]
    pub fn samp14(&self) -> SAMP14_R {
        SAMP14_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - SAMP15"]
    #[inline(always)]
    pub fn samp15(&self) -> SAMP15_R {
        SAMP15_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - SAMP16"]
    #[inline(always)]
    pub fn samp16(&self) -> SAMP16_R {
        SAMP16_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - SAMP17"]
    #[inline(always)]
    pub fn samp17(&self) -> SAMP17_R {
        SAMP17_R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SAMP10"]
    #[inline(always)]
    pub fn samp10(&mut self) -> SAMP10_W {
        SAMP10_W::new(self)
    }
    #[doc = "Bits 3:5 - SAMP11"]
    #[inline(always)]
    pub fn samp11(&mut self) -> SAMP11_W {
        SAMP11_W::new(self)
    }
    #[doc = "Bits 6:8 - SAMP12"]
    #[inline(always)]
    pub fn samp12(&mut self) -> SAMP12_W {
        SAMP12_W::new(self)
    }
    #[doc = "Bits 9:11 - SAMP13"]
    #[inline(always)]
    pub fn samp13(&mut self) -> SAMP13_W {
        SAMP13_W::new(self)
    }
    #[doc = "Bits 12:14 - SAMP14"]
    #[inline(always)]
    pub fn samp14(&mut self) -> SAMP14_W {
        SAMP14_W::new(self)
    }
    #[doc = "Bits 15:17 - SAMP15"]
    #[inline(always)]
    pub fn samp15(&mut self) -> SAMP15_W {
        SAMP15_W::new(self)
    }
    #[doc = "Bits 18:20 - SAMP16"]
    #[inline(always)]
    pub fn samp16(&mut self) -> SAMP16_W {
        SAMP16_W::new(self)
    }
    #[doc = "Bits 21:23 - SAMP17"]
    #[inline(always)]
    pub fn samp17(&mut self) -> SAMP17_W {
        SAMP17_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_SMPR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_smpr1](index.html) module"]
pub struct ADC_SMPR1_SPEC;
impl crate::RegisterSpec for ADC_SMPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_smpr1::R](R) reader structure"]
impl crate::Readable for ADC_SMPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_smpr1::W](W) writer structure"]
impl crate::Writable for ADC_SMPR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_SMPR1 to value 0"]
impl crate::Resettable for ADC_SMPR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
