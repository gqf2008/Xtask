#[doc = "Register `RCC_CFG` reader"]
pub struct R(crate::R<RCC_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_CFG` writer"]
pub struct W(crate::W<RCC_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CFG_SPEC>;
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
impl From<crate::W<RCC_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCLKSW` reader - SCLKSW"]
pub type SCLKSW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLKSW` writer - SCLKSW"]
pub type SCLKSW_W<'a> = crate::FieldWriter<'a, u32, RCC_CFG_SPEC, u8, u8, 2, 0>;
#[doc = "Field `SCLKSTS` reader - SCLKSTS"]
pub type SCLKSTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLKSTS` writer - SCLKSTS"]
pub type SCLKSTS_W<'a> = crate::FieldWriter<'a, u32, RCC_CFG_SPEC, u8, u8, 2, 2>;
#[doc = "Field `AHBPRES` reader - AHBPRES"]
pub type AHBPRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AHBPRES` writer - AHBPRES"]
pub type AHBPRES_W<'a> = crate::FieldWriter<'a, u32, RCC_CFG_SPEC, u8, u8, 4, 4>;
#[doc = "Field `APB1PRES` reader - APB1PRES"]
pub type APB1PRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `APB1PRES` writer - APB1PRES"]
pub type APB1PRES_W<'a> = crate::FieldWriter<'a, u32, RCC_CFG_SPEC, u8, u8, 3, 8>;
#[doc = "Field `APB2PRES` reader - APB2PRES"]
pub type APB2PRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `APB2PRES` writer - APB2PRES"]
pub type APB2PRES_W<'a> = crate::FieldWriter<'a, u32, RCC_CFG_SPEC, u8, u8, 3, 11>;
#[doc = "Field `PLLSRC` reader - PLLSRC"]
pub type PLLSRC_R = crate::BitReader<bool>;
#[doc = "Field `PLLSRC` writer - PLLSRC"]
pub type PLLSRC_W<'a> = crate::BitWriter<'a, u32, RCC_CFG_SPEC, bool, 16>;
#[doc = "Field `PLLHSEPRES` reader - PLLHSEPRES"]
pub type PLLHSEPRES_R = crate::BitReader<bool>;
#[doc = "Field `PLLHSEPRES` writer - PLLHSEPRES"]
pub type PLLHSEPRES_W<'a> = crate::BitWriter<'a, u32, RCC_CFG_SPEC, bool, 17>;
#[doc = "Field `PLLMULFCT` reader - PLLMULFCT"]
pub type PLLMULFCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLMULFCT` writer - PLLMULFCT"]
pub type PLLMULFCT_W<'a> = crate::FieldWriter<'a, u32, RCC_CFG_SPEC, u8, u8, 4, 18>;
#[doc = "Field `USBPRES` reader - USBPRES"]
pub type USBPRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBPRES` writer - USBPRES"]
pub type USBPRES_W<'a> = crate::FieldWriter<'a, u32, RCC_CFG_SPEC, u8, u8, 2, 22>;
#[doc = "Field `MCO` reader - MCO"]
pub type MCO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCO` writer - MCO"]
pub type MCO_W<'a> = crate::FieldWriter<'a, u32, RCC_CFG_SPEC, u8, u8, 3, 24>;
#[doc = "Field `PLLMULFCT_H` reader - PLLMULFCT_H"]
pub type PLLMULFCT_H_R = crate::BitReader<bool>;
#[doc = "Field `PLLMULFCT_H` writer - PLLMULFCT_H"]
pub type PLLMULFCT_H_W<'a> = crate::BitWriter<'a, u32, RCC_CFG_SPEC, bool, 27>;
#[doc = "Field `MCOPRES` reader - MCOPRES"]
pub type MCOPRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCOPRES` writer - MCOPRES"]
pub type MCOPRES_W<'a> = crate::FieldWriter<'a, u32, RCC_CFG_SPEC, u8, u8, 4, 28>;
impl R {
    #[doc = "Bits 0:1 - SCLKSW"]
    #[inline(always)]
    pub fn sclksw(&self) -> SCLKSW_R {
        SCLKSW_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SCLKSTS"]
    #[inline(always)]
    pub fn sclksts(&self) -> SCLKSTS_R {
        SCLKSTS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHBPRES"]
    #[inline(always)]
    pub fn ahbpres(&self) -> AHBPRES_R {
        AHBPRES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - APB1PRES"]
    #[inline(always)]
    pub fn apb1pres(&self) -> APB1PRES_R {
        APB1PRES_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - APB2PRES"]
    #[inline(always)]
    pub fn apb2pres(&self) -> APB2PRES_R {
        APB2PRES_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 16 - PLLSRC"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PLLHSEPRES"]
    #[inline(always)]
    pub fn pllhsepres(&self) -> PLLHSEPRES_R {
        PLLHSEPRES_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - PLLMULFCT"]
    #[inline(always)]
    pub fn pllmulfct(&self) -> PLLMULFCT_R {
        PLLMULFCT_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - USBPRES"]
    #[inline(always)]
    pub fn usbpres(&self) -> USBPRES_R {
        USBPRES_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:26 - MCO"]
    #[inline(always)]
    pub fn mco(&self) -> MCO_R {
        MCO_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - PLLMULFCT_H"]
    #[inline(always)]
    pub fn pllmulfct_h(&self) -> PLLMULFCT_H_R {
        PLLMULFCT_H_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - MCOPRES"]
    #[inline(always)]
    pub fn mcopres(&self) -> MCOPRES_R {
        MCOPRES_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SCLKSW"]
    #[inline(always)]
    pub fn sclksw(&mut self) -> SCLKSW_W {
        SCLKSW_W::new(self)
    }
    #[doc = "Bits 2:3 - SCLKSTS"]
    #[inline(always)]
    pub fn sclksts(&mut self) -> SCLKSTS_W {
        SCLKSTS_W::new(self)
    }
    #[doc = "Bits 4:7 - AHBPRES"]
    #[inline(always)]
    pub fn ahbpres(&mut self) -> AHBPRES_W {
        AHBPRES_W::new(self)
    }
    #[doc = "Bits 8:10 - APB1PRES"]
    #[inline(always)]
    pub fn apb1pres(&mut self) -> APB1PRES_W {
        APB1PRES_W::new(self)
    }
    #[doc = "Bits 11:13 - APB2PRES"]
    #[inline(always)]
    pub fn apb2pres(&mut self) -> APB2PRES_W {
        APB2PRES_W::new(self)
    }
    #[doc = "Bit 16 - PLLSRC"]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W {
        PLLSRC_W::new(self)
    }
    #[doc = "Bit 17 - PLLHSEPRES"]
    #[inline(always)]
    pub fn pllhsepres(&mut self) -> PLLHSEPRES_W {
        PLLHSEPRES_W::new(self)
    }
    #[doc = "Bits 18:21 - PLLMULFCT"]
    #[inline(always)]
    pub fn pllmulfct(&mut self) -> PLLMULFCT_W {
        PLLMULFCT_W::new(self)
    }
    #[doc = "Bits 22:23 - USBPRES"]
    #[inline(always)]
    pub fn usbpres(&mut self) -> USBPRES_W {
        USBPRES_W::new(self)
    }
    #[doc = "Bits 24:26 - MCO"]
    #[inline(always)]
    pub fn mco(&mut self) -> MCO_W {
        MCO_W::new(self)
    }
    #[doc = "Bit 27 - PLLMULFCT_H"]
    #[inline(always)]
    pub fn pllmulfct_h(&mut self) -> PLLMULFCT_H_W {
        PLLMULFCT_H_W::new(self)
    }
    #[doc = "Bits 28:31 - MCOPRES"]
    #[inline(always)]
    pub fn mcopres(&mut self) -> MCOPRES_W {
        MCOPRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_cfg](index.html) module"]
pub struct RCC_CFG_SPEC;
impl crate::RegisterSpec for RCC_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_cfg::R](R) reader structure"]
impl crate::Readable for RCC_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_cfg::W](W) writer structure"]
impl crate::Writable for RCC_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_CFG to value 0x2000_0000"]
impl crate::Resettable for RCC_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000_0000
    }
}
