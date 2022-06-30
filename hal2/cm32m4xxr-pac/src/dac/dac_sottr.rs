#[doc = "Register `DAC_SOTTR` reader"]
pub struct R(crate::R<DAC_SOTTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_SOTTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_SOTTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_SOTTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_SOTTR` writer"]
pub struct W(crate::W<DAC_SOTTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_SOTTR_SPEC>;
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
impl From<crate::W<DAC_SOTTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_SOTTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TR1EN` reader - TR1EN"]
pub type TR1EN_R = crate::BitReader<bool>;
#[doc = "Field `TR1EN` writer - TR1EN"]
pub type TR1EN_W<'a> = crate::BitWriter<'a, u32, DAC_SOTTR_SPEC, bool, 0>;
#[doc = "Field `TR2EN` reader - TR2EN"]
pub type TR2EN_R = crate::BitReader<bool>;
#[doc = "Field `TR2EN` writer - TR2EN"]
pub type TR2EN_W<'a> = crate::BitWriter<'a, u32, DAC_SOTTR_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - TR1EN"]
    #[inline(always)]
    pub fn tr1en(&self) -> TR1EN_R {
        TR1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TR2EN"]
    #[inline(always)]
    pub fn tr2en(&self) -> TR2EN_R {
        TR2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TR1EN"]
    #[inline(always)]
    pub fn tr1en(&mut self) -> TR1EN_W {
        TR1EN_W::new(self)
    }
    #[doc = "Bit 1 - TR2EN"]
    #[inline(always)]
    pub fn tr2en(&mut self) -> TR2EN_W {
        TR2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC_SOTTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_sottr](index.html) module"]
pub struct DAC_SOTTR_SPEC;
impl crate::RegisterSpec for DAC_SOTTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_sottr::R](R) reader structure"]
impl crate::Readable for DAC_SOTTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_sottr::W](W) writer structure"]
impl crate::Writable for DAC_SOTTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC_SOTTR to value 0"]
impl crate::Resettable for DAC_SOTTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
