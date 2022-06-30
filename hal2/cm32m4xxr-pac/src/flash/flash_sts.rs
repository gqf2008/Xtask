#[doc = "Register `FLASH_STS` reader"]
pub struct R(crate::R<FLASH_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_STS` writer"]
pub struct W(crate::W<FLASH_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_STS_SPEC>;
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
impl From<crate::W<FLASH_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSY` reader - BUSY"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - BUSY"]
pub type BUSY_W<'a> = crate::BitWriter<'a, u32, FLASH_STS_SPEC, bool, 0>;
#[doc = "Field `RDKEYERR` reader - RDKEYERR"]
pub type RDKEYERR_R = crate::BitReader<bool>;
#[doc = "Field `RDKEYERR` writer - RDKEYERR"]
pub type RDKEYERR_W<'a> = crate::BitWriter<'a, u32, FLASH_STS_SPEC, bool, 1>;
#[doc = "Field `PGERR` reader - PGERR"]
pub type PGERR_R = crate::BitReader<bool>;
#[doc = "Field `PGERR` writer - PGERR"]
pub type PGERR_W<'a> = crate::BitWriter<'a, u32, FLASH_STS_SPEC, bool, 2>;
#[doc = "Field `PVERR` reader - PVERR"]
pub type PVERR_R = crate::BitReader<bool>;
#[doc = "Field `PVERR` writer - PVERR"]
pub type PVERR_W<'a> = crate::BitWriter<'a, u32, FLASH_STS_SPEC, bool, 3>;
#[doc = "Field `WRPERR` reader - WRPERR"]
pub type WRPERR_R = crate::BitReader<bool>;
#[doc = "Field `WRPERR` writer - WRPERR"]
pub type WRPERR_W<'a> = crate::BitWriter<'a, u32, FLASH_STS_SPEC, bool, 4>;
#[doc = "Field `EOP` reader - EOP"]
pub type EOP_R = crate::BitReader<bool>;
#[doc = "Field `EOP` writer - EOP"]
pub type EOP_W<'a> = crate::BitWriter<'a, u32, FLASH_STS_SPEC, bool, 5>;
#[doc = "Field `EVERR` reader - EVERR"]
pub type EVERR_R = crate::BitReader<bool>;
#[doc = "Field `EVERR` writer - EVERR"]
pub type EVERR_W<'a> = crate::BitWriter<'a, u32, FLASH_STS_SPEC, bool, 6>;
#[doc = "Field `ECCERR` reader - ECCERR"]
pub type ECCERR_R = crate::BitReader<bool>;
#[doc = "Field `ECCERR` writer - ECCERR"]
pub type ECCERR_W<'a> = crate::BitWriter<'a, u32, FLASH_STS_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RDKEYERR"]
    #[inline(always)]
    pub fn rdkeyerr(&self) -> RDKEYERR_R {
        RDKEYERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PGERR"]
    #[inline(always)]
    pub fn pgerr(&self) -> PGERR_R {
        PGERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PVERR"]
    #[inline(always)]
    pub fn pverr(&self) -> PVERR_R {
        PVERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WRPERR"]
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EOP"]
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EVERR"]
    #[inline(always)]
    pub fn everr(&self) -> EVERR_R {
        EVERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ECCERR"]
    #[inline(always)]
    pub fn eccerr(&self) -> ECCERR_R {
        ECCERR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BUSY"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W::new(self)
    }
    #[doc = "Bit 1 - RDKEYERR"]
    #[inline(always)]
    pub fn rdkeyerr(&mut self) -> RDKEYERR_W {
        RDKEYERR_W::new(self)
    }
    #[doc = "Bit 2 - PGERR"]
    #[inline(always)]
    pub fn pgerr(&mut self) -> PGERR_W {
        PGERR_W::new(self)
    }
    #[doc = "Bit 3 - PVERR"]
    #[inline(always)]
    pub fn pverr(&mut self) -> PVERR_W {
        PVERR_W::new(self)
    }
    #[doc = "Bit 4 - WRPERR"]
    #[inline(always)]
    pub fn wrperr(&mut self) -> WRPERR_W {
        WRPERR_W::new(self)
    }
    #[doc = "Bit 5 - EOP"]
    #[inline(always)]
    pub fn eop(&mut self) -> EOP_W {
        EOP_W::new(self)
    }
    #[doc = "Bit 6 - EVERR"]
    #[inline(always)]
    pub fn everr(&mut self) -> EVERR_W {
        EVERR_W::new(self)
    }
    #[doc = "Bit 7 - ECCERR"]
    #[inline(always)]
    pub fn eccerr(&mut self) -> ECCERR_W {
        ECCERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH_STS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_sts](index.html) module"]
pub struct FLASH_STS_SPEC;
impl crate::RegisterSpec for FLASH_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_sts::R](R) reader structure"]
impl crate::Readable for FLASH_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_sts::W](W) writer structure"]
impl crate::Writable for FLASH_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_STS to value 0"]
impl crate::Resettable for FLASH_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
