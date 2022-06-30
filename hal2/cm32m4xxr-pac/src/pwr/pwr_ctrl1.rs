#[doc = "Register `PWR_CTRL1` reader"]
pub struct R(crate::R<PWR_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_CTRL1` writer"]
pub struct W(crate::W<PWR_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_CTRL1_SPEC>;
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
impl From<crate::W<PWR_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `__LPS` reader - __LPS"]
pub type __LPS_R = crate::BitReader<bool>;
#[doc = "Field `__LPS` writer - __LPS"]
pub type __LPS_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL1_SPEC, bool, 0>;
#[doc = "Field `__PDS` reader - __PDS"]
pub type __PDS_R = crate::BitReader<bool>;
#[doc = "Field `__PDS` writer - __PDS"]
pub type __PDS_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL1_SPEC, bool, 1>;
#[doc = "Field `__CWKUP` reader - __CWKUP"]
pub type __CWKUP_R = crate::BitReader<bool>;
#[doc = "Field `__CWKUP` writer - __CWKUP"]
pub type __CWKUP_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL1_SPEC, bool, 2>;
#[doc = "Field `__CSBVBAT` reader - __CSBVBAT"]
pub type __CSBVBAT_R = crate::BitReader<bool>;
#[doc = "Field `__CSBVBAT` writer - __CSBVBAT"]
pub type __CSBVBAT_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL1_SPEC, bool, 3>;
#[doc = "Field `__PVDEN` reader - __PVDEN"]
pub type __PVDEN_R = crate::BitReader<bool>;
#[doc = "Field `__PVDEN` writer - __PVDEN"]
pub type __PVDEN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL1_SPEC, bool, 4>;
#[doc = "Field `PRS` reader - PRS"]
pub type PRS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRS` writer - PRS"]
pub type PRS_W<'a> = crate::FieldWriter<'a, u32, PWR_CTRL1_SPEC, u8, u8, 3, 5>;
#[doc = "Field `DBKP` reader - DBKP"]
pub type DBKP_R = crate::BitReader<bool>;
#[doc = "Field `DBKP` writer - DBKP"]
pub type DBKP_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL1_SPEC, bool, 8>;
#[doc = "Field `MSB` reader - MSB"]
pub type MSB_R = crate::BitReader<bool>;
#[doc = "Field `MSB` writer - MSB"]
pub type MSB_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL1_SPEC, bool, 9>;
impl R {
    #[doc = "Bit 0 - __LPS"]
    #[inline(always)]
    pub fn __lps(&self) -> __LPS_R {
        __LPS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - __PDS"]
    #[inline(always)]
    pub fn __pds(&self) -> __PDS_R {
        __PDS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - __CWKUP"]
    #[inline(always)]
    pub fn __cwkup(&self) -> __CWKUP_R {
        __CWKUP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - __CSBVBAT"]
    #[inline(always)]
    pub fn __csbvbat(&self) -> __CSBVBAT_R {
        __CSBVBAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - __PVDEN"]
    #[inline(always)]
    pub fn __pvden(&self) -> __PVDEN_R {
        __PVDEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - PRS"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - DBKP"]
    #[inline(always)]
    pub fn dbkp(&self) -> DBKP_R {
        DBKP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MSB"]
    #[inline(always)]
    pub fn msb(&self) -> MSB_R {
        MSB_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - __LPS"]
    #[inline(always)]
    pub fn __lps(&mut self) -> __LPS_W {
        __LPS_W::new(self)
    }
    #[doc = "Bit 1 - __PDS"]
    #[inline(always)]
    pub fn __pds(&mut self) -> __PDS_W {
        __PDS_W::new(self)
    }
    #[doc = "Bit 2 - __CWKUP"]
    #[inline(always)]
    pub fn __cwkup(&mut self) -> __CWKUP_W {
        __CWKUP_W::new(self)
    }
    #[doc = "Bit 3 - __CSBVBAT"]
    #[inline(always)]
    pub fn __csbvbat(&mut self) -> __CSBVBAT_W {
        __CSBVBAT_W::new(self)
    }
    #[doc = "Bit 4 - __PVDEN"]
    #[inline(always)]
    pub fn __pvden(&mut self) -> __PVDEN_W {
        __PVDEN_W::new(self)
    }
    #[doc = "Bits 5:7 - PRS"]
    #[inline(always)]
    pub fn prs(&mut self) -> PRS_W {
        PRS_W::new(self)
    }
    #[doc = "Bit 8 - DBKP"]
    #[inline(always)]
    pub fn dbkp(&mut self) -> DBKP_W {
        DBKP_W::new(self)
    }
    #[doc = "Bit 9 - MSB"]
    #[inline(always)]
    pub fn msb(&mut self) -> MSB_W {
        MSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWR_CTRL1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_ctrl1](index.html) module"]
pub struct PWR_CTRL1_SPEC;
impl crate::RegisterSpec for PWR_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_ctrl1::R](R) reader structure"]
impl crate::Readable for PWR_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_ctrl1::W](W) writer structure"]
impl crate::Writable for PWR_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_CTRL1 to value 0"]
impl crate::Resettable for PWR_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
