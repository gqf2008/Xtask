#[doc = "Register `AFIO_RMP_CFG4` reader"]
pub struct R(crate::R<AFIO_RMP_CFG4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIO_RMP_CFG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIO_RMP_CFG4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIO_RMP_CFG4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFIO_RMP_CFG4` writer"]
pub struct W(crate::W<AFIO_RMP_CFG4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFIO_RMP_CFG4_SPEC>;
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
impl From<crate::W<AFIO_RMP_CFG4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFIO_RMP_CFG4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP1_RMP` reader - COMP1_RMP"]
pub type COMP1_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP1_RMP` writer - COMP1_RMP"]
pub type COMP1_RMP_W<'a> = crate::FieldWriter<'a, u32, AFIO_RMP_CFG4_SPEC, u8, u8, 2, 0>;
#[doc = "Field `COMP2_RMP` reader - COMP2_RMP"]
pub type COMP2_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP2_RMP` writer - COMP2_RMP"]
pub type COMP2_RMP_W<'a> = crate::FieldWriter<'a, u32, AFIO_RMP_CFG4_SPEC, u8, u8, 2, 2>;
#[doc = "Field `COMP3_RMP` reader - COMP3_RMP"]
pub type COMP3_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP3_RMP` writer - COMP3_RMP"]
pub type COMP3_RMP_W<'a> = crate::FieldWriter<'a, u32, AFIO_RMP_CFG4_SPEC, u8, u8, 2, 4>;
#[doc = "Field `COMP4_RMP` reader - COMP4_RMP"]
pub type COMP4_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP4_RMP` writer - COMP4_RMP"]
pub type COMP4_RMP_W<'a> = crate::FieldWriter<'a, u32, AFIO_RMP_CFG4_SPEC, u8, u8, 2, 6>;
#[doc = "Field `COMP5_RMP` reader - COMP5_RMP"]
pub type COMP5_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP5_RMP` writer - COMP5_RMP"]
pub type COMP5_RMP_W<'a> = crate::FieldWriter<'a, u32, AFIO_RMP_CFG4_SPEC, u8, u8, 2, 8>;
#[doc = "Field `COMP6_RMP` reader - COMP6_RMP"]
pub type COMP6_RMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP6_RMP` writer - COMP6_RMP"]
pub type COMP6_RMP_W<'a> = crate::FieldWriter<'a, u32, AFIO_RMP_CFG4_SPEC, u8, u8, 2, 10>;
#[doc = "Field `COMP7_RMP` reader - COMP7_RMP"]
pub type COMP7_RMP_R = crate::BitReader<bool>;
#[doc = "Field `COMP7_RMP` writer - COMP7_RMP"]
pub type COMP7_RMP_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG4_SPEC, bool, 12>;
#[doc = "Field `ADC3_ETRI` reader - ADC3_ETRI"]
pub type ADC3_ETRI_R = crate::BitReader<bool>;
#[doc = "Field `ADC3_ETRI` writer - ADC3_ETRI"]
pub type ADC3_ETRI_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG4_SPEC, bool, 14>;
#[doc = "Field `ADC3_ETRR` reader - ADC3_ETRR"]
pub type ADC3_ETRR_R = crate::BitReader<bool>;
#[doc = "Field `ADC3_ETRR` writer - ADC3_ETRR"]
pub type ADC3_ETRR_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG4_SPEC, bool, 15>;
#[doc = "Field `ADC4_ETRI` reader - ADC4_ETRI"]
pub type ADC4_ETRI_R = crate::BitReader<bool>;
#[doc = "Field `ADC4_ETRI` writer - ADC4_ETRI"]
pub type ADC4_ETRI_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG4_SPEC, bool, 16>;
#[doc = "Field `ADC4_ETRR` reader - ADC4_ETRR"]
pub type ADC4_ETRR_R = crate::BitReader<bool>;
#[doc = "Field `ADC4_ETRR` writer - ADC4_ETRR"]
pub type ADC4_ETRR_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG4_SPEC, bool, 17>;
#[doc = "Field `TSC_OUT_CTRL` reader - TSC_OUT_CTRL"]
pub type TSC_OUT_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `TSC_OUT_CTRL` writer - TSC_OUT_CTRL"]
pub type TSC_OUT_CTRL_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG4_SPEC, bool, 18>;
#[doc = "Field `QSPI_XIP_EN` reader - QSPI_XIP_EN"]
pub type QSPI_XIP_EN_R = crate::BitReader<bool>;
#[doc = "Field `QSPI_XIP_EN` writer - QSPI_XIP_EN"]
pub type QSPI_XIP_EN_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG4_SPEC, bool, 19>;
#[doc = "Field `SPI1_NSS` reader - SPI1_NSS"]
pub type SPI1_NSS_R = crate::BitReader<bool>;
#[doc = "Field `SPI1_NSS` writer - SPI1_NSS"]
pub type SPI1_NSS_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG4_SPEC, bool, 22>;
#[doc = "Field `SPI2_NSS` reader - SPI2_NSS"]
pub type SPI2_NSS_R = crate::BitReader<bool>;
#[doc = "Field `SPI2_NSS` writer - SPI2_NSS"]
pub type SPI2_NSS_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG4_SPEC, bool, 23>;
#[doc = "Field `SPI3_NSS` reader - SPI3_NSS"]
pub type SPI3_NSS_R = crate::BitReader<bool>;
#[doc = "Field `SPI3_NSS` writer - SPI3_NSS"]
pub type SPI3_NSS_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG4_SPEC, bool, 24>;
#[doc = "Field `QSPI_MISO` reader - QSPI_MISO"]
pub type QSPI_MISO_R = crate::BitReader<bool>;
#[doc = "Field `QSPI_MISO` writer - QSPI_MISO"]
pub type QSPI_MISO_W<'a> = crate::BitWriter<'a, u32, AFIO_RMP_CFG4_SPEC, bool, 25>;
impl R {
    #[doc = "Bits 0:1 - COMP1_RMP"]
    #[inline(always)]
    pub fn comp1_rmp(&self) -> COMP1_RMP_R {
        COMP1_RMP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - COMP2_RMP"]
    #[inline(always)]
    pub fn comp2_rmp(&self) -> COMP2_RMP_R {
        COMP2_RMP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - COMP3_RMP"]
    #[inline(always)]
    pub fn comp3_rmp(&self) -> COMP3_RMP_R {
        COMP3_RMP_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - COMP4_RMP"]
    #[inline(always)]
    pub fn comp4_rmp(&self) -> COMP4_RMP_R {
        COMP4_RMP_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - COMP5_RMP"]
    #[inline(always)]
    pub fn comp5_rmp(&self) -> COMP5_RMP_R {
        COMP5_RMP_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - COMP6_RMP"]
    #[inline(always)]
    pub fn comp6_rmp(&self) -> COMP6_RMP_R {
        COMP6_RMP_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - COMP7_RMP"]
    #[inline(always)]
    pub fn comp7_rmp(&self) -> COMP7_RMP_R {
        COMP7_RMP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC3_ETRI"]
    #[inline(always)]
    pub fn adc3_etri(&self) -> ADC3_ETRI_R {
        ADC3_ETRI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC3_ETRR"]
    #[inline(always)]
    pub fn adc3_etrr(&self) -> ADC3_ETRR_R {
        ADC3_ETRR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC4_ETRI"]
    #[inline(always)]
    pub fn adc4_etri(&self) -> ADC4_ETRI_R {
        ADC4_ETRI_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC4_ETRR"]
    #[inline(always)]
    pub fn adc4_etrr(&self) -> ADC4_ETRR_R {
        ADC4_ETRR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TSC_OUT_CTRL"]
    #[inline(always)]
    pub fn tsc_out_ctrl(&self) -> TSC_OUT_CTRL_R {
        TSC_OUT_CTRL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - QSPI_XIP_EN"]
    #[inline(always)]
    pub fn qspi_xip_en(&self) -> QSPI_XIP_EN_R {
        QSPI_XIP_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - SPI1_NSS"]
    #[inline(always)]
    pub fn spi1_nss(&self) -> SPI1_NSS_R {
        SPI1_NSS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SPI2_NSS"]
    #[inline(always)]
    pub fn spi2_nss(&self) -> SPI2_NSS_R {
        SPI2_NSS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SPI3_NSS"]
    #[inline(always)]
    pub fn spi3_nss(&self) -> SPI3_NSS_R {
        SPI3_NSS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - QSPI_MISO"]
    #[inline(always)]
    pub fn qspi_miso(&self) -> QSPI_MISO_R {
        QSPI_MISO_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - COMP1_RMP"]
    #[inline(always)]
    pub fn comp1_rmp(&mut self) -> COMP1_RMP_W {
        COMP1_RMP_W::new(self)
    }
    #[doc = "Bits 2:3 - COMP2_RMP"]
    #[inline(always)]
    pub fn comp2_rmp(&mut self) -> COMP2_RMP_W {
        COMP2_RMP_W::new(self)
    }
    #[doc = "Bits 4:5 - COMP3_RMP"]
    #[inline(always)]
    pub fn comp3_rmp(&mut self) -> COMP3_RMP_W {
        COMP3_RMP_W::new(self)
    }
    #[doc = "Bits 6:7 - COMP4_RMP"]
    #[inline(always)]
    pub fn comp4_rmp(&mut self) -> COMP4_RMP_W {
        COMP4_RMP_W::new(self)
    }
    #[doc = "Bits 8:9 - COMP5_RMP"]
    #[inline(always)]
    pub fn comp5_rmp(&mut self) -> COMP5_RMP_W {
        COMP5_RMP_W::new(self)
    }
    #[doc = "Bits 10:11 - COMP6_RMP"]
    #[inline(always)]
    pub fn comp6_rmp(&mut self) -> COMP6_RMP_W {
        COMP6_RMP_W::new(self)
    }
    #[doc = "Bit 12 - COMP7_RMP"]
    #[inline(always)]
    pub fn comp7_rmp(&mut self) -> COMP7_RMP_W {
        COMP7_RMP_W::new(self)
    }
    #[doc = "Bit 14 - ADC3_ETRI"]
    #[inline(always)]
    pub fn adc3_etri(&mut self) -> ADC3_ETRI_W {
        ADC3_ETRI_W::new(self)
    }
    #[doc = "Bit 15 - ADC3_ETRR"]
    #[inline(always)]
    pub fn adc3_etrr(&mut self) -> ADC3_ETRR_W {
        ADC3_ETRR_W::new(self)
    }
    #[doc = "Bit 16 - ADC4_ETRI"]
    #[inline(always)]
    pub fn adc4_etri(&mut self) -> ADC4_ETRI_W {
        ADC4_ETRI_W::new(self)
    }
    #[doc = "Bit 17 - ADC4_ETRR"]
    #[inline(always)]
    pub fn adc4_etrr(&mut self) -> ADC4_ETRR_W {
        ADC4_ETRR_W::new(self)
    }
    #[doc = "Bit 18 - TSC_OUT_CTRL"]
    #[inline(always)]
    pub fn tsc_out_ctrl(&mut self) -> TSC_OUT_CTRL_W {
        TSC_OUT_CTRL_W::new(self)
    }
    #[doc = "Bit 19 - QSPI_XIP_EN"]
    #[inline(always)]
    pub fn qspi_xip_en(&mut self) -> QSPI_XIP_EN_W {
        QSPI_XIP_EN_W::new(self)
    }
    #[doc = "Bit 22 - SPI1_NSS"]
    #[inline(always)]
    pub fn spi1_nss(&mut self) -> SPI1_NSS_W {
        SPI1_NSS_W::new(self)
    }
    #[doc = "Bit 23 - SPI2_NSS"]
    #[inline(always)]
    pub fn spi2_nss(&mut self) -> SPI2_NSS_W {
        SPI2_NSS_W::new(self)
    }
    #[doc = "Bit 24 - SPI3_NSS"]
    #[inline(always)]
    pub fn spi3_nss(&mut self) -> SPI3_NSS_W {
        SPI3_NSS_W::new(self)
    }
    #[doc = "Bit 25 - QSPI_MISO"]
    #[inline(always)]
    pub fn qspi_miso(&mut self) -> QSPI_MISO_W {
        QSPI_MISO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO_RMP_CFG4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_rmp_cfg4](index.html) module"]
pub struct AFIO_RMP_CFG4_SPEC;
impl crate::RegisterSpec for AFIO_RMP_CFG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afio_rmp_cfg4::R](R) reader structure"]
impl crate::Readable for AFIO_RMP_CFG4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afio_rmp_cfg4::W](W) writer structure"]
impl crate::Writable for AFIO_RMP_CFG4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFIO_RMP_CFG4 to value 0"]
impl crate::Resettable for AFIO_RMP_CFG4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
