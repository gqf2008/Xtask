#[doc = "Register `QSPI_ENH_CTRL0` reader"]
pub struct R(crate::R<QSPI_ENH_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_ENH_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_ENH_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_ENH_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_ENH_CTRL0` writer"]
pub struct W(crate::W<QSPI_ENH_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_ENH_CTRL0_SPEC>;
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
impl From<crate::W<QSPI_ENH_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_ENH_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRANS_TYPE` reader - TRANS_TYPE"]
pub type TRANS_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRANS_TYPE` writer - TRANS_TYPE"]
pub type TRANS_TYPE_W<'a> = crate::FieldWriter<'a, u32, QSPI_ENH_CTRL0_SPEC, u8, u8, 2, 0>;
#[doc = "Field `ADDR_LEN` reader - ADDR_LEN"]
pub type ADDR_LEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR_LEN` writer - ADDR_LEN"]
pub type ADDR_LEN_W<'a> = crate::FieldWriter<'a, u32, QSPI_ENH_CTRL0_SPEC, u8, u8, 4, 2>;
#[doc = "Field `MD_BIT_EN` reader - MD_BIT_EN"]
pub type MD_BIT_EN_R = crate::BitReader<bool>;
#[doc = "Field `MD_BIT_EN` writer - MD_BIT_EN"]
pub type MD_BIT_EN_W<'a> = crate::BitWriter<'a, u32, QSPI_ENH_CTRL0_SPEC, bool, 7>;
#[doc = "Field `INST_L` reader - INST_L"]
pub type INST_L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INST_L` writer - INST_L"]
pub type INST_L_W<'a> = crate::FieldWriter<'a, u32, QSPI_ENH_CTRL0_SPEC, u8, u8, 2, 8>;
#[doc = "Field `WAIT_CYCLES` reader - WAIT_CYCLES"]
pub type WAIT_CYCLES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAIT_CYCLES` writer - WAIT_CYCLES"]
pub type WAIT_CYCLES_W<'a> = crate::FieldWriter<'a, u32, QSPI_ENH_CTRL0_SPEC, u8, u8, 5, 11>;
#[doc = "Field `SPI_DDR_EN` reader - SPI_DDR_EN"]
pub type SPI_DDR_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI_DDR_EN` writer - SPI_DDR_EN"]
pub type SPI_DDR_EN_W<'a> = crate::BitWriter<'a, u32, QSPI_ENH_CTRL0_SPEC, bool, 16>;
#[doc = "Field `INST_DDR_EN` reader - INST_DDR_EN"]
pub type INST_DDR_EN_R = crate::BitReader<bool>;
#[doc = "Field `INST_DDR_EN` writer - INST_DDR_EN"]
pub type INST_DDR_EN_W<'a> = crate::BitWriter<'a, u32, QSPI_ENH_CTRL0_SPEC, bool, 17>;
#[doc = "Field `XIP_DFS_HC` reader - XIP_DFS_HC"]
pub type XIP_DFS_HC_R = crate::BitReader<bool>;
#[doc = "Field `XIP_DFS_HC` writer - XIP_DFS_HC"]
pub type XIP_DFS_HC_W<'a> = crate::BitWriter<'a, u32, QSPI_ENH_CTRL0_SPEC, bool, 19>;
#[doc = "Field `XIP_INST_EN` reader - XIP_INST_EN"]
pub type XIP_INST_EN_R = crate::BitReader<bool>;
#[doc = "Field `XIP_INST_EN` writer - XIP_INST_EN"]
pub type XIP_INST_EN_W<'a> = crate::BitWriter<'a, u32, QSPI_ENH_CTRL0_SPEC, bool, 20>;
#[doc = "Field `XIP_CT_EN` reader - XIP_CT_EN"]
pub type XIP_CT_EN_R = crate::BitReader<bool>;
#[doc = "Field `XIP_CT_EN` writer - XIP_CT_EN"]
pub type XIP_CT_EN_W<'a> = crate::BitWriter<'a, u32, QSPI_ENH_CTRL0_SPEC, bool, 21>;
#[doc = "Field `XIP_MBL` reader - XIP_MBL"]
pub type XIP_MBL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XIP_MBL` writer - XIP_MBL"]
pub type XIP_MBL_W<'a> = crate::FieldWriter<'a, u32, QSPI_ENH_CTRL0_SPEC, u8, u8, 2, 26>;
#[doc = "Field `CLK_STRETCH_EN` reader - CLK_STRETCH_EN"]
pub type CLK_STRETCH_EN_R = crate::BitReader<bool>;
#[doc = "Field `CLK_STRETCH_EN` writer - CLK_STRETCH_EN"]
pub type CLK_STRETCH_EN_W<'a> = crate::BitWriter<'a, u32, QSPI_ENH_CTRL0_SPEC, bool, 30>;
impl R {
    #[doc = "Bits 0:1 - TRANS_TYPE"]
    #[inline(always)]
    pub fn trans_type(&self) -> TRANS_TYPE_R {
        TRANS_TYPE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - ADDR_LEN"]
    #[inline(always)]
    pub fn addr_len(&self) -> ADDR_LEN_R {
        ADDR_LEN_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - MD_BIT_EN"]
    #[inline(always)]
    pub fn md_bit_en(&self) -> MD_BIT_EN_R {
        MD_BIT_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - INST_L"]
    #[inline(always)]
    pub fn inst_l(&self) -> INST_L_R {
        INST_L_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 11:15 - WAIT_CYCLES"]
    #[inline(always)]
    pub fn wait_cycles(&self) -> WAIT_CYCLES_R {
        WAIT_CYCLES_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - SPI_DDR_EN"]
    #[inline(always)]
    pub fn spi_ddr_en(&self) -> SPI_DDR_EN_R {
        SPI_DDR_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - INST_DDR_EN"]
    #[inline(always)]
    pub fn inst_ddr_en(&self) -> INST_DDR_EN_R {
        INST_DDR_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - XIP_DFS_HC"]
    #[inline(always)]
    pub fn xip_dfs_hc(&self) -> XIP_DFS_HC_R {
        XIP_DFS_HC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - XIP_INST_EN"]
    #[inline(always)]
    pub fn xip_inst_en(&self) -> XIP_INST_EN_R {
        XIP_INST_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - XIP_CT_EN"]
    #[inline(always)]
    pub fn xip_ct_en(&self) -> XIP_CT_EN_R {
        XIP_CT_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 26:27 - XIP_MBL"]
    #[inline(always)]
    pub fn xip_mbl(&self) -> XIP_MBL_R {
        XIP_MBL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 30 - CLK_STRETCH_EN"]
    #[inline(always)]
    pub fn clk_stretch_en(&self) -> CLK_STRETCH_EN_R {
        CLK_STRETCH_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - TRANS_TYPE"]
    #[inline(always)]
    pub fn trans_type(&mut self) -> TRANS_TYPE_W {
        TRANS_TYPE_W::new(self)
    }
    #[doc = "Bits 2:5 - ADDR_LEN"]
    #[inline(always)]
    pub fn addr_len(&mut self) -> ADDR_LEN_W {
        ADDR_LEN_W::new(self)
    }
    #[doc = "Bit 7 - MD_BIT_EN"]
    #[inline(always)]
    pub fn md_bit_en(&mut self) -> MD_BIT_EN_W {
        MD_BIT_EN_W::new(self)
    }
    #[doc = "Bits 8:9 - INST_L"]
    #[inline(always)]
    pub fn inst_l(&mut self) -> INST_L_W {
        INST_L_W::new(self)
    }
    #[doc = "Bits 11:15 - WAIT_CYCLES"]
    #[inline(always)]
    pub fn wait_cycles(&mut self) -> WAIT_CYCLES_W {
        WAIT_CYCLES_W::new(self)
    }
    #[doc = "Bit 16 - SPI_DDR_EN"]
    #[inline(always)]
    pub fn spi_ddr_en(&mut self) -> SPI_DDR_EN_W {
        SPI_DDR_EN_W::new(self)
    }
    #[doc = "Bit 17 - INST_DDR_EN"]
    #[inline(always)]
    pub fn inst_ddr_en(&mut self) -> INST_DDR_EN_W {
        INST_DDR_EN_W::new(self)
    }
    #[doc = "Bit 19 - XIP_DFS_HC"]
    #[inline(always)]
    pub fn xip_dfs_hc(&mut self) -> XIP_DFS_HC_W {
        XIP_DFS_HC_W::new(self)
    }
    #[doc = "Bit 20 - XIP_INST_EN"]
    #[inline(always)]
    pub fn xip_inst_en(&mut self) -> XIP_INST_EN_W {
        XIP_INST_EN_W::new(self)
    }
    #[doc = "Bit 21 - XIP_CT_EN"]
    #[inline(always)]
    pub fn xip_ct_en(&mut self) -> XIP_CT_EN_W {
        XIP_CT_EN_W::new(self)
    }
    #[doc = "Bits 26:27 - XIP_MBL"]
    #[inline(always)]
    pub fn xip_mbl(&mut self) -> XIP_MBL_W {
        XIP_MBL_W::new(self)
    }
    #[doc = "Bit 30 - CLK_STRETCH_EN"]
    #[inline(always)]
    pub fn clk_stretch_en(&mut self) -> CLK_STRETCH_EN_W {
        CLK_STRETCH_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI_ENH_CTRL0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_enh_ctrl0](index.html) module"]
pub struct QSPI_ENH_CTRL0_SPEC;
impl crate::RegisterSpec for QSPI_ENH_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_enh_ctrl0::R](R) reader structure"]
impl crate::Readable for QSPI_ENH_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_enh_ctrl0::W](W) writer structure"]
impl crate::Writable for QSPI_ENH_CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_ENH_CTRL0 to value 0x0002_0000"]
impl crate::Resettable for QSPI_ENH_CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_0000
    }
}
