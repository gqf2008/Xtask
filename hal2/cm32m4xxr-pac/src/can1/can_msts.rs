#[doc = "Register `CAN_MSTS` reader"]
pub struct R(crate::R<CAN_MSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_MSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAN_MSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAN_MSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_MSTS` writer"]
pub struct W(crate::W<CAN_MSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_MSTS_SPEC>;
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
impl From<crate::W<CAN_MSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAN_MSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIAK` reader - INIAK"]
pub type INIAK_R = crate::BitReader<bool>;
#[doc = "Field `INIAK` writer - INIAK"]
pub type INIAK_W<'a> = crate::BitWriter<'a, u32, CAN_MSTS_SPEC, bool, 0>;
#[doc = "Field `SLPAK` reader - SLPAK"]
pub type SLPAK_R = crate::BitReader<bool>;
#[doc = "Field `SLPAK` writer - SLPAK"]
pub type SLPAK_W<'a> = crate::BitWriter<'a, u32, CAN_MSTS_SPEC, bool, 1>;
#[doc = "Field `ERRINT` reader - ERRINT"]
pub type ERRINT_R = crate::BitReader<bool>;
#[doc = "Field `ERRINT` writer - ERRINT"]
pub type ERRINT_W<'a> = crate::BitWriter<'a, u32, CAN_MSTS_SPEC, bool, 2>;
#[doc = "Field `WKUINT` reader - WKUINT"]
pub type WKUINT_R = crate::BitReader<bool>;
#[doc = "Field `WKUINT` writer - WKUINT"]
pub type WKUINT_W<'a> = crate::BitWriter<'a, u32, CAN_MSTS_SPEC, bool, 3>;
#[doc = "Field `SLAKINT` reader - SLAKINT"]
pub type SLAKINT_R = crate::BitReader<bool>;
#[doc = "Field `SLAKINT` writer - SLAKINT"]
pub type SLAKINT_W<'a> = crate::BitWriter<'a, u32, CAN_MSTS_SPEC, bool, 4>;
#[doc = "Field `TXMD` reader - TXMD"]
pub type TXMD_R = crate::BitReader<bool>;
#[doc = "Field `TXMD` writer - TXMD"]
pub type TXMD_W<'a> = crate::BitWriter<'a, u32, CAN_MSTS_SPEC, bool, 8>;
#[doc = "Field `RXMD` reader - RXMD"]
pub type RXMD_R = crate::BitReader<bool>;
#[doc = "Field `RXMD` writer - RXMD"]
pub type RXMD_W<'a> = crate::BitWriter<'a, u32, CAN_MSTS_SPEC, bool, 9>;
#[doc = "Field `LSMP` reader - LSMP"]
pub type LSMP_R = crate::BitReader<bool>;
#[doc = "Field `LSMP` writer - LSMP"]
pub type LSMP_W<'a> = crate::BitWriter<'a, u32, CAN_MSTS_SPEC, bool, 10>;
#[doc = "Field `RXS` reader - RXS"]
pub type RXS_R = crate::BitReader<bool>;
#[doc = "Field `RXS` writer - RXS"]
pub type RXS_W<'a> = crate::BitWriter<'a, u32, CAN_MSTS_SPEC, bool, 11>;
impl R {
    #[doc = "Bit 0 - INIAK"]
    #[inline(always)]
    pub fn iniak(&self) -> INIAK_R {
        INIAK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SLPAK"]
    #[inline(always)]
    pub fn slpak(&self) -> SLPAK_R {
        SLPAK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ERRINT"]
    #[inline(always)]
    pub fn errint(&self) -> ERRINT_R {
        ERRINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WKUINT"]
    #[inline(always)]
    pub fn wkuint(&self) -> WKUINT_R {
        WKUINT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SLAKINT"]
    #[inline(always)]
    pub fn slakint(&self) -> SLAKINT_R {
        SLAKINT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - TXMD"]
    #[inline(always)]
    pub fn txmd(&self) -> TXMD_R {
        TXMD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXMD"]
    #[inline(always)]
    pub fn rxmd(&self) -> RXMD_R {
        RXMD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LSMP"]
    #[inline(always)]
    pub fn lsmp(&self) -> LSMP_R {
        LSMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RXS"]
    #[inline(always)]
    pub fn rxs(&self) -> RXS_R {
        RXS_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INIAK"]
    #[inline(always)]
    pub fn iniak(&mut self) -> INIAK_W {
        INIAK_W::new(self)
    }
    #[doc = "Bit 1 - SLPAK"]
    #[inline(always)]
    pub fn slpak(&mut self) -> SLPAK_W {
        SLPAK_W::new(self)
    }
    #[doc = "Bit 2 - ERRINT"]
    #[inline(always)]
    pub fn errint(&mut self) -> ERRINT_W {
        ERRINT_W::new(self)
    }
    #[doc = "Bit 3 - WKUINT"]
    #[inline(always)]
    pub fn wkuint(&mut self) -> WKUINT_W {
        WKUINT_W::new(self)
    }
    #[doc = "Bit 4 - SLAKINT"]
    #[inline(always)]
    pub fn slakint(&mut self) -> SLAKINT_W {
        SLAKINT_W::new(self)
    }
    #[doc = "Bit 8 - TXMD"]
    #[inline(always)]
    pub fn txmd(&mut self) -> TXMD_W {
        TXMD_W::new(self)
    }
    #[doc = "Bit 9 - RXMD"]
    #[inline(always)]
    pub fn rxmd(&mut self) -> RXMD_W {
        RXMD_W::new(self)
    }
    #[doc = "Bit 10 - LSMP"]
    #[inline(always)]
    pub fn lsmp(&mut self) -> LSMP_W {
        LSMP_W::new(self)
    }
    #[doc = "Bit 11 - RXS"]
    #[inline(always)]
    pub fn rxs(&mut self) -> RXS_W {
        RXS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN_MSTS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_msts](index.html) module"]
pub struct CAN_MSTS_SPEC;
impl crate::RegisterSpec for CAN_MSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_msts::R](R) reader structure"]
impl crate::Readable for CAN_MSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_msts::W](W) writer structure"]
impl crate::Writable for CAN_MSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_MSTS to value 0x0c02"]
impl crate::Resettable for CAN_MSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c02
    }
}
