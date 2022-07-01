#[doc = "Register `WWDG_CTRL` reader"]
pub struct R(crate::R<WWDG_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WWDG_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WWDG_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WWDG_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WWDG_CTRL` writer"]
pub struct W(crate::W<WWDG_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WWDG_CTRL_SPEC>;
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
impl From<crate::W<WWDG_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WWDG_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T` reader - T Timer"]
pub type T_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T` writer - T Timer"]
pub type T_W<'a> = crate::FieldWriter<'a, u32, WWDG_CTRL_SPEC, u8, u8, 7, 0>;
#[doc = "Field `ACTB` reader - ACTB"]
pub type ACTB_R = crate::BitReader<bool>;
#[doc = "Field `ACTB` writer - ACTB"]
pub type ACTB_W<'a> = crate::BitWriter<'a, u32, WWDG_CTRL_SPEC, bool, 7>;
impl R {
    #[doc = "Bits 0:6 - T Timer"]
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - ACTB"]
    #[inline(always)]
    pub fn actb(&self) -> ACTB_R {
        ACTB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - T Timer"]
    #[inline(always)]
    pub fn t(&mut self) -> T_W {
        T_W::new(self)
    }
    #[doc = "Bit 7 - ACTB"]
    #[inline(always)]
    pub fn actb(&mut self) -> ACTB_W {
        ACTB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WWDG_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wwdg_ctrl](index.html) module"]
pub struct WWDG_CTRL_SPEC;
impl crate::RegisterSpec for WWDG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wwdg_ctrl::R](R) reader structure"]
impl crate::Readable for WWDG_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wwdg_ctrl::W](W) writer structure"]
impl crate::Writable for WWDG_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WWDG_CTRL to value 0x7f"]
impl crate::Resettable for WWDG_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7f
    }
}
