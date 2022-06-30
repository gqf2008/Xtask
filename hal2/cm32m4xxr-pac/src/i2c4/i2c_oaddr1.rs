#[doc = "Register `I2C_OADDR1` reader"]
pub struct R(crate::R<I2C_OADDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_OADDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_OADDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_OADDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_OADDR1` writer"]
pub struct W(crate::W<I2C_OADDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_OADDR1_SPEC>;
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
impl From<crate::W<I2C_OADDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_OADDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR0` reader - ADDR0"]
pub type ADDR0_R = crate::BitReader<bool>;
#[doc = "Field `ADDR0` writer - ADDR0"]
pub type ADDR0_W<'a> = crate::BitWriter<'a, u32, I2C_OADDR1_SPEC, bool, 0>;
#[doc = "Field `ADDR` reader - ADDR"]
pub type ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR` writer - ADDR"]
pub type ADDR_W<'a> = crate::FieldWriter<'a, u32, I2C_OADDR1_SPEC, u8, u8, 7, 1>;
#[doc = "Field `ADDR_H` reader - ADDR_H"]
pub type ADDR_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR_H` writer - ADDR_H"]
pub type ADDR_H_W<'a> = crate::FieldWriter<'a, u32, I2C_OADDR1_SPEC, u8, u8, 2, 8>;
#[doc = "Field `ADDRMODE` reader - ADDRMODE"]
pub type ADDRMODE_R = crate::BitReader<bool>;
#[doc = "Field `ADDRMODE` writer - ADDRMODE"]
pub type ADDRMODE_W<'a> = crate::BitWriter<'a, u32, I2C_OADDR1_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - ADDR0"]
    #[inline(always)]
    pub fn addr0(&self) -> ADDR0_R {
        ADDR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - ADDR"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - ADDR_H"]
    #[inline(always)]
    pub fn addr_h(&self) -> ADDR_H_R {
        ADDR_H_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - ADDRMODE"]
    #[inline(always)]
    pub fn addrmode(&self) -> ADDRMODE_R {
        ADDRMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADDR0"]
    #[inline(always)]
    pub fn addr0(&mut self) -> ADDR0_W {
        ADDR0_W::new(self)
    }
    #[doc = "Bits 1:7 - ADDR"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W::new(self)
    }
    #[doc = "Bits 8:9 - ADDR_H"]
    #[inline(always)]
    pub fn addr_h(&mut self) -> ADDR_H_W {
        ADDR_H_W::new(self)
    }
    #[doc = "Bit 15 - ADDRMODE"]
    #[inline(always)]
    pub fn addrmode(&mut self) -> ADDRMODE_W {
        ADDRMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C_OADDR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_oaddr1](index.html) module"]
pub struct I2C_OADDR1_SPEC;
impl crate::RegisterSpec for I2C_OADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_oaddr1::R](R) reader structure"]
impl crate::Readable for I2C_OADDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_oaddr1::W](W) writer structure"]
impl crate::Writable for I2C_OADDR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_OADDR1 to value 0x4000"]
impl crate::Resettable for I2C_OADDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000
    }
}
