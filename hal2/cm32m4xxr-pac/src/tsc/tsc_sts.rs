#[doc = "Register `TSC_STS` reader"]
pub struct R(crate::R<TSC_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSC_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSC_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSC_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSC_STS` writer"]
pub struct W(crate::W<TSC_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSC_STS_SPEC>;
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
impl From<crate::W<TSC_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSC_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT_VAL` reader - CNT_VAL"]
pub type CNT_VAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CNT_VAL` writer - CNT_VAL"]
pub type CNT_VAL_W<'a> = crate::FieldWriter<'a, u32, TSC_STS_SPEC, u16, u16, 11, 0>;
#[doc = "Field `LESS_DET` reader - LESS_DET"]
pub type LESS_DET_R = crate::BitReader<bool>;
#[doc = "Field `LESS_DET` writer - LESS_DET"]
pub type LESS_DET_W<'a> = crate::BitWriter<'a, u32, TSC_STS_SPEC, bool, 12>;
#[doc = "Field `GREAT_DET` reader - GREAT_DET"]
pub type GREAT_DET_R = crate::BitReader<bool>;
#[doc = "Field `GREAT_DET` writer - GREAT_DET"]
pub type GREAT_DET_W<'a> = crate::BitWriter<'a, u32, TSC_STS_SPEC, bool, 13>;
#[doc = "Field `CHN_NUM` reader - CHN_NUM"]
pub type CHN_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_NUM` writer - CHN_NUM"]
pub type CHN_NUM_W<'a> = crate::FieldWriter<'a, u32, TSC_STS_SPEC, u8, u8, 5, 16>;
impl R {
    #[doc = "Bits 0:10 - CNT_VAL"]
    #[inline(always)]
    pub fn cnt_val(&self) -> CNT_VAL_R {
        CNT_VAL_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 12 - LESS_DET"]
    #[inline(always)]
    pub fn less_det(&self) -> LESS_DET_R {
        LESS_DET_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GREAT_DET"]
    #[inline(always)]
    pub fn great_det(&self) -> GREAT_DET_R {
        GREAT_DET_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:20 - CHN_NUM"]
    #[inline(always)]
    pub fn chn_num(&self) -> CHN_NUM_R {
        CHN_NUM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - CNT_VAL"]
    #[inline(always)]
    pub fn cnt_val(&mut self) -> CNT_VAL_W {
        CNT_VAL_W::new(self)
    }
    #[doc = "Bit 12 - LESS_DET"]
    #[inline(always)]
    pub fn less_det(&mut self) -> LESS_DET_W {
        LESS_DET_W::new(self)
    }
    #[doc = "Bit 13 - GREAT_DET"]
    #[inline(always)]
    pub fn great_det(&mut self) -> GREAT_DET_W {
        GREAT_DET_W::new(self)
    }
    #[doc = "Bits 16:20 - CHN_NUM"]
    #[inline(always)]
    pub fn chn_num(&mut self) -> CHN_NUM_W {
        CHN_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSC_STS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsc_sts](index.html) module"]
pub struct TSC_STS_SPEC;
impl crate::RegisterSpec for TSC_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsc_sts::R](R) reader structure"]
impl crate::Readable for TSC_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsc_sts::W](W) writer structure"]
impl crate::Writable for TSC_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSC_STS to value 0"]
impl crate::Resettable for TSC_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
