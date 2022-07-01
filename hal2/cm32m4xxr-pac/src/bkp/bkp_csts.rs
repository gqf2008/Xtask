#[doc = "Register `BKP_CSTS` reader"]
pub struct R(crate::R<BKP_CSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BKP_CSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BKP_CSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BKP_CSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BKP_CSTS` writer"]
pub struct W(crate::W<BKP_CSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BKP_CSTS_SPEC>;
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
impl From<crate::W<BKP_CSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BKP_CSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRTE` reader - CLRTE"]
pub type CLRTE_R = crate::BitReader<bool>;
#[doc = "Field `CLRTE` writer - CLRTE"]
pub type CLRTE_W<'a> = crate::BitWriter<'a, u32, BKP_CSTS_SPEC, bool, 0>;
#[doc = "Field `CLRTINT` reader - CLRTINT"]
pub type CLRTINT_R = crate::BitReader<bool>;
#[doc = "Field `CLRTINT` writer - CLRTINT"]
pub type CLRTINT_W<'a> = crate::BitWriter<'a, u32, BKP_CSTS_SPEC, bool, 1>;
#[doc = "Field `TPINT_EN` reader - TPINT_EN"]
pub type TPINT_EN_R = crate::BitReader<bool>;
#[doc = "Field `TPINT_EN` writer - TPINT_EN"]
pub type TPINT_EN_W<'a> = crate::BitWriter<'a, u32, BKP_CSTS_SPEC, bool, 2>;
#[doc = "Field `TEF` reader - TEF"]
pub type TEF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEF` writer - TEF"]
pub type TEF_W<'a> = crate::FieldWriter<'a, u32, BKP_CSTS_SPEC, u8, u8, 6, 3>;
#[doc = "Field `TINTF` reader - TINTF"]
pub type TINTF_R = crate::BitReader<bool>;
#[doc = "Field `TINTF` writer - TINTF"]
pub type TINTF_W<'a> = crate::BitWriter<'a, u32, BKP_CSTS_SPEC, bool, 9>;
impl R {
    #[doc = "Bit 0 - CLRTE"]
    #[inline(always)]
    pub fn clrte(&self) -> CLRTE_R {
        CLRTE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CLRTINT"]
    #[inline(always)]
    pub fn clrtint(&self) -> CLRTINT_R {
        CLRTINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TPINT_EN"]
    #[inline(always)]
    pub fn tpint_en(&self) -> TPINT_EN_R {
        TPINT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:8 - TEF"]
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new(((self.bits >> 3) & 0x3f) as u8)
    }
    #[doc = "Bit 9 - TINTF"]
    #[inline(always)]
    pub fn tintf(&self) -> TINTF_R {
        TINTF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CLRTE"]
    #[inline(always)]
    pub fn clrte(&mut self) -> CLRTE_W {
        CLRTE_W::new(self)
    }
    #[doc = "Bit 1 - CLRTINT"]
    #[inline(always)]
    pub fn clrtint(&mut self) -> CLRTINT_W {
        CLRTINT_W::new(self)
    }
    #[doc = "Bit 2 - TPINT_EN"]
    #[inline(always)]
    pub fn tpint_en(&mut self) -> TPINT_EN_W {
        TPINT_EN_W::new(self)
    }
    #[doc = "Bits 3:8 - TEF"]
    #[inline(always)]
    pub fn tef(&mut self) -> TEF_W {
        TEF_W::new(self)
    }
    #[doc = "Bit 9 - TINTF"]
    #[inline(always)]
    pub fn tintf(&mut self) -> TINTF_W {
        TINTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BKP_CSTS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp_csts](index.html) module"]
pub struct BKP_CSTS_SPEC;
impl crate::RegisterSpec for BKP_CSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bkp_csts::R](R) reader structure"]
impl crate::Readable for BKP_CSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bkp_csts::W](W) writer structure"]
impl crate::Writable for BKP_CSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BKP_CSTS to value 0"]
impl crate::Resettable for BKP_CSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
