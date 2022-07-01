#[doc = "Register `I2C_STS1` reader"]
pub struct R(crate::R<I2C_STS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_STS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_STS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_STS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_STS1` writer"]
pub struct W(crate::W<I2C_STS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_STS1_SPEC>;
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
impl From<crate::W<I2C_STS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_STS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STARTBF` reader - STARTBF"]
pub type STARTBF_R = crate::BitReader<bool>;
#[doc = "Field `STARTBF` writer - STARTBF"]
pub type STARTBF_W<'a> = crate::BitWriter<'a, u32, I2C_STS1_SPEC, bool, 0>;
#[doc = "Field `ADDRF` reader - ADDRF"]
pub type ADDRF_R = crate::BitReader<bool>;
#[doc = "Field `ADDRF` writer - ADDRF"]
pub type ADDRF_W<'a> = crate::BitWriter<'a, u32, I2C_STS1_SPEC, bool, 1>;
#[doc = "Field `BSF` reader - BSF"]
pub type BSF_R = crate::BitReader<bool>;
#[doc = "Field `BSF` writer - BSF"]
pub type BSF_W<'a> = crate::BitWriter<'a, u32, I2C_STS1_SPEC, bool, 2>;
#[doc = "Field `ADDR10F` reader - ADDR10F"]
pub type ADDR10F_R = crate::BitReader<bool>;
#[doc = "Field `ADDR10F` writer - ADDR10F"]
pub type ADDR10F_W<'a> = crate::BitWriter<'a, u32, I2C_STS1_SPEC, bool, 3>;
#[doc = "Field `STOPF` reader - STOPF"]
pub type STOPF_R = crate::BitReader<bool>;
#[doc = "Field `STOPF` writer - STOPF"]
pub type STOPF_W<'a> = crate::BitWriter<'a, u32, I2C_STS1_SPEC, bool, 4>;
#[doc = "Field `RXDATNE` reader - RXDATNE"]
pub type RXDATNE_R = crate::BitReader<bool>;
#[doc = "Field `RXDATNE` writer - RXDATNE"]
pub type RXDATNE_W<'a> = crate::BitWriter<'a, u32, I2C_STS1_SPEC, bool, 6>;
#[doc = "Field `TXDATE` reader - TXDATE"]
pub type TXDATE_R = crate::BitReader<bool>;
#[doc = "Field `TXDATE` writer - TXDATE"]
pub type TXDATE_W<'a> = crate::BitWriter<'a, u32, I2C_STS1_SPEC, bool, 7>;
#[doc = "Field `BUSERR` reader - BUSERR"]
pub type BUSERR_R = crate::BitReader<bool>;
#[doc = "Field `BUSERR` writer - BUSERR"]
pub type BUSERR_W<'a> = crate::BitWriter<'a, u32, I2C_STS1_SPEC, bool, 8>;
#[doc = "Field `ARLOST` reader - ARLOST"]
pub type ARLOST_R = crate::BitReader<bool>;
#[doc = "Field `ARLOST` writer - ARLOST"]
pub type ARLOST_W<'a> = crate::BitWriter<'a, u32, I2C_STS1_SPEC, bool, 9>;
#[doc = "Field `ACKFAIL` reader - ACKFAIL"]
pub type ACKFAIL_R = crate::BitReader<bool>;
#[doc = "Field `ACKFAIL` writer - ACKFAIL"]
pub type ACKFAIL_W<'a> = crate::BitWriter<'a, u32, I2C_STS1_SPEC, bool, 10>;
#[doc = "Field `OVERRUN` reader - OVERRUN"]
pub type OVERRUN_R = crate::BitReader<bool>;
#[doc = "Field `OVERRUN` writer - OVERRUN"]
pub type OVERRUN_W<'a> = crate::BitWriter<'a, u32, I2C_STS1_SPEC, bool, 11>;
#[doc = "Field `PECERR` reader - PECERR"]
pub type PECERR_R = crate::BitReader<bool>;
#[doc = "Field `PECERR` writer - PECERR"]
pub type PECERR_W<'a> = crate::BitWriter<'a, u32, I2C_STS1_SPEC, bool, 12>;
#[doc = "Field `TIMOUT` reader - TIMOUT"]
pub type TIMOUT_R = crate::BitReader<bool>;
#[doc = "Field `TIMOUT` writer - TIMOUT"]
pub type TIMOUT_W<'a> = crate::BitWriter<'a, u32, I2C_STS1_SPEC, bool, 14>;
#[doc = "Field `SMBALERT` reader - SMBALERT"]
pub type SMBALERT_R = crate::BitReader<bool>;
#[doc = "Field `SMBALERT` writer - SMBALERT"]
pub type SMBALERT_W<'a> = crate::BitWriter<'a, u32, I2C_STS1_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - STARTBF"]
    #[inline(always)]
    pub fn startbf(&self) -> STARTBF_R {
        STARTBF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADDRF"]
    #[inline(always)]
    pub fn addrf(&self) -> ADDRF_R {
        ADDRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BSF"]
    #[inline(always)]
    pub fn bsf(&self) -> BSF_R {
        BSF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADDR10F"]
    #[inline(always)]
    pub fn addr10f(&self) -> ADDR10F_R {
        ADDR10F_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - STOPF"]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - RXDATNE"]
    #[inline(always)]
    pub fn rxdatne(&self) -> RXDATNE_R {
        RXDATNE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXDATE"]
    #[inline(always)]
    pub fn txdate(&self) -> TXDATE_R {
        TXDATE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BUSERR"]
    #[inline(always)]
    pub fn buserr(&self) -> BUSERR_R {
        BUSERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ARLOST"]
    #[inline(always)]
    pub fn arlost(&self) -> ARLOST_R {
        ARLOST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ACKFAIL"]
    #[inline(always)]
    pub fn ackfail(&self) -> ACKFAIL_R {
        ACKFAIL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OVERRUN"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PECERR"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - TIMOUT"]
    #[inline(always)]
    pub fn timout(&self) -> TIMOUT_R {
        TIMOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SMBALERT"]
    #[inline(always)]
    pub fn smbalert(&self) -> SMBALERT_R {
        SMBALERT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - STARTBF"]
    #[inline(always)]
    pub fn startbf(&mut self) -> STARTBF_W {
        STARTBF_W::new(self)
    }
    #[doc = "Bit 1 - ADDRF"]
    #[inline(always)]
    pub fn addrf(&mut self) -> ADDRF_W {
        ADDRF_W::new(self)
    }
    #[doc = "Bit 2 - BSF"]
    #[inline(always)]
    pub fn bsf(&mut self) -> BSF_W {
        BSF_W::new(self)
    }
    #[doc = "Bit 3 - ADDR10F"]
    #[inline(always)]
    pub fn addr10f(&mut self) -> ADDR10F_W {
        ADDR10F_W::new(self)
    }
    #[doc = "Bit 4 - STOPF"]
    #[inline(always)]
    pub fn stopf(&mut self) -> STOPF_W {
        STOPF_W::new(self)
    }
    #[doc = "Bit 6 - RXDATNE"]
    #[inline(always)]
    pub fn rxdatne(&mut self) -> RXDATNE_W {
        RXDATNE_W::new(self)
    }
    #[doc = "Bit 7 - TXDATE"]
    #[inline(always)]
    pub fn txdate(&mut self) -> TXDATE_W {
        TXDATE_W::new(self)
    }
    #[doc = "Bit 8 - BUSERR"]
    #[inline(always)]
    pub fn buserr(&mut self) -> BUSERR_W {
        BUSERR_W::new(self)
    }
    #[doc = "Bit 9 - ARLOST"]
    #[inline(always)]
    pub fn arlost(&mut self) -> ARLOST_W {
        ARLOST_W::new(self)
    }
    #[doc = "Bit 10 - ACKFAIL"]
    #[inline(always)]
    pub fn ackfail(&mut self) -> ACKFAIL_W {
        ACKFAIL_W::new(self)
    }
    #[doc = "Bit 11 - OVERRUN"]
    #[inline(always)]
    pub fn overrun(&mut self) -> OVERRUN_W {
        OVERRUN_W::new(self)
    }
    #[doc = "Bit 12 - PECERR"]
    #[inline(always)]
    pub fn pecerr(&mut self) -> PECERR_W {
        PECERR_W::new(self)
    }
    #[doc = "Bit 14 - TIMOUT"]
    #[inline(always)]
    pub fn timout(&mut self) -> TIMOUT_W {
        TIMOUT_W::new(self)
    }
    #[doc = "Bit 15 - SMBALERT"]
    #[inline(always)]
    pub fn smbalert(&mut self) -> SMBALERT_W {
        SMBALERT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C_STS1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_sts1](index.html) module"]
pub struct I2C_STS1_SPEC;
impl crate::RegisterSpec for I2C_STS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_sts1::R](R) reader structure"]
impl crate::Readable for I2C_STS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_sts1::W](W) writer structure"]
impl crate::Writable for I2C_STS1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_STS1 to value 0"]
impl crate::Resettable for I2C_STS1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
