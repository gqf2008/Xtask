#[doc = "Register `ADC_CTRL1` reader"]
pub struct R(crate::R<ADC_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_CTRL1` writer"]
pub struct W(crate::W<ADC_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CTRL1_SPEC>;
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
impl From<crate::W<ADC_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AWDGCH` reader - AWDGCH"]
pub type AWDGCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWDGCH` writer - AWDGCH"]
pub type AWDGCH_W<'a> = crate::FieldWriter<'a, u32, ADC_CTRL1_SPEC, u8, u8, 5, 0>;
#[doc = "Field `ENDIEN` reader - ENDIEN"]
pub type ENDIEN_R = crate::BitReader<bool>;
#[doc = "Field `ENDIEN` writer - ENDIEN"]
pub type ENDIEN_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL1_SPEC, bool, 5>;
#[doc = "Field `AWDGIEN` reader - AWDGIEN"]
pub type AWDGIEN_R = crate::BitReader<bool>;
#[doc = "Field `AWDGIEN` writer - AWDGIEN"]
pub type AWDGIEN_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL1_SPEC, bool, 6>;
#[doc = "Field `JENDCIEN` reader - JENDCIEN"]
pub type JENDCIEN_R = crate::BitReader<bool>;
#[doc = "Field `JENDCIEN` writer - JENDCIEN"]
pub type JENDCIEN_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL1_SPEC, bool, 7>;
#[doc = "Field `SCANMD` reader - SCANMD"]
pub type SCANMD_R = crate::BitReader<bool>;
#[doc = "Field `SCANMD` writer - SCANMD"]
pub type SCANMD_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL1_SPEC, bool, 8>;
#[doc = "Field `AWDGSGLEN` reader - AWDGSGLEN"]
pub type AWDGSGLEN_R = crate::BitReader<bool>;
#[doc = "Field `AWDGSGLEN` writer - AWDGSGLEN"]
pub type AWDGSGLEN_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL1_SPEC, bool, 9>;
#[doc = "Field `AUTOJC` reader - AUTOJC"]
pub type AUTOJC_R = crate::BitReader<bool>;
#[doc = "Field `AUTOJC` writer - AUTOJC"]
pub type AUTOJC_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL1_SPEC, bool, 10>;
#[doc = "Field `DREGCH` reader - DREGCH"]
pub type DREGCH_R = crate::BitReader<bool>;
#[doc = "Field `DREGCH` writer - DREGCH"]
pub type DREGCH_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL1_SPEC, bool, 11>;
#[doc = "Field `DJCH` reader - DJCH"]
pub type DJCH_R = crate::BitReader<bool>;
#[doc = "Field `DJCH` writer - DJCH"]
pub type DJCH_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL1_SPEC, bool, 12>;
#[doc = "Field `DTU` reader - DTU"]
pub type DTU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTU` writer - DTU"]
pub type DTU_W<'a> = crate::FieldWriter<'a, u32, ADC_CTRL1_SPEC, u8, u8, 3, 13>;
#[doc = "Field `DUSEL` reader - DUSEL"]
pub type DUSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DUSEL` writer - DUSEL"]
pub type DUSEL_W<'a> = crate::FieldWriter<'a, u32, ADC_CTRL1_SPEC, u8, u8, 4, 16>;
#[doc = "Field `AWDGEJCH` reader - AWDGEJCH"]
pub type AWDGEJCH_R = crate::BitReader<bool>;
#[doc = "Field `AWDGEJCH` writer - AWDGEJCH"]
pub type AWDGEJCH_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL1_SPEC, bool, 22>;
#[doc = "Field `AWDGERCH` reader - AWDGERCH"]
pub type AWDGERCH_R = crate::BitReader<bool>;
#[doc = "Field `AWDGERCH` writer - AWDGERCH"]
pub type AWDGERCH_W<'a> = crate::BitWriter<'a, u32, ADC_CTRL1_SPEC, bool, 23>;
impl R {
    #[doc = "Bits 0:4 - AWDGCH"]
    #[inline(always)]
    pub fn awdgch(&self) -> AWDGCH_R {
        AWDGCH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - ENDIEN"]
    #[inline(always)]
    pub fn endien(&self) -> ENDIEN_R {
        ENDIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AWDGIEN"]
    #[inline(always)]
    pub fn awdgien(&self) -> AWDGIEN_R {
        AWDGIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - JENDCIEN"]
    #[inline(always)]
    pub fn jendcien(&self) -> JENDCIEN_R {
        JENDCIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCANMD"]
    #[inline(always)]
    pub fn scanmd(&self) -> SCANMD_R {
        SCANMD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AWDGSGLEN"]
    #[inline(always)]
    pub fn awdgsglen(&self) -> AWDGSGLEN_R {
        AWDGSGLEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AUTOJC"]
    #[inline(always)]
    pub fn autojc(&self) -> AUTOJC_R {
        AUTOJC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DREGCH"]
    #[inline(always)]
    pub fn dregch(&self) -> DREGCH_R {
        DREGCH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DJCH"]
    #[inline(always)]
    pub fn djch(&self) -> DJCH_R {
        DJCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - DTU"]
    #[inline(always)]
    pub fn dtu(&self) -> DTU_R {
        DTU_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - DUSEL"]
    #[inline(always)]
    pub fn dusel(&self) -> DUSEL_R {
        DUSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - AWDGEJCH"]
    #[inline(always)]
    pub fn awdgejch(&self) -> AWDGEJCH_R {
        AWDGEJCH_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - AWDGERCH"]
    #[inline(always)]
    pub fn awdgerch(&self) -> AWDGERCH_R {
        AWDGERCH_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - AWDGCH"]
    #[inline(always)]
    pub fn awdgch(&mut self) -> AWDGCH_W {
        AWDGCH_W::new(self)
    }
    #[doc = "Bit 5 - ENDIEN"]
    #[inline(always)]
    pub fn endien(&mut self) -> ENDIEN_W {
        ENDIEN_W::new(self)
    }
    #[doc = "Bit 6 - AWDGIEN"]
    #[inline(always)]
    pub fn awdgien(&mut self) -> AWDGIEN_W {
        AWDGIEN_W::new(self)
    }
    #[doc = "Bit 7 - JENDCIEN"]
    #[inline(always)]
    pub fn jendcien(&mut self) -> JENDCIEN_W {
        JENDCIEN_W::new(self)
    }
    #[doc = "Bit 8 - SCANMD"]
    #[inline(always)]
    pub fn scanmd(&mut self) -> SCANMD_W {
        SCANMD_W::new(self)
    }
    #[doc = "Bit 9 - AWDGSGLEN"]
    #[inline(always)]
    pub fn awdgsglen(&mut self) -> AWDGSGLEN_W {
        AWDGSGLEN_W::new(self)
    }
    #[doc = "Bit 10 - AUTOJC"]
    #[inline(always)]
    pub fn autojc(&mut self) -> AUTOJC_W {
        AUTOJC_W::new(self)
    }
    #[doc = "Bit 11 - DREGCH"]
    #[inline(always)]
    pub fn dregch(&mut self) -> DREGCH_W {
        DREGCH_W::new(self)
    }
    #[doc = "Bit 12 - DJCH"]
    #[inline(always)]
    pub fn djch(&mut self) -> DJCH_W {
        DJCH_W::new(self)
    }
    #[doc = "Bits 13:15 - DTU"]
    #[inline(always)]
    pub fn dtu(&mut self) -> DTU_W {
        DTU_W::new(self)
    }
    #[doc = "Bits 16:19 - DUSEL"]
    #[inline(always)]
    pub fn dusel(&mut self) -> DUSEL_W {
        DUSEL_W::new(self)
    }
    #[doc = "Bit 22 - AWDGEJCH"]
    #[inline(always)]
    pub fn awdgejch(&mut self) -> AWDGEJCH_W {
        AWDGEJCH_W::new(self)
    }
    #[doc = "Bit 23 - AWDGERCH"]
    #[inline(always)]
    pub fn awdgerch(&mut self) -> AWDGERCH_W {
        AWDGERCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_CTRL1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ctrl1](index.html) module"]
pub struct ADC_CTRL1_SPEC;
impl crate::RegisterSpec for ADC_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ctrl1::R](R) reader structure"]
impl crate::Readable for ADC_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ctrl1::W](W) writer structure"]
impl crate::Writable for ADC_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_CTRL1 to value 0"]
impl crate::Resettable for ADC_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
