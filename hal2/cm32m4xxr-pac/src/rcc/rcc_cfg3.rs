#[doc = "Register `RCC_CFG3` reader"]
pub struct R(crate::R<RCC_CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_CFG3` writer"]
pub struct W(crate::W<RCC_CFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CFG3_SPEC>;
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
impl From<crate::W<RCC_CFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BORRSTEN` reader - BORRSTEN"]
pub type BORRSTEN_R = crate::BitReader<bool>;
#[doc = "Field `BORRSTEN` writer - BORRSTEN"]
pub type BORRSTEN_W<'a> = crate::BitWriter<'a, u32, RCC_CFG3_SPEC, bool, 6>;
#[doc = "Field `TRNG1MPRES` reader - TRNG1MPRES"]
pub type TRNG1MPRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRNG1MPRES` writer - TRNG1MPRES"]
pub type TRNG1MPRES_W<'a> = crate::FieldWriter<'a, u32, RCC_CFG3_SPEC, u8, u8, 5, 11>;
#[doc = "Field `TRNG1MSEL` reader - TRNG1MSEL"]
pub type TRNG1MSEL_R = crate::BitReader<bool>;
#[doc = "Field `TRNG1MSEL` writer - TRNG1MSEL"]
pub type TRNG1MSEL_W<'a> = crate::BitWriter<'a, u32, RCC_CFG3_SPEC, bool, 17>;
#[doc = "Field `TRNG1MEN` reader - TRNG1MEN"]
pub type TRNG1MEN_R = crate::BitReader<bool>;
#[doc = "Field `TRNG1MEN` writer - TRNG1MEN"]
pub type TRNG1MEN_W<'a> = crate::BitWriter<'a, u32, RCC_CFG3_SPEC, bool, 18>;
impl R {
    #[doc = "Bit 6 - BORRSTEN"]
    #[inline(always)]
    pub fn borrsten(&self) -> BORRSTEN_R {
        BORRSTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 11:15 - TRNG1MPRES"]
    #[inline(always)]
    pub fn trng1mpres(&self) -> TRNG1MPRES_R {
        TRNG1MPRES_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 17 - TRNG1MSEL"]
    #[inline(always)]
    pub fn trng1msel(&self) -> TRNG1MSEL_R {
        TRNG1MSEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TRNG1MEN"]
    #[inline(always)]
    pub fn trng1men(&self) -> TRNG1MEN_R {
        TRNG1MEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - BORRSTEN"]
    #[inline(always)]
    pub fn borrsten(&mut self) -> BORRSTEN_W {
        BORRSTEN_W::new(self)
    }
    #[doc = "Bits 11:15 - TRNG1MPRES"]
    #[inline(always)]
    pub fn trng1mpres(&mut self) -> TRNG1MPRES_W {
        TRNG1MPRES_W::new(self)
    }
    #[doc = "Bit 17 - TRNG1MSEL"]
    #[inline(always)]
    pub fn trng1msel(&mut self) -> TRNG1MSEL_W {
        TRNG1MSEL_W::new(self)
    }
    #[doc = "Bit 18 - TRNG1MEN"]
    #[inline(always)]
    pub fn trng1men(&mut self) -> TRNG1MEN_W {
        TRNG1MEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC_CFG3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_cfg3](index.html) module"]
pub struct RCC_CFG3_SPEC;
impl crate::RegisterSpec for RCC_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_cfg3::R](R) reader structure"]
impl crate::Readable for RCC_CFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_cfg3::W](W) writer structure"]
impl crate::Writable for RCC_CFG3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_CFG3 to value 0x3840"]
impl crate::Resettable for RCC_CFG3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3840
    }
}
