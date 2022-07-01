#[doc = "Register `OPA_LOCK` reader"]
pub struct R(crate::R<OPA_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPA_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPA_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPA_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPA_LOCK` writer"]
pub struct W(crate::W<OPA_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPA_LOCK_SPEC>;
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
impl From<crate::W<OPA_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPA_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPAMP1LK` reader - OPAMP1LK"]
pub type OPAMP1LK_R = crate::BitReader<bool>;
#[doc = "Field `OPAMP1LK` writer - OPAMP1LK"]
pub type OPAMP1LK_W<'a> = crate::BitWriter<'a, u32, OPA_LOCK_SPEC, bool, 0>;
#[doc = "Field `OPAMP2LK` reader - OPAMP2LK"]
pub type OPAMP2LK_R = crate::BitReader<bool>;
#[doc = "Field `OPAMP2LK` writer - OPAMP2LK"]
pub type OPAMP2LK_W<'a> = crate::BitWriter<'a, u32, OPA_LOCK_SPEC, bool, 1>;
#[doc = "Field `OPAMP3LK` reader - OPAMP3LK"]
pub type OPAMP3LK_R = crate::BitReader<bool>;
#[doc = "Field `OPAMP3LK` writer - OPAMP3LK"]
pub type OPAMP3LK_W<'a> = crate::BitWriter<'a, u32, OPA_LOCK_SPEC, bool, 2>;
#[doc = "Field `OPAMP4LK` reader - OPAMP4LK"]
pub type OPAMP4LK_R = crate::BitReader<bool>;
#[doc = "Field `OPAMP4LK` writer - OPAMP4LK"]
pub type OPAMP4LK_W<'a> = crate::BitWriter<'a, u32, OPA_LOCK_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - OPAMP1LK"]
    #[inline(always)]
    pub fn opamp1lk(&self) -> OPAMP1LK_R {
        OPAMP1LK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OPAMP2LK"]
    #[inline(always)]
    pub fn opamp2lk(&self) -> OPAMP2LK_R {
        OPAMP2LK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OPAMP3LK"]
    #[inline(always)]
    pub fn opamp3lk(&self) -> OPAMP3LK_R {
        OPAMP3LK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OPAMP4LK"]
    #[inline(always)]
    pub fn opamp4lk(&self) -> OPAMP4LK_R {
        OPAMP4LK_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OPAMP1LK"]
    #[inline(always)]
    pub fn opamp1lk(&mut self) -> OPAMP1LK_W {
        OPAMP1LK_W::new(self)
    }
    #[doc = "Bit 1 - OPAMP2LK"]
    #[inline(always)]
    pub fn opamp2lk(&mut self) -> OPAMP2LK_W {
        OPAMP2LK_W::new(self)
    }
    #[doc = "Bit 2 - OPAMP3LK"]
    #[inline(always)]
    pub fn opamp3lk(&mut self) -> OPAMP3LK_W {
        OPAMP3LK_W::new(self)
    }
    #[doc = "Bit 3 - OPAMP4LK"]
    #[inline(always)]
    pub fn opamp4lk(&mut self) -> OPAMP4LK_W {
        OPAMP4LK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPA_LOCK\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa_lock](index.html) module"]
pub struct OPA_LOCK_SPEC;
impl crate::RegisterSpec for OPA_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opa_lock::R](R) reader structure"]
impl crate::Readable for OPA_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opa_lock::W](W) writer structure"]
impl crate::Writable for OPA_LOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPA_LOCK to value 0"]
impl crate::Resettable for OPA_LOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
