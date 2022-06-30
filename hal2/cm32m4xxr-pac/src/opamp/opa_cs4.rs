#[doc = "Register `OPA_CS4` reader"]
pub struct R(crate::R<OPA_CS4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPA_CS4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPA_CS4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPA_CS4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPA_CS4` writer"]
pub struct W(crate::W<OPA_CS4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPA_CS4_SPEC>;
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
impl From<crate::W<OPA_CS4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPA_CS4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - EN"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - EN"]
pub type EN_W<'a> = crate::BitWriter<'a, u32, OPA_CS4_SPEC, bool, 0>;
#[doc = "Field `MOD` reader - MOD"]
pub type MOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MOD` writer - MOD"]
pub type MOD_W<'a> = crate::FieldWriter<'a, u32, OPA_CS4_SPEC, u8, u8, 2, 1>;
#[doc = "Field `PGAGAN` reader - PGAGAN"]
pub type PGAGAN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PGAGAN` writer - PGAGAN"]
pub type PGAGAN_W<'a> = crate::FieldWriter<'a, u32, OPA_CS4_SPEC, u8, u8, 3, 3>;
#[doc = "Field `VMSEL` reader - VMSEL"]
pub type VMSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VMSEL` writer - VMSEL"]
pub type VMSEL_W<'a> = crate::FieldWriter<'a, u32, OPA_CS4_SPEC, u8, u8, 2, 6>;
#[doc = "Field `VPSEL` reader - VPSEL"]
pub type VPSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VPSEL` writer - VPSEL"]
pub type VPSEL_W<'a> = crate::FieldWriter<'a, u32, OPA_CS4_SPEC, u8, u8, 3, 8>;
#[doc = "Field `CALON` reader - CALON"]
pub type CALON_R = crate::BitReader<bool>;
#[doc = "Field `CALON` writer - CALON"]
pub type CALON_W<'a> = crate::BitWriter<'a, u32, OPA_CS4_SPEC, bool, 11>;
#[doc = "Field `TSTREF` reader - TSTREF"]
pub type TSTREF_R = crate::BitReader<bool>;
#[doc = "Field `TSTREF` writer - TSTREF"]
pub type TSTREF_W<'a> = crate::BitWriter<'a, u32, OPA_CS4_SPEC, bool, 13>;
#[doc = "Field `CALOUT` reader - CALOUT"]
pub type CALOUT_R = crate::BitReader<bool>;
#[doc = "Field `CALOUT` writer - CALOUT"]
pub type CALOUT_W<'a> = crate::BitWriter<'a, u32, OPA_CS4_SPEC, bool, 14>;
#[doc = "Field `RANGE` reader - RANGE"]
pub type RANGE_R = crate::BitReader<bool>;
#[doc = "Field `RANGE` writer - RANGE"]
pub type RANGE_W<'a> = crate::BitWriter<'a, u32, OPA_CS4_SPEC, bool, 15>;
#[doc = "Field `TCMEN` reader - TCMEN"]
pub type TCMEN_R = crate::BitReader<bool>;
#[doc = "Field `TCMEN` writer - TCMEN"]
pub type TCMEN_W<'a> = crate::BitWriter<'a, u32, OPA_CS4_SPEC, bool, 16>;
#[doc = "Field `VMSSEL` reader - VMSSEL"]
pub type VMSSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VMSSEL` writer - VMSSEL"]
pub type VMSSEL_W<'a> = crate::FieldWriter<'a, u32, OPA_CS4_SPEC, u8, u8, 2, 17>;
#[doc = "Field `VPSSEL` reader - VPSSEL"]
pub type VPSSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VPSSEL` writer - VPSSEL"]
pub type VPSSEL_W<'a> = crate::FieldWriter<'a, u32, OPA_CS4_SPEC, u8, u8, 3, 19>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - MOD"]
    #[inline(always)]
    pub fn mod_(&self) -> MOD_R {
        MOD_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:5 - PGAGAN"]
    #[inline(always)]
    pub fn pgagan(&self) -> PGAGAN_R {
        PGAGAN_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - VMSEL"]
    #[inline(always)]
    pub fn vmsel(&self) -> VMSEL_R {
        VMSEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - VPSEL"]
    #[inline(always)]
    pub fn vpsel(&self) -> VPSEL_R {
        VPSEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - CALON"]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - TSTREF"]
    #[inline(always)]
    pub fn tstref(&self) -> TSTREF_R {
        TSTREF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CALOUT"]
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RANGE"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TCMEN"]
    #[inline(always)]
    pub fn tcmen(&self) -> TCMEN_R {
        TCMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - VMSSEL"]
    #[inline(always)]
    pub fn vmssel(&self) -> VMSSEL_R {
        VMSSEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:21 - VPSSEL"]
    #[inline(always)]
    pub fn vpssel(&self) -> VPSSEL_R {
        VPSSEL_R::new(((self.bits >> 19) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W::new(self)
    }
    #[doc = "Bits 1:2 - MOD"]
    #[inline(always)]
    pub fn mod_(&mut self) -> MOD_W {
        MOD_W::new(self)
    }
    #[doc = "Bits 3:5 - PGAGAN"]
    #[inline(always)]
    pub fn pgagan(&mut self) -> PGAGAN_W {
        PGAGAN_W::new(self)
    }
    #[doc = "Bits 6:7 - VMSEL"]
    #[inline(always)]
    pub fn vmsel(&mut self) -> VMSEL_W {
        VMSEL_W::new(self)
    }
    #[doc = "Bits 8:10 - VPSEL"]
    #[inline(always)]
    pub fn vpsel(&mut self) -> VPSEL_W {
        VPSEL_W::new(self)
    }
    #[doc = "Bit 11 - CALON"]
    #[inline(always)]
    pub fn calon(&mut self) -> CALON_W {
        CALON_W::new(self)
    }
    #[doc = "Bit 13 - TSTREF"]
    #[inline(always)]
    pub fn tstref(&mut self) -> TSTREF_W {
        TSTREF_W::new(self)
    }
    #[doc = "Bit 14 - CALOUT"]
    #[inline(always)]
    pub fn calout(&mut self) -> CALOUT_W {
        CALOUT_W::new(self)
    }
    #[doc = "Bit 15 - RANGE"]
    #[inline(always)]
    pub fn range(&mut self) -> RANGE_W {
        RANGE_W::new(self)
    }
    #[doc = "Bit 16 - TCMEN"]
    #[inline(always)]
    pub fn tcmen(&mut self) -> TCMEN_W {
        TCMEN_W::new(self)
    }
    #[doc = "Bits 17:18 - VMSSEL"]
    #[inline(always)]
    pub fn vmssel(&mut self) -> VMSSEL_W {
        VMSSEL_W::new(self)
    }
    #[doc = "Bits 19:21 - VPSSEL"]
    #[inline(always)]
    pub fn vpssel(&mut self) -> VPSSEL_W {
        VPSSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OPA_CS4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa_cs4](index.html) module"]
pub struct OPA_CS4_SPEC;
impl crate::RegisterSpec for OPA_CS4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opa_cs4::R](R) reader structure"]
impl crate::Readable for OPA_CS4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opa_cs4::W](W) writer structure"]
impl crate::Writable for OPA_CS4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OPA_CS4 to value 0"]
impl crate::Resettable for OPA_CS4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
