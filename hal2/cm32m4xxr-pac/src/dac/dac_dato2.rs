#[doc = "Register `DAC_DATO2` reader"]
pub struct R(crate::R<DAC_DATO2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_DATO2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_DATO2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_DATO2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_DATO2` writer"]
pub struct W(crate::W<DAC_DATO2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_DATO2_SPEC>;
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
impl From<crate::W<DAC_DATO2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_DATO2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACCH2DO` reader - DACCH2DO"]
pub type DACCH2DO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DACCH2DO` writer - DACCH2DO"]
pub type DACCH2DO_W<'a> = crate::FieldWriter<'a, u32, DAC_DATO2_SPEC, u16, u16, 12, 0>;
impl R {
    #[doc = "Bits 0:11 - DACCH2DO"]
    #[inline(always)]
    pub fn dacch2do(&self) -> DACCH2DO_R {
        DACCH2DO_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DACCH2DO"]
    #[inline(always)]
    pub fn dacch2do(&mut self) -> DACCH2DO_W {
        DACCH2DO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC_DATO2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_dato2](index.html) module"]
pub struct DAC_DATO2_SPEC;
impl crate::RegisterSpec for DAC_DATO2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_dato2::R](R) reader structure"]
impl crate::Readable for DAC_DATO2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_dato2::W](W) writer structure"]
impl crate::Writable for DAC_DATO2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC_DATO2 to value 0"]
impl crate::Resettable for DAC_DATO2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
