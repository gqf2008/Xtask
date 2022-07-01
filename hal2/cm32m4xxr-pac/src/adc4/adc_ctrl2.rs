#[doc = "Register `ADC_CTRL2` reader"]
pub struct R(crate::R<ADC_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_CTRL2` writer"]
pub struct W(crate::W<ADC_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CTRL2_SPEC>;
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
impl From<crate::W<ADC_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ON` reader - ON"]
pub type ON_R = crate::BitReader<bool>;
#[doc = "Field `ON` writer - ON"]
pub type ON_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL2_SPEC, bool, 0>;
#[doc = "Field `CTU` reader - CTU"]
pub type CTU_R = crate::BitReader<bool>;
#[doc = "Field `CTU` writer - CTU"]
pub type CTU_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL2_SPEC, bool, 1>;
#[doc = "Field `ENCAL` reader - ENCAL"]
pub type ENCAL_R = crate::BitReader<bool>;
#[doc = "Field `ENCAL` writer - ENCAL"]
pub type ENCAL_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL2_SPEC, bool, 2>;
#[doc = "Field `ENDMA` reader - ENDMA"]
pub type ENDMA_R = crate::BitReader<bool>;
#[doc = "Field `ENDMA` writer - ENDMA"]
pub type ENDMA_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL2_SPEC, bool, 8>;
#[doc = "Field `ALIG` reader - ALIG"]
pub type ALIG_R = crate::BitReader<bool>;
#[doc = "Field `ALIG` writer - ALIG"]
pub type ALIG_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL2_SPEC, bool, 11>;
#[doc = "Field `EXTJSEL` reader - EXTJSEL"]
pub type EXTJSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTJSEL` writer - EXTJSEL"]
pub type EXTJSEL_W<'a> = crate::FieldWriter<'a, u32, ADC_CTRL2_SPEC, u8, u8, 3, 12>;
#[doc = "Field `EXTJTRIG` reader - EXTJTRIG"]
pub type EXTJTRIG_R = crate::BitReader<bool>;
#[doc = "Field `EXTJTRIG` writer - EXTJTRIG"]
pub type EXTJTRIG_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL2_SPEC, bool, 15>;
#[doc = "Field `EXTRSEL` reader - EXTRSEL"]
pub type EXTRSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTRSEL` writer - EXTRSEL"]
pub type EXTRSEL_W<'a> = crate::FieldWriter<'a, u32, ADC_CTRL2_SPEC, u8, u8, 3, 17>;
#[doc = "Field `EXTRTRIG` reader - EXTRTRIG"]
pub type EXTRTRIG_R = crate::BitReader<bool>;
#[doc = "Field `EXTRTRIG` writer - EXTRTRIG"]
pub type EXTRTRIG_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL2_SPEC, bool, 20>;
#[doc = "Field `SWSTRJCH` reader - SWSTRJCH"]
pub type SWSTRJCH_R = crate::BitReader<bool>;
#[doc = "Field `SWSTRJCH` writer - SWSTRJCH"]
pub type SWSTRJCH_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL2_SPEC, bool, 21>;
#[doc = "Field `SWSTRRCH` reader - SWSTRRCH"]
pub type SWSTRRCH_R = crate::BitReader<bool>;
#[doc = "Field `SWSTRRCH` writer - SWSTRRCH"]
pub type SWSTRRCH_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL2_SPEC, bool, 22>;
#[doc = "Field `TEMPEN` reader - TEMPEN"]
pub type TEMPEN_R = crate::BitReader<bool>;
#[doc = "Field `TEMPEN` writer - TEMPEN"]
pub type TEMPEN_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL2_SPEC, bool, 23>;
impl R {
    #[doc = "Bit 0 - ON"]
    #[inline(always)]
    pub fn on(&self) -> ON_R {
        ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTU"]
    #[inline(always)]
    pub fn ctu(&self) -> CTU_R {
        CTU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ENCAL"]
    #[inline(always)]
    pub fn encal(&self) -> ENCAL_R {
        ENCAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - ENDMA"]
    #[inline(always)]
    pub fn endma(&self) -> ENDMA_R {
        ENDMA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - ALIG"]
    #[inline(always)]
    pub fn alig(&self) -> ALIG_R {
        ALIG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - EXTJSEL"]
    #[inline(always)]
    pub fn extjsel(&self) -> EXTJSEL_R {
        EXTJSEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - EXTJTRIG"]
    #[inline(always)]
    pub fn extjtrig(&self) -> EXTJTRIG_R {
        EXTJTRIG_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:19 - EXTRSEL"]
    #[inline(always)]
    pub fn extrsel(&self) -> EXTRSEL_R {
        EXTRSEL_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - EXTRTRIG"]
    #[inline(always)]
    pub fn extrtrig(&self) -> EXTRTRIG_R {
        EXTRTRIG_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SWSTRJCH"]
    #[inline(always)]
    pub fn swstrjch(&self) -> SWSTRJCH_R {
        SWSTRJCH_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SWSTRRCH"]
    #[inline(always)]
    pub fn swstrrch(&self) -> SWSTRRCH_R {
        SWSTRRCH_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TEMPEN"]
    #[inline(always)]
    pub fn tempen(&self) -> TEMPEN_R {
        TEMPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ON"]
    #[inline(always)]
    pub fn on(&mut self) -> ON_W {
        ON_W::new(self)
    }
    #[doc = "Bit 1 - CTU"]
    #[inline(always)]
    pub fn ctu(&mut self) -> CTU_W {
        CTU_W::new(self)
    }
    #[doc = "Bit 2 - ENCAL"]
    #[inline(always)]
    pub fn encal(&mut self) -> ENCAL_W {
        ENCAL_W::new(self)
    }
    #[doc = "Bit 8 - ENDMA"]
    #[inline(always)]
    pub fn endma(&mut self) -> ENDMA_W {
        ENDMA_W::new(self)
    }
    #[doc = "Bit 11 - ALIG"]
    #[inline(always)]
    pub fn alig(&mut self) -> ALIG_W {
        ALIG_W::new(self)
    }
    #[doc = "Bits 12:14 - EXTJSEL"]
    #[inline(always)]
    pub fn extjsel(&mut self) -> EXTJSEL_W {
        EXTJSEL_W::new(self)
    }
    #[doc = "Bit 15 - EXTJTRIG"]
    #[inline(always)]
    pub fn extjtrig(&mut self) -> EXTJTRIG_W {
        EXTJTRIG_W::new(self)
    }
    #[doc = "Bits 17:19 - EXTRSEL"]
    #[inline(always)]
    pub fn extrsel(&mut self) -> EXTRSEL_W {
        EXTRSEL_W::new(self)
    }
    #[doc = "Bit 20 - EXTRTRIG"]
    #[inline(always)]
    pub fn extrtrig(&mut self) -> EXTRTRIG_W {
        EXTRTRIG_W::new(self)
    }
    #[doc = "Bit 21 - SWSTRJCH"]
    #[inline(always)]
    pub fn swstrjch(&mut self) -> SWSTRJCH_W {
        SWSTRJCH_W::new(self)
    }
    #[doc = "Bit 22 - SWSTRRCH"]
    #[inline(always)]
    pub fn swstrrch(&mut self) -> SWSTRRCH_W {
        SWSTRRCH_W::new(self)
    }
    #[doc = "Bit 23 - TEMPEN"]
    #[inline(always)]
    pub fn tempen(&mut self) -> TEMPEN_W {
        TEMPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_CTRL2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ctrl2](index.html) module"]
pub struct ADC_CTRL2_SPEC;
impl crate::RegisterSpec for ADC_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ctrl2::R](R) reader structure"]
impl crate::Readable for ADC_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ctrl2::W](W) writer structure"]
impl crate::Writable for ADC_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_CTRL2 to value 0"]
impl crate::Resettable for ADC_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
