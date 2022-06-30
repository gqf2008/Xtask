#[doc = "Register `ADC_CTRL3` reader"]
pub struct R(crate::R<ADC_CTRL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CTRL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CTRL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CTRL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_CTRL3` writer"]
pub struct W(crate::W<ADC_CTRL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CTRL3_SPEC>;
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
impl From<crate::W<ADC_CTRL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CTRL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RES` reader - RES"]
pub type RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RES` writer - RES"]
pub type RES_W<'a> = crate::FieldWriter<'a, u32, ADC_CTRL3_SPEC, u8, u8, 2, 0>;
#[doc = "Field `CALDIF` reader - CALDIF"]
pub type CALDIF_R = crate::BitReader<bool>;
#[doc = "Field `CALDIF` writer - CALDIF"]
pub type CALDIF_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL3_SPEC, bool, 2>;
#[doc = "Field `CALALD` reader - CALALD"]
pub type CALALD_R = crate::BitReader<bool>;
#[doc = "Field `CALALD` writer - CALALD"]
pub type CALALD_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL3_SPEC, bool, 3>;
#[doc = "Field `CKMOD` reader - CKMOD"]
pub type CKMOD_R = crate::BitReader<bool>;
#[doc = "Field `CKMOD` writer - CKMOD"]
pub type CKMOD_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL3_SPEC, bool, 4>;
#[doc = "Field `RDY` reader - RDY"]
pub type RDY_R = crate::BitReader<bool>;
#[doc = "Field `RDY` writer - RDY"]
pub type RDY_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL3_SPEC, bool, 5>;
#[doc = "Field `PDRDY` reader - PDRDY"]
pub type PDRDY_R = crate::BitReader<bool>;
#[doc = "Field `PDRDY` writer - PDRDY"]
pub type PDRDY_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL3_SPEC, bool, 6>;
#[doc = "Field `BPCAL` reader - BPCAL"]
pub type BPCAL_R = crate::BitReader<bool>;
#[doc = "Field `BPCAL` writer - BPCAL"]
pub type BPCAL_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL3_SPEC, bool, 7>;
#[doc = "Field `ENDCAIEN` reader - ENDCAIEN"]
pub type ENDCAIEN_R = crate::BitReader<bool>;
#[doc = "Field `ENDCAIEN` writer - ENDCAIEN"]
pub type ENDCAIEN_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL3_SPEC, bool, 8>;
#[doc = "Field `JENDCAIEN` reader - JENDCAIEN"]
pub type JENDCAIEN_R = crate::BitReader<bool>;
#[doc = "Field `JENDCAIEN` writer - JENDCAIEN"]
pub type JENDCAIEN_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL3_SPEC, bool, 9>;
#[doc = "Field `DPWMOD` reader - DPWMOD"]
pub type DPWMOD_R = crate::BitReader<bool>;
#[doc = "Field `DPWMOD` writer - DPWMOD"]
pub type DPWMOD_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL3_SPEC, bool, 10>;
#[doc = "Field `VABTMEN` reader - VABTMEN"]
pub type VABTMEN_R = crate::BitReader<bool>;
#[doc = "Field `VABTMEN` writer - VABTMEN"]
pub type VABTMEN_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL3_SPEC, bool, 11>;
impl R {
    #[doc = "Bits 0:1 - RES"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - CALDIF"]
    #[inline(always)]
    pub fn caldif(&self) -> CALDIF_R {
        CALDIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CALALD"]
    #[inline(always)]
    pub fn calald(&self) -> CALALD_R {
        CALALD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CKMOD"]
    #[inline(always)]
    pub fn ckmod(&self) -> CKMOD_R {
        CKMOD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RDY"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PDRDY"]
    #[inline(always)]
    pub fn pdrdy(&self) -> PDRDY_R {
        PDRDY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BPCAL"]
    #[inline(always)]
    pub fn bpcal(&self) -> BPCAL_R {
        BPCAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ENDCAIEN"]
    #[inline(always)]
    pub fn endcaien(&self) -> ENDCAIEN_R {
        ENDCAIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - JENDCAIEN"]
    #[inline(always)]
    pub fn jendcaien(&self) -> JENDCAIEN_R {
        JENDCAIEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DPWMOD"]
    #[inline(always)]
    pub fn dpwmod(&self) -> DPWMOD_R {
        DPWMOD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - VABTMEN"]
    #[inline(always)]
    pub fn vabtmen(&self) -> VABTMEN_R {
        VABTMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - RES"]
    #[inline(always)]
    pub fn res(&mut self) -> RES_W {
        RES_W::new(self)
    }
    #[doc = "Bit 2 - CALDIF"]
    #[inline(always)]
    pub fn caldif(&mut self) -> CALDIF_W {
        CALDIF_W::new(self)
    }
    #[doc = "Bit 3 - CALALD"]
    #[inline(always)]
    pub fn calald(&mut self) -> CALALD_W {
        CALALD_W::new(self)
    }
    #[doc = "Bit 4 - CKMOD"]
    #[inline(always)]
    pub fn ckmod(&mut self) -> CKMOD_W {
        CKMOD_W::new(self)
    }
    #[doc = "Bit 5 - RDY"]
    #[inline(always)]
    pub fn rdy(&mut self) -> RDY_W {
        RDY_W::new(self)
    }
    #[doc = "Bit 6 - PDRDY"]
    #[inline(always)]
    pub fn pdrdy(&mut self) -> PDRDY_W {
        PDRDY_W::new(self)
    }
    #[doc = "Bit 7 - BPCAL"]
    #[inline(always)]
    pub fn bpcal(&mut self) -> BPCAL_W {
        BPCAL_W::new(self)
    }
    #[doc = "Bit 8 - ENDCAIEN"]
    #[inline(always)]
    pub fn endcaien(&mut self) -> ENDCAIEN_W {
        ENDCAIEN_W::new(self)
    }
    #[doc = "Bit 9 - JENDCAIEN"]
    #[inline(always)]
    pub fn jendcaien(&mut self) -> JENDCAIEN_W {
        JENDCAIEN_W::new(self)
    }
    #[doc = "Bit 10 - DPWMOD"]
    #[inline(always)]
    pub fn dpwmod(&mut self) -> DPWMOD_W {
        DPWMOD_W::new(self)
    }
    #[doc = "Bit 11 - VABTMEN"]
    #[inline(always)]
    pub fn vabtmen(&mut self) -> VABTMEN_W {
        VABTMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_CTRL3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ctrl3](index.html) module"]
pub struct ADC_CTRL3_SPEC;
impl crate::RegisterSpec for ADC_CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ctrl3::R](R) reader structure"]
impl crate::Readable for ADC_CTRL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ctrl3::W](W) writer structure"]
impl crate::Writable for ADC_CTRL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_CTRL3 to value 0"]
impl crate::Resettable for ADC_CTRL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
