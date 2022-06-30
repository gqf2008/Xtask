#[doc = "Register `RCC_BDCTRL` reader"]
pub struct R(crate::R<RCC_BDCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_BDCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_BDCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_BDCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_BDCTRL` writer"]
pub struct W(crate::W<RCC_BDCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_BDCTRL_SPEC>;
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
impl From<crate::W<RCC_BDCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_BDCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSEEN` reader - LSEEN"]
pub type LSEEN_R = crate::BitReader<bool>;
#[doc = "Field `LSEEN` writer - LSEEN"]
pub type LSEEN_W<'a> = crate::BitWriter<'a, u32, RCC_BDCTRL_SPEC, bool, 0>;
#[doc = "Field `LSERD` reader - LSERDIF"]
pub type LSERD_R = crate::BitReader<bool>;
#[doc = "Field `LSERD` writer - LSERDIF"]
pub type LSERD_W<'a> = crate::BitWriter<'a, u32, RCC_BDCTRL_SPEC, bool, 1>;
#[doc = "Field `LSEBP` reader - LSEBP"]
pub type LSEBP_R = crate::BitReader<bool>;
#[doc = "Field `LSEBP` writer - LSEBP"]
pub type LSEBP_W<'a> = crate::BitWriter<'a, u32, RCC_BDCTRL_SPEC, bool, 2>;
#[doc = "Field `RTCSEL` reader - RTCSEL"]
pub type RTCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTCSEL` writer - RTCSEL"]
pub type RTCSEL_W<'a> = crate::FieldWriter<'a, u32, RCC_BDCTRL_SPEC, u8, u8, 2, 8>;
#[doc = "Field `RTCEN` reader - RTCEN"]
pub type RTCEN_R = crate::BitReader<bool>;
#[doc = "Field `RTCEN` writer - RTCEN"]
pub type RTCEN_W<'a> = crate::BitWriter<'a, u32, RCC_BDCTRL_SPEC, bool, 15>;
#[doc = "Field `BDSFTRST` reader - BDSFTRST"]
pub type BDSFTRST_R = crate::BitReader<bool>;
#[doc = "Field `BDSFTRST` writer - BDSFTRST"]
pub type BDSFTRST_W<'a> = crate::BitWriter<'a, u32, RCC_BDCTRL_SPEC, bool, 16>;
impl R {
    #[doc = "Bit 0 - LSEEN"]
    #[inline(always)]
    pub fn lseen(&self) -> LSEEN_R {
        LSEEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSERDIF"]
    #[inline(always)]
    pub fn lserd(&self) -> LSERD_R {
        LSERD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LSEBP"]
    #[inline(always)]
    pub fn lsebp(&self) -> LSEBP_R {
        LSEBP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTCSEL"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - RTCEN"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - BDSFTRST"]
    #[inline(always)]
    pub fn bdsftrst(&self) -> BDSFTRST_R {
        BDSFTRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSEEN"]
    #[inline(always)]
    pub fn lseen(&mut self) -> LSEEN_W {
        LSEEN_W::new(self)
    }
    #[doc = "Bit 1 - LSERDIF"]
    #[inline(always)]
    pub fn lserd(&mut self) -> LSERD_W {
        LSERD_W::new(self)
    }
    #[doc = "Bit 2 - LSEBP"]
    #[inline(always)]
    pub fn lsebp(&mut self) -> LSEBP_W {
        LSEBP_W::new(self)
    }
    #[doc = "Bits 8:9 - RTCSEL"]
    #[inline(always)]
    pub fn rtcsel(&mut self) -> RTCSEL_W {
        RTCSEL_W::new(self)
    }
    #[doc = "Bit 15 - RTCEN"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W {
        RTCEN_W::new(self)
    }
    #[doc = "Bit 16 - BDSFTRST"]
    #[inline(always)]
    pub fn bdsftrst(&mut self) -> BDSFTRST_W {
        BDSFTRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC_BDCTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_bdctrl](index.html) module"]
pub struct RCC_BDCTRL_SPEC;
impl crate::RegisterSpec for RCC_BDCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_bdctrl::R](R) reader structure"]
impl crate::Readable for RCC_BDCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_bdctrl::W](W) writer structure"]
impl crate::Writable for RCC_BDCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_BDCTRL to value 0"]
impl crate::Resettable for RCC_BDCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
