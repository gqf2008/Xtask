#[doc = "Register `ADC_STS` reader"]
pub struct R(crate::R<ADC_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_STS` writer"]
pub struct W(crate::W<ADC_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_STS_SPEC>;
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
impl From<crate::W<ADC_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AWDG` reader - AWDG"]
pub type AWDG_R = crate::BitReader<bool>;
#[doc = "Field `AWDG` writer - AWDG"]
pub type AWDG_W<'a> = crate::BitWriter<'a, u32, ADC_STS_SPEC, bool, 0>;
#[doc = "Field `ENDC` reader - ENDC"]
pub type ENDC_R = crate::BitReader<bool>;
#[doc = "Field `ENDC` writer - ENDC"]
pub type ENDC_W<'a> = crate::BitWriter<'a, u32, ADC_STS_SPEC, bool, 1>;
#[doc = "Field `JENDC` reader - JENDC"]
pub type JENDC_R = crate::BitReader<bool>;
#[doc = "Field `JENDC` writer - JENDC"]
pub type JENDC_W<'a> = crate::BitWriter<'a, u32, ADC_STS_SPEC, bool, 2>;
#[doc = "Field `JSTR` reader - JSTR"]
pub type JSTR_R = crate::BitReader<bool>;
#[doc = "Field `JSTR` writer - JSTR"]
pub type JSTR_W<'a> = crate::BitWriter<'a, u32, ADC_STS_SPEC, bool, 3>;
#[doc = "Field `STR` reader - STR"]
pub type STR_R = crate::BitReader<bool>;
#[doc = "Field `STR` writer - STR"]
pub type STR_W<'a> = crate::BitWriter<'a, u32, ADC_STS_SPEC, bool, 4>;
#[doc = "Field `ENDCA` reader - ENDCA"]
pub type ENDCA_R = crate::BitReader<bool>;
#[doc = "Field `ENDCA` writer - ENDCA"]
pub type ENDCA_W<'a> = crate::BitWriter<'a, u32, ADC_STS_SPEC, bool, 5>;
#[doc = "Field `JENDCA` reader - JENDCA"]
pub type JENDCA_R = crate::BitReader<bool>;
#[doc = "Field `JENDCA` writer - JENDCA"]
pub type JENDCA_W<'a> = crate::BitWriter<'a, u32, ADC_STS_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 0 - AWDG"]
    #[inline(always)]
    pub fn awdg(&self) -> AWDG_R {
        AWDG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ENDC"]
    #[inline(always)]
    pub fn endc(&self) -> ENDC_R {
        ENDC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - JENDC"]
    #[inline(always)]
    pub fn jendc(&self) -> JENDC_R {
        JENDC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - JSTR"]
    #[inline(always)]
    pub fn jstr(&self) -> JSTR_R {
        JSTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - STR"]
    #[inline(always)]
    pub fn str(&self) -> STR_R {
        STR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ENDCA"]
    #[inline(always)]
    pub fn endca(&self) -> ENDCA_R {
        ENDCA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - JENDCA"]
    #[inline(always)]
    pub fn jendca(&self) -> JENDCA_R {
        JENDCA_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AWDG"]
    #[inline(always)]
    pub fn awdg(&mut self) -> AWDG_W {
        AWDG_W::new(self)
    }
    #[doc = "Bit 1 - ENDC"]
    #[inline(always)]
    pub fn endc(&mut self) -> ENDC_W {
        ENDC_W::new(self)
    }
    #[doc = "Bit 2 - JENDC"]
    #[inline(always)]
    pub fn jendc(&mut self) -> JENDC_W {
        JENDC_W::new(self)
    }
    #[doc = "Bit 3 - JSTR"]
    #[inline(always)]
    pub fn jstr(&mut self) -> JSTR_W {
        JSTR_W::new(self)
    }
    #[doc = "Bit 4 - STR"]
    #[inline(always)]
    pub fn str(&mut self) -> STR_W {
        STR_W::new(self)
    }
    #[doc = "Bit 5 - ENDCA"]
    #[inline(always)]
    pub fn endca(&mut self) -> ENDCA_W {
        ENDCA_W::new(self)
    }
    #[doc = "Bit 6 - JENDCA"]
    #[inline(always)]
    pub fn jendca(&mut self) -> JENDCA_W {
        JENDCA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_STS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_sts](index.html) module"]
pub struct ADC_STS_SPEC;
impl crate::RegisterSpec for ADC_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_sts::R](R) reader structure"]
impl crate::Readable for ADC_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_sts::W](W) writer structure"]
impl crate::Writable for ADC_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_STS to value 0"]
impl crate::Resettable for ADC_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
