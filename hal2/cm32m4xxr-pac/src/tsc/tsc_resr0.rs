#[doc = "Register `TSC_RESR0` reader"]
pub struct R(crate::R<TSC_RESR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSC_RESR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSC_RESR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSC_RESR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSC_RESR0` writer"]
pub struct W(crate::W<TSC_RESR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSC_RESR0_SPEC>;
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
impl From<crate::W<TSC_RESR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSC_RESR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHN_RESIST0` reader - CHN_RESIST0"]
pub type CHN_RESIST0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_RESIST0` writer - CHN_RESIST0"]
pub type CHN_RESIST0_W<'a> = crate::FieldWriter<'a, u32, TSC_RESR0_SPEC, u8, u8, 3, 0>;
#[doc = "Field `CHN_RESIST1` reader - CHN_RESIST1"]
pub type CHN_RESIST1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_RESIST1` writer - CHN_RESIST1"]
pub type CHN_RESIST1_W<'a> = crate::FieldWriter<'a, u32, TSC_RESR0_SPEC, u8, u8, 3, 4>;
#[doc = "Field `CHN_RESIST2` reader - CHN_RESIST2"]
pub type CHN_RESIST2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_RESIST2` writer - CHN_RESIST2"]
pub type CHN_RESIST2_W<'a> = crate::FieldWriter<'a, u32, TSC_RESR0_SPEC, u8, u8, 3, 8>;
#[doc = "Field `CHN_RESIST3` reader - CHN_RESIST3"]
pub type CHN_RESIST3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_RESIST3` writer - CHN_RESIST3"]
pub type CHN_RESIST3_W<'a> = crate::FieldWriter<'a, u32, TSC_RESR0_SPEC, u8, u8, 3, 12>;
#[doc = "Field `CHN_RESIST4` reader - CHN_RESIST4"]
pub type CHN_RESIST4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_RESIST4` writer - CHN_RESIST4"]
pub type CHN_RESIST4_W<'a> = crate::FieldWriter<'a, u32, TSC_RESR0_SPEC, u8, u8, 3, 16>;
#[doc = "Field `CHN_RESIST5` reader - CHN_RESIST5"]
pub type CHN_RESIST5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_RESIST5` writer - CHN_RESIST5"]
pub type CHN_RESIST5_W<'a> = crate::FieldWriter<'a, u32, TSC_RESR0_SPEC, u8, u8, 3, 20>;
#[doc = "Field `CHN_RESIST6` reader - CHN_RESIST6"]
pub type CHN_RESIST6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_RESIST6` writer - CHN_RESIST6"]
pub type CHN_RESIST6_W<'a> = crate::FieldWriter<'a, u32, TSC_RESR0_SPEC, u8, u8, 3, 24>;
#[doc = "Field `CHN_RESIST7` reader - CHN_RESIST7"]
pub type CHN_RESIST7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_RESIST7` writer - CHN_RESIST7"]
pub type CHN_RESIST7_W<'a> = crate::FieldWriter<'a, u32, TSC_RESR0_SPEC, u8, u8, 3, 28>;
impl R {
    #[doc = "Bits 0:2 - CHN_RESIST0"]
    #[inline(always)]
    pub fn chn_resist0(&self) -> CHN_RESIST0_R {
        CHN_RESIST0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - CHN_RESIST1"]
    #[inline(always)]
    pub fn chn_resist1(&self) -> CHN_RESIST1_R {
        CHN_RESIST1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - CHN_RESIST2"]
    #[inline(always)]
    pub fn chn_resist2(&self) -> CHN_RESIST2_R {
        CHN_RESIST2_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - CHN_RESIST3"]
    #[inline(always)]
    pub fn chn_resist3(&self) -> CHN_RESIST3_R {
        CHN_RESIST3_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - CHN_RESIST4"]
    #[inline(always)]
    pub fn chn_resist4(&self) -> CHN_RESIST4_R {
        CHN_RESIST4_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - CHN_RESIST5"]
    #[inline(always)]
    pub fn chn_resist5(&self) -> CHN_RESIST5_R {
        CHN_RESIST5_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - CHN_RESIST6"]
    #[inline(always)]
    pub fn chn_resist6(&self) -> CHN_RESIST6_R {
        CHN_RESIST6_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - CHN_RESIST7"]
    #[inline(always)]
    pub fn chn_resist7(&self) -> CHN_RESIST7_R {
        CHN_RESIST7_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CHN_RESIST0"]
    #[inline(always)]
    pub fn chn_resist0(&mut self) -> CHN_RESIST0_W {
        CHN_RESIST0_W::new(self)
    }
    #[doc = "Bits 4:6 - CHN_RESIST1"]
    #[inline(always)]
    pub fn chn_resist1(&mut self) -> CHN_RESIST1_W {
        CHN_RESIST1_W::new(self)
    }
    #[doc = "Bits 8:10 - CHN_RESIST2"]
    #[inline(always)]
    pub fn chn_resist2(&mut self) -> CHN_RESIST2_W {
        CHN_RESIST2_W::new(self)
    }
    #[doc = "Bits 12:14 - CHN_RESIST3"]
    #[inline(always)]
    pub fn chn_resist3(&mut self) -> CHN_RESIST3_W {
        CHN_RESIST3_W::new(self)
    }
    #[doc = "Bits 16:18 - CHN_RESIST4"]
    #[inline(always)]
    pub fn chn_resist4(&mut self) -> CHN_RESIST4_W {
        CHN_RESIST4_W::new(self)
    }
    #[doc = "Bits 20:22 - CHN_RESIST5"]
    #[inline(always)]
    pub fn chn_resist5(&mut self) -> CHN_RESIST5_W {
        CHN_RESIST5_W::new(self)
    }
    #[doc = "Bits 24:26 - CHN_RESIST6"]
    #[inline(always)]
    pub fn chn_resist6(&mut self) -> CHN_RESIST6_W {
        CHN_RESIST6_W::new(self)
    }
    #[doc = "Bits 28:30 - CHN_RESIST7"]
    #[inline(always)]
    pub fn chn_resist7(&mut self) -> CHN_RESIST7_W {
        CHN_RESIST7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSC_RESR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsc_resr0](index.html) module"]
pub struct TSC_RESR0_SPEC;
impl crate::RegisterSpec for TSC_RESR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsc_resr0::R](R) reader structure"]
impl crate::Readable for TSC_RESR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsc_resr0::W](W) writer structure"]
impl crate::Writable for TSC_RESR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSC_RESR0 to value 0x4444_4444"]
impl crate::Resettable for TSC_RESR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4444_4444
    }
}
