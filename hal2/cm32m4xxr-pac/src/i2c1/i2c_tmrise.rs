#[doc = "Register `I2C_TMRISE` reader"]
pub struct R(crate::R<I2C_TMRISE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_TMRISE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_TMRISE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_TMRISE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_TMRISE` writer"]
pub struct W(crate::W<I2C_TMRISE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_TMRISE_SPEC>;
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
impl From<crate::W<I2C_TMRISE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_TMRISE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMRISE` reader - TMRISE"]
pub type TMRISE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMRISE` writer - TMRISE"]
pub type TMRISE_W<'a> = crate::FieldWriter<'a, u32, I2C_TMRISE_SPEC, u8, u8, 6, 0>;
impl R {
    #[doc = "Bits 0:5 - TMRISE"]
    #[inline(always)]
    pub fn tmrise(&self) -> TMRISE_R {
        TMRISE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - TMRISE"]
    #[inline(always)]
    pub fn tmrise(&mut self) -> TMRISE_W {
        TMRISE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C_TMRISE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_tmrise](index.html) module"]
pub struct I2C_TMRISE_SPEC;
impl crate::RegisterSpec for I2C_TMRISE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_tmrise::R](R) reader structure"]
impl crate::Readable for I2C_TMRISE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_tmrise::W](W) writer structure"]
impl crate::Writable for I2C_TMRISE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_TMRISE to value 0"]
impl crate::Resettable for I2C_TMRISE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
