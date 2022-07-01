#[doc = "Register `RTC_INITSTS` reader"]
pub struct R(crate::R<RTC_INITSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_INITSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_INITSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_INITSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_INITSTS` writer"]
pub struct W(crate::W<RTC_INITSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_INITSTS_SPEC>;
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
impl From<crate::W<RTC_INITSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_INITSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALAWF` reader - ALAWF"]
pub type ALAWF_R = crate::BitReader<bool>;
#[doc = "Field `ALAWF` writer - ALAWF"]
pub type ALAWF_W<'a> = crate::BitWriter<'a, u32, RTC_INITSTS_SPEC, bool, 0>;
#[doc = "Field `ALBWF` reader - ALBWF"]
pub type ALBWF_R = crate::BitReader<bool>;
#[doc = "Field `ALBWF` writer - ALBWF"]
pub type ALBWF_W<'a> = crate::BitWriter<'a, u32, RTC_INITSTS_SPEC, bool, 1>;
#[doc = "Field `WTWF` reader - WTWF"]
pub type WTWF_R = crate::BitReader<bool>;
#[doc = "Field `WTWF` writer - WTWF"]
pub type WTWF_W<'a> = crate::BitWriter<'a, u32, RTC_INITSTS_SPEC, bool, 2>;
#[doc = "Field `SHOPF` reader - SHOPF"]
pub type SHOPF_R = crate::BitReader<bool>;
#[doc = "Field `SHOPF` writer - SHOPF"]
pub type SHOPF_W<'a> = crate::BitWriter<'a, u32, RTC_INITSTS_SPEC, bool, 3>;
#[doc = "Field `INITSF` reader - INITSF"]
pub type INITSF_R = crate::BitReader<bool>;
#[doc = "Field `INITSF` writer - INITSF"]
pub type INITSF_W<'a> = crate::BitWriter<'a, u32, RTC_INITSTS_SPEC, bool, 4>;
#[doc = "Field `RSYF` reader - RSYF"]
pub type RSYF_R = crate::BitReader<bool>;
#[doc = "Field `RSYF` writer - RSYF"]
pub type RSYF_W<'a> = crate::BitWriter<'a, u32, RTC_INITSTS_SPEC, bool, 5>;
#[doc = "Field `INITF` reader - INITF"]
pub type INITF_R = crate::BitReader<bool>;
#[doc = "Field `INITF` writer - INITF"]
pub type INITF_W<'a> = crate::BitWriter<'a, u32, RTC_INITSTS_SPEC, bool, 6>;
#[doc = "Field `INITM` reader - INITM"]
pub type INITM_R = crate::BitReader<bool>;
#[doc = "Field `INITM` writer - INITM"]
pub type INITM_W<'a> = crate::BitWriter<'a, u32, RTC_INITSTS_SPEC, bool, 7>;
#[doc = "Field `ALAF` reader - ALAF"]
pub type ALAF_R = crate::BitReader<bool>;
#[doc = "Field `ALAF` writer - ALAF"]
pub type ALAF_W<'a> = crate::BitWriter<'a, u32, RTC_INITSTS_SPEC, bool, 8>;
#[doc = "Field `ALBF` reader - ALBF"]
pub type ALBF_R = crate::BitReader<bool>;
#[doc = "Field `ALBF` writer - ALBF"]
pub type ALBF_W<'a> = crate::BitWriter<'a, u32, RTC_INITSTS_SPEC, bool, 9>;
#[doc = "Field `WTF` reader - WTF"]
pub type WTF_R = crate::BitReader<bool>;
#[doc = "Field `WTF` writer - WTF"]
pub type WTF_W<'a> = crate::BitWriter<'a, u32, RTC_INITSTS_SPEC, bool, 10>;
#[doc = "Field `TISF` reader - TISF"]
pub type TISF_R = crate::BitReader<bool>;
#[doc = "Field `TISF` writer - TISF"]
pub type TISF_W<'a> = crate::BitWriter<'a, u32, RTC_INITSTS_SPEC, bool, 11>;
#[doc = "Field `TISOVF` reader - TISOVF"]
pub type TISOVF_R = crate::BitReader<bool>;
#[doc = "Field `TISOVF` writer - TISOVF"]
pub type TISOVF_W<'a> = crate::BitWriter<'a, u32, RTC_INITSTS_SPEC, bool, 12>;
#[doc = "Field `RECPF` reader - RECPF"]
pub type RECPF_R = crate::BitReader<bool>;
#[doc = "Field `RECPF` writer - RECPF"]
pub type RECPF_W<'a> = crate::BitWriter<'a, u32, RTC_INITSTS_SPEC, bool, 16>;
impl R {
    #[doc = "Bit 0 - ALAWF"]
    #[inline(always)]
    pub fn alawf(&self) -> ALAWF_R {
        ALAWF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ALBWF"]
    #[inline(always)]
    pub fn albwf(&self) -> ALBWF_R {
        ALBWF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WTWF"]
    #[inline(always)]
    pub fn wtwf(&self) -> WTWF_R {
        WTWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SHOPF"]
    #[inline(always)]
    pub fn shopf(&self) -> SHOPF_R {
        SHOPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - INITSF"]
    #[inline(always)]
    pub fn initsf(&self) -> INITSF_R {
        INITSF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RSYF"]
    #[inline(always)]
    pub fn rsyf(&self) -> RSYF_R {
        RSYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - INITF"]
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - INITM"]
    #[inline(always)]
    pub fn initm(&self) -> INITM_R {
        INITM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ALAF"]
    #[inline(always)]
    pub fn alaf(&self) -> ALAF_R {
        ALAF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ALBF"]
    #[inline(always)]
    pub fn albf(&self) -> ALBF_R {
        ALBF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - WTF"]
    #[inline(always)]
    pub fn wtf(&self) -> WTF_R {
        WTF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TISF"]
    #[inline(always)]
    pub fn tisf(&self) -> TISF_R {
        TISF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TISOVF"]
    #[inline(always)]
    pub fn tisovf(&self) -> TISOVF_R {
        TISOVF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - RECPF"]
    #[inline(always)]
    pub fn recpf(&self) -> RECPF_R {
        RECPF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ALAWF"]
    #[inline(always)]
    pub fn alawf(&mut self) -> ALAWF_W {
        ALAWF_W::new(self)
    }
    #[doc = "Bit 1 - ALBWF"]
    #[inline(always)]
    pub fn albwf(&mut self) -> ALBWF_W {
        ALBWF_W::new(self)
    }
    #[doc = "Bit 2 - WTWF"]
    #[inline(always)]
    pub fn wtwf(&mut self) -> WTWF_W {
        WTWF_W::new(self)
    }
    #[doc = "Bit 3 - SHOPF"]
    #[inline(always)]
    pub fn shopf(&mut self) -> SHOPF_W {
        SHOPF_W::new(self)
    }
    #[doc = "Bit 4 - INITSF"]
    #[inline(always)]
    pub fn initsf(&mut self) -> INITSF_W {
        INITSF_W::new(self)
    }
    #[doc = "Bit 5 - RSYF"]
    #[inline(always)]
    pub fn rsyf(&mut self) -> RSYF_W {
        RSYF_W::new(self)
    }
    #[doc = "Bit 6 - INITF"]
    #[inline(always)]
    pub fn initf(&mut self) -> INITF_W {
        INITF_W::new(self)
    }
    #[doc = "Bit 7 - INITM"]
    #[inline(always)]
    pub fn initm(&mut self) -> INITM_W {
        INITM_W::new(self)
    }
    #[doc = "Bit 8 - ALAF"]
    #[inline(always)]
    pub fn alaf(&mut self) -> ALAF_W {
        ALAF_W::new(self)
    }
    #[doc = "Bit 9 - ALBF"]
    #[inline(always)]
    pub fn albf(&mut self) -> ALBF_W {
        ALBF_W::new(self)
    }
    #[doc = "Bit 10 - WTF"]
    #[inline(always)]
    pub fn wtf(&mut self) -> WTF_W {
        WTF_W::new(self)
    }
    #[doc = "Bit 11 - TISF"]
    #[inline(always)]
    pub fn tisf(&mut self) -> TISF_W {
        TISF_W::new(self)
    }
    #[doc = "Bit 12 - TISOVF"]
    #[inline(always)]
    pub fn tisovf(&mut self) -> TISOVF_W {
        TISOVF_W::new(self)
    }
    #[doc = "Bit 16 - RECPF"]
    #[inline(always)]
    pub fn recpf(&mut self) -> RECPF_W {
        RECPF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_INITSTS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_initsts](index.html) module"]
pub struct RTC_INITSTS_SPEC;
impl crate::RegisterSpec for RTC_INITSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_initsts::R](R) reader structure"]
impl crate::Readable for RTC_INITSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_initsts::W](W) writer structure"]
impl crate::Writable for RTC_INITSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_INITSTS to value 0x07"]
impl crate::Resettable for RTC_INITSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
