#[doc = "Register `I2C_STS2` reader"]
pub struct R(crate::R<I2C_STS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_STS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_STS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_STS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_STS2` writer"]
pub struct W(crate::W<I2C_STS2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_STS2_SPEC>;
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
impl From<crate::W<I2C_STS2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_STS2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSMODE` reader - MSMODE"]
pub type MSMODE_R = crate::BitReader<bool>;
#[doc = "Field `MSMODE` writer - MSMODE"]
pub type MSMODE_W<'a> = crate::BitWriter<'a, u32, I2C_STS2_SPEC, bool, 0>;
#[doc = "Field `BUSY` reader - BUSY"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - BUSY"]
pub type BUSY_W<'a> = crate::BitWriter<'a, u32, I2C_STS2_SPEC, bool, 1>;
#[doc = "Field `TRF` reader - TRF"]
pub type TRF_R = crate::BitReader<bool>;
#[doc = "Field `TRF` writer - TRF"]
pub type TRF_W<'a> = crate::BitWriter<'a, u32, I2C_STS2_SPEC, bool, 2>;
#[doc = "Field `GCALLADDR` reader - GCALLADDR"]
pub type GCALLADDR_R = crate::BitReader<bool>;
#[doc = "Field `GCALLADDR` writer - GCALLADDR"]
pub type GCALLADDR_W<'a> = crate::BitWriter<'a, u32, I2C_STS2_SPEC, bool, 4>;
#[doc = "Field `SMBDADDR` reader - SMBDADDR"]
pub type SMBDADDR_R = crate::BitReader<bool>;
#[doc = "Field `SMBDADDR` writer - SMBDADDR"]
pub type SMBDADDR_W<'a> = crate::BitWriter<'a, u32, I2C_STS2_SPEC, bool, 5>;
#[doc = "Field `SMBHADDR` reader - SMBHADDR"]
pub type SMBHADDR_R = crate::BitReader<bool>;
#[doc = "Field `SMBHADDR` writer - SMBHADDR"]
pub type SMBHADDR_W<'a> = crate::BitWriter<'a, u32, I2C_STS2_SPEC, bool, 6>;
#[doc = "Field `DUALFLAG` reader - DUALFLAG"]
pub type DUALFLAG_R = crate::BitReader<bool>;
#[doc = "Field `DUALFLAG` writer - DUALFLAG"]
pub type DUALFLAG_W<'a> = crate::BitWriter<'a, u32, I2C_STS2_SPEC, bool, 7>;
#[doc = "Field `PECVAL` reader - PECVAL"]
pub type PECVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PECVAL` writer - PECVAL"]
pub type PECVAL_W<'a> = crate::FieldWriter<'a, u32, I2C_STS2_SPEC, u8, u8, 8, 8>;
impl R {
    #[doc = "Bit 0 - MSMODE"]
    #[inline(always)]
    pub fn msmode(&self) -> MSMODE_R {
        MSMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TRF"]
    #[inline(always)]
    pub fn trf(&self) -> TRF_R {
        TRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - GCALLADDR"]
    #[inline(always)]
    pub fn gcalladdr(&self) -> GCALLADDR_R {
        GCALLADDR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SMBDADDR"]
    #[inline(always)]
    pub fn smbdaddr(&self) -> SMBDADDR_R {
        SMBDADDR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SMBHADDR"]
    #[inline(always)]
    pub fn smbhaddr(&self) -> SMBHADDR_R {
        SMBHADDR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DUALFLAG"]
    #[inline(always)]
    pub fn dualflag(&self) -> DUALFLAG_R {
        DUALFLAG_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - PECVAL"]
    #[inline(always)]
    pub fn pecval(&self) -> PECVAL_R {
        PECVAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MSMODE"]
    #[inline(always)]
    pub fn msmode(&mut self) -> MSMODE_W {
        MSMODE_W::new(self)
    }
    #[doc = "Bit 1 - BUSY"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W::new(self)
    }
    #[doc = "Bit 2 - TRF"]
    #[inline(always)]
    pub fn trf(&mut self) -> TRF_W {
        TRF_W::new(self)
    }
    #[doc = "Bit 4 - GCALLADDR"]
    #[inline(always)]
    pub fn gcalladdr(&mut self) -> GCALLADDR_W {
        GCALLADDR_W::new(self)
    }
    #[doc = "Bit 5 - SMBDADDR"]
    #[inline(always)]
    pub fn smbdaddr(&mut self) -> SMBDADDR_W {
        SMBDADDR_W::new(self)
    }
    #[doc = "Bit 6 - SMBHADDR"]
    #[inline(always)]
    pub fn smbhaddr(&mut self) -> SMBHADDR_W {
        SMBHADDR_W::new(self)
    }
    #[doc = "Bit 7 - DUALFLAG"]
    #[inline(always)]
    pub fn dualflag(&mut self) -> DUALFLAG_W {
        DUALFLAG_W::new(self)
    }
    #[doc = "Bits 8:15 - PECVAL"]
    #[inline(always)]
    pub fn pecval(&mut self) -> PECVAL_W {
        PECVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C_STS2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_sts2](index.html) module"]
pub struct I2C_STS2_SPEC;
impl crate::RegisterSpec for I2C_STS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_sts2::R](R) reader structure"]
impl crate::Readable for I2C_STS2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_sts2::W](W) writer structure"]
impl crate::Writable for I2C_STS2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_STS2 to value 0"]
impl crate::Resettable for I2C_STS2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
