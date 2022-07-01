#[doc = "Register `QSPI_MW_CTRL` reader"]
pub struct R(crate::R<QSPI_MW_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_MW_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_MW_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_MW_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_MW_CTRL` writer"]
pub struct W(crate::W<QSPI_MW_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_MW_CTRL_SPEC>;
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
impl From<crate::W<QSPI_MW_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_MW_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MWMOD` reader - MWMOD"]
pub type MWMOD_R = crate::BitReader<bool>;
#[doc = "Field `MWMOD` writer - MWMOD"]
pub type MWMOD_W<'a> = crate::BitWriter<'a, u32, QSPI_MW_CTRL_SPEC, bool, 0>;
#[doc = "Field `MC_DIR` reader - MC_DIR"]
pub type MC_DIR_R = crate::BitReader<bool>;
#[doc = "Field `MC_DIR` writer - MC_DIR"]
pub type MC_DIR_W<'a> = crate::BitWriter<'a, u32, QSPI_MW_CTRL_SPEC, bool, 1>;
#[doc = "Field `MHS_EN` reader - MHS_EN"]
pub type MHS_EN_R = crate::BitReader<bool>;
#[doc = "Field `MHS_EN` writer - MHS_EN"]
pub type MHS_EN_W<'a> = crate::BitWriter<'a, u32, QSPI_MW_CTRL_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 0 - MWMOD"]
    #[inline(always)]
    pub fn mwmod(&self) -> MWMOD_R {
        MWMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MC_DIR"]
    #[inline(always)]
    pub fn mc_dir(&self) -> MC_DIR_R {
        MC_DIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MHS_EN"]
    #[inline(always)]
    pub fn mhs_en(&self) -> MHS_EN_R {
        MHS_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MWMOD"]
    #[inline(always)]
    pub fn mwmod(&mut self) -> MWMOD_W {
        MWMOD_W::new(self)
    }
    #[doc = "Bit 1 - MC_DIR"]
    #[inline(always)]
    pub fn mc_dir(&mut self) -> MC_DIR_W {
        MC_DIR_W::new(self)
    }
    #[doc = "Bit 2 - MHS_EN"]
    #[inline(always)]
    pub fn mhs_en(&mut self) -> MHS_EN_W {
        MHS_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI_MW_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_mw_ctrl](index.html) module"]
pub struct QSPI_MW_CTRL_SPEC;
impl crate::RegisterSpec for QSPI_MW_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_mw_ctrl::R](R) reader structure"]
impl crate::Readable for QSPI_MW_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_mw_ctrl::W](W) writer structure"]
impl crate::Writable for QSPI_MW_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_MW_CTRL to value 0"]
impl crate::Resettable for QSPI_MW_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
