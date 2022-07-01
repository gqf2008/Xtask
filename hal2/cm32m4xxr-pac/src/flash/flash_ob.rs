#[doc = "Register `FLASH_OB` reader"]
pub struct R(crate::R<FLASH_OB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_OB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_OB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_OB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_OB` writer"]
pub struct W(crate::W<FLASH_OB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_OB_SPEC>;
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
impl From<crate::W<FLASH_OB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_OB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OBERR` reader - OBERR"]
pub type OBERR_R = crate::BitReader<bool>;
#[doc = "Field `OBERR` writer - OBERR"]
pub type OBERR_W<'a> = crate::BitWriter<'a, u32, FLASH_OB_SPEC, bool, 0>;
#[doc = "Field `RDPRT1` reader - RDPRT1"]
pub type RDPRT1_R = crate::BitReader<bool>;
#[doc = "Field `RDPRT1` writer - RDPRT1"]
pub type RDPRT1_W<'a> = crate::BitWriter<'a, u32, FLASH_OB_SPEC, bool, 1>;
#[doc = "Field `WDG_SW` reader - WDG_SW"]
pub type WDG_SW_R = crate::BitReader<bool>;
#[doc = "Field `WDG_SW` writer - WDG_SW"]
pub type WDG_SW_W<'a> = crate::BitWriter<'a, u32, FLASH_OB_SPEC, bool, 2>;
#[doc = "Field `nRST_STOP0` reader - nRST_STOP0"]
pub type NRST_STOP0_R = crate::BitReader<bool>;
#[doc = "Field `nRST_STOP0` writer - nRST_STOP0"]
pub type NRST_STOP0_W<'a> = crate::BitWriter<'a, u32, FLASH_OB_SPEC, bool, 3>;
#[doc = "Field `nRST_STDBY` reader - nRST_STDBY"]
pub type NRST_STDBY_R = crate::BitReader<bool>;
#[doc = "Field `nRST_STDBY` writer - nRST_STDBY"]
pub type NRST_STDBY_W<'a> = crate::BitWriter<'a, u32, FLASH_OB_SPEC, bool, 4>;
#[doc = "Field `Not_Used` reader - Not_Used"]
pub type NOT_USED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Not_Used` writer - Not_Used"]
pub type NOT_USED_W<'a> = crate::FieldWriter<'a, u32, FLASH_OB_SPEC, u8, u8, 5, 5>;
#[doc = "Field `Data0` reader - Data0"]
pub type DATA0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Data0` writer - Data0"]
pub type DATA0_W<'a> = crate::FieldWriter<'a, u32, FLASH_OB_SPEC, u8, u8, 8, 10>;
#[doc = "Field `Data1` reader - Data1"]
pub type DATA1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Data1` writer - Data1"]
pub type DATA1_W<'a> = crate::FieldWriter<'a, u32, FLASH_OB_SPEC, u8, u8, 8, 18>;
#[doc = "Field `RDPRT2` reader - RDPRT2"]
pub type RDPRT2_R = crate::BitReader<bool>;
#[doc = "Field `RDPRT2` writer - RDPRT2"]
pub type RDPRT2_W<'a> = crate::BitWriter<'a, u32, FLASH_OB_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - OBERR"]
    #[inline(always)]
    pub fn oberr(&self) -> OBERR_R {
        OBERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RDPRT1"]
    #[inline(always)]
    pub fn rdprt1(&self) -> RDPRT1_R {
        RDPRT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WDG_SW"]
    #[inline(always)]
    pub fn wdg_sw(&self) -> WDG_SW_R {
        WDG_SW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - nRST_STOP0"]
    #[inline(always)]
    pub fn n_rst_stop0(&self) -> NRST_STOP0_R {
        NRST_STOP0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> NRST_STDBY_R {
        NRST_STDBY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:9 - Not_Used"]
    #[inline(always)]
    pub fn not_used(&self) -> NOT_USED_R {
        NOT_USED_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:17 - Data0"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bits 18:25 - Data1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bit 31 - RDPRT2"]
    #[inline(always)]
    pub fn rdprt2(&self) -> RDPRT2_R {
        RDPRT2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OBERR"]
    #[inline(always)]
    pub fn oberr(&mut self) -> OBERR_W {
        OBERR_W::new(self)
    }
    #[doc = "Bit 1 - RDPRT1"]
    #[inline(always)]
    pub fn rdprt1(&mut self) -> RDPRT1_W {
        RDPRT1_W::new(self)
    }
    #[doc = "Bit 2 - WDG_SW"]
    #[inline(always)]
    pub fn wdg_sw(&mut self) -> WDG_SW_W {
        WDG_SW_W::new(self)
    }
    #[doc = "Bit 3 - nRST_STOP0"]
    #[inline(always)]
    pub fn n_rst_stop0(&mut self) -> NRST_STOP0_W {
        NRST_STOP0_W::new(self)
    }
    #[doc = "Bit 4 - nRST_STDBY"]
    #[inline(always)]
    pub fn n_rst_stdby(&mut self) -> NRST_STDBY_W {
        NRST_STDBY_W::new(self)
    }
    #[doc = "Bits 5:9 - Not_Used"]
    #[inline(always)]
    pub fn not_used(&mut self) -> NOT_USED_W {
        NOT_USED_W::new(self)
    }
    #[doc = "Bits 10:17 - Data0"]
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W {
        DATA0_W::new(self)
    }
    #[doc = "Bits 18:25 - Data1"]
    #[inline(always)]
    pub fn data1(&mut self) -> DATA1_W {
        DATA1_W::new(self)
    }
    #[doc = "Bit 31 - RDPRT2"]
    #[inline(always)]
    pub fn rdprt2(&mut self) -> RDPRT2_W {
        RDPRT2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH_OB\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ob](index.html) module"]
pub struct FLASH_OB_SPEC;
impl crate::RegisterSpec for FLASH_OB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_ob::R](R) reader structure"]
impl crate::Readable for FLASH_OB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_ob::W](W) writer structure"]
impl crate::Writable for FLASH_OB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_OB to value 0x03ff_fffc"]
impl crate::Resettable for FLASH_OB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03ff_fffc
    }
}
