#[doc = "Register `PWR_CTRL2` reader"]
pub struct R(crate::R<PWR_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_CTRL2` writer"]
pub struct W(crate::W<PWR_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_CTRL2_SPEC>;
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
impl From<crate::W<PWR_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `__STOP2S` reader - __STOP2S"]
pub type __STOP2S_R = crate::BitReader<bool>;
#[doc = "Field `__STOP2S` writer - __STOP2S"]
pub type __STOP2S_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL2_SPEC, bool, 0>;
#[doc = "Field `SR2VBRET` reader - SR2VBRET"]
pub type SR2VBRET_R = crate::BitReader<bool>;
#[doc = "Field `SR2VBRET` writer - SR2VBRET"]
pub type SR2VBRET_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL2_SPEC, bool, 1>;
#[doc = "Field `SR2STBRET` reader - SR2STBRET"]
pub type SR2STBRET_R = crate::BitReader<bool>;
#[doc = "Field `SR2STBRET` writer - SR2STBRET"]
pub type SR2STBRET_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL2_SPEC, bool, 2>;
#[doc = "Field `TMPWPEN` reader - TMPWPEN"]
pub type TMPWPEN_R = crate::BitReader<bool>;
#[doc = "Field `TMPWPEN` writer - TMPWPEN"]
pub type TMPWPEN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL2_SPEC, bool, 3>;
#[doc = "Field `LSITRIM` reader - LSITRIM"]
pub type LSITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LSITRIM` writer - LSITRIM"]
pub type LSITRIM_W<'a> = crate::FieldWriter<'a, u32, PWR_CTRL2_SPEC, u8, u8, 5, 4>;
#[doc = "Field `IWDGWPEN` reader - IWDGWPEN"]
pub type IWDGWPEN_R = crate::BitReader<bool>;
#[doc = "Field `IWDGWPEN` writer - IWDGWPEN"]
pub type IWDGWPEN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL2_SPEC, bool, 9>;
#[doc = "Field `IWDGRSTEN` reader - IWDGRSTEN"]
pub type IWDGRSTEN_R = crate::BitReader<bool>;
#[doc = "Field `IWDGRSTEN` writer - IWDGRSTEN"]
pub type IWDGRSTEN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL2_SPEC, bool, 10>;
impl R {
    #[doc = "Bit 0 - __STOP2S"]
    #[inline(always)]
    pub fn __stop2s(&self) -> __STOP2S_R {
        __STOP2S_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SR2VBRET"]
    #[inline(always)]
    pub fn sr2vbret(&self) -> SR2VBRET_R {
        SR2VBRET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SR2STBRET"]
    #[inline(always)]
    pub fn sr2stbret(&self) -> SR2STBRET_R {
        SR2STBRET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TMPWPEN"]
    #[inline(always)]
    pub fn tmpwpen(&self) -> TMPWPEN_R {
        TMPWPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:8 - LSITRIM"]
    #[inline(always)]
    pub fn lsitrim(&self) -> LSITRIM_R {
        LSITRIM_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 9 - IWDGWPEN"]
    #[inline(always)]
    pub fn iwdgwpen(&self) -> IWDGWPEN_R {
        IWDGWPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IWDGRSTEN"]
    #[inline(always)]
    pub fn iwdgrsten(&self) -> IWDGRSTEN_R {
        IWDGRSTEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - __STOP2S"]
    #[inline(always)]
    pub fn __stop2s(&mut self) -> __STOP2S_W {
        __STOP2S_W::new(self)
    }
    #[doc = "Bit 1 - SR2VBRET"]
    #[inline(always)]
    pub fn sr2vbret(&mut self) -> SR2VBRET_W {
        SR2VBRET_W::new(self)
    }
    #[doc = "Bit 2 - SR2STBRET"]
    #[inline(always)]
    pub fn sr2stbret(&mut self) -> SR2STBRET_W {
        SR2STBRET_W::new(self)
    }
    #[doc = "Bit 3 - TMPWPEN"]
    #[inline(always)]
    pub fn tmpwpen(&mut self) -> TMPWPEN_W {
        TMPWPEN_W::new(self)
    }
    #[doc = "Bits 4:8 - LSITRIM"]
    #[inline(always)]
    pub fn lsitrim(&mut self) -> LSITRIM_W {
        LSITRIM_W::new(self)
    }
    #[doc = "Bit 9 - IWDGWPEN"]
    #[inline(always)]
    pub fn iwdgwpen(&mut self) -> IWDGWPEN_W {
        IWDGWPEN_W::new(self)
    }
    #[doc = "Bit 10 - IWDGRSTEN"]
    #[inline(always)]
    pub fn iwdgrsten(&mut self) -> IWDGRSTEN_W {
        IWDGRSTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWR_CTRL2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_ctrl2](index.html) module"]
pub struct PWR_CTRL2_SPEC;
impl crate::RegisterSpec for PWR_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_ctrl2::R](R) reader structure"]
impl crate::Readable for PWR_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_ctrl2::W](W) writer structure"]
impl crate::Writable for PWR_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_CTRL2 to value 0x06e4"]
impl crate::Resettable for PWR_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06e4
    }
}
