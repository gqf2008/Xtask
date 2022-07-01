#[doc = "Register `AFIO_RMP_CFG5` reader"]
pub struct R(crate::R<AFIO_RMP_CFG5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIO_RMP_CFG5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIO_RMP_CFG5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIO_RMP_CFG5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFIO_RMP_CFG5` writer"]
pub struct W(crate::W<AFIO_RMP_CFG5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFIO_RMP_CFG5_SPEC>;
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
impl From<crate::W<AFIO_RMP_CFG5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFIO_RMP_CFG5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECLAMP1_RST_EN` reader - ECLAMP1_RST_EN"]
pub type ECLAMP1_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `ECLAMP1_RST_EN` writer - ECLAMP1_RST_EN"]
pub type ECLAMP1_RST_EN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG5_SPEC, bool, 0>;
#[doc = "Field `ECLAMP2_RST_EN` reader - ECLAMP2_RST_EN"]
pub type ECLAMP2_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `ECLAMP2_RST_EN` writer - ECLAMP2_RST_EN"]
pub type ECLAMP2_RST_EN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG5_SPEC, bool, 1>;
#[doc = "Field `ECLAMP3_RST_EN` reader - ECLAMP3_RST_EN"]
pub type ECLAMP3_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `ECLAMP3_RST_EN` writer - ECLAMP3_RST_EN"]
pub type ECLAMP3_RST_EN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG5_SPEC, bool, 2>;
#[doc = "Field `ECLAMP4_RST_EN` reader - ECLAMP4_RST_EN"]
pub type ECLAMP4_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `ECLAMP4_RST_EN` writer - ECLAMP4_RST_EN"]
pub type ECLAMP4_RST_EN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG5_SPEC, bool, 3>;
#[doc = "Field `EGBN1_RST_EN` reader - EGBN1_RST_EN"]
pub type EGBN1_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `EGBN1_RST_EN` writer - EGBN1_RST_EN"]
pub type EGBN1_RST_EN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG5_SPEC, bool, 4>;
#[doc = "Field `EGBN2_RST_EN` reader - EGBN2_RST_EN"]
pub type EGBN2_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `EGBN2_RST_EN` writer - EGBN2_RST_EN"]
pub type EGBN2_RST_EN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG5_SPEC, bool, 5>;
#[doc = "Field `EGBN3_RST_EN` reader - EGBN3_RST_EN"]
pub type EGBN3_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `EGBN3_RST_EN` writer - EGBN3_RST_EN"]
pub type EGBN3_RST_EN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG5_SPEC, bool, 6>;
#[doc = "Field `EGBN4_RST_EN` reader - EGBN4_RST_EN"]
pub type EGBN4_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `EGBN4_RST_EN` writer - EGBN4_RST_EN"]
pub type EGBN4_RST_EN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG5_SPEC, bool, 7>;
#[doc = "Field `EGB1_RST_EN` reader - EGB1_RST_EN"]
pub type EGB1_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `EGB1_RST_EN` writer - EGB1_RST_EN"]
pub type EGB1_RST_EN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG5_SPEC, bool, 8>;
#[doc = "Field `EGB2_RST_EN` reader - EGB2_RST_EN"]
pub type EGB2_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `EGB2_RST_EN` writer - EGB2_RST_EN"]
pub type EGB2_RST_EN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG5_SPEC, bool, 9>;
#[doc = "Field `EGB3_RST_EN` reader - EGB3_RST_EN"]
pub type EGB3_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `EGB3_RST_EN` writer - EGB3_RST_EN"]
pub type EGB3_RST_EN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG5_SPEC, bool, 10>;
#[doc = "Field `EGB4_RST_EN` reader - EGB4_RST_EN"]
pub type EGB4_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `EGB4_RST_EN` writer - EGB4_RST_EN"]
pub type EGB4_RST_EN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG5_SPEC, bool, 11>;
#[doc = "Field `ECLAMP1_DET_EN` reader - ECLAMP1_DET_EN"]
pub type ECLAMP1_DET_EN_R = crate::BitReader<bool>;
#[doc = "Field `ECLAMP1_DET_EN` writer - ECLAMP1_DET_EN"]
pub type ECLAMP1_DET_EN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG5_SPEC, bool, 12>;
#[doc = "Field `ECLAMP2_DET_EN` reader - ECLAMP2_DET_EN"]
pub type ECLAMP2_DET_EN_R = crate::BitReader<bool>;
#[doc = "Field `ECLAMP2_DET_EN` writer - ECLAMP2_DET_EN"]
pub type ECLAMP2_DET_EN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG5_SPEC, bool, 13>;
#[doc = "Field `ECLAMP3_DET_EN` reader - ECLAMP3_DET_EN"]
pub type ECLAMP3_DET_EN_R = crate::BitReader<bool>;
#[doc = "Field `ECLAMP3_DET_EN` writer - ECLAMP3_DET_EN"]
pub type ECLAMP3_DET_EN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG5_SPEC, bool, 14>;
#[doc = "Field `ECLAMP4_DET_EN` reader - ECLAMP4_DET_EN"]
pub type ECLAMP4_DET_EN_R = crate::BitReader<bool>;
#[doc = "Field `ECLAMP4_DET_EN` writer - ECLAMP4_DET_EN"]
pub type ECLAMP4_DET_EN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG5_SPEC, bool, 15>;
#[doc = "Field `EGBN1_DET_EN` reader - EGBN1_DET_EN"]
pub type EGBN1_DET_EN_R = crate::BitReader<bool>;
#[doc = "Field `EGBN1_DET_EN` writer - EGBN1_DET_EN"]
pub type EGBN1_DET_EN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG5_SPEC, bool, 16>;
#[doc = "Field `EGBN2_DET_EN` reader - EGBN2_DET_EN"]
pub type EGBN2_DET_EN_R = crate::BitReader<bool>;
#[doc = "Field `EGBN2_DET_EN` writer - EGBN2_DET_EN"]
pub type EGBN2_DET_EN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG5_SPEC, bool, 17>;
#[doc = "Field `EGBN3_DET_EN` reader - EGBN3_DET_EN"]
pub type EGBN3_DET_EN_R = crate::BitReader<bool>;
#[doc = "Field `EGBN3_DET_EN` writer - EGBN3_DET_EN"]
pub type EGBN3_DET_EN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG5_SPEC, bool, 18>;
#[doc = "Field `EGBN4_DET_EN` reader - EGBN4_DET_EN"]
pub type EGBN4_DET_EN_R = crate::BitReader<bool>;
#[doc = "Field `EGBN4_DET_EN` writer - EGBN4_DET_EN"]
pub type EGBN4_DET_EN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG5_SPEC, bool, 19>;
#[doc = "Field `EGB1_DETEN` reader - EGB1_DETEN"]
pub type EGB1_DETEN_R = crate::BitReader<bool>;
#[doc = "Field `EGB1_DETEN` writer - EGB1_DETEN"]
pub type EGB1_DETEN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG5_SPEC, bool, 20>;
#[doc = "Field `EGB2_DET_EN` reader - EGB2_DET_EN"]
pub type EGB2_DET_EN_R = crate::BitReader<bool>;
#[doc = "Field `EGB2_DET_EN` writer - EGB2_DET_EN"]
pub type EGB2_DET_EN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG5_SPEC, bool, 21>;
#[doc = "Field `EGB3_DET_EN` reader - EGB3_DET_EN"]
pub type EGB3_DET_EN_R = crate::BitReader<bool>;
#[doc = "Field `EGB3_DET_EN` writer - EGB3_DET_EN"]
pub type EGB3_DET_EN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG5_SPEC, bool, 22>;
#[doc = "Field `EGB4_DET_EN` reader - EGB4_DET_EN"]
pub type EGB4_DET_EN_R = crate::BitReader<bool>;
#[doc = "Field `EGB4_DET_EN` writer - EGB4_DET_EN"]
pub type EGB4_DET_EN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG5_SPEC, bool, 23>;
impl R {
    #[doc = "Bit 0 - ECLAMP1_RST_EN"]
    #[inline(always)]
    pub fn eclamp1_rst_en(&self) -> ECLAMP1_RST_EN_R {
        ECLAMP1_RST_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECLAMP2_RST_EN"]
    #[inline(always)]
    pub fn eclamp2_rst_en(&self) -> ECLAMP2_RST_EN_R {
        ECLAMP2_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ECLAMP3_RST_EN"]
    #[inline(always)]
    pub fn eclamp3_rst_en(&self) -> ECLAMP3_RST_EN_R {
        ECLAMP3_RST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECLAMP4_RST_EN"]
    #[inline(always)]
    pub fn eclamp4_rst_en(&self) -> ECLAMP4_RST_EN_R {
        ECLAMP4_RST_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EGBN1_RST_EN"]
    #[inline(always)]
    pub fn egbn1_rst_en(&self) -> EGBN1_RST_EN_R {
        EGBN1_RST_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EGBN2_RST_EN"]
    #[inline(always)]
    pub fn egbn2_rst_en(&self) -> EGBN2_RST_EN_R {
        EGBN2_RST_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EGBN3_RST_EN"]
    #[inline(always)]
    pub fn egbn3_rst_en(&self) -> EGBN3_RST_EN_R {
        EGBN3_RST_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - EGBN4_RST_EN"]
    #[inline(always)]
    pub fn egbn4_rst_en(&self) -> EGBN4_RST_EN_R {
        EGBN4_RST_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EGB1_RST_EN"]
    #[inline(always)]
    pub fn egb1_rst_en(&self) -> EGB1_RST_EN_R {
        EGB1_RST_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EGB2_RST_EN"]
    #[inline(always)]
    pub fn egb2_rst_en(&self) -> EGB2_RST_EN_R {
        EGB2_RST_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EGB3_RST_EN"]
    #[inline(always)]
    pub fn egb3_rst_en(&self) -> EGB3_RST_EN_R {
        EGB3_RST_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EGB4_RST_EN"]
    #[inline(always)]
    pub fn egb4_rst_en(&self) -> EGB4_RST_EN_R {
        EGB4_RST_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ECLAMP1_DET_EN"]
    #[inline(always)]
    pub fn eclamp1_det_en(&self) -> ECLAMP1_DET_EN_R {
        ECLAMP1_DET_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ECLAMP2_DET_EN"]
    #[inline(always)]
    pub fn eclamp2_det_en(&self) -> ECLAMP2_DET_EN_R {
        ECLAMP2_DET_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ECLAMP3_DET_EN"]
    #[inline(always)]
    pub fn eclamp3_det_en(&self) -> ECLAMP3_DET_EN_R {
        ECLAMP3_DET_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ECLAMP4_DET_EN"]
    #[inline(always)]
    pub fn eclamp4_det_en(&self) -> ECLAMP4_DET_EN_R {
        ECLAMP4_DET_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - EGBN1_DET_EN"]
    #[inline(always)]
    pub fn egbn1_det_en(&self) -> EGBN1_DET_EN_R {
        EGBN1_DET_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - EGBN2_DET_EN"]
    #[inline(always)]
    pub fn egbn2_det_en(&self) -> EGBN2_DET_EN_R {
        EGBN2_DET_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - EGBN3_DET_EN"]
    #[inline(always)]
    pub fn egbn3_det_en(&self) -> EGBN3_DET_EN_R {
        EGBN3_DET_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - EGBN4_DET_EN"]
    #[inline(always)]
    pub fn egbn4_det_en(&self) -> EGBN4_DET_EN_R {
        EGBN4_DET_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - EGB1_DETEN"]
    #[inline(always)]
    pub fn egb1_deten(&self) -> EGB1_DETEN_R {
        EGB1_DETEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - EGB2_DET_EN"]
    #[inline(always)]
    pub fn egb2_det_en(&self) -> EGB2_DET_EN_R {
        EGB2_DET_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - EGB3_DET_EN"]
    #[inline(always)]
    pub fn egb3_det_en(&self) -> EGB3_DET_EN_R {
        EGB3_DET_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - EGB4_DET_EN"]
    #[inline(always)]
    pub fn egb4_det_en(&self) -> EGB4_DET_EN_R {
        EGB4_DET_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECLAMP1_RST_EN"]
    #[inline(always)]
    pub fn eclamp1_rst_en(&mut self) -> ECLAMP1_RST_EN_W {
        ECLAMP1_RST_EN_W::new(self)
    }
    #[doc = "Bit 1 - ECLAMP2_RST_EN"]
    #[inline(always)]
    pub fn eclamp2_rst_en(&mut self) -> ECLAMP2_RST_EN_W {
        ECLAMP2_RST_EN_W::new(self)
    }
    #[doc = "Bit 2 - ECLAMP3_RST_EN"]
    #[inline(always)]
    pub fn eclamp3_rst_en(&mut self) -> ECLAMP3_RST_EN_W {
        ECLAMP3_RST_EN_W::new(self)
    }
    #[doc = "Bit 3 - ECLAMP4_RST_EN"]
    #[inline(always)]
    pub fn eclamp4_rst_en(&mut self) -> ECLAMP4_RST_EN_W {
        ECLAMP4_RST_EN_W::new(self)
    }
    #[doc = "Bit 4 - EGBN1_RST_EN"]
    #[inline(always)]
    pub fn egbn1_rst_en(&mut self) -> EGBN1_RST_EN_W {
        EGBN1_RST_EN_W::new(self)
    }
    #[doc = "Bit 5 - EGBN2_RST_EN"]
    #[inline(always)]
    pub fn egbn2_rst_en(&mut self) -> EGBN2_RST_EN_W {
        EGBN2_RST_EN_W::new(self)
    }
    #[doc = "Bit 6 - EGBN3_RST_EN"]
    #[inline(always)]
    pub fn egbn3_rst_en(&mut self) -> EGBN3_RST_EN_W {
        EGBN3_RST_EN_W::new(self)
    }
    #[doc = "Bit 7 - EGBN4_RST_EN"]
    #[inline(always)]
    pub fn egbn4_rst_en(&mut self) -> EGBN4_RST_EN_W {
        EGBN4_RST_EN_W::new(self)
    }
    #[doc = "Bit 8 - EGB1_RST_EN"]
    #[inline(always)]
    pub fn egb1_rst_en(&mut self) -> EGB1_RST_EN_W {
        EGB1_RST_EN_W::new(self)
    }
    #[doc = "Bit 9 - EGB2_RST_EN"]
    #[inline(always)]
    pub fn egb2_rst_en(&mut self) -> EGB2_RST_EN_W {
        EGB2_RST_EN_W::new(self)
    }
    #[doc = "Bit 10 - EGB3_RST_EN"]
    #[inline(always)]
    pub fn egb3_rst_en(&mut self) -> EGB3_RST_EN_W {
        EGB3_RST_EN_W::new(self)
    }
    #[doc = "Bit 11 - EGB4_RST_EN"]
    #[inline(always)]
    pub fn egb4_rst_en(&mut self) -> EGB4_RST_EN_W {
        EGB4_RST_EN_W::new(self)
    }
    #[doc = "Bit 12 - ECLAMP1_DET_EN"]
    #[inline(always)]
    pub fn eclamp1_det_en(&mut self) -> ECLAMP1_DET_EN_W {
        ECLAMP1_DET_EN_W::new(self)
    }
    #[doc = "Bit 13 - ECLAMP2_DET_EN"]
    #[inline(always)]
    pub fn eclamp2_det_en(&mut self) -> ECLAMP2_DET_EN_W {
        ECLAMP2_DET_EN_W::new(self)
    }
    #[doc = "Bit 14 - ECLAMP3_DET_EN"]
    #[inline(always)]
    pub fn eclamp3_det_en(&mut self) -> ECLAMP3_DET_EN_W {
        ECLAMP3_DET_EN_W::new(self)
    }
    #[doc = "Bit 15 - ECLAMP4_DET_EN"]
    #[inline(always)]
    pub fn eclamp4_det_en(&mut self) -> ECLAMP4_DET_EN_W {
        ECLAMP4_DET_EN_W::new(self)
    }
    #[doc = "Bit 16 - EGBN1_DET_EN"]
    #[inline(always)]
    pub fn egbn1_det_en(&mut self) -> EGBN1_DET_EN_W {
        EGBN1_DET_EN_W::new(self)
    }
    #[doc = "Bit 17 - EGBN2_DET_EN"]
    #[inline(always)]
    pub fn egbn2_det_en(&mut self) -> EGBN2_DET_EN_W {
        EGBN2_DET_EN_W::new(self)
    }
    #[doc = "Bit 18 - EGBN3_DET_EN"]
    #[inline(always)]
    pub fn egbn3_det_en(&mut self) -> EGBN3_DET_EN_W {
        EGBN3_DET_EN_W::new(self)
    }
    #[doc = "Bit 19 - EGBN4_DET_EN"]
    #[inline(always)]
    pub fn egbn4_det_en(&mut self) -> EGBN4_DET_EN_W {
        EGBN4_DET_EN_W::new(self)
    }
    #[doc = "Bit 20 - EGB1_DETEN"]
    #[inline(always)]
    pub fn egb1_deten(&mut self) -> EGB1_DETEN_W {
        EGB1_DETEN_W::new(self)
    }
    #[doc = "Bit 21 - EGB2_DET_EN"]
    #[inline(always)]
    pub fn egb2_det_en(&mut self) -> EGB2_DET_EN_W {
        EGB2_DET_EN_W::new(self)
    }
    #[doc = "Bit 22 - EGB3_DET_EN"]
    #[inline(always)]
    pub fn egb3_det_en(&mut self) -> EGB3_DET_EN_W {
        EGB3_DET_EN_W::new(self)
    }
    #[doc = "Bit 23 - EGB4_DET_EN"]
    #[inline(always)]
    pub fn egb4_det_en(&mut self) -> EGB4_DET_EN_W {
        EGB4_DET_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO_RMP_CFG5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_rmp_cfg5](index.html) module"]
pub struct AFIO_RMP_CFG5_SPEC;
impl crate::RegisterSpec for AFIO_RMP_CFG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afio_rmp_cfg5::R](R) reader structure"]
impl crate::Readable for AFIO_RMP_CFG5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afio_rmp_cfg5::W](W) writer structure"]
impl crate::Writable for AFIO_RMP_CFG5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFIO_RMP_CFG5 to value 0"]
impl crate::Resettable for AFIO_RMP_CFG5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
