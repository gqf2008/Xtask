#[doc = "Register `RCC_AHBPRST` reader"]
pub struct R(crate::R<RCC_AHBPRST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_AHBPRST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_AHBPRST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_AHBPRST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_AHBPRST` writer"]
pub struct W(crate::W<RCC_AHBPRST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_AHBPRST_SPEC>;
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
impl From<crate::W<RCC_AHBPRST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_AHBPRST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RNGCRST` reader - RNGCRST"]
pub type RNGCRST_R = crate::BitReader<bool>;
#[doc = "Field `RNGCRST` writer - RNGCRST"]
pub type RNGCRST_W<'a> = crate::BitWriter<'a, u32, RCC_AHBPRST_SPEC, bool, 9>;
#[doc = "Field `SACRST` reader - SACRST"]
pub type SACRST_R = crate::BitReader<bool>;
#[doc = "Field `SACRST` writer - SACRST"]
pub type SACRST_W<'a> = crate::BitWriter<'a, u32, RCC_AHBPRST_SPEC, bool, 11>;
#[doc = "Field `ADC1RST` reader - ADC1RST"]
pub type ADC1RST_R = crate::BitReader<bool>;
#[doc = "Field `ADC1RST` writer - ADC1RST"]
pub type ADC1RST_W<'a> = crate::BitWriter<'a, u32, RCC_AHBPRST_SPEC, bool, 12>;
#[doc = "Field `ADC2RST` reader - ADC2RST"]
pub type ADC2RST_R = crate::BitReader<bool>;
#[doc = "Field `ADC2RST` writer - ADC2RST"]
pub type ADC2RST_W<'a> = crate::BitWriter<'a, u32, RCC_AHBPRST_SPEC, bool, 13>;
#[doc = "Field `ADC3RST` reader - ADC3RST"]
pub type ADC3RST_R = crate::BitReader<bool>;
#[doc = "Field `ADC3RST` writer - ADC3RST"]
pub type ADC3RST_W<'a> = crate::BitWriter<'a, u32, RCC_AHBPRST_SPEC, bool, 14>;
#[doc = "Field `ADC4RST` reader - ADC4RST"]
pub type ADC4RST_R = crate::BitReader<bool>;
#[doc = "Field `ADC4RST` writer - ADC4RST"]
pub type ADC4RST_W<'a> = crate::BitWriter<'a, u32, RCC_AHBPRST_SPEC, bool, 15>;
#[doc = "Field `QSPIRST` reader - QSPIRST"]
pub type QSPIRST_R = crate::BitReader<bool>;
#[doc = "Field `QSPIRST` writer - QSPIRST"]
pub type QSPIRST_W<'a> = crate::BitWriter<'a, u32, RCC_AHBPRST_SPEC, bool, 17>;
impl R {
    #[doc = "Bit 9 - RNGCRST"]
    #[inline(always)]
    pub fn rngcrst(&self) -> RNGCRST_R {
        RNGCRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - SACRST"]
    #[inline(always)]
    pub fn sacrst(&self) -> SACRST_R {
        SACRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ADC1RST"]
    #[inline(always)]
    pub fn adc1rst(&self) -> ADC1RST_R {
        ADC1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC2RST"]
    #[inline(always)]
    pub fn adc2rst(&self) -> ADC2RST_R {
        ADC2RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC3RST"]
    #[inline(always)]
    pub fn adc3rst(&self) -> ADC3RST_R {
        ADC3RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC4RST"]
    #[inline(always)]
    pub fn adc4rst(&self) -> ADC4RST_R {
        ADC4RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - QSPIRST"]
    #[inline(always)]
    pub fn qspirst(&self) -> QSPIRST_R {
        QSPIRST_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - RNGCRST"]
    #[inline(always)]
    pub fn rngcrst(&mut self) -> RNGCRST_W {
        RNGCRST_W::new(self)
    }
    #[doc = "Bit 11 - SACRST"]
    #[inline(always)]
    pub fn sacrst(&mut self) -> SACRST_W {
        SACRST_W::new(self)
    }
    #[doc = "Bit 12 - ADC1RST"]
    #[inline(always)]
    pub fn adc1rst(&mut self) -> ADC1RST_W {
        ADC1RST_W::new(self)
    }
    #[doc = "Bit 13 - ADC2RST"]
    #[inline(always)]
    pub fn adc2rst(&mut self) -> ADC2RST_W {
        ADC2RST_W::new(self)
    }
    #[doc = "Bit 14 - ADC3RST"]
    #[inline(always)]
    pub fn adc3rst(&mut self) -> ADC3RST_W {
        ADC3RST_W::new(self)
    }
    #[doc = "Bit 15 - ADC4RST"]
    #[inline(always)]
    pub fn adc4rst(&mut self) -> ADC4RST_W {
        ADC4RST_W::new(self)
    }
    #[doc = "Bit 17 - QSPIRST"]
    #[inline(always)]
    pub fn qspirst(&mut self) -> QSPIRST_W {
        QSPIRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC_AHBPRST\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ahbprst](index.html) module"]
pub struct RCC_AHBPRST_SPEC;
impl crate::RegisterSpec for RCC_AHBPRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ahbprst::R](R) reader structure"]
impl crate::Readable for RCC_AHBPRST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ahbprst::W](W) writer structure"]
impl crate::Writable for RCC_AHBPRST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_AHBPRST to value 0"]
impl crate::Resettable for RCC_AHBPRST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
