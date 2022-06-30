#[doc = "Register `RCC_CFG2` reader"]
pub struct R(crate::R<RCC_CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_CFG2` writer"]
pub struct W(crate::W<RCC_CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CFG2_SPEC>;
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
impl From<crate::W<RCC_CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCHPRES` reader - ADCHPRES"]
pub type ADCHPRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCHPRES` writer - ADCHPRES"]
pub type ADCHPRES_W<'a> = crate::FieldWriter<'a, u32, RCC_CFG2_SPEC, u8, u8, 4, 0>;
#[doc = "Field `ADCPLLPRES` reader - ADCPLLPRES"]
pub type ADCPLLPRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADCPLLPRES` writer - ADCPLLPRES"]
pub type ADCPLLPRES_W<'a> = crate::FieldWriter<'a, u32, RCC_CFG2_SPEC, u8, u8, 5, 4>;
#[doc = "Field `ADC1MSEL` reader - ADC1MSEL"]
pub type ADC1MSEL_R = crate::BitReader<bool>;
#[doc = "Field `ADC1MSEL` writer - ADC1MSEL"]
pub type ADC1MSEL_W<'a> = crate::BitWriter<'a, u32, RCC_CFG2_SPEC, bool, 10>;
#[doc = "Field `ADC1MPRES` reader - ADC1MPRES"]
pub type ADC1MPRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC1MPRES` writer - ADC1MPRES"]
pub type ADC1MPRES_W<'a> = crate::FieldWriter<'a, u32, RCC_CFG2_SPEC, u8, u8, 5, 11>;
#[doc = "Field `RNGCPRES` reader - RNGCPRES"]
pub type RNGCPRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RNGCPRES` writer - RNGCPRES"]
pub type RNGCPRES_W<'a> = crate::FieldWriter<'a, u32, RCC_CFG2_SPEC, u8, u8, 5, 24>;
#[doc = "Field `TIMCLKSEL` reader - TIMCLKSEL"]
pub type TIMCLKSEL_R = crate::BitReader<bool>;
#[doc = "Field `TIMCLKSEL` writer - TIMCLKSEL"]
pub type TIMCLKSEL_W<'a> = crate::BitWriter<'a, u32, RCC_CFG2_SPEC, bool, 29>;
impl R {
    #[doc = "Bits 0:3 - ADCHPRES"]
    #[inline(always)]
    pub fn adchpres(&self) -> ADCHPRES_R {
        ADCHPRES_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - ADCPLLPRES"]
    #[inline(always)]
    pub fn adcpllpres(&self) -> ADCPLLPRES_R {
        ADCPLLPRES_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 10 - ADC1MSEL"]
    #[inline(always)]
    pub fn adc1msel(&self) -> ADC1MSEL_R {
        ADC1MSEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:15 - ADC1MPRES"]
    #[inline(always)]
    pub fn adc1mpres(&self) -> ADC1MPRES_R {
        ADC1MPRES_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - RNGCPRES"]
    #[inline(always)]
    pub fn rngcpres(&self) -> RNGCPRES_R {
        RNGCPRES_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - TIMCLKSEL"]
    #[inline(always)]
    pub fn timclksel(&self) -> TIMCLKSEL_R {
        TIMCLKSEL_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADCHPRES"]
    #[inline(always)]
    pub fn adchpres(&mut self) -> ADCHPRES_W {
        ADCHPRES_W::new(self)
    }
    #[doc = "Bits 4:8 - ADCPLLPRES"]
    #[inline(always)]
    pub fn adcpllpres(&mut self) -> ADCPLLPRES_W {
        ADCPLLPRES_W::new(self)
    }
    #[doc = "Bit 10 - ADC1MSEL"]
    #[inline(always)]
    pub fn adc1msel(&mut self) -> ADC1MSEL_W {
        ADC1MSEL_W::new(self)
    }
    #[doc = "Bits 11:15 - ADC1MPRES"]
    #[inline(always)]
    pub fn adc1mpres(&mut self) -> ADC1MPRES_W {
        ADC1MPRES_W::new(self)
    }
    #[doc = "Bits 24:28 - RNGCPRES"]
    #[inline(always)]
    pub fn rngcpres(&mut self) -> RNGCPRES_W {
        RNGCPRES_W::new(self)
    }
    #[doc = "Bit 29 - TIMCLKSEL"]
    #[inline(always)]
    pub fn timclksel(&mut self) -> TIMCLKSEL_W {
        TIMCLKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC_CFG2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_cfg2](index.html) module"]
pub struct RCC_CFG2_SPEC;
impl crate::RegisterSpec for RCC_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_cfg2::R](R) reader structure"]
impl crate::Readable for RCC_CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_cfg2::W](W) writer structure"]
impl crate::Writable for RCC_CFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_CFG2 to value 0x3800"]
impl crate::Resettable for RCC_CFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3800
    }
}
