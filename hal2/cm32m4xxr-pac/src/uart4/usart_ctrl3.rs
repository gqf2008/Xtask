#[doc = "Register `USART_CTRL3` reader"]
pub struct R(crate::R<USART_CTRL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USART_CTRL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USART_CTRL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USART_CTRL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USART_CTRL3` writer"]
pub struct W(crate::W<USART_CTRL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USART_CTRL3_SPEC>;
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
impl From<crate::W<USART_CTRL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USART_CTRL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERRIEN` reader - ERRIEN"]
pub type ERRIEN_R = crate::BitReader<bool>;
#[doc = "Field `ERRIEN` writer - ERRIEN"]
pub type ERRIEN_W<'a> = crate::BitWriter<'a, u32, USART_CTRL3_SPEC, bool, 0>;
#[doc = "Field `IRDAMEN` reader - IRDAMEN"]
pub type IRDAMEN_R = crate::BitReader<bool>;
#[doc = "Field `IRDAMEN` writer - IRDAMEN"]
pub type IRDAMEN_W<'a> = crate::BitWriter<'a, u32, USART_CTRL3_SPEC, bool, 1>;
#[doc = "Field `IRDALP` reader - IRDALP"]
pub type IRDALP_R = crate::BitReader<bool>;
#[doc = "Field `IRDALP` writer - IRDALP"]
pub type IRDALP_W<'a> = crate::BitWriter<'a, u32, USART_CTRL3_SPEC, bool, 2>;
#[doc = "Field `HDMEN` reader - HDMEN"]
pub type HDMEN_R = crate::BitReader<bool>;
#[doc = "Field `HDMEN` writer - HDMEN"]
pub type HDMEN_W<'a> = crate::BitWriter<'a, u32, USART_CTRL3_SPEC, bool, 3>;
#[doc = "Field `SCNACK` reader - SCNACK"]
pub type SCNACK_R = crate::BitReader<bool>;
#[doc = "Field `SCNACK` writer - SCNACK"]
pub type SCNACK_W<'a> = crate::BitWriter<'a, u32, USART_CTRL3_SPEC, bool, 4>;
#[doc = "Field `SCMEN` reader - SCMEN"]
pub type SCMEN_R = crate::BitReader<bool>;
#[doc = "Field `SCMEN` writer - SCMEN"]
pub type SCMEN_W<'a> = crate::BitWriter<'a, u32, USART_CTRL3_SPEC, bool, 5>;
#[doc = "Field `DMARXEN` reader - DMARXEN"]
pub type DMARXEN_R = crate::BitReader<bool>;
#[doc = "Field `DMARXEN` writer - DMARXEN"]
pub type DMARXEN_W<'a> = crate::BitWriter<'a, u32, USART_CTRL3_SPEC, bool, 6>;
#[doc = "Field `DMATXEN` reader - DMATXEN"]
pub type DMATXEN_R = crate::BitReader<bool>;
#[doc = "Field `DMATXEN` writer - DMATXEN"]
pub type DMATXEN_W<'a> = crate::BitWriter<'a, u32, USART_CTRL3_SPEC, bool, 7>;
#[doc = "Field `RTSEN` reader - RTSEN"]
pub type RTSEN_R = crate::BitReader<bool>;
#[doc = "Field `RTSEN` writer - RTSEN"]
pub type RTSEN_W<'a> = crate::BitWriter<'a, u32, USART_CTRL3_SPEC, bool, 8>;
#[doc = "Field `CTSEN` reader - CTSEN"]
pub type CTSEN_R = crate::BitReader<bool>;
#[doc = "Field `CTSEN` writer - CTSEN"]
pub type CTSEN_W<'a> = crate::BitWriter<'a, u32, USART_CTRL3_SPEC, bool, 9>;
#[doc = "Field `CTSIEN` reader - CTSIEN"]
pub type CTSIEN_R = crate::BitReader<bool>;
#[doc = "Field `CTSIEN` writer - CTSIEN"]
pub type CTSIEN_W<'a> = crate::BitWriter<'a, u32, USART_CTRL3_SPEC, bool, 10>;
impl R {
    #[doc = "Bit 0 - ERRIEN"]
    #[inline(always)]
    pub fn errien(&self) -> ERRIEN_R {
        ERRIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRDAMEN"]
    #[inline(always)]
    pub fn irdamen(&self) -> IRDAMEN_R {
        IRDAMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IRDALP"]
    #[inline(always)]
    pub fn irdalp(&self) -> IRDALP_R {
        IRDALP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HDMEN"]
    #[inline(always)]
    pub fn hdmen(&self) -> HDMEN_R {
        HDMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCNACK"]
    #[inline(always)]
    pub fn scnack(&self) -> SCNACK_R {
        SCNACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCMEN"]
    #[inline(always)]
    pub fn scmen(&self) -> SCMEN_R {
        SCMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMARXEN"]
    #[inline(always)]
    pub fn dmarxen(&self) -> DMARXEN_R {
        DMARXEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMATXEN"]
    #[inline(always)]
    pub fn dmatxen(&self) -> DMATXEN_R {
        DMATXEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTSEN"]
    #[inline(always)]
    pub fn rtsen(&self) -> RTSEN_R {
        RTSEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTSEN"]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTSIEN"]
    #[inline(always)]
    pub fn ctsien(&self) -> CTSIEN_R {
        CTSIEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ERRIEN"]
    #[inline(always)]
    pub fn errien(&mut self) -> ERRIEN_W {
        ERRIEN_W::new(self)
    }
    #[doc = "Bit 1 - IRDAMEN"]
    #[inline(always)]
    pub fn irdamen(&mut self) -> IRDAMEN_W {
        IRDAMEN_W::new(self)
    }
    #[doc = "Bit 2 - IRDALP"]
    #[inline(always)]
    pub fn irdalp(&mut self) -> IRDALP_W {
        IRDALP_W::new(self)
    }
    #[doc = "Bit 3 - HDMEN"]
    #[inline(always)]
    pub fn hdmen(&mut self) -> HDMEN_W {
        HDMEN_W::new(self)
    }
    #[doc = "Bit 4 - SCNACK"]
    #[inline(always)]
    pub fn scnack(&mut self) -> SCNACK_W {
        SCNACK_W::new(self)
    }
    #[doc = "Bit 5 - SCMEN"]
    #[inline(always)]
    pub fn scmen(&mut self) -> SCMEN_W {
        SCMEN_W::new(self)
    }
    #[doc = "Bit 6 - DMARXEN"]
    #[inline(always)]
    pub fn dmarxen(&mut self) -> DMARXEN_W {
        DMARXEN_W::new(self)
    }
    #[doc = "Bit 7 - DMATXEN"]
    #[inline(always)]
    pub fn dmatxen(&mut self) -> DMATXEN_W {
        DMATXEN_W::new(self)
    }
    #[doc = "Bit 8 - RTSEN"]
    #[inline(always)]
    pub fn rtsen(&mut self) -> RTSEN_W {
        RTSEN_W::new(self)
    }
    #[doc = "Bit 9 - CTSEN"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CTSEN_W {
        CTSEN_W::new(self)
    }
    #[doc = "Bit 10 - CTSIEN"]
    #[inline(always)]
    pub fn ctsien(&mut self) -> CTSIEN_W {
        CTSIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART_CTRL3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_ctrl3](index.html) module"]
pub struct USART_CTRL3_SPEC;
impl crate::RegisterSpec for USART_CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usart_ctrl3::R](R) reader structure"]
impl crate::Readable for USART_CTRL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usart_ctrl3::W](W) writer structure"]
impl crate::Writable for USART_CTRL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USART_CTRL3 to value 0"]
impl crate::Resettable for USART_CTRL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
