#[doc = "Register `COMP_WINMODE` reader"]
pub struct R(crate::R<COMP_WINMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP_WINMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP_WINMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP_WINMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP_WINMODE` writer"]
pub struct W(crate::W<COMP_WINMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP_WINMODE_SPEC>;
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
impl From<crate::W<COMP_WINMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP_WINMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP12MD` reader - CMP12MD"]
pub type CMP12MD_R = crate::BitReader<bool>;
#[doc = "Field `CMP12MD` writer - CMP12MD"]
pub type CMP12MD_W<'a> = crate::BitWriter<'a, u32, COMP_WINMODE_SPEC, bool, 0>;
#[doc = "Field `CMP34MD` reader - CMP34MD"]
pub type CMP34MD_R = crate::BitReader<bool>;
#[doc = "Field `CMP34MD` writer - CMP34MD"]
pub type CMP34MD_W<'a> = crate::BitWriter<'a, u32, COMP_WINMODE_SPEC, bool, 1>;
#[doc = "Field `CMP56MD` reader - CMP56MD"]
pub type CMP56MD_R = crate::BitReader<bool>;
#[doc = "Field `CMP56MD` writer - CMP56MD"]
pub type CMP56MD_W<'a> = crate::BitWriter<'a, u32, COMP_WINMODE_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 0 - CMP12MD"]
    #[inline(always)]
    pub fn cmp12md(&self) -> CMP12MD_R {
        CMP12MD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMP34MD"]
    #[inline(always)]
    pub fn cmp34md(&self) -> CMP34MD_R {
        CMP34MD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CMP56MD"]
    #[inline(always)]
    pub fn cmp56md(&self) -> CMP56MD_R {
        CMP56MD_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMP12MD"]
    #[inline(always)]
    pub fn cmp12md(&mut self) -> CMP12MD_W {
        CMP12MD_W::new(self)
    }
    #[doc = "Bit 1 - CMP34MD"]
    #[inline(always)]
    pub fn cmp34md(&mut self) -> CMP34MD_W {
        CMP34MD_W::new(self)
    }
    #[doc = "Bit 2 - CMP56MD"]
    #[inline(always)]
    pub fn cmp56md(&mut self) -> CMP56MD_W {
        CMP56MD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "COMP_WINMODE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_winmode](index.html) module"]
pub struct COMP_WINMODE_SPEC;
impl crate::RegisterSpec for COMP_WINMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp_winmode::R](R) reader structure"]
impl crate::Readable for COMP_WINMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp_winmode::W](W) writer structure"]
impl crate::Writable for COMP_WINMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP_WINMODE to value 0"]
impl crate::Resettable for COMP_WINMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
