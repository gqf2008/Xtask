#[doc = "Register `CAN_TSTS` reader"]
pub struct R(crate::R<CAN_TSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_TSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAN_TSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAN_TSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_TSTS` writer"]
pub struct W(crate::W<CAN_TSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_TSTS_SPEC>;
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
impl From<crate::W<CAN_TSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAN_TSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RQCPM0` reader - RQCPM0"]
pub type RQCPM0_R = crate::BitReader<bool>;
#[doc = "Field `RQCPM0` writer - RQCPM0"]
pub type RQCPM0_W<'a> = crate::BitWriter<'a, u32, CAN_TSTS_SPEC, bool, 0>;
#[doc = "Field `TXOKM0` reader - TXOKM0"]
pub type TXOKM0_R = crate::BitReader<bool>;
#[doc = "Field `TXOKM0` writer - TXOKM0"]
pub type TXOKM0_W<'a> = crate::BitWriter<'a, u32, CAN_TSTS_SPEC, bool, 1>;
#[doc = "Field `ALSTM0` reader - ALSTM0"]
pub type ALSTM0_R = crate::BitReader<bool>;
#[doc = "Field `ALSTM0` writer - ALSTM0"]
pub type ALSTM0_W<'a> = crate::BitWriter<'a, u32, CAN_TSTS_SPEC, bool, 2>;
#[doc = "Field `TERRM0` reader - TERRM0"]
pub type TERRM0_R = crate::BitReader<bool>;
#[doc = "Field `TERRM0` writer - TERRM0"]
pub type TERRM0_W<'a> = crate::BitWriter<'a, u32, CAN_TSTS_SPEC, bool, 3>;
#[doc = "Field `ABRQM0` reader - ABRQM0"]
pub type ABRQM0_R = crate::BitReader<bool>;
#[doc = "Field `ABRQM0` writer - ABRQM0"]
pub type ABRQM0_W<'a> = crate::BitWriter<'a, u32, CAN_TSTS_SPEC, bool, 7>;
#[doc = "Field `RQCPM1` reader - RQCPM1"]
pub type RQCPM1_R = crate::BitReader<bool>;
#[doc = "Field `RQCPM1` writer - RQCPM1"]
pub type RQCPM1_W<'a> = crate::BitWriter<'a, u32, CAN_TSTS_SPEC, bool, 8>;
#[doc = "Field `TXOKM1` reader - TXOKM1"]
pub type TXOKM1_R = crate::BitReader<bool>;
#[doc = "Field `TXOKM1` writer - TXOKM1"]
pub type TXOKM1_W<'a> = crate::BitWriter<'a, u32, CAN_TSTS_SPEC, bool, 9>;
#[doc = "Field `ALSTM1` reader - ALSTM1"]
pub type ALSTM1_R = crate::BitReader<bool>;
#[doc = "Field `ALSTM1` writer - ALSTM1"]
pub type ALSTM1_W<'a> = crate::BitWriter<'a, u32, CAN_TSTS_SPEC, bool, 10>;
#[doc = "Field `TERRM1` reader - TERRM1"]
pub type TERRM1_R = crate::BitReader<bool>;
#[doc = "Field `TERRM1` writer - TERRM1"]
pub type TERRM1_W<'a> = crate::BitWriter<'a, u32, CAN_TSTS_SPEC, bool, 11>;
#[doc = "Field `ABRQM1` reader - ABRQM1"]
pub type ABRQM1_R = crate::BitReader<bool>;
#[doc = "Field `ABRQM1` writer - ABRQM1"]
pub type ABRQM1_W<'a> = crate::BitWriter<'a, u32, CAN_TSTS_SPEC, bool, 15>;
#[doc = "Field `RQCPM2` reader - RQCPM2"]
pub type RQCPM2_R = crate::BitReader<bool>;
#[doc = "Field `RQCPM2` writer - RQCPM2"]
pub type RQCPM2_W<'a> = crate::BitWriter<'a, u32, CAN_TSTS_SPEC, bool, 16>;
#[doc = "Field `TXOKM2` reader - TXOKM2"]
pub type TXOKM2_R = crate::BitReader<bool>;
#[doc = "Field `TXOKM2` writer - TXOKM2"]
pub type TXOKM2_W<'a> = crate::BitWriter<'a, u32, CAN_TSTS_SPEC, bool, 17>;
#[doc = "Field `ALSTM2` reader - ALSTM2"]
pub type ALSTM2_R = crate::BitReader<bool>;
#[doc = "Field `ALSTM2` writer - ALSTM2"]
pub type ALSTM2_W<'a> = crate::BitWriter<'a, u32, CAN_TSTS_SPEC, bool, 18>;
#[doc = "Field `TERRM2` reader - TERRM2"]
pub type TERRM2_R = crate::BitReader<bool>;
#[doc = "Field `TERRM2` writer - TERRM2"]
pub type TERRM2_W<'a> = crate::BitWriter<'a, u32, CAN_TSTS_SPEC, bool, 19>;
#[doc = "Field `ABRQM2` reader - ABRQM2"]
pub type ABRQM2_R = crate::BitReader<bool>;
#[doc = "Field `ABRQM2` writer - ABRQM2"]
pub type ABRQM2_W<'a> = crate::BitWriter<'a, u32, CAN_TSTS_SPEC, bool, 23>;
#[doc = "Field `CODE` reader - CODE"]
pub type CODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CODE` writer - CODE"]
pub type CODE_W<'a> = crate::FieldWriter<'a, u32, CAN_TSTS_SPEC, u8, u8, 2, 24>;
#[doc = "Field `TMEM` reader - TMEM"]
pub type TMEM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMEM` writer - TMEM"]
pub type TMEM_W<'a> = crate::FieldWriter<'a, u32, CAN_TSTS_SPEC, u8, u8, 3, 26>;
#[doc = "Field `LOWM` reader - LOWM"]
pub type LOWM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOWM` writer - LOWM"]
pub type LOWM_W<'a> = crate::FieldWriter<'a, u32, CAN_TSTS_SPEC, u8, u8, 3, 29>;
impl R {
    #[doc = "Bit 0 - RQCPM0"]
    #[inline(always)]
    pub fn rqcpm0(&self) -> RQCPM0_R {
        RQCPM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXOKM0"]
    #[inline(always)]
    pub fn txokm0(&self) -> TXOKM0_R {
        TXOKM0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ALSTM0"]
    #[inline(always)]
    pub fn alstm0(&self) -> ALSTM0_R {
        ALSTM0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TERRM0"]
    #[inline(always)]
    pub fn terrm0(&self) -> TERRM0_R {
        TERRM0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - ABRQM0"]
    #[inline(always)]
    pub fn abrqm0(&self) -> ABRQM0_R {
        ABRQM0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RQCPM1"]
    #[inline(always)]
    pub fn rqcpm1(&self) -> RQCPM1_R {
        RQCPM1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TXOKM1"]
    #[inline(always)]
    pub fn txokm1(&self) -> TXOKM1_R {
        TXOKM1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ALSTM1"]
    #[inline(always)]
    pub fn alstm1(&self) -> ALSTM1_R {
        ALSTM1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TERRM1"]
    #[inline(always)]
    pub fn terrm1(&self) -> TERRM1_R {
        TERRM1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - ABRQM1"]
    #[inline(always)]
    pub fn abrqm1(&self) -> ABRQM1_R {
        ABRQM1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RQCPM2"]
    #[inline(always)]
    pub fn rqcpm2(&self) -> RQCPM2_R {
        RQCPM2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TXOKM2"]
    #[inline(always)]
    pub fn txokm2(&self) -> TXOKM2_R {
        TXOKM2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ALSTM2"]
    #[inline(always)]
    pub fn alstm2(&self) -> ALSTM2_R {
        ALSTM2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TERRM2"]
    #[inline(always)]
    pub fn terrm2(&self) -> TERRM2_R {
        TERRM2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - ABRQM2"]
    #[inline(always)]
    pub fn abrqm2(&self) -> ABRQM2_R {
        ABRQM2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - CODE"]
    #[inline(always)]
    pub fn code(&self) -> CODE_R {
        CODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:28 - TMEM"]
    #[inline(always)]
    pub fn tmem(&self) -> TMEM_R {
        TMEM_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - LOWM"]
    #[inline(always)]
    pub fn lowm(&self) -> LOWM_R {
        LOWM_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RQCPM0"]
    #[inline(always)]
    pub fn rqcpm0(&mut self) -> RQCPM0_W {
        RQCPM0_W::new(self)
    }
    #[doc = "Bit 1 - TXOKM0"]
    #[inline(always)]
    pub fn txokm0(&mut self) -> TXOKM0_W {
        TXOKM0_W::new(self)
    }
    #[doc = "Bit 2 - ALSTM0"]
    #[inline(always)]
    pub fn alstm0(&mut self) -> ALSTM0_W {
        ALSTM0_W::new(self)
    }
    #[doc = "Bit 3 - TERRM0"]
    #[inline(always)]
    pub fn terrm0(&mut self) -> TERRM0_W {
        TERRM0_W::new(self)
    }
    #[doc = "Bit 7 - ABRQM0"]
    #[inline(always)]
    pub fn abrqm0(&mut self) -> ABRQM0_W {
        ABRQM0_W::new(self)
    }
    #[doc = "Bit 8 - RQCPM1"]
    #[inline(always)]
    pub fn rqcpm1(&mut self) -> RQCPM1_W {
        RQCPM1_W::new(self)
    }
    #[doc = "Bit 9 - TXOKM1"]
    #[inline(always)]
    pub fn txokm1(&mut self) -> TXOKM1_W {
        TXOKM1_W::new(self)
    }
    #[doc = "Bit 10 - ALSTM1"]
    #[inline(always)]
    pub fn alstm1(&mut self) -> ALSTM1_W {
        ALSTM1_W::new(self)
    }
    #[doc = "Bit 11 - TERRM1"]
    #[inline(always)]
    pub fn terrm1(&mut self) -> TERRM1_W {
        TERRM1_W::new(self)
    }
    #[doc = "Bit 15 - ABRQM1"]
    #[inline(always)]
    pub fn abrqm1(&mut self) -> ABRQM1_W {
        ABRQM1_W::new(self)
    }
    #[doc = "Bit 16 - RQCPM2"]
    #[inline(always)]
    pub fn rqcpm2(&mut self) -> RQCPM2_W {
        RQCPM2_W::new(self)
    }
    #[doc = "Bit 17 - TXOKM2"]
    #[inline(always)]
    pub fn txokm2(&mut self) -> TXOKM2_W {
        TXOKM2_W::new(self)
    }
    #[doc = "Bit 18 - ALSTM2"]
    #[inline(always)]
    pub fn alstm2(&mut self) -> ALSTM2_W {
        ALSTM2_W::new(self)
    }
    #[doc = "Bit 19 - TERRM2"]
    #[inline(always)]
    pub fn terrm2(&mut self) -> TERRM2_W {
        TERRM2_W::new(self)
    }
    #[doc = "Bit 23 - ABRQM2"]
    #[inline(always)]
    pub fn abrqm2(&mut self) -> ABRQM2_W {
        ABRQM2_W::new(self)
    }
    #[doc = "Bits 24:25 - CODE"]
    #[inline(always)]
    pub fn code(&mut self) -> CODE_W {
        CODE_W::new(self)
    }
    #[doc = "Bits 26:28 - TMEM"]
    #[inline(always)]
    pub fn tmem(&mut self) -> TMEM_W {
        TMEM_W::new(self)
    }
    #[doc = "Bits 29:31 - LOWM"]
    #[inline(always)]
    pub fn lowm(&mut self) -> LOWM_W {
        LOWM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN_TSTS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_tsts](index.html) module"]
pub struct CAN_TSTS_SPEC;
impl crate::RegisterSpec for CAN_TSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_tsts::R](R) reader structure"]
impl crate::Readable for CAN_TSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_tsts::W](W) writer structure"]
impl crate::Writable for CAN_TSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_TSTS to value 0x1c00_0000"]
impl crate::Resettable for CAN_TSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1c00_0000
    }
}
