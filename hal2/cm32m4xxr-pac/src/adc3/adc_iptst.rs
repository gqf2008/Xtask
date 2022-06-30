#[doc = "Register `ADC_IPTST` reader"]
pub struct R(crate::R<ADC_IPTST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_IPTST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_IPTST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_IPTST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_IPTST` writer"]
pub struct W(crate::W<ADC_IPTST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_IPTST_SPEC>;
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
impl From<crate::W<ADC_IPTST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_IPTST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENCTR` reader - ENCTR"]
pub type ENCTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENCTR` writer - ENCTR"]
pub type ENCTR_W<'a> = crate::FieldWriter<'a, u32, ADC_IPTST_SPEC, u8, u8, 3, 0>;
#[doc = "Field `SREFLDO` reader - SREFLDO"]
pub type SREFLDO_R = crate::BitReader<bool>;
#[doc = "Field `SREFLDO` writer - SREFLDO"]
pub type SREFLDO_W<'a> = crate::BitWriter<'a, u32, ADC_IPTST_SPEC, bool, 3>;
#[doc = "Field `SRNGLDO` reader - SRNGLDO"]
pub type SRNGLDO_R = crate::BitReader<bool>;
#[doc = "Field `SRNGLDO` writer - SRNGLDO"]
pub type SRNGLDO_W<'a> = crate::BitWriter<'a, u32, ADC_IPTST_SPEC, bool, 4>;
#[doc = "Field `EXTCTRL` reader - EXTCTRL"]
pub type EXTCTRL_R = crate::BitReader<bool>;
#[doc = "Field `EXTCTRL` writer - EXTCTRL"]
pub type EXTCTRL_W<'a> = crate::BitWriter<'a, u32, ADC_IPTST_SPEC, bool, 5>;
impl R {
    #[doc = "Bits 0:2 - ENCTR"]
    #[inline(always)]
    pub fn enctr(&self) -> ENCTR_R {
        ENCTR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - SREFLDO"]
    #[inline(always)]
    pub fn srefldo(&self) -> SREFLDO_R {
        SREFLDO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SRNGLDO"]
    #[inline(always)]
    pub fn srngldo(&self) -> SRNGLDO_R {
        SRNGLDO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EXTCTRL"]
    #[inline(always)]
    pub fn extctrl(&self) -> EXTCTRL_R {
        EXTCTRL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - ENCTR"]
    #[inline(always)]
    pub fn enctr(&mut self) -> ENCTR_W {
        ENCTR_W::new(self)
    }
    #[doc = "Bit 3 - SREFLDO"]
    #[inline(always)]
    pub fn srefldo(&mut self) -> SREFLDO_W {
        SREFLDO_W::new(self)
    }
    #[doc = "Bit 4 - SRNGLDO"]
    #[inline(always)]
    pub fn srngldo(&mut self) -> SRNGLDO_W {
        SRNGLDO_W::new(self)
    }
    #[doc = "Bit 5 - EXTCTRL"]
    #[inline(always)]
    pub fn extctrl(&mut self) -> EXTCTRL_W {
        EXTCTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_IPTST\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_iptst](index.html) module"]
pub struct ADC_IPTST_SPEC;
impl crate::RegisterSpec for ADC_IPTST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_iptst::R](R) reader structure"]
impl crate::Readable for ADC_IPTST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_iptst::W](W) writer structure"]
impl crate::Writable for ADC_IPTST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_IPTST to value 0x08"]
impl crate::Resettable for ADC_IPTST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
