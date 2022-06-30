#[doc = "Register `COMP7_CTRL` reader"]
pub struct R(crate::R<COMP7_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP7_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP7_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP7_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP7_CTRL` writer"]
pub struct W(crate::W<COMP7_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP7_CTRL_SPEC>;
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
impl From<crate::W<COMP7_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP7_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - EN"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - EN"]
pub type EN_W<'a> = crate::BitWriter<'a, u32, COMP7_CTRL_SPEC, bool, 0>;
#[doc = "Field `INMSEL` reader - INMSEL"]
pub type INMSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INMSEL` writer - INMSEL"]
pub type INMSEL_W<'a> = crate::FieldWriter<'a, u32, COMP7_CTRL_SPEC, u8, u8, 3, 1>;
#[doc = "Field `INPSEL` reader - INPSEL"]
pub type INPSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INPSEL` writer - INPSEL"]
pub type INPSEL_W<'a> = crate::FieldWriter<'a, u32, COMP7_CTRL_SPEC, u8, u8, 3, 4>;
#[doc = "Field `OUTSEL` reader - OUTSEL"]
pub type OUTSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUTSEL` writer - OUTSEL"]
pub type OUTSEL_W<'a> = crate::FieldWriter<'a, u32, COMP7_CTRL_SPEC, u8, u8, 4, 7>;
#[doc = "Field `POL` reader - POL"]
pub type POL_R = crate::BitReader<bool>;
#[doc = "Field `POL` writer - POL"]
pub type POL_W<'a> = crate::BitWriter<'a, u32, COMP7_CTRL_SPEC, bool, 11>;
#[doc = "Field `HYST` reader - HYST"]
pub type HYST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HYST` writer - HYST"]
pub type HYST_W<'a> = crate::FieldWriter<'a, u32, COMP7_CTRL_SPEC, u8, u8, 2, 12>;
#[doc = "Field `BLKING` reader - BLKING"]
pub type BLKING_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BLKING` writer - BLKING"]
pub type BLKING_W<'a> = crate::FieldWriter<'a, u32, COMP7_CTRL_SPEC, u8, u8, 3, 14>;
#[doc = "Field `OUT` reader - OUT"]
pub type OUT_R = crate::BitReader<bool>;
#[doc = "Field `OUT` writer - OUT"]
pub type OUT_W<'a> = crate::BitWriter<'a, u32, COMP7_CTRL_SPEC, bool, 17>;
#[doc = "Field `INPDAC` reader - INPDAC"]
pub type INPDAC_R = crate::BitReader<bool>;
#[doc = "Field `INPDAC` writer - INPDAC"]
pub type INPDAC_W<'a> = crate::BitWriter<'a, u32, COMP7_CTRL_SPEC, bool, 18>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - INMSEL"]
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - INPSEL"]
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:10 - OUTSEL"]
    #[inline(always)]
    pub fn outsel(&self) -> OUTSEL_R {
        OUTSEL_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - POL"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - HYST"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:16 - BLKING"]
    #[inline(always)]
    pub fn blking(&self) -> BLKING_R {
        BLKING_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 17 - OUT"]
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - INPDAC"]
    #[inline(always)]
    pub fn inpdac(&self) -> INPDAC_R {
        INPDAC_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W::new(self)
    }
    #[doc = "Bits 1:3 - INMSEL"]
    #[inline(always)]
    pub fn inmsel(&mut self) -> INMSEL_W {
        INMSEL_W::new(self)
    }
    #[doc = "Bits 4:6 - INPSEL"]
    #[inline(always)]
    pub fn inpsel(&mut self) -> INPSEL_W {
        INPSEL_W::new(self)
    }
    #[doc = "Bits 7:10 - OUTSEL"]
    #[inline(always)]
    pub fn outsel(&mut self) -> OUTSEL_W {
        OUTSEL_W::new(self)
    }
    #[doc = "Bit 11 - POL"]
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W {
        POL_W::new(self)
    }
    #[doc = "Bits 12:13 - HYST"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W {
        HYST_W::new(self)
    }
    #[doc = "Bits 14:16 - BLKING"]
    #[inline(always)]
    pub fn blking(&mut self) -> BLKING_W {
        BLKING_W::new(self)
    }
    #[doc = "Bit 17 - OUT"]
    #[inline(always)]
    pub fn out(&mut self) -> OUT_W {
        OUT_W::new(self)
    }
    #[doc = "Bit 18 - INPDAC"]
    #[inline(always)]
    pub fn inpdac(&mut self) -> INPDAC_W {
        INPDAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "COMP7_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp7_ctrl](index.html) module"]
pub struct COMP7_CTRL_SPEC;
impl crate::RegisterSpec for COMP7_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp7_ctrl::R](R) reader structure"]
impl crate::Readable for COMP7_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp7_ctrl::W](W) writer structure"]
impl crate::Writable for COMP7_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMP7_CTRL to value 0"]
impl crate::Resettable for COMP7_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
