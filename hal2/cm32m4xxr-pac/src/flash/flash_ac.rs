#[doc = "Register `FLASH_AC` reader"]
pub struct R(crate::R<FLASH_AC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_AC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_AC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_AC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_AC` writer"]
pub struct W(crate::W<FLASH_AC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_AC_SPEC>;
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
impl From<crate::W<FLASH_AC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_AC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LATENCY` reader - LATENCY"]
pub type LATENCY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LATENCY` writer - LATENCY"]
pub type LATENCY_W<'a> = crate::FieldWriter<'a, u32, FLASH_AC_SPEC, u8, u8, 3, 0>;
#[doc = "Field `PRFTBFE` reader - PRFTBFE"]
pub type PRFTBFE_R = crate::BitReader<bool>;
#[doc = "Field `PRFTBFE` writer - PRFTBFE"]
pub type PRFTBFE_W<'a> = crate::BitWriter<'a, u32, FLASH_AC_SPEC, bool, 4>;
#[doc = "Field `PRFTBFS` reader - PRFTBFS"]
pub type PRFTBFS_R = crate::BitReader<bool>;
#[doc = "Field `PRFTBFS` writer - PRFTBFS"]
pub type PRFTBFS_W<'a> = crate::BitWriter<'a, u32, FLASH_AC_SPEC, bool, 5>;
#[doc = "Field `ICAHRST` reader - ICAHRST"]
pub type ICAHRST_R = crate::BitReader<bool>;
#[doc = "Field `ICAHRST` writer - ICAHRST"]
pub type ICAHRST_W<'a> = crate::BitWriter<'a, u32, FLASH_AC_SPEC, bool, 6>;
#[doc = "Field `ICAHEN` reader - ICAHEN"]
pub type ICAHEN_R = crate::BitReader<bool>;
#[doc = "Field `ICAHEN` writer - ICAHEN"]
pub type ICAHEN_W<'a> = crate::BitWriter<'a, u32, FLASH_AC_SPEC, bool, 7>;
impl R {
    #[doc = "Bits 0:2 - LATENCY"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - PRFTBFE"]
    #[inline(always)]
    pub fn prftbfe(&self) -> PRFTBFE_R {
        PRFTBFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PRFTBFS"]
    #[inline(always)]
    pub fn prftbfs(&self) -> PRFTBFS_R {
        PRFTBFS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ICAHRST"]
    #[inline(always)]
    pub fn icahrst(&self) -> ICAHRST_R {
        ICAHRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ICAHEN"]
    #[inline(always)]
    pub fn icahen(&self) -> ICAHEN_R {
        ICAHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - LATENCY"]
    #[inline(always)]
    pub fn latency(&mut self) -> LATENCY_W {
        LATENCY_W::new(self)
    }
    #[doc = "Bit 4 - PRFTBFE"]
    #[inline(always)]
    pub fn prftbfe(&mut self) -> PRFTBFE_W {
        PRFTBFE_W::new(self)
    }
    #[doc = "Bit 5 - PRFTBFS"]
    #[inline(always)]
    pub fn prftbfs(&mut self) -> PRFTBFS_W {
        PRFTBFS_W::new(self)
    }
    #[doc = "Bit 6 - ICAHRST"]
    #[inline(always)]
    pub fn icahrst(&mut self) -> ICAHRST_W {
        ICAHRST_W::new(self)
    }
    #[doc = "Bit 7 - ICAHEN"]
    #[inline(always)]
    pub fn icahen(&mut self) -> ICAHEN_W {
        ICAHEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH_AC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ac](index.html) module"]
pub struct FLASH_AC_SPEC;
impl crate::RegisterSpec for FLASH_AC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_ac::R](R) reader structure"]
impl crate::Readable for FLASH_AC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_ac::W](W) writer structure"]
impl crate::Writable for FLASH_AC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_AC to value 0x30"]
impl crate::Resettable for FLASH_AC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x30
    }
}
