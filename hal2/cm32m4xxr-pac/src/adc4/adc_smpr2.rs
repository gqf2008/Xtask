#[doc = "Register `ADC_SMPR2` reader"]
pub struct R(crate::R<ADC_SMPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_SMPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_SMPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_SMPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_SMPR2` writer"]
pub struct W(crate::W<ADC_SMPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_SMPR2_SPEC>;
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
impl From<crate::W<ADC_SMPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_SMPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAMP0` reader - SAMP0"]
pub type SAMP0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMP0` writer - SAMP0"]
pub type SAMP0_W<'a> = crate::FieldWriter<'a, u32, ADC_SMPR2_SPEC, u8, u8, 3, 0>;
#[doc = "Field `SAMP1` reader - SAMP1"]
pub type SAMP1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMP1` writer - SAMP1"]
pub type SAMP1_W<'a> = crate::FieldWriter<'a, u32, ADC_SMPR2_SPEC, u8, u8, 3, 3>;
#[doc = "Field `SAMP2` reader - SAMP2"]
pub type SAMP2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMP2` writer - SAMP2"]
pub type SAMP2_W<'a> = crate::FieldWriter<'a, u32, ADC_SMPR2_SPEC, u8, u8, 3, 6>;
#[doc = "Field `SAMP3` reader - SAMP3"]
pub type SAMP3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMP3` writer - SAMP3"]
pub type SAMP3_W<'a> = crate::FieldWriter<'a, u32, ADC_SMPR2_SPEC, u8, u8, 3, 9>;
#[doc = "Field `SAMP4` reader - SAMP4"]
pub type SAMP4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMP4` writer - SAMP4"]
pub type SAMP4_W<'a> = crate::FieldWriter<'a, u32, ADC_SMPR2_SPEC, u8, u8, 3, 12>;
#[doc = "Field `SAMP5` reader - SAMP5"]
pub type SAMP5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMP5` writer - SAMP5"]
pub type SAMP5_W<'a> = crate::FieldWriter<'a, u32, ADC_SMPR2_SPEC, u8, u8, 3, 15>;
#[doc = "Field `SAMP6` reader - SAMP6"]
pub type SAMP6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMP6` writer - SAMP6"]
pub type SAMP6_W<'a> = crate::FieldWriter<'a, u32, ADC_SMPR2_SPEC, u8, u8, 3, 18>;
#[doc = "Field `SAMP7` reader - SAMP7"]
pub type SAMP7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMP7` writer - SAMP7"]
pub type SAMP7_W<'a> = crate::FieldWriter<'a, u32, ADC_SMPR2_SPEC, u8, u8, 3, 21>;
#[doc = "Field `SAMP8` reader - SAMP8"]
pub type SAMP8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMP8` writer - SAMP8"]
pub type SAMP8_W<'a> = crate::FieldWriter<'a, u32, ADC_SMPR2_SPEC, u8, u8, 3, 24>;
#[doc = "Field `SAMP9` reader - SAMP9"]
pub type SAMP9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMP9` writer - SAMP9"]
pub type SAMP9_W<'a> = crate::FieldWriter<'a, u32, ADC_SMPR2_SPEC, u8, u8, 3, 27>;
impl R {
    #[doc = "Bits 0:2 - SAMP0"]
    #[inline(always)]
    pub fn samp0(&self) -> SAMP0_R {
        SAMP0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - SAMP1"]
    #[inline(always)]
    pub fn samp1(&self) -> SAMP1_R {
        SAMP1_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - SAMP2"]
    #[inline(always)]
    pub fn samp2(&self) -> SAMP2_R {
        SAMP2_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - SAMP3"]
    #[inline(always)]
    pub fn samp3(&self) -> SAMP3_R {
        SAMP3_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - SAMP4"]
    #[inline(always)]
    pub fn samp4(&self) -> SAMP4_R {
        SAMP4_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - SAMP5"]
    #[inline(always)]
    pub fn samp5(&self) -> SAMP5_R {
        SAMP5_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - SAMP6"]
    #[inline(always)]
    pub fn samp6(&self) -> SAMP6_R {
        SAMP6_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - SAMP7"]
    #[inline(always)]
    pub fn samp7(&self) -> SAMP7_R {
        SAMP7_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - SAMP8"]
    #[inline(always)]
    pub fn samp8(&self) -> SAMP8_R {
        SAMP8_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - SAMP9"]
    #[inline(always)]
    pub fn samp9(&self) -> SAMP9_R {
        SAMP9_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SAMP0"]
    #[inline(always)]
    pub fn samp0(&mut self) -> SAMP0_W {
        SAMP0_W::new(self)
    }
    #[doc = "Bits 3:5 - SAMP1"]
    #[inline(always)]
    pub fn samp1(&mut self) -> SAMP1_W {
        SAMP1_W::new(self)
    }
    #[doc = "Bits 6:8 - SAMP2"]
    #[inline(always)]
    pub fn samp2(&mut self) -> SAMP2_W {
        SAMP2_W::new(self)
    }
    #[doc = "Bits 9:11 - SAMP3"]
    #[inline(always)]
    pub fn samp3(&mut self) -> SAMP3_W {
        SAMP3_W::new(self)
    }
    #[doc = "Bits 12:14 - SAMP4"]
    #[inline(always)]
    pub fn samp4(&mut self) -> SAMP4_W {
        SAMP4_W::new(self)
    }
    #[doc = "Bits 15:17 - SAMP5"]
    #[inline(always)]
    pub fn samp5(&mut self) -> SAMP5_W {
        SAMP5_W::new(self)
    }
    #[doc = "Bits 18:20 - SAMP6"]
    #[inline(always)]
    pub fn samp6(&mut self) -> SAMP6_W {
        SAMP6_W::new(self)
    }
    #[doc = "Bits 21:23 - SAMP7"]
    #[inline(always)]
    pub fn samp7(&mut self) -> SAMP7_W {
        SAMP7_W::new(self)
    }
    #[doc = "Bits 24:26 - SAMP8"]
    #[inline(always)]
    pub fn samp8(&mut self) -> SAMP8_W {
        SAMP8_W::new(self)
    }
    #[doc = "Bits 27:29 - SAMP9"]
    #[inline(always)]
    pub fn samp9(&mut self) -> SAMP9_W {
        SAMP9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_SMPR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_smpr2](index.html) module"]
pub struct ADC_SMPR2_SPEC;
impl crate::RegisterSpec for ADC_SMPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_smpr2::R](R) reader structure"]
impl crate::Readable for ADC_SMPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_smpr2::W](W) writer structure"]
impl crate::Writable for ADC_SMPR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_SMPR2 to value 0"]
impl crate::Resettable for ADC_SMPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
