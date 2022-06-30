#[doc = "Register `TIMx_CCMOD3` reader"]
pub struct R(crate::R<TIMX_CCMOD3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_CCMOD3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_CCMOD3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_CCMOD3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_CCMOD3` writer"]
pub struct W(crate::W<TIMX_CCMOD3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_CCMOD3_SPEC>;
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
impl From<crate::W<TIMX_CCMOD3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_CCMOD3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OC5FEN` reader - OC5FEN"]
pub type OC5FEN_R = crate::BitReader<bool>;
#[doc = "Field `OC5FEN` writer - OC5FEN"]
pub type OC5FEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCMOD3_SPEC, bool, 2>;
#[doc = "Field `OC5PEN` reader - OC5PEN"]
pub type OC5PEN_R = crate::BitReader<bool>;
#[doc = "Field `OC5PEN` writer - OC5PEN"]
pub type OC5PEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCMOD3_SPEC, bool, 3>;
#[doc = "Field `OC5MD` reader - OC5MD"]
pub type OC5MD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OC5MD` writer - OC5MD"]
pub type OC5MD_W<'a> = crate::FieldWriter<'a, u32, TIMX_CCMOD3_SPEC, u8, u8, 3, 4>;
#[doc = "Field `OC5CEN` reader - OC5CEN"]
pub type OC5CEN_R = crate::BitReader<bool>;
#[doc = "Field `OC5CEN` writer - OC5CEN"]
pub type OC5CEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCMOD3_SPEC, bool, 7>;
#[doc = "Field `OC6FEN` reader - OC6FEN"]
pub type OC6FEN_R = crate::BitReader<bool>;
#[doc = "Field `OC6FEN` writer - OC6FEN"]
pub type OC6FEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCMOD3_SPEC, bool, 10>;
#[doc = "Field `OC6PEN` reader - OC6PEN"]
pub type OC6PEN_R = crate::BitReader<bool>;
#[doc = "Field `OC6PEN` writer - OC6PEN"]
pub type OC6PEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCMOD3_SPEC, bool, 11>;
#[doc = "Field `OC6MD` reader - OC6MD"]
pub type OC6MD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OC6MD` writer - OC6MD"]
pub type OC6MD_W<'a> = crate::FieldWriter<'a, u32, TIMX_CCMOD3_SPEC, u8, u8, 3, 12>;
#[doc = "Field `OC6CEN` reader - OC6CEN"]
pub type OC6CEN_R = crate::BitReader<bool>;
#[doc = "Field `OC6CEN` writer - OC6CEN"]
pub type OC6CEN_W<'a> = crate::BitWriter<'a, u32, TIMX_CCMOD3_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 2 - OC5FEN"]
    #[inline(always)]
    pub fn oc5fen(&self) -> OC5FEN_R {
        OC5FEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OC5PEN"]
    #[inline(always)]
    pub fn oc5pen(&self) -> OC5PEN_R {
        OC5PEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - OC5MD"]
    #[inline(always)]
    pub fn oc5md(&self) -> OC5MD_R {
        OC5MD_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - OC5CEN"]
    #[inline(always)]
    pub fn oc5cen(&self) -> OC5CEN_R {
        OC5CEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - OC6FEN"]
    #[inline(always)]
    pub fn oc6fen(&self) -> OC6FEN_R {
        OC6FEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OC6PEN"]
    #[inline(always)]
    pub fn oc6pen(&self) -> OC6PEN_R {
        OC6PEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - OC6MD"]
    #[inline(always)]
    pub fn oc6md(&self) -> OC6MD_R {
        OC6MD_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - OC6CEN"]
    #[inline(always)]
    pub fn oc6cen(&self) -> OC6CEN_R {
        OC6CEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - OC5FEN"]
    #[inline(always)]
    pub fn oc5fen(&mut self) -> OC5FEN_W {
        OC5FEN_W::new(self)
    }
    #[doc = "Bit 3 - OC5PEN"]
    #[inline(always)]
    pub fn oc5pen(&mut self) -> OC5PEN_W {
        OC5PEN_W::new(self)
    }
    #[doc = "Bits 4:6 - OC5MD"]
    #[inline(always)]
    pub fn oc5md(&mut self) -> OC5MD_W {
        OC5MD_W::new(self)
    }
    #[doc = "Bit 7 - OC5CEN"]
    #[inline(always)]
    pub fn oc5cen(&mut self) -> OC5CEN_W {
        OC5CEN_W::new(self)
    }
    #[doc = "Bit 10 - OC6FEN"]
    #[inline(always)]
    pub fn oc6fen(&mut self) -> OC6FEN_W {
        OC6FEN_W::new(self)
    }
    #[doc = "Bit 11 - OC6PEN"]
    #[inline(always)]
    pub fn oc6pen(&mut self) -> OC6PEN_W {
        OC6PEN_W::new(self)
    }
    #[doc = "Bits 12:14 - OC6MD"]
    #[inline(always)]
    pub fn oc6md(&mut self) -> OC6MD_W {
        OC6MD_W::new(self)
    }
    #[doc = "Bit 15 - OC6CEN"]
    #[inline(always)]
    pub fn oc6cen(&mut self) -> OC6CEN_W {
        OC6CEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMx_CCMOD3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccmod3](index.html) module"]
pub struct TIMX_CCMOD3_SPEC;
impl crate::RegisterSpec for TIMX_CCMOD3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timx_ccmod3::R](R) reader structure"]
impl crate::Readable for TIMX_CCMOD3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_ccmod3::W](W) writer structure"]
impl crate::Writable for TIMX_CCMOD3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMx_CCMOD3 to value 0"]
impl crate::Resettable for TIMX_CCMOD3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
