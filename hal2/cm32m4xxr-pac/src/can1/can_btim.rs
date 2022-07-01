#[doc = "Register `CAN_BTIM` reader"]
pub struct R(crate::R<CAN_BTIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_BTIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAN_BTIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAN_BTIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_BTIM` writer"]
pub struct W(crate::W<CAN_BTIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_BTIM_SPEC>;
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
impl From<crate::W<CAN_BTIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAN_BTIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRTP` reader - BRTP"]
pub type BRTP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BRTP` writer - BRTP"]
pub type BRTP_W<'a> = crate::FieldWriter<'a, u32, CAN_BTIM_SPEC, u16, u16, 10, 0>;
#[doc = "Field `TBS1` reader - TBS1"]
pub type TBS1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TBS1` writer - TBS1"]
pub type TBS1_W<'a> = crate::FieldWriter<'a, u32, CAN_BTIM_SPEC, u8, u8, 4, 16>;
#[doc = "Field `TBS2` reader - TBS2"]
pub type TBS2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TBS2` writer - TBS2"]
pub type TBS2_W<'a> = crate::FieldWriter<'a, u32, CAN_BTIM_SPEC, u8, u8, 3, 20>;
#[doc = "Field `RSJW` reader - RSJW"]
pub type RSJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSJW` writer - RSJW"]
pub type RSJW_W<'a> = crate::FieldWriter<'a, u32, CAN_BTIM_SPEC, u8, u8, 2, 24>;
#[doc = "Field `LBM` reader - LBM"]
pub type LBM_R = crate::BitReader<bool>;
#[doc = "Field `LBM` writer - LBM"]
pub type LBM_W<'a> = crate::BitWriter<'a, u32, CAN_BTIM_SPEC, bool, 30>;
#[doc = "Field `SLM` reader - SLM"]
pub type SLM_R = crate::BitReader<bool>;
#[doc = "Field `SLM` writer - SLM"]
pub type SLM_W<'a> = crate::BitWriter<'a, u32, CAN_BTIM_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:9 - BRTP"]
    #[inline(always)]
    pub fn brtp(&self) -> BRTP_R {
        BRTP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:19 - TBS1"]
    #[inline(always)]
    pub fn tbs1(&self) -> TBS1_R {
        TBS1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - TBS2"]
    #[inline(always)]
    pub fn tbs2(&self) -> TBS2_R {
        TBS2_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - RSJW"]
    #[inline(always)]
    pub fn rsjw(&self) -> RSJW_R {
        RSJW_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - LBM"]
    #[inline(always)]
    pub fn lbm(&self) -> LBM_R {
        LBM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SLM"]
    #[inline(always)]
    pub fn slm(&self) -> SLM_R {
        SLM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - BRTP"]
    #[inline(always)]
    pub fn brtp(&mut self) -> BRTP_W {
        BRTP_W::new(self)
    }
    #[doc = "Bits 16:19 - TBS1"]
    #[inline(always)]
    pub fn tbs1(&mut self) -> TBS1_W {
        TBS1_W::new(self)
    }
    #[doc = "Bits 20:22 - TBS2"]
    #[inline(always)]
    pub fn tbs2(&mut self) -> TBS2_W {
        TBS2_W::new(self)
    }
    #[doc = "Bits 24:25 - RSJW"]
    #[inline(always)]
    pub fn rsjw(&mut self) -> RSJW_W {
        RSJW_W::new(self)
    }
    #[doc = "Bit 30 - LBM"]
    #[inline(always)]
    pub fn lbm(&mut self) -> LBM_W {
        LBM_W::new(self)
    }
    #[doc = "Bit 31 - SLM"]
    #[inline(always)]
    pub fn slm(&mut self) -> SLM_W {
        SLM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN_BTIM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_btim](index.html) module"]
pub struct CAN_BTIM_SPEC;
impl crate::RegisterSpec for CAN_BTIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_btim::R](R) reader structure"]
impl crate::Readable for CAN_BTIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_btim::W](W) writer structure"]
impl crate::Writable for CAN_BTIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_BTIM to value 0x0023_0000"]
impl crate::Resettable for CAN_BTIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0023_0000
    }
}
