#[doc = "Register `PWR_CTRL4` reader"]
pub struct R(crate::R<PWR_CTRL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CTRL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CTRL4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CTRL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_CTRL4` writer"]
pub struct W(crate::W<PWR_CTRL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_CTRL4_SPEC>;
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
impl From<crate::W<PWR_CTRL4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_CTRL4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTR` reader - CTR"]
pub type CTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTR` writer - CTR"]
pub type CTR_W<'a> = crate::FieldWriter<'a, u32, PWR_CTRL4_SPEC, u8, u8, 2, 0>;
#[doc = "Field `AGCC` reader - AGCC"]
pub type AGCC_R = crate::BitReader<bool>;
#[doc = "Field `AGCC` writer - AGCC"]
pub type AGCC_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL4_SPEC, bool, 2>;
#[doc = "Field `BSTR` reader - BSTR"]
pub type BSTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BSTR` writer - BSTR"]
pub type BSTR_W<'a> = crate::FieldWriter<'a, u32, PWR_CTRL4_SPEC, u8, u8, 4, 3>;
#[doc = "Field `BUFFTRV` reader - BUFFTRV"]
pub type BUFFTRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUFFTRV` writer - BUFFTRV"]
pub type BUFFTRV_W<'a> = crate::FieldWriter<'a, u32, PWR_CTRL4_SPEC, u8, u8, 2, 7>;
impl R {
    #[doc = "Bits 0:1 - CTR"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - AGCC"]
    #[inline(always)]
    pub fn agcc(&self) -> AGCC_R {
        AGCC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - BSTR"]
    #[inline(always)]
    pub fn bstr(&self) -> BSTR_R {
        BSTR_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:8 - BUFFTRV"]
    #[inline(always)]
    pub fn bufftrv(&self) -> BUFFTRV_R {
        BUFFTRV_R::new(((self.bits >> 7) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CTR"]
    #[inline(always)]
    pub fn ctr(&mut self) -> CTR_W {
        CTR_W::new(self)
    }
    #[doc = "Bit 2 - AGCC"]
    #[inline(always)]
    pub fn agcc(&mut self) -> AGCC_W {
        AGCC_W::new(self)
    }
    #[doc = "Bits 3:6 - BSTR"]
    #[inline(always)]
    pub fn bstr(&mut self) -> BSTR_W {
        BSTR_W::new(self)
    }
    #[doc = "Bits 7:8 - BUFFTRV"]
    #[inline(always)]
    pub fn bufftrv(&mut self) -> BUFFTRV_W {
        BUFFTRV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWR_CTRL4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_ctrl4](index.html) module"]
pub struct PWR_CTRL4_SPEC;
impl crate::RegisterSpec for PWR_CTRL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_ctrl4::R](R) reader structure"]
impl crate::Readable for PWR_CTRL4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_ctrl4::W](W) writer structure"]
impl crate::Writable for PWR_CTRL4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_CTRL4 to value 0x0141"]
impl crate::Resettable for PWR_CTRL4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0141
    }
}
