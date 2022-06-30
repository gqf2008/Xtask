#[doc = "Register `RTC_CTRL` reader"]
pub struct R(crate::R<RTC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CTRL` writer"]
pub struct W(crate::W<RTC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CTRL_SPEC>;
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
impl From<crate::W<RTC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUPSEL` reader - WKUPSEL"]
pub type WKUPSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WKUPSEL` writer - WKUPSEL"]
pub type WKUPSEL_W<'a> = crate::FieldWriter<'a, u32, RTC_CTRL_SPEC, u8, u8, 3, 0>;
#[doc = "Field `BYPS` reader - BYPS"]
pub type BYPS_R = crate::BitReader<bool>;
#[doc = "Field `BYPS` writer - BYPS"]
pub type BYPS_W<'a> = crate::BitWriter<'a, u32, RTC_CTRL_SPEC, bool, 5>;
#[doc = "Field `HFMT` reader - HFMT"]
pub type HFMT_R = crate::BitReader<bool>;
#[doc = "Field `HFMT` writer - HFMT"]
pub type HFMT_W<'a> = crate::BitWriter<'a, u32, RTC_CTRL_SPEC, bool, 6>;
#[doc = "Field `ALAEN` reader - ALAEN"]
pub type ALAEN_R = crate::BitReader<bool>;
#[doc = "Field `ALAEN` writer - ALAEN"]
pub type ALAEN_W<'a> = crate::BitWriter<'a, u32, RTC_CTRL_SPEC, bool, 8>;
#[doc = "Field `ALBEN` reader - ALBEN"]
pub type ALBEN_R = crate::BitReader<bool>;
#[doc = "Field `ALBEN` writer - ALBEN"]
pub type ALBEN_W<'a> = crate::BitWriter<'a, u32, RTC_CTRL_SPEC, bool, 9>;
#[doc = "Field `WTEN` reader - WTEN"]
pub type WTEN_R = crate::BitReader<bool>;
#[doc = "Field `WTEN` writer - WTEN"]
pub type WTEN_W<'a> = crate::BitWriter<'a, u32, RTC_CTRL_SPEC, bool, 10>;
#[doc = "Field `TSEN` reader - TSEN"]
pub type TSEN_R = crate::BitReader<bool>;
#[doc = "Field `TSEN` writer - TSEN"]
pub type TSEN_W<'a> = crate::BitWriter<'a, u32, RTC_CTRL_SPEC, bool, 11>;
#[doc = "Field `ALAIEN` reader - ALAIEN"]
pub type ALAIEN_R = crate::BitReader<bool>;
#[doc = "Field `ALAIEN` writer - ALAIEN"]
pub type ALAIEN_W<'a> = crate::BitWriter<'a, u32, RTC_CTRL_SPEC, bool, 12>;
#[doc = "Field `ALBIEN` reader - ALBIEN"]
pub type ALBIEN_R = crate::BitReader<bool>;
#[doc = "Field `ALBIEN` writer - ALBIEN"]
pub type ALBIEN_W<'a> = crate::BitWriter<'a, u32, RTC_CTRL_SPEC, bool, 13>;
#[doc = "Field `WTIEN` reader - WTIEN"]
pub type WTIEN_R = crate::BitReader<bool>;
#[doc = "Field `WTIEN` writer - WTIEN"]
pub type WTIEN_W<'a> = crate::BitWriter<'a, u32, RTC_CTRL_SPEC, bool, 14>;
#[doc = "Field `AD1H` reader - AD1H"]
pub type AD1H_R = crate::BitReader<bool>;
#[doc = "Field `AD1H` writer - AD1H"]
pub type AD1H_W<'a> = crate::BitWriter<'a, u32, RTC_CTRL_SPEC, bool, 16>;
#[doc = "Field `SU1H` reader - SU1H"]
pub type SU1H_R = crate::BitReader<bool>;
#[doc = "Field `SU1H` writer - SU1H"]
pub type SU1H_W<'a> = crate::BitWriter<'a, u32, RTC_CTRL_SPEC, bool, 17>;
#[doc = "Field `BAKP` reader - BAKP"]
pub type BAKP_R = crate::BitReader<bool>;
#[doc = "Field `BAKP` writer - BAKP"]
pub type BAKP_W<'a> = crate::BitWriter<'a, u32, RTC_CTRL_SPEC, bool, 18>;
#[doc = "Field `CALOSEL` reader - CALOSEL"]
pub type CALOSEL_R = crate::BitReader<bool>;
#[doc = "Field `CALOSEL` writer - CALOSEL"]
pub type CALOSEL_W<'a> = crate::BitWriter<'a, u32, RTC_CTRL_SPEC, bool, 19>;
#[doc = "Field `OPOL` reader - OPOL"]
pub type OPOL_R = crate::BitReader<bool>;
#[doc = "Field `OPOL` writer - OPOL"]
pub type OPOL_W<'a> = crate::BitWriter<'a, u32, RTC_CTRL_SPEC, bool, 20>;
#[doc = "Field `OUTSEL` reader - OUTSEL"]
pub type OUTSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUTSEL` writer - OUTSEL"]
pub type OUTSEL_W<'a> = crate::FieldWriter<'a, u32, RTC_CTRL_SPEC, u8, u8, 2, 21>;
#[doc = "Field `COEN` reader - COEN"]
pub type COEN_R = crate::BitReader<bool>;
#[doc = "Field `COEN` writer - COEN"]
pub type COEN_W<'a> = crate::BitWriter<'a, u32, RTC_CTRL_SPEC, bool, 23>;
impl R {
    #[doc = "Bits 0:2 - WKUPSEL"]
    #[inline(always)]
    pub fn wkupsel(&self) -> WKUPSEL_R {
        WKUPSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - BYPS"]
    #[inline(always)]
    pub fn byps(&self) -> BYPS_R {
        BYPS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HFMT"]
    #[inline(always)]
    pub fn hfmt(&self) -> HFMT_R {
        HFMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - ALAEN"]
    #[inline(always)]
    pub fn alaen(&self) -> ALAEN_R {
        ALAEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ALBEN"]
    #[inline(always)]
    pub fn alben(&self) -> ALBEN_R {
        ALBEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - WTEN"]
    #[inline(always)]
    pub fn wten(&self) -> WTEN_R {
        WTEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TSEN"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ALAIEN"]
    #[inline(always)]
    pub fn alaien(&self) -> ALAIEN_R {
        ALAIEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ALBIEN"]
    #[inline(always)]
    pub fn albien(&self) -> ALBIEN_R {
        ALBIEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - WTIEN"]
    #[inline(always)]
    pub fn wtien(&self) -> WTIEN_R {
        WTIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - AD1H"]
    #[inline(always)]
    pub fn ad1h(&self) -> AD1H_R {
        AD1H_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SU1H"]
    #[inline(always)]
    pub fn su1h(&self) -> SU1H_R {
        SU1H_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - BAKP"]
    #[inline(always)]
    pub fn bakp(&self) -> BAKP_R {
        BAKP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CALOSEL"]
    #[inline(always)]
    pub fn calosel(&self) -> CALOSEL_R {
        CALOSEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OPOL"]
    #[inline(always)]
    pub fn opol(&self) -> OPOL_R {
        OPOL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - OUTSEL"]
    #[inline(always)]
    pub fn outsel(&self) -> OUTSEL_R {
        OUTSEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - COEN"]
    #[inline(always)]
    pub fn coen(&self) -> COEN_R {
        COEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - WKUPSEL"]
    #[inline(always)]
    pub fn wkupsel(&mut self) -> WKUPSEL_W {
        WKUPSEL_W::new(self)
    }
    #[doc = "Bit 5 - BYPS"]
    #[inline(always)]
    pub fn byps(&mut self) -> BYPS_W {
        BYPS_W::new(self)
    }
    #[doc = "Bit 6 - HFMT"]
    #[inline(always)]
    pub fn hfmt(&mut self) -> HFMT_W {
        HFMT_W::new(self)
    }
    #[doc = "Bit 8 - ALAEN"]
    #[inline(always)]
    pub fn alaen(&mut self) -> ALAEN_W {
        ALAEN_W::new(self)
    }
    #[doc = "Bit 9 - ALBEN"]
    #[inline(always)]
    pub fn alben(&mut self) -> ALBEN_W {
        ALBEN_W::new(self)
    }
    #[doc = "Bit 10 - WTEN"]
    #[inline(always)]
    pub fn wten(&mut self) -> WTEN_W {
        WTEN_W::new(self)
    }
    #[doc = "Bit 11 - TSEN"]
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W {
        TSEN_W::new(self)
    }
    #[doc = "Bit 12 - ALAIEN"]
    #[inline(always)]
    pub fn alaien(&mut self) -> ALAIEN_W {
        ALAIEN_W::new(self)
    }
    #[doc = "Bit 13 - ALBIEN"]
    #[inline(always)]
    pub fn albien(&mut self) -> ALBIEN_W {
        ALBIEN_W::new(self)
    }
    #[doc = "Bit 14 - WTIEN"]
    #[inline(always)]
    pub fn wtien(&mut self) -> WTIEN_W {
        WTIEN_W::new(self)
    }
    #[doc = "Bit 16 - AD1H"]
    #[inline(always)]
    pub fn ad1h(&mut self) -> AD1H_W {
        AD1H_W::new(self)
    }
    #[doc = "Bit 17 - SU1H"]
    #[inline(always)]
    pub fn su1h(&mut self) -> SU1H_W {
        SU1H_W::new(self)
    }
    #[doc = "Bit 18 - BAKP"]
    #[inline(always)]
    pub fn bakp(&mut self) -> BAKP_W {
        BAKP_W::new(self)
    }
    #[doc = "Bit 19 - CALOSEL"]
    #[inline(always)]
    pub fn calosel(&mut self) -> CALOSEL_W {
        CALOSEL_W::new(self)
    }
    #[doc = "Bit 20 - OPOL"]
    #[inline(always)]
    pub fn opol(&mut self) -> OPOL_W {
        OPOL_W::new(self)
    }
    #[doc = "Bits 21:22 - OUTSEL"]
    #[inline(always)]
    pub fn outsel(&mut self) -> OUTSEL_W {
        OUTSEL_W::new(self)
    }
    #[doc = "Bit 23 - COEN"]
    #[inline(always)]
    pub fn coen(&mut self) -> COEN_W {
        COEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_ctrl](index.html) module"]
pub struct RTC_CTRL_SPEC;
impl crate::RegisterSpec for RTC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_ctrl::R](R) reader structure"]
impl crate::Readable for RTC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_ctrl::W](W) writer structure"]
impl crate::Writable for RTC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CTRL to value 0"]
impl crate::Resettable for RTC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
