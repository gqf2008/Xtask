#[doc = "Register `XIP_CTRL` reader"]
pub struct R(crate::R<XIP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XIP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XIP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XIP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XIP_CTRL` writer"]
pub struct W(crate::W<XIP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XIP_CTRL_SPEC>;
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
impl From<crate::W<XIP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XIP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRF` reader - FRF"]
pub type FRF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRF` writer - FRF"]
pub type FRF_W<'a> = crate::FieldWriter<'a, u32, XIP_CTRL_SPEC, u8, u8, 2, 0>;
#[doc = "Field `TRANS_TYPE` reader - TRANS_TYPE"]
pub type TRANS_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRANS_TYPE` writer - TRANS_TYPE"]
pub type TRANS_TYPE_W<'a> = crate::FieldWriter<'a, u32, XIP_CTRL_SPEC, u8, u8, 2, 2>;
#[doc = "Field `ADDR_LEN` reader - ADDR_LEN"]
pub type ADDR_LEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR_LEN` writer - ADDR_LEN"]
pub type ADDR_LEN_W<'a> = crate::FieldWriter<'a, u32, XIP_CTRL_SPEC, u8, u8, 4, 4>;
#[doc = "Field `INST_L` reader - INST_L"]
pub type INST_L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INST_L` writer - INST_L"]
pub type INST_L_W<'a> = crate::FieldWriter<'a, u32, XIP_CTRL_SPEC, u8, u8, 2, 9>;
#[doc = "Field `MD_BITS_EN` reader - MD_BITS_EN"]
pub type MD_BITS_EN_R = crate::BitReader<bool>;
#[doc = "Field `MD_BITS_EN` writer - MD_BITS_EN"]
pub type MD_BITS_EN_W<'a> = crate::BitWriter<'a, u32, XIP_CTRL_SPEC, bool, 12>;
#[doc = "Field `WAIT_CYCLES` reader - WAIT_CYCLES"]
pub type WAIT_CYCLES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAIT_CYCLES` writer - WAIT_CYCLES"]
pub type WAIT_CYCLES_W<'a> = crate::FieldWriter<'a, u32, XIP_CTRL_SPEC, u8, u8, 5, 13>;
#[doc = "Field `DFS_HC` reader - DFS_HC"]
pub type DFS_HC_R = crate::BitReader<bool>;
#[doc = "Field `DFS_HC` writer - DFS_HC"]
pub type DFS_HC_W<'a> = crate::BitWriter<'a, u32, XIP_CTRL_SPEC, bool, 18>;
#[doc = "Field `DDR_EN` reader - DDR_EN"]
pub type DDR_EN_R = crate::BitReader<bool>;
#[doc = "Field `DDR_EN` writer - DDR_EN"]
pub type DDR_EN_W<'a> = crate::BitWriter<'a, u32, XIP_CTRL_SPEC, bool, 19>;
#[doc = "Field `INST_DDR_EN` reader - INST_DDR_EN"]
pub type INST_DDR_EN_R = crate::BitReader<bool>;
#[doc = "Field `INST_DDR_EN` writer - INST_DDR_EN"]
pub type INST_DDR_EN_W<'a> = crate::BitWriter<'a, u32, XIP_CTRL_SPEC, bool, 20>;
#[doc = "Field `XIP_INST_EN` reader - XIP_INST_EN"]
pub type XIP_INST_EN_R = crate::BitReader<bool>;
#[doc = "Field `XIP_INST_EN` writer - XIP_INST_EN"]
pub type XIP_INST_EN_W<'a> = crate::BitWriter<'a, u32, XIP_CTRL_SPEC, bool, 22>;
#[doc = "Field `XIP_CT_EN` reader - XIP_CT_EN"]
pub type XIP_CT_EN_R = crate::BitReader<bool>;
#[doc = "Field `XIP_CT_EN` writer - XIP_CT_EN"]
pub type XIP_CT_EN_W<'a> = crate::BitWriter<'a, u32, XIP_CTRL_SPEC, bool, 23>;
#[doc = "Field `XIP_MBL` reader - XIP_MBL"]
pub type XIP_MBL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XIP_MBL` writer - XIP_MBL"]
pub type XIP_MBL_W<'a> = crate::FieldWriter<'a, u32, XIP_CTRL_SPEC, u8, u8, 2, 26>;
impl R {
    #[doc = "Bits 0:1 - FRF"]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - TRANS_TYPE"]
    #[inline(always)]
    pub fn trans_type(&self) -> TRANS_TYPE_R {
        TRANS_TYPE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - ADDR_LEN"]
    #[inline(always)]
    pub fn addr_len(&self) -> ADDR_LEN_R {
        ADDR_LEN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 9:10 - INST_L"]
    #[inline(always)]
    pub fn inst_l(&self) -> INST_L_R {
        INST_L_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 12 - MD_BITS_EN"]
    #[inline(always)]
    pub fn md_bits_en(&self) -> MD_BITS_EN_R {
        MD_BITS_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:17 - WAIT_CYCLES"]
    #[inline(always)]
    pub fn wait_cycles(&self) -> WAIT_CYCLES_R {
        WAIT_CYCLES_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bit 18 - DFS_HC"]
    #[inline(always)]
    pub fn dfs_hc(&self) -> DFS_HC_R {
        DFS_HC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DDR_EN"]
    #[inline(always)]
    pub fn ddr_en(&self) -> DDR_EN_R {
        DDR_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - INST_DDR_EN"]
    #[inline(always)]
    pub fn inst_ddr_en(&self) -> INST_DDR_EN_R {
        INST_DDR_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - XIP_INST_EN"]
    #[inline(always)]
    pub fn xip_inst_en(&self) -> XIP_INST_EN_R {
        XIP_INST_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - XIP_CT_EN"]
    #[inline(always)]
    pub fn xip_ct_en(&self) -> XIP_CT_EN_R {
        XIP_CT_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 26:27 - XIP_MBL"]
    #[inline(always)]
    pub fn xip_mbl(&self) -> XIP_MBL_R {
        XIP_MBL_R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - FRF"]
    #[inline(always)]
    pub fn frf(&mut self) -> FRF_W {
        FRF_W::new(self)
    }
    #[doc = "Bits 2:3 - TRANS_TYPE"]
    #[inline(always)]
    pub fn trans_type(&mut self) -> TRANS_TYPE_W {
        TRANS_TYPE_W::new(self)
    }
    #[doc = "Bits 4:7 - ADDR_LEN"]
    #[inline(always)]
    pub fn addr_len(&mut self) -> ADDR_LEN_W {
        ADDR_LEN_W::new(self)
    }
    #[doc = "Bits 9:10 - INST_L"]
    #[inline(always)]
    pub fn inst_l(&mut self) -> INST_L_W {
        INST_L_W::new(self)
    }
    #[doc = "Bit 12 - MD_BITS_EN"]
    #[inline(always)]
    pub fn md_bits_en(&mut self) -> MD_BITS_EN_W {
        MD_BITS_EN_W::new(self)
    }
    #[doc = "Bits 13:17 - WAIT_CYCLES"]
    #[inline(always)]
    pub fn wait_cycles(&mut self) -> WAIT_CYCLES_W {
        WAIT_CYCLES_W::new(self)
    }
    #[doc = "Bit 18 - DFS_HC"]
    #[inline(always)]
    pub fn dfs_hc(&mut self) -> DFS_HC_W {
        DFS_HC_W::new(self)
    }
    #[doc = "Bit 19 - DDR_EN"]
    #[inline(always)]
    pub fn ddr_en(&mut self) -> DDR_EN_W {
        DDR_EN_W::new(self)
    }
    #[doc = "Bit 20 - INST_DDR_EN"]
    #[inline(always)]
    pub fn inst_ddr_en(&mut self) -> INST_DDR_EN_W {
        INST_DDR_EN_W::new(self)
    }
    #[doc = "Bit 22 - XIP_INST_EN"]
    #[inline(always)]
    pub fn xip_inst_en(&mut self) -> XIP_INST_EN_W {
        XIP_INST_EN_W::new(self)
    }
    #[doc = "Bit 23 - XIP_CT_EN"]
    #[inline(always)]
    pub fn xip_ct_en(&mut self) -> XIP_CT_EN_W {
        XIP_CT_EN_W::new(self)
    }
    #[doc = "Bits 26:27 - XIP_MBL"]
    #[inline(always)]
    pub fn xip_mbl(&mut self) -> XIP_MBL_W {
        XIP_MBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XIP_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xip_ctrl](index.html) module"]
pub struct XIP_CTRL_SPEC;
impl crate::RegisterSpec for XIP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xip_ctrl::R](R) reader structure"]
impl crate::Readable for XIP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xip_ctrl::W](W) writer structure"]
impl crate::Writable for XIP_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XIP_CTRL to value 0"]
impl crate::Resettable for XIP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
