#[doc = "Register `RCC_CTRLSTS` reader"]
pub struct R(crate::R<RCC_CTRLSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CTRLSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CTRLSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CTRLSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_CTRLSTS` writer"]
pub struct W(crate::W<RCC_CTRLSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CTRLSTS_SPEC>;
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
impl From<crate::W<RCC_CTRLSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CTRLSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSIEN` reader - LSIEN"]
pub type LSIEN_R = crate::BitReader<bool>;
#[doc = "Field `LSIEN` writer - LSIEN"]
pub type LSIEN_W<'a> = crate::BitWriter<'a, u32, RCC_CTRLSTS_SPEC, bool, 0>;
#[doc = "Field `LSIRD` reader - LSIRD"]
pub type LSIRD_R = crate::BitReader<bool>;
#[doc = "Field `LSIRD` writer - LSIRD"]
pub type LSIRD_W<'a> = crate::BitWriter<'a, u32, RCC_CTRLSTS_SPEC, bool, 1>;
#[doc = "Field `BORRSTF` reader - BORRSTF"]
pub type BORRSTF_R = crate::BitReader<bool>;
#[doc = "Field `BORRSTF` writer - BORRSTF"]
pub type BORRSTF_W<'a> = crate::BitWriter<'a, u32, RCC_CTRLSTS_SPEC, bool, 19>;
#[doc = "Field `RETEMCF` reader - RETEMCF"]
pub type RETEMCF_R = crate::BitReader<bool>;
#[doc = "Field `RETEMCF` writer - RETEMCF"]
pub type RETEMCF_W<'a> = crate::BitWriter<'a, u32, RCC_CTRLSTS_SPEC, bool, 20>;
#[doc = "Field `BKPEMCF` reader - BKPEMCF"]
pub type BKPEMCF_R = crate::BitReader<bool>;
#[doc = "Field `BKPEMCF` writer - BKPEMCF"]
pub type BKPEMCF_W<'a> = crate::BitWriter<'a, u32, RCC_CTRLSTS_SPEC, bool, 21>;
#[doc = "Field `RAMRSTF` reader - RAMRSTF"]
pub type RAMRSTF_R = crate::BitReader<bool>;
#[doc = "Field `RAMRSTF` writer - RAMRSTF"]
pub type RAMRSTF_W<'a> = crate::BitWriter<'a, u32, RCC_CTRLSTS_SPEC, bool, 23>;
#[doc = "Field `RMRSTF` reader - RMRSTF"]
pub type RMRSTF_R = crate::BitReader<bool>;
#[doc = "Field `RMRSTF` writer - RMRSTF"]
pub type RMRSTF_W<'a> = crate::BitWriter<'a, u32, RCC_CTRLSTS_SPEC, bool, 24>;
#[doc = "Field `MMURSTF` reader - MMURSTF"]
pub type MMURSTF_R = crate::BitReader<bool>;
#[doc = "Field `MMURSTF` writer - MMURSTF"]
pub type MMURSTF_W<'a> = crate::BitWriter<'a, u32, RCC_CTRLSTS_SPEC, bool, 25>;
#[doc = "Field `PINRSTF` reader - PINRSTF"]
pub type PINRSTF_R = crate::BitReader<bool>;
#[doc = "Field `PINRSTF` writer - PINRSTF"]
pub type PINRSTF_W<'a> = crate::BitWriter<'a, u32, RCC_CTRLSTS_SPEC, bool, 26>;
#[doc = "Field `PORRSTF` reader - PORRSTF"]
pub type PORRSTF_R = crate::BitReader<bool>;
#[doc = "Field `PORRSTF` writer - PORRSTF"]
pub type PORRSTF_W<'a> = crate::BitWriter<'a, u32, RCC_CTRLSTS_SPEC, bool, 27>;
#[doc = "Field `SFTRSTF` reader - SFTRSTF"]
pub type SFTRSTF_R = crate::BitReader<bool>;
#[doc = "Field `SFTRSTF` writer - SFTRSTF"]
pub type SFTRSTF_W<'a> = crate::BitWriter<'a, u32, RCC_CTRLSTS_SPEC, bool, 28>;
#[doc = "Field `IWDGRSTF` reader - IWDGRSTF"]
pub type IWDGRSTF_R = crate::BitReader<bool>;
#[doc = "Field `IWDGRSTF` writer - IWDGRSTF"]
pub type IWDGRSTF_W<'a> = crate::BitWriter<'a, u32, RCC_CTRLSTS_SPEC, bool, 29>;
#[doc = "Field `WWDGRSTF` reader - WWDGRSTF"]
pub type WWDGRSTF_R = crate::BitReader<bool>;
#[doc = "Field `WWDGRSTF` writer - WWDGRSTF"]
pub type WWDGRSTF_W<'a> = crate::BitWriter<'a, u32, RCC_CTRLSTS_SPEC, bool, 30>;
#[doc = "Field `LPWRRSTF` reader - LPWRRSTF"]
pub type LPWRRSTF_R = crate::BitReader<bool>;
#[doc = "Field `LPWRRSTF` writer - LPWRRSTF"]
pub type LPWRRSTF_W<'a> = crate::BitWriter<'a, u32, RCC_CTRLSTS_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - LSIEN"]
    #[inline(always)]
    pub fn lsien(&self) -> LSIEN_R {
        LSIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSIRD"]
    #[inline(always)]
    pub fn lsird(&self) -> LSIRD_R {
        LSIRD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 19 - BORRSTF"]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - RETEMCF"]
    #[inline(always)]
    pub fn retemcf(&self) -> RETEMCF_R {
        RETEMCF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - BKPEMCF"]
    #[inline(always)]
    pub fn bkpemcf(&self) -> BKPEMCF_R {
        BKPEMCF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - RAMRSTF"]
    #[inline(always)]
    pub fn ramrstf(&self) -> RAMRSTF_R {
        RAMRSTF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - RMRSTF"]
    #[inline(always)]
    pub fn rmrstf(&self) -> RMRSTF_R {
        RMRSTF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MMURSTF"]
    #[inline(always)]
    pub fn mmurstf(&self) -> MMURSTF_R {
        MMURSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PINRSTF"]
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PORRSTF"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - SFTRSTF"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - IWDGRSTF"]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - WWDGRSTF"]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LPWRRSTF"]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSIEN"]
    #[inline(always)]
    pub fn lsien(&mut self) -> LSIEN_W {
        LSIEN_W::new(self)
    }
    #[doc = "Bit 1 - LSIRD"]
    #[inline(always)]
    pub fn lsird(&mut self) -> LSIRD_W {
        LSIRD_W::new(self)
    }
    #[doc = "Bit 19 - BORRSTF"]
    #[inline(always)]
    pub fn borrstf(&mut self) -> BORRSTF_W {
        BORRSTF_W::new(self)
    }
    #[doc = "Bit 20 - RETEMCF"]
    #[inline(always)]
    pub fn retemcf(&mut self) -> RETEMCF_W {
        RETEMCF_W::new(self)
    }
    #[doc = "Bit 21 - BKPEMCF"]
    #[inline(always)]
    pub fn bkpemcf(&mut self) -> BKPEMCF_W {
        BKPEMCF_W::new(self)
    }
    #[doc = "Bit 23 - RAMRSTF"]
    #[inline(always)]
    pub fn ramrstf(&mut self) -> RAMRSTF_W {
        RAMRSTF_W::new(self)
    }
    #[doc = "Bit 24 - RMRSTF"]
    #[inline(always)]
    pub fn rmrstf(&mut self) -> RMRSTF_W {
        RMRSTF_W::new(self)
    }
    #[doc = "Bit 25 - MMURSTF"]
    #[inline(always)]
    pub fn mmurstf(&mut self) -> MMURSTF_W {
        MMURSTF_W::new(self)
    }
    #[doc = "Bit 26 - PINRSTF"]
    #[inline(always)]
    pub fn pinrstf(&mut self) -> PINRSTF_W {
        PINRSTF_W::new(self)
    }
    #[doc = "Bit 27 - PORRSTF"]
    #[inline(always)]
    pub fn porrstf(&mut self) -> PORRSTF_W {
        PORRSTF_W::new(self)
    }
    #[doc = "Bit 28 - SFTRSTF"]
    #[inline(always)]
    pub fn sftrstf(&mut self) -> SFTRSTF_W {
        SFTRSTF_W::new(self)
    }
    #[doc = "Bit 29 - IWDGRSTF"]
    #[inline(always)]
    pub fn iwdgrstf(&mut self) -> IWDGRSTF_W {
        IWDGRSTF_W::new(self)
    }
    #[doc = "Bit 30 - WWDGRSTF"]
    #[inline(always)]
    pub fn wwdgrstf(&mut self) -> WWDGRSTF_W {
        WWDGRSTF_W::new(self)
    }
    #[doc = "Bit 31 - LPWRRSTF"]
    #[inline(always)]
    pub fn lpwrrstf(&mut self) -> LPWRRSTF_W {
        LPWRRSTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC_CTRLSTS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ctrlsts](index.html) module"]
pub struct RCC_CTRLSTS_SPEC;
impl crate::RegisterSpec for RCC_CTRLSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ctrlsts::R](R) reader structure"]
impl crate::Readable for RCC_CTRLSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ctrlsts::W](W) writer structure"]
impl crate::Writable for RCC_CTRLSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_CTRLSTS to value 0x0c00_0003"]
impl crate::Resettable for RCC_CTRLSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c00_0003
    }
}
