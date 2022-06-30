#[doc = "Register `CAN_MCTRL` reader"]
pub struct R(crate::R<CAN_MCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_MCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAN_MCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAN_MCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_MCTRL` writer"]
pub struct W(crate::W<CAN_MCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_MCTRL_SPEC>;
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
impl From<crate::W<CAN_MCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAN_MCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIRQ` reader - INIRQ"]
pub type INIRQ_R = crate::BitReader<bool>;
#[doc = "Field `INIRQ` writer - INIRQ"]
pub type INIRQ_W<'a> = crate::BitWriter<'a, u32, CAN_MCTRL_SPEC, bool, 0>;
#[doc = "Field `SLPRQ` reader - SLPRQ"]
pub type SLPRQ_R = crate::BitReader<bool>;
#[doc = "Field `SLPRQ` writer - SLPRQ"]
pub type SLPRQ_W<'a> = crate::BitWriter<'a, u32, CAN_MCTRL_SPEC, bool, 1>;
#[doc = "Field `TXFP` reader - TXFP"]
pub type TXFP_R = crate::BitReader<bool>;
#[doc = "Field `TXFP` writer - TXFP"]
pub type TXFP_W<'a> = crate::BitWriter<'a, u32, CAN_MCTRL_SPEC, bool, 2>;
#[doc = "Field `RFLM` reader - RFLM"]
pub type RFLM_R = crate::BitReader<bool>;
#[doc = "Field `RFLM` writer - RFLM"]
pub type RFLM_W<'a> = crate::BitWriter<'a, u32, CAN_MCTRL_SPEC, bool, 3>;
#[doc = "Field `NART` reader - NART"]
pub type NART_R = crate::BitReader<bool>;
#[doc = "Field `NART` writer - NART"]
pub type NART_W<'a> = crate::BitWriter<'a, u32, CAN_MCTRL_SPEC, bool, 4>;
#[doc = "Field `AWKUM` reader - AWKUM"]
pub type AWKUM_R = crate::BitReader<bool>;
#[doc = "Field `AWKUM` writer - AWKUM"]
pub type AWKUM_W<'a> = crate::BitWriter<'a, u32, CAN_MCTRL_SPEC, bool, 5>;
#[doc = "Field `ABOM` reader - ABOM"]
pub type ABOM_R = crate::BitReader<bool>;
#[doc = "Field `ABOM` writer - ABOM"]
pub type ABOM_W<'a> = crate::BitWriter<'a, u32, CAN_MCTRL_SPEC, bool, 6>;
#[doc = "Field `TTCM` reader - TTCM"]
pub type TTCM_R = crate::BitReader<bool>;
#[doc = "Field `TTCM` writer - TTCM"]
pub type TTCM_W<'a> = crate::BitWriter<'a, u32, CAN_MCTRL_SPEC, bool, 7>;
#[doc = "Field `MRST` reader - MRST"]
pub type MRST_R = crate::BitReader<bool>;
#[doc = "Field `MRST` writer - MRST"]
pub type MRST_W<'a> = crate::BitWriter<'a, u32, CAN_MCTRL_SPEC, bool, 15>;
#[doc = "Field `DBGF` reader - DBGF"]
pub type DBGF_R = crate::BitReader<bool>;
#[doc = "Field `DBGF` writer - DBGF"]
pub type DBGF_W<'a> = crate::BitWriter<'a, u32, CAN_MCTRL_SPEC, bool, 16>;
impl R {
    #[doc = "Bit 0 - INIRQ"]
    #[inline(always)]
    pub fn inirq(&self) -> INIRQ_R {
        INIRQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SLPRQ"]
    #[inline(always)]
    pub fn slprq(&self) -> SLPRQ_R {
        SLPRQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXFP"]
    #[inline(always)]
    pub fn txfp(&self) -> TXFP_R {
        TXFP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RFLM"]
    #[inline(always)]
    pub fn rflm(&self) -> RFLM_R {
        RFLM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NART"]
    #[inline(always)]
    pub fn nart(&self) -> NART_R {
        NART_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AWKUM"]
    #[inline(always)]
    pub fn awkum(&self) -> AWKUM_R {
        AWKUM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ABOM"]
    #[inline(always)]
    pub fn abom(&self) -> ABOM_R {
        ABOM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TTCM"]
    #[inline(always)]
    pub fn ttcm(&self) -> TTCM_R {
        TTCM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - MRST"]
    #[inline(always)]
    pub fn mrst(&self) -> MRST_R {
        MRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DBGF"]
    #[inline(always)]
    pub fn dbgf(&self) -> DBGF_R {
        DBGF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INIRQ"]
    #[inline(always)]
    pub fn inirq(&mut self) -> INIRQ_W {
        INIRQ_W::new(self)
    }
    #[doc = "Bit 1 - SLPRQ"]
    #[inline(always)]
    pub fn slprq(&mut self) -> SLPRQ_W {
        SLPRQ_W::new(self)
    }
    #[doc = "Bit 2 - TXFP"]
    #[inline(always)]
    pub fn txfp(&mut self) -> TXFP_W {
        TXFP_W::new(self)
    }
    #[doc = "Bit 3 - RFLM"]
    #[inline(always)]
    pub fn rflm(&mut self) -> RFLM_W {
        RFLM_W::new(self)
    }
    #[doc = "Bit 4 - NART"]
    #[inline(always)]
    pub fn nart(&mut self) -> NART_W {
        NART_W::new(self)
    }
    #[doc = "Bit 5 - AWKUM"]
    #[inline(always)]
    pub fn awkum(&mut self) -> AWKUM_W {
        AWKUM_W::new(self)
    }
    #[doc = "Bit 6 - ABOM"]
    #[inline(always)]
    pub fn abom(&mut self) -> ABOM_W {
        ABOM_W::new(self)
    }
    #[doc = "Bit 7 - TTCM"]
    #[inline(always)]
    pub fn ttcm(&mut self) -> TTCM_W {
        TTCM_W::new(self)
    }
    #[doc = "Bit 15 - MRST"]
    #[inline(always)]
    pub fn mrst(&mut self) -> MRST_W {
        MRST_W::new(self)
    }
    #[doc = "Bit 16 - DBGF"]
    #[inline(always)]
    pub fn dbgf(&mut self) -> DBGF_W {
        DBGF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN_MCTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_mctrl](index.html) module"]
pub struct CAN_MCTRL_SPEC;
impl crate::RegisterSpec for CAN_MCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_mctrl::R](R) reader structure"]
impl crate::Readable for CAN_MCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_mctrl::W](W) writer structure"]
impl crate::Writable for CAN_MCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_MCTRL to value 0x0001_0002"]
impl crate::Resettable for CAN_MCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0002
    }
}
