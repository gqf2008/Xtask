#[doc = "Register `TIMx_BKDT` reader"]
pub struct R(crate::R<TIMX_BKDT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_BKDT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_BKDT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_BKDT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_BKDT` writer"]
pub struct W(crate::W<TIMX_BKDT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_BKDT_SPEC>;
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
impl From<crate::W<TIMX_BKDT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_BKDT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTGN` reader - DTGN"]
pub type DTGN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTGN` writer - DTGN"]
pub type DTGN_W<'a> = crate::FieldWriter<'a, u32, TIMX_BKDT_SPEC, u8, u8, 8, 0>;
#[doc = "Field `LCKCFG` reader - LCKCFG"]
pub type LCKCFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LCKCFG` writer - LCKCFG"]
pub type LCKCFG_W<'a> = crate::FieldWriter<'a, u32, TIMX_BKDT_SPEC, u8, u8, 2, 8>;
#[doc = "Field `OSSI` reader - OSSI"]
pub type OSSI_R = crate::BitReader<bool>;
#[doc = "Field `OSSI` writer - OSSI"]
pub type OSSI_W<'a> = crate::BitWriter<'a, u32, TIMX_BKDT_SPEC, bool, 10>;
#[doc = "Field `OSSR` reader - OSSR"]
pub type OSSR_R = crate::BitReader<bool>;
#[doc = "Field `OSSR` writer - OSSR"]
pub type OSSR_W<'a> = crate::BitWriter<'a, u32, TIMX_BKDT_SPEC, bool, 11>;
#[doc = "Field `BKEN` reader - BKEN"]
pub type BKEN_R = crate::BitReader<bool>;
#[doc = "Field `BKEN` writer - BKEN"]
pub type BKEN_W<'a> = crate::BitWriter<'a, u32, TIMX_BKDT_SPEC, bool, 12>;
#[doc = "Field `BKP` reader - BKP"]
pub type BKP_R = crate::BitReader<bool>;
#[doc = "Field `BKP` writer - BKP"]
pub type BKP_W<'a> = crate::BitWriter<'a, u32, TIMX_BKDT_SPEC, bool, 13>;
#[doc = "Field `AOEN` reader - AOEN"]
pub type AOEN_R = crate::BitReader<bool>;
#[doc = "Field `AOEN` writer - AOEN"]
pub type AOEN_W<'a> = crate::BitWriter<'a, u32, TIMX_BKDT_SPEC, bool, 14>;
#[doc = "Field `MOEN` reader - MOEN"]
pub type MOEN_R = crate::BitReader<bool>;
#[doc = "Field `MOEN` writer - MOEN"]
pub type MOEN_W<'a> = crate::BitWriter<'a, u32, TIMX_BKDT_SPEC, bool, 15>;
impl R {
    #[doc = "Bits 0:7 - DTGN"]
    #[inline(always)]
    pub fn dtgn(&self) -> DTGN_R {
        DTGN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - LCKCFG"]
    #[inline(always)]
    pub fn lckcfg(&self) -> LCKCFG_R {
        LCKCFG_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - OSSI"]
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OSSR"]
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BKEN"]
    #[inline(always)]
    pub fn bken(&self) -> BKEN_R {
        BKEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - BKP"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AOEN"]
    #[inline(always)]
    pub fn aoen(&self) -> AOEN_R {
        AOEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MOEN"]
    #[inline(always)]
    pub fn moen(&self) -> MOEN_R {
        MOEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - DTGN"]
    #[inline(always)]
    pub fn dtgn(&mut self) -> DTGN_W {
        DTGN_W::new(self)
    }
    #[doc = "Bits 8:9 - LCKCFG"]
    #[inline(always)]
    pub fn lckcfg(&mut self) -> LCKCFG_W {
        LCKCFG_W::new(self)
    }
    #[doc = "Bit 10 - OSSI"]
    #[inline(always)]
    pub fn ossi(&mut self) -> OSSI_W {
        OSSI_W::new(self)
    }
    #[doc = "Bit 11 - OSSR"]
    #[inline(always)]
    pub fn ossr(&mut self) -> OSSR_W {
        OSSR_W::new(self)
    }
    #[doc = "Bit 12 - BKEN"]
    #[inline(always)]
    pub fn bken(&mut self) -> BKEN_W {
        BKEN_W::new(self)
    }
    #[doc = "Bit 13 - BKP"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W {
        BKP_W::new(self)
    }
    #[doc = "Bit 14 - AOEN"]
    #[inline(always)]
    pub fn aoen(&mut self) -> AOEN_W {
        AOEN_W::new(self)
    }
    #[doc = "Bit 15 - MOEN"]
    #[inline(always)]
    pub fn moen(&mut self) -> MOEN_W {
        MOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMx_BKDT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_bkdt](index.html) module"]
pub struct TIMX_BKDT_SPEC;
impl crate::RegisterSpec for TIMX_BKDT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timx_bkdt::R](R) reader structure"]
impl crate::Readable for TIMX_BKDT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_bkdt::W](W) writer structure"]
impl crate::Writable for TIMX_BKDT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMx_BKDT to value 0"]
impl crate::Resettable for TIMX_BKDT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
