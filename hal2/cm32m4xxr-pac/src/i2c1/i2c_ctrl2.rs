#[doc = "Register `I2C_CTRL2` reader"]
pub struct R(crate::R<I2C_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_CTRL2` writer"]
pub struct W(crate::W<I2C_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_CTRL2_SPEC>;
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
impl From<crate::W<I2C_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKFREQ` reader - CLKFREQ"]
pub type CLKFREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKFREQ` writer - CLKFREQ"]
pub type CLKFREQ_W<'a> = crate::FieldWriter<'a, u32, I2C_CTRL2_SPEC, u8, u8, 6, 0>;
#[doc = "Field `ERRINTEN` reader - ERRINTEN"]
pub type ERRINTEN_R = crate::BitReader<bool>;
#[doc = "Field `ERRINTEN` writer - ERRINTEN"]
pub type ERRINTEN_W<'a> = crate::BitWriter<'a, u32, I2C_CTRL2_SPEC, bool, 8>;
#[doc = "Field `EVTINTEN` reader - EVTINTEN"]
pub type EVTINTEN_R = crate::BitReader<bool>;
#[doc = "Field `EVTINTEN` writer - EVTINTEN"]
pub type EVTINTEN_W<'a> = crate::BitWriter<'a, u32, I2C_CTRL2_SPEC, bool, 9>;
#[doc = "Field `BUFINTEN` reader - BUFINTEN"]
pub type BUFINTEN_R = crate::BitReader<bool>;
#[doc = "Field `BUFINTEN` writer - BUFINTEN"]
pub type BUFINTEN_W<'a> = crate::BitWriter<'a, u32, I2C_CTRL2_SPEC, bool, 10>;
#[doc = "Field `DMAEN` reader - DMAEN"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - DMAEN"]
pub type DMAEN_W<'a> = crate::BitWriter<'a, u32, I2C_CTRL2_SPEC, bool, 11>;
#[doc = "Field `DMALAST` reader - DMALAST"]
pub type DMALAST_R = crate::BitReader<bool>;
#[doc = "Field `DMALAST` writer - DMALAST"]
pub type DMALAST_W<'a> = crate::BitWriter<'a, u32, I2C_CTRL2_SPEC, bool, 12>;
impl R {
    #[doc = "Bits 0:5 - CLKFREQ"]
    #[inline(always)]
    pub fn clkfreq(&self) -> CLKFREQ_R {
        CLKFREQ_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - ERRINTEN"]
    #[inline(always)]
    pub fn errinten(&self) -> ERRINTEN_R {
        ERRINTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EVTINTEN"]
    #[inline(always)]
    pub fn evtinten(&self) -> EVTINTEN_R {
        EVTINTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BUFINTEN"]
    #[inline(always)]
    pub fn bufinten(&self) -> BUFINTEN_R {
        BUFINTEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMALAST"]
    #[inline(always)]
    pub fn dmalast(&self) -> DMALAST_R {
        DMALAST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - CLKFREQ"]
    #[inline(always)]
    pub fn clkfreq(&mut self) -> CLKFREQ_W {
        CLKFREQ_W::new(self)
    }
    #[doc = "Bit 8 - ERRINTEN"]
    #[inline(always)]
    pub fn errinten(&mut self) -> ERRINTEN_W {
        ERRINTEN_W::new(self)
    }
    #[doc = "Bit 9 - EVTINTEN"]
    #[inline(always)]
    pub fn evtinten(&mut self) -> EVTINTEN_W {
        EVTINTEN_W::new(self)
    }
    #[doc = "Bit 10 - BUFINTEN"]
    #[inline(always)]
    pub fn bufinten(&mut self) -> BUFINTEN_W {
        BUFINTEN_W::new(self)
    }
    #[doc = "Bit 11 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 12 - DMALAST"]
    #[inline(always)]
    pub fn dmalast(&mut self) -> DMALAST_W {
        DMALAST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C_CTRL2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_ctrl2](index.html) module"]
pub struct I2C_CTRL2_SPEC;
impl crate::RegisterSpec for I2C_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_ctrl2::R](R) reader structure"]
impl crate::Readable for I2C_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_ctrl2::W](W) writer structure"]
impl crate::Writable for I2C_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_CTRL2 to value 0"]
impl crate::Resettable for I2C_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
