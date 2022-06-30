#[doc = "Register `I2C_DAT` reader"]
pub struct R(crate::R<I2C_DAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_DAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_DAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_DAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_DAT` writer"]
pub struct W(crate::W<I2C_DAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_DAT_SPEC>;
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
impl From<crate::W<I2C_DAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_DAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - DATA"]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA` writer - DATA"]
pub type DATA_W<'a> = crate::FieldWriter<'a, u32, I2C_DAT_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - DATA"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C_DAT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_dat](index.html) module"]
pub struct I2C_DAT_SPEC;
impl crate::RegisterSpec for I2C_DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_dat::R](R) reader structure"]
impl crate::Readable for I2C_DAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_dat::W](W) writer structure"]
impl crate::Writable for I2C_DAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_DAT to value 0"]
impl crate::Resettable for I2C_DAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
