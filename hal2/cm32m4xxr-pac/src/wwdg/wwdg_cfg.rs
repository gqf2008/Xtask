#[doc = "Register `WWDG_CFG` reader"]
pub struct R(crate::R<WWDG_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WWDG_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WWDG_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WWDG_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WWDG_CFG` writer"]
pub struct W(crate::W<WWDG_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WWDG_CFG_SPEC>;
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
impl From<crate::W<WWDG_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WWDG_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `W` reader - W"]
pub type W_R = crate::FieldReader<u8, u8>;
#[doc = "Field `W` writer - W"]
pub type W_W<'a> = crate::FieldWriter<'a, u32, WWDG_CFG_SPEC, u8, u8, 7, 0>;
#[doc = "Field `TIMERB` reader - TIMERB"]
pub type TIMERB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMERB` writer - TIMERB"]
pub type TIMERB_W<'a> = crate::FieldWriter<'a, u32, WWDG_CFG_SPEC, u8, u8, 2, 7>;
#[doc = "Field `EWINT` reader - EWINT"]
pub type EWINT_R = crate::BitReader<bool>;
#[doc = "Field `EWINT` writer - EWINT"]
pub type EWINT_W<'a> = crate::BitWriter<'a, u32, WWDG_CFG_SPEC, bool, 9>;
impl R {
    #[doc = "Bits 0:6 - W"]
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - TIMERB"]
    #[inline(always)]
    pub fn timerb(&self) -> TIMERB_R {
        TIMERB_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - EWINT"]
    #[inline(always)]
    pub fn ewint(&self) -> EWINT_R {
        EWINT_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - W"]
    #[inline(always)]
    pub fn w(&mut self) -> W_W {
        W_W::new(self)
    }
    #[doc = "Bits 7:8 - TIMERB"]
    #[inline(always)]
    pub fn timerb(&mut self) -> TIMERB_W {
        TIMERB_W::new(self)
    }
    #[doc = "Bit 9 - EWINT"]
    #[inline(always)]
    pub fn ewint(&mut self) -> EWINT_W {
        EWINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WWDG_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wwdg_cfg](index.html) module"]
pub struct WWDG_CFG_SPEC;
impl crate::RegisterSpec for WWDG_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wwdg_cfg::R](R) reader structure"]
impl crate::Readable for WWDG_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wwdg_cfg::W](W) writer structure"]
impl crate::Writable for WWDG_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WWDG_CFG to value 0x7f"]
impl crate::Resettable for WWDG_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7f
    }
}
