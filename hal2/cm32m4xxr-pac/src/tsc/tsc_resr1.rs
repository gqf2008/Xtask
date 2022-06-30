#[doc = "Register `TSC_RESR1` reader"]
pub struct R(crate::R<TSC_RESR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSC_RESR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSC_RESR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSC_RESR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSC_RESR1` writer"]
pub struct W(crate::W<TSC_RESR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSC_RESR1_SPEC>;
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
impl From<crate::W<TSC_RESR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSC_RESR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHN_RESIST8` reader - CHN_RESIST8"]
pub type CHN_RESIST8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_RESIST8` writer - CHN_RESIST8"]
pub type CHN_RESIST8_W<'a> = crate::FieldWriter<'a, u32, TSC_RESR1_SPEC, u8, u8, 3, 0>;
#[doc = "Field `CHN_RESIST9` reader - CHN_RESIST9"]
pub type CHN_RESIST9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_RESIST9` writer - CHN_RESIST9"]
pub type CHN_RESIST9_W<'a> = crate::FieldWriter<'a, u32, TSC_RESR1_SPEC, u8, u8, 3, 4>;
#[doc = "Field `CHN_RESIST10` reader - CHN_RESIST10"]
pub type CHN_RESIST10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_RESIST10` writer - CHN_RESIST10"]
pub type CHN_RESIST10_W<'a> = crate::FieldWriter<'a, u32, TSC_RESR1_SPEC, u8, u8, 3, 8>;
#[doc = "Field `CHN_RESIST11` reader - CHN_RESIST11"]
pub type CHN_RESIST11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_RESIST11` writer - CHN_RESIST11"]
pub type CHN_RESIST11_W<'a> = crate::FieldWriter<'a, u32, TSC_RESR1_SPEC, u8, u8, 3, 12>;
#[doc = "Field `CHN_RESIST12` reader - CHN_RESIST12"]
pub type CHN_RESIST12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_RESIST12` writer - CHN_RESIST12"]
pub type CHN_RESIST12_W<'a> = crate::FieldWriter<'a, u32, TSC_RESR1_SPEC, u8, u8, 3, 16>;
#[doc = "Field `CHN_RESIST13` reader - CHN_RESIST13"]
pub type CHN_RESIST13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_RESIST13` writer - CHN_RESIST13"]
pub type CHN_RESIST13_W<'a> = crate::FieldWriter<'a, u32, TSC_RESR1_SPEC, u8, u8, 3, 20>;
#[doc = "Field `CHN_RESIST14` reader - CHN_RESIST14"]
pub type CHN_RESIST14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_RESIST14` writer - CHN_RESIST14"]
pub type CHN_RESIST14_W<'a> = crate::FieldWriter<'a, u32, TSC_RESR1_SPEC, u8, u8, 3, 24>;
#[doc = "Field `CHN_RESIST15` reader - CHN_RESIST15"]
pub type CHN_RESIST15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHN_RESIST15` writer - CHN_RESIST15"]
pub type CHN_RESIST15_W<'a> = crate::FieldWriter<'a, u32, TSC_RESR1_SPEC, u8, u8, 3, 28>;
impl R {
    #[doc = "Bits 0:2 - CHN_RESIST8"]
    #[inline(always)]
    pub fn chn_resist8(&self) -> CHN_RESIST8_R {
        CHN_RESIST8_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - CHN_RESIST9"]
    #[inline(always)]
    pub fn chn_resist9(&self) -> CHN_RESIST9_R {
        CHN_RESIST9_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - CHN_RESIST10"]
    #[inline(always)]
    pub fn chn_resist10(&self) -> CHN_RESIST10_R {
        CHN_RESIST10_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - CHN_RESIST11"]
    #[inline(always)]
    pub fn chn_resist11(&self) -> CHN_RESIST11_R {
        CHN_RESIST11_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - CHN_RESIST12"]
    #[inline(always)]
    pub fn chn_resist12(&self) -> CHN_RESIST12_R {
        CHN_RESIST12_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - CHN_RESIST13"]
    #[inline(always)]
    pub fn chn_resist13(&self) -> CHN_RESIST13_R {
        CHN_RESIST13_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - CHN_RESIST14"]
    #[inline(always)]
    pub fn chn_resist14(&self) -> CHN_RESIST14_R {
        CHN_RESIST14_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - CHN_RESIST15"]
    #[inline(always)]
    pub fn chn_resist15(&self) -> CHN_RESIST15_R {
        CHN_RESIST15_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CHN_RESIST8"]
    #[inline(always)]
    pub fn chn_resist8(&mut self) -> CHN_RESIST8_W {
        CHN_RESIST8_W::new(self)
    }
    #[doc = "Bits 4:6 - CHN_RESIST9"]
    #[inline(always)]
    pub fn chn_resist9(&mut self) -> CHN_RESIST9_W {
        CHN_RESIST9_W::new(self)
    }
    #[doc = "Bits 8:10 - CHN_RESIST10"]
    #[inline(always)]
    pub fn chn_resist10(&mut self) -> CHN_RESIST10_W {
        CHN_RESIST10_W::new(self)
    }
    #[doc = "Bits 12:14 - CHN_RESIST11"]
    #[inline(always)]
    pub fn chn_resist11(&mut self) -> CHN_RESIST11_W {
        CHN_RESIST11_W::new(self)
    }
    #[doc = "Bits 16:18 - CHN_RESIST12"]
    #[inline(always)]
    pub fn chn_resist12(&mut self) -> CHN_RESIST12_W {
        CHN_RESIST12_W::new(self)
    }
    #[doc = "Bits 20:22 - CHN_RESIST13"]
    #[inline(always)]
    pub fn chn_resist13(&mut self) -> CHN_RESIST13_W {
        CHN_RESIST13_W::new(self)
    }
    #[doc = "Bits 24:26 - CHN_RESIST14"]
    #[inline(always)]
    pub fn chn_resist14(&mut self) -> CHN_RESIST14_W {
        CHN_RESIST14_W::new(self)
    }
    #[doc = "Bits 28:30 - CHN_RESIST15"]
    #[inline(always)]
    pub fn chn_resist15(&mut self) -> CHN_RESIST15_W {
        CHN_RESIST15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSC_RESR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsc_resr1](index.html) module"]
pub struct TSC_RESR1_SPEC;
impl crate::RegisterSpec for TSC_RESR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsc_resr1::R](R) reader structure"]
impl crate::Readable for TSC_RESR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsc_resr1::W](W) writer structure"]
impl crate::Writable for TSC_RESR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSC_RESR1 to value 0x4444_4444"]
impl crate::Resettable for TSC_RESR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4444_4444
    }
}
