#[doc = "Register `PWR_CTRL3` reader"]
pub struct R(crate::R<PWR_CTRL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CTRL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CTRL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CTRL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_CTRL3` writer"]
pub struct W(crate::W<PWR_CTRL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_CTRL3_SPEC>;
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
impl From<crate::W<PWR_CTRL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_CTRL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKPM` reader - BKPM"]
pub type BKPM_R = crate::BitReader<bool>;
#[doc = "Field `BKPM` writer - BKPM"]
pub type BKPM_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL3_SPEC, bool, 0>;
#[doc = "Field `MRTR` reader - MRTR"]
pub type MRTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MRTR` writer - MRTR"]
pub type MRTR_W<'a> = crate::FieldWriter<'a, u32, PWR_CTRL3_SPEC, u8, u8, 4, 1>;
#[doc = "Field `BKRTR` reader - BKRTR"]
pub type BKRTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BKRTR` writer - BKRTR"]
pub type BKRTR_W<'a> = crate::FieldWriter<'a, u32, PWR_CTRL3_SPEC, u8, u8, 5, 5>;
#[doc = "Field `VDBKPORTR` reader - VDBKPORTR"]
pub type VDBKPORTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VDBKPORTR` writer - VDBKPORTR"]
pub type VDBKPORTR_W<'a> = crate::FieldWriter<'a, u32, PWR_CTRL3_SPEC, u8, u8, 2, 10>;
#[doc = "Field `BKRBSTR` reader - BKRBSTR"]
pub type BKRBSTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BKRBSTR` writer - BKRBSTR"]
pub type BKRBSTR_W<'a> = crate::FieldWriter<'a, u32, PWR_CTRL3_SPEC, u8, u8, 2, 12>;
#[doc = "Field `BKRCTRL` reader - BKRCTRL"]
pub type BKRCTRL_R = crate::BitReader<bool>;
#[doc = "Field `BKRCTRL` writer - BKRCTRL"]
pub type BKRCTRL_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL3_SPEC, bool, 14>;
#[doc = "Field `MRLPTR` reader - MRLPTR"]
pub type MRLPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MRLPTR` writer - MRLPTR"]
pub type MRLPTR_W<'a> = crate::FieldWriter<'a, u32, PWR_CTRL3_SPEC, u8, u8, 4, 15>;
impl R {
    #[doc = "Bit 0 - BKPM"]
    #[inline(always)]
    pub fn bkpm(&self) -> BKPM_R {
        BKPM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - MRTR"]
    #[inline(always)]
    pub fn mrtr(&self) -> MRTR_R {
        MRTR_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:9 - BKRTR"]
    #[inline(always)]
    pub fn bkrtr(&self) -> BKRTR_R {
        BKRTR_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:11 - VDBKPORTR"]
    #[inline(always)]
    pub fn vdbkportr(&self) -> VDBKPORTR_R {
        VDBKPORTR_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - BKRBSTR"]
    #[inline(always)]
    pub fn bkrbstr(&self) -> BKRBSTR_R {
        BKRBSTR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - BKRCTRL"]
    #[inline(always)]
    pub fn bkrctrl(&self) -> BKRCTRL_R {
        BKRCTRL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:18 - MRLPTR"]
    #[inline(always)]
    pub fn mrlptr(&self) -> MRLPTR_R {
        MRLPTR_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - BKPM"]
    #[inline(always)]
    pub fn bkpm(&mut self) -> BKPM_W {
        BKPM_W::new(self)
    }
    #[doc = "Bits 1:4 - MRTR"]
    #[inline(always)]
    pub fn mrtr(&mut self) -> MRTR_W {
        MRTR_W::new(self)
    }
    #[doc = "Bits 5:9 - BKRTR"]
    #[inline(always)]
    pub fn bkrtr(&mut self) -> BKRTR_W {
        BKRTR_W::new(self)
    }
    #[doc = "Bits 10:11 - VDBKPORTR"]
    #[inline(always)]
    pub fn vdbkportr(&mut self) -> VDBKPORTR_W {
        VDBKPORTR_W::new(self)
    }
    #[doc = "Bits 12:13 - BKRBSTR"]
    #[inline(always)]
    pub fn bkrbstr(&mut self) -> BKRBSTR_W {
        BKRBSTR_W::new(self)
    }
    #[doc = "Bit 14 - BKRCTRL"]
    #[inline(always)]
    pub fn bkrctrl(&mut self) -> BKRCTRL_W {
        BKRCTRL_W::new(self)
    }
    #[doc = "Bits 15:18 - MRLPTR"]
    #[inline(always)]
    pub fn mrlptr(&mut self) -> MRLPTR_W {
        MRLPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWR_CTRL3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_ctrl3](index.html) module"]
pub struct PWR_CTRL3_SPEC;
impl crate::RegisterSpec for PWR_CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_ctrl3::R](R) reader structure"]
impl crate::Readable for PWR_CTRL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_ctrl3::W](W) writer structure"]
impl crate::Writable for PWR_CTRL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_CTRL3 to value 0x0004_5b70"]
impl crate::Resettable for PWR_CTRL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0004_5b70
    }
}
