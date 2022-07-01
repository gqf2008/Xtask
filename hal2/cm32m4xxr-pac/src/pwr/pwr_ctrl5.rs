#[doc = "Register `PWR_CTRL5` reader"]
pub struct R(crate::R<PWR_CTRL5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CTRL5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CTRL5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CTRL5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_CTRL5` writer"]
pub struct W(crate::W<PWR_CTRL5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_CTRL5_SPEC>;
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
impl From<crate::W<PWR_CTRL5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_CTRL5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MRCEN` reader - MRCEN"]
pub type MRCEN_R = crate::BitReader<bool>;
#[doc = "Field `MRCEN` writer - MRCEN"]
pub type MRCEN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL5_SPEC, bool, 0>;
#[doc = "Field `MRLPEN` reader - MRLPEN"]
pub type MRLPEN_R = crate::BitReader<bool>;
#[doc = "Field `MRLPEN` writer - MRLPEN"]
pub type MRLPEN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL5_SPEC, bool, 1>;
#[doc = "Field `FLSLMCEN` reader - FLSLMCEN"]
pub type FLSLMCEN_R = crate::BitReader<bool>;
#[doc = "Field `FLSLMCEN` writer - FLSLMCEN"]
pub type FLSLMCEN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL5_SPEC, bool, 2>;
#[doc = "Field `FLPDMCEN` reader - FLPDMCEN"]
pub type FLPDMCEN_R = crate::BitReader<bool>;
#[doc = "Field `FLPDMCEN` writer - FLPDMCEN"]
pub type FLPDMCEN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL5_SPEC, bool, 3>;
#[doc = "Field `PLLCEN` reader - PLLCEN"]
pub type PLLCEN_R = crate::BitReader<bool>;
#[doc = "Field `PLLCEN` writer - PLLCEN"]
pub type PLLCEN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL5_SPEC, bool, 4>;
#[doc = "Field `HSICEN` reader - HSICEN"]
pub type HSICEN_R = crate::BitReader<bool>;
#[doc = "Field `HSICEN` writer - HSICEN"]
pub type HSICEN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL5_SPEC, bool, 5>;
#[doc = "Field `HSECEN` reader - HSECEN"]
pub type HSECEN_R = crate::BitReader<bool>;
#[doc = "Field `HSECEN` writer - HSECEN"]
pub type HSECEN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL5_SPEC, bool, 6>;
#[doc = "Field `PVSCEN` reader - PVSCEN"]
pub type PVSCEN_R = crate::BitReader<bool>;
#[doc = "Field `PVSCEN` writer - PVSCEN"]
pub type PVSCEN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL5_SPEC, bool, 7>;
#[doc = "Field `PVSRETCEN` reader - PVSRETCEN"]
pub type PVSRETCEN_R = crate::BitReader<bool>;
#[doc = "Field `PVSRETCEN` writer - PVSRETCEN"]
pub type PVSRETCEN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL5_SPEC, bool, 8>;
#[doc = "Field `SRCEN` reader - SRCEN"]
pub type SRCEN_R = crate::BitReader<bool>;
#[doc = "Field `SRCEN` writer - SRCEN"]
pub type SRCEN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL5_SPEC, bool, 9>;
#[doc = "Field `BGCEN` reader - BGCEN"]
pub type BGCEN_R = crate::BitReader<bool>;
#[doc = "Field `BGCEN` writer - BGCEN"]
pub type BGCEN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL5_SPEC, bool, 10>;
impl R {
    #[doc = "Bit 0 - MRCEN"]
    #[inline(always)]
    pub fn mrcen(&self) -> MRCEN_R {
        MRCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MRLPEN"]
    #[inline(always)]
    pub fn mrlpen(&self) -> MRLPEN_R {
        MRLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FLSLMCEN"]
    #[inline(always)]
    pub fn flslmcen(&self) -> FLSLMCEN_R {
        FLSLMCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FLPDMCEN"]
    #[inline(always)]
    pub fn flpdmcen(&self) -> FLPDMCEN_R {
        FLPDMCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLLCEN"]
    #[inline(always)]
    pub fn pllcen(&self) -> PLLCEN_R {
        PLLCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HSICEN"]
    #[inline(always)]
    pub fn hsicen(&self) -> HSICEN_R {
        HSICEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HSECEN"]
    #[inline(always)]
    pub fn hsecen(&self) -> HSECEN_R {
        HSECEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PVSCEN"]
    #[inline(always)]
    pub fn pvscen(&self) -> PVSCEN_R {
        PVSCEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PVSRETCEN"]
    #[inline(always)]
    pub fn pvsretcen(&self) -> PVSRETCEN_R {
        PVSRETCEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRCEN"]
    #[inline(always)]
    pub fn srcen(&self) -> SRCEN_R {
        SRCEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BGCEN"]
    #[inline(always)]
    pub fn bgcen(&self) -> BGCEN_R {
        BGCEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MRCEN"]
    #[inline(always)]
    pub fn mrcen(&mut self) -> MRCEN_W {
        MRCEN_W::new(self)
    }
    #[doc = "Bit 1 - MRLPEN"]
    #[inline(always)]
    pub fn mrlpen(&mut self) -> MRLPEN_W {
        MRLPEN_W::new(self)
    }
    #[doc = "Bit 2 - FLSLMCEN"]
    #[inline(always)]
    pub fn flslmcen(&mut self) -> FLSLMCEN_W {
        FLSLMCEN_W::new(self)
    }
    #[doc = "Bit 3 - FLPDMCEN"]
    #[inline(always)]
    pub fn flpdmcen(&mut self) -> FLPDMCEN_W {
        FLPDMCEN_W::new(self)
    }
    #[doc = "Bit 4 - PLLCEN"]
    #[inline(always)]
    pub fn pllcen(&mut self) -> PLLCEN_W {
        PLLCEN_W::new(self)
    }
    #[doc = "Bit 5 - HSICEN"]
    #[inline(always)]
    pub fn hsicen(&mut self) -> HSICEN_W {
        HSICEN_W::new(self)
    }
    #[doc = "Bit 6 - HSECEN"]
    #[inline(always)]
    pub fn hsecen(&mut self) -> HSECEN_W {
        HSECEN_W::new(self)
    }
    #[doc = "Bit 7 - PVSCEN"]
    #[inline(always)]
    pub fn pvscen(&mut self) -> PVSCEN_W {
        PVSCEN_W::new(self)
    }
    #[doc = "Bit 8 - PVSRETCEN"]
    #[inline(always)]
    pub fn pvsretcen(&mut self) -> PVSRETCEN_W {
        PVSRETCEN_W::new(self)
    }
    #[doc = "Bit 9 - SRCEN"]
    #[inline(always)]
    pub fn srcen(&mut self) -> SRCEN_W {
        SRCEN_W::new(self)
    }
    #[doc = "Bit 10 - BGCEN"]
    #[inline(always)]
    pub fn bgcen(&mut self) -> BGCEN_W {
        BGCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWR_CTRL5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_ctrl5](index.html) module"]
pub struct PWR_CTRL5_SPEC;
impl crate::RegisterSpec for PWR_CTRL5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_ctrl5::R](R) reader structure"]
impl crate::Readable for PWR_CTRL5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_ctrl5::W](W) writer structure"]
impl crate::Writable for PWR_CTRL5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_CTRL5 to value 0x07ff"]
impl crate::Resettable for PWR_CTRL5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07ff
    }
}
