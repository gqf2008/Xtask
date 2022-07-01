#[doc = "Register `I2C_OADDR2` reader"]
pub struct R(crate::R<I2C_OADDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_OADDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_OADDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_OADDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_OADDR2` writer"]
pub struct W(crate::W<I2C_OADDR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_OADDR2_SPEC>;
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
impl From<crate::W<I2C_OADDR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_OADDR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUALEN` reader - DUALEN"]
pub type DUALEN_R = crate::BitReader<bool>;
#[doc = "Field `DUALEN` writer - DUALEN"]
pub type DUALEN_W<'a> = crate::BitWriter<'a, u32, I2C_OADDR2_SPEC, bool, 0>;
#[doc = "Field `ADDR2` reader - ADDR2"]
pub type ADDR2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR2` writer - ADDR2"]
pub type ADDR2_W<'a> = crate::FieldWriter<'a, u32, I2C_OADDR2_SPEC, u8, u8, 7, 1>;
impl R {
    #[doc = "Bit 0 - DUALEN"]
    #[inline(always)]
    pub fn dualen(&self) -> DUALEN_R {
        DUALEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - ADDR2"]
    #[inline(always)]
    pub fn addr2(&self) -> ADDR2_R {
        ADDR2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DUALEN"]
    #[inline(always)]
    pub fn dualen(&mut self) -> DUALEN_W {
        DUALEN_W::new(self)
    }
    #[doc = "Bits 1:7 - ADDR2"]
    #[inline(always)]
    pub fn addr2(&mut self) -> ADDR2_W {
        ADDR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C_OADDR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_oaddr2](index.html) module"]
pub struct I2C_OADDR2_SPEC;
impl crate::RegisterSpec for I2C_OADDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_oaddr2::R](R) reader structure"]
impl crate::Readable for I2C_OADDR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_oaddr2::W](W) writer structure"]
impl crate::Writable for I2C_OADDR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_OADDR2 to value 0"]
impl crate::Resettable for I2C_OADDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
