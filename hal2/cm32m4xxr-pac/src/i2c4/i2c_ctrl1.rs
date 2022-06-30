#[doc = "Register `I2C_CTRL1` reader"]
pub struct R(crate::R<I2C_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_CTRL1` writer"]
pub struct W(crate::W<I2C_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_CTRL1_SPEC>;
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
impl From<crate::W<I2C_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - EN"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - EN"]
pub type EN_W<'a> = crate::BitWriter<'a, u32, I2C_CTRL1_SPEC, bool, 0>;
#[doc = "Field `SMBMODE` reader - SMBMODE"]
pub type SMBMODE_R = crate::BitReader<bool>;
#[doc = "Field `SMBMODE` writer - SMBMODE"]
pub type SMBMODE_W<'a> = crate::BitWriter<'a, u32, I2C_CTRL1_SPEC, bool, 1>;
#[doc = "Field `SMBTYPE` reader - SMBTYPE"]
pub type SMBTYPE_R = crate::BitReader<bool>;
#[doc = "Field `SMBTYPE` writer - SMBTYPE"]
pub type SMBTYPE_W<'a> = crate::BitWriter<'a, u32, I2C_CTRL1_SPEC, bool, 3>;
#[doc = "Field `ARPEN` reader - ARPEN"]
pub type ARPEN_R = crate::BitReader<bool>;
#[doc = "Field `ARPEN` writer - ARPEN"]
pub type ARPEN_W<'a> = crate::BitWriter<'a, u32, I2C_CTRL1_SPEC, bool, 4>;
#[doc = "Field `PECEN` reader - PECEN"]
pub type PECEN_R = crate::BitReader<bool>;
#[doc = "Field `PECEN` writer - PECEN"]
pub type PECEN_W<'a> = crate::BitWriter<'a, u32, I2C_CTRL1_SPEC, bool, 5>;
#[doc = "Field `GCEN` reader - GCEN"]
pub type GCEN_R = crate::BitReader<bool>;
#[doc = "Field `GCEN` writer - GCEN"]
pub type GCEN_W<'a> = crate::BitWriter<'a, u32, I2C_CTRL1_SPEC, bool, 6>;
#[doc = "Field `NOEXTEND` reader - NOEXTEND"]
pub type NOEXTEND_R = crate::BitReader<bool>;
#[doc = "Field `NOEXTEND` writer - NOEXTEND"]
pub type NOEXTEND_W<'a> = crate::BitWriter<'a, u32, I2C_CTRL1_SPEC, bool, 7>;
#[doc = "Field `STARTGEN` reader - STARTGEN"]
pub type STARTGEN_R = crate::BitReader<bool>;
#[doc = "Field `STARTGEN` writer - STARTGEN"]
pub type STARTGEN_W<'a> = crate::BitWriter<'a, u32, I2C_CTRL1_SPEC, bool, 8>;
#[doc = "Field `STOPGEN` reader - STOPGEN"]
pub type STOPGEN_R = crate::BitReader<bool>;
#[doc = "Field `STOPGEN` writer - STOPGEN"]
pub type STOPGEN_W<'a> = crate::BitWriter<'a, u32, I2C_CTRL1_SPEC, bool, 9>;
#[doc = "Field `ACKEN` reader - ACKEN"]
pub type ACKEN_R = crate::BitReader<bool>;
#[doc = "Field `ACKEN` writer - ACKEN"]
pub type ACKEN_W<'a> = crate::BitWriter<'a, u32, I2C_CTRL1_SPEC, bool, 10>;
#[doc = "Field `ACKPOS` reader - ACKPOS"]
pub type ACKPOS_R = crate::BitReader<bool>;
#[doc = "Field `ACKPOS` writer - ACKPOS"]
pub type ACKPOS_W<'a> = crate::BitWriter<'a, u32, I2C_CTRL1_SPEC, bool, 11>;
#[doc = "Field `PEC` reader - PEC"]
pub type PEC_R = crate::BitReader<bool>;
#[doc = "Field `PEC` writer - PEC"]
pub type PEC_W<'a> = crate::BitWriter<'a, u32, I2C_CTRL1_SPEC, bool, 12>;
#[doc = "Field `SMBALERT` reader - SMBALERT"]
pub type SMBALERT_R = crate::BitReader<bool>;
#[doc = "Field `SMBALERT` writer - SMBALERT"]
pub type SMBALERT_W<'a> = crate::BitWriter<'a, u32, I2C_CTRL1_SPEC, bool, 13>;
#[doc = "Field `SWRESET` reader - SWRESET"]
pub type SWRESET_R = crate::BitReader<bool>;
#[doc = "Field `SWRESET` writer - SWRESET"]
pub type SWRESET_W<'a> = crate::BitWriter<'a, u32, I2C_CTRL1_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMBMODE"]
    #[inline(always)]
    pub fn smbmode(&self) -> SMBMODE_R {
        SMBMODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SMBTYPE"]
    #[inline(always)]
    pub fn smbtype(&self) -> SMBTYPE_R {
        SMBTYPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ARPEN"]
    #[inline(always)]
    pub fn arpen(&self) -> ARPEN_R {
        ARPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PECEN"]
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GCEN"]
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NOEXTEND"]
    #[inline(always)]
    pub fn noextend(&self) -> NOEXTEND_R {
        NOEXTEND_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - STARTGEN"]
    #[inline(always)]
    pub fn startgen(&self) -> STARTGEN_R {
        STARTGEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - STOPGEN"]
    #[inline(always)]
    pub fn stopgen(&self) -> STOPGEN_R {
        STOPGEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ACKEN"]
    #[inline(always)]
    pub fn acken(&self) -> ACKEN_R {
        ACKEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ACKPOS"]
    #[inline(always)]
    pub fn ackpos(&self) -> ACKPOS_R {
        ACKPOS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PEC"]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMBALERT"]
    #[inline(always)]
    pub fn smbalert(&self) -> SMBALERT_R {
        SMBALERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - SWRESET"]
    #[inline(always)]
    pub fn swreset(&self) -> SWRESET_R {
        SWRESET_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - SMBMODE"]
    #[inline(always)]
    pub fn smbmode(&mut self) -> SMBMODE_W {
        SMBMODE_W::new(self)
    }
    #[doc = "Bit 3 - SMBTYPE"]
    #[inline(always)]
    pub fn smbtype(&mut self) -> SMBTYPE_W {
        SMBTYPE_W::new(self)
    }
    #[doc = "Bit 4 - ARPEN"]
    #[inline(always)]
    pub fn arpen(&mut self) -> ARPEN_W {
        ARPEN_W::new(self)
    }
    #[doc = "Bit 5 - PECEN"]
    #[inline(always)]
    pub fn pecen(&mut self) -> PECEN_W {
        PECEN_W::new(self)
    }
    #[doc = "Bit 6 - GCEN"]
    #[inline(always)]
    pub fn gcen(&mut self) -> GCEN_W {
        GCEN_W::new(self)
    }
    #[doc = "Bit 7 - NOEXTEND"]
    #[inline(always)]
    pub fn noextend(&mut self) -> NOEXTEND_W {
        NOEXTEND_W::new(self)
    }
    #[doc = "Bit 8 - STARTGEN"]
    #[inline(always)]
    pub fn startgen(&mut self) -> STARTGEN_W {
        STARTGEN_W::new(self)
    }
    #[doc = "Bit 9 - STOPGEN"]
    #[inline(always)]
    pub fn stopgen(&mut self) -> STOPGEN_W {
        STOPGEN_W::new(self)
    }
    #[doc = "Bit 10 - ACKEN"]
    #[inline(always)]
    pub fn acken(&mut self) -> ACKEN_W {
        ACKEN_W::new(self)
    }
    #[doc = "Bit 11 - ACKPOS"]
    #[inline(always)]
    pub fn ackpos(&mut self) -> ACKPOS_W {
        ACKPOS_W::new(self)
    }
    #[doc = "Bit 12 - PEC"]
    #[inline(always)]
    pub fn pec(&mut self) -> PEC_W {
        PEC_W::new(self)
    }
    #[doc = "Bit 13 - SMBALERT"]
    #[inline(always)]
    pub fn smbalert(&mut self) -> SMBALERT_W {
        SMBALERT_W::new(self)
    }
    #[doc = "Bit 15 - SWRESET"]
    #[inline(always)]
    pub fn swreset(&mut self) -> SWRESET_W {
        SWRESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C_CTRL1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_ctrl1](index.html) module"]
pub struct I2C_CTRL1_SPEC;
impl crate::RegisterSpec for I2C_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_ctrl1::R](R) reader structure"]
impl crate::Readable for I2C_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_ctrl1::W](W) writer structure"]
impl crate::Writable for I2C_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_CTRL1 to value 0"]
impl crate::Resettable for I2C_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
