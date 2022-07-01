#[doc = "Register `TSC_RESR2` reader"]
pub struct R(crate::R<TSC_RESR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSC_RESR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSC_RESR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSC_RESR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSC_RESR2` writer"]
pub struct W(crate::W<TSC_RESR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSC_RESR2_SPEC>;
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
impl From<crate::W<TSC_RESR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSC_RESR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHN_RESIST16` reader - CHN_RESIST16"]
pub type CHN_RESIST16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_RESIST16` writer - CHN_RESIST16"]
pub type CHN_RESIST16_W<'a> = crate::FieldWriter<'a, u32, TSC_RESR2_SPEC, u8, u8, 3, 0>;
#[doc = "Field `CHN_RESIST17` reader - CHN_RESIST17"]
pub type CHN_RESIST17_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_RESIST17` writer - CHN_RESIST17"]
pub type CHN_RESIST17_W<'a> = crate::FieldWriter<'a, u32, TSC_RESR2_SPEC, u8, u8, 3, 4>;
#[doc = "Field `CHN_RESIST18` reader - CHN_RESIST18"]
pub type CHN_RESIST18_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_RESIST18` writer - CHN_RESIST18"]
pub type CHN_RESIST18_W<'a> = crate::FieldWriter<'a, u32, TSC_RESR2_SPEC, u8, u8, 3, 8>;
#[doc = "Field `CHN_RESIST19` reader - CHN_RESIST19"]
pub type CHN_RESIST19_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_RESIST19` writer - CHN_RESIST19"]
pub type CHN_RESIST19_W<'a> = crate::FieldWriter<'a, u32, TSC_RESR2_SPEC, u8, u8, 3, 12>;
#[doc = "Field `CHN_RESIST20` reader - CHN_RESIST20"]
pub type CHN_RESIST20_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_RESIST20` writer - CHN_RESIST20"]
pub type CHN_RESIST20_W<'a> = crate::FieldWriter<'a, u32, TSC_RESR2_SPEC, u8, u8, 3, 16>;
#[doc = "Field `CHN_RESIST21` reader - CHN_RESIST21"]
pub type CHN_RESIST21_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_RESIST21` writer - CHN_RESIST21"]
pub type CHN_RESIST21_W<'a> = crate::FieldWriter<'a, u32, TSC_RESR2_SPEC, u8, u8, 3, 20>;
#[doc = "Field `CHN_RESIST22` reader - CHN_RESIST22"]
pub type CHN_RESIST22_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_RESIST22` writer - CHN_RESIST22"]
pub type CHN_RESIST22_W<'a> = crate::FieldWriter<'a, u32, TSC_RESR2_SPEC, u8, u8, 3, 24>;
#[doc = "Field `CHN_RESIST23` reader - CHN_RESIST23"]
pub type CHN_RESIST23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_RESIST23` writer - CHN_RESIST23"]
pub type CHN_RESIST23_W<'a> = crate::FieldWriter<'a, u32, TSC_RESR2_SPEC, u8, u8, 3, 28>;
impl R {
    #[doc = "Bits 0:2 - CHN_RESIST16"]
    #[inline(always)]
    pub fn chn_resist16(&self) -> CHN_RESIST16_R {
        CHN_RESIST16_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - CHN_RESIST17"]
    #[inline(always)]
    pub fn chn_resist17(&self) -> CHN_RESIST17_R {
        CHN_RESIST17_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - CHN_RESIST18"]
    #[inline(always)]
    pub fn chn_resist18(&self) -> CHN_RESIST18_R {
        CHN_RESIST18_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - CHN_RESIST19"]
    #[inline(always)]
    pub fn chn_resist19(&self) -> CHN_RESIST19_R {
        CHN_RESIST19_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - CHN_RESIST20"]
    #[inline(always)]
    pub fn chn_resist20(&self) -> CHN_RESIST20_R {
        CHN_RESIST20_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - CHN_RESIST21"]
    #[inline(always)]
    pub fn chn_resist21(&self) -> CHN_RESIST21_R {
        CHN_RESIST21_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - CHN_RESIST22"]
    #[inline(always)]
    pub fn chn_resist22(&self) -> CHN_RESIST22_R {
        CHN_RESIST22_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - CHN_RESIST23"]
    #[inline(always)]
    pub fn chn_resist23(&self) -> CHN_RESIST23_R {
        CHN_RESIST23_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CHN_RESIST16"]
    #[inline(always)]
    pub fn chn_resist16(&mut self) -> CHN_RESIST16_W {
        CHN_RESIST16_W::new(self)
    }
    #[doc = "Bits 4:6 - CHN_RESIST17"]
    #[inline(always)]
    pub fn chn_resist17(&mut self) -> CHN_RESIST17_W {
        CHN_RESIST17_W::new(self)
    }
    #[doc = "Bits 8:10 - CHN_RESIST18"]
    #[inline(always)]
    pub fn chn_resist18(&mut self) -> CHN_RESIST18_W {
        CHN_RESIST18_W::new(self)
    }
    #[doc = "Bits 12:14 - CHN_RESIST19"]
    #[inline(always)]
    pub fn chn_resist19(&mut self) -> CHN_RESIST19_W {
        CHN_RESIST19_W::new(self)
    }
    #[doc = "Bits 16:18 - CHN_RESIST20"]
    #[inline(always)]
    pub fn chn_resist20(&mut self) -> CHN_RESIST20_W {
        CHN_RESIST20_W::new(self)
    }
    #[doc = "Bits 20:22 - CHN_RESIST21"]
    #[inline(always)]
    pub fn chn_resist21(&mut self) -> CHN_RESIST21_W {
        CHN_RESIST21_W::new(self)
    }
    #[doc = "Bits 24:26 - CHN_RESIST22"]
    #[inline(always)]
    pub fn chn_resist22(&mut self) -> CHN_RESIST22_W {
        CHN_RESIST22_W::new(self)
    }
    #[doc = "Bits 28:30 - CHN_RESIST23"]
    #[inline(always)]
    pub fn chn_resist23(&mut self) -> CHN_RESIST23_W {
        CHN_RESIST23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSC_RESR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsc_resr2](index.html) module"]
pub struct TSC_RESR2_SPEC;
impl crate::RegisterSpec for TSC_RESR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsc_resr2::R](R) reader structure"]
impl crate::Readable for TSC_RESR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsc_resr2::W](W) writer structure"]
impl crate::Writable for TSC_RESR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSC_RESR2 to value 0x4444_4444"]
impl crate::Resettable for TSC_RESR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4444_4444
    }
}
