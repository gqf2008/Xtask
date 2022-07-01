#[doc = "Register `AFIO_ECTRL` reader"]
pub struct R(crate::R<AFIO_ECTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFIO_ECTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFIO_ECTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFIO_ECTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFIO_ECTRL` writer"]
pub struct W(crate::W<AFIO_ECTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFIO_ECTRL_SPEC>;
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
impl From<crate::W<AFIO_ECTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFIO_ECTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIN_SEL` reader - PIN_SEL"]
pub type PIN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN_SEL` writer - PIN_SEL"]
pub type PIN_SEL_W<'a> = crate::FieldWriter<'a, u32, AFIO_ECTRL_SPEC, u8, u8, 4, 0>;
#[doc = "Field `PORT_SEL` reader - PORT_SEL"]
pub type PORT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PORT_SEL` writer - PORT_SEL"]
pub type PORT_SEL_W<'a> = crate::FieldWriter<'a, u32, AFIO_ECTRL_SPEC, u8, u8, 3, 4>;
#[doc = "Field `EOE` reader - EOE"]
pub type EOE_R = crate::BitReader<bool>;
#[doc = "Field `EOE` writer - EOE"]
pub type EOE_W<'a> = crate::BitWriter<'a, u32, AFIO_ECTRL_SPEC, bool, 7>;
impl R {
    #[doc = "Bits 0:3 - PIN_SEL"]
    #[inline(always)]
    pub fn pin_sel(&self) -> PIN_SEL_R {
        PIN_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - PORT_SEL"]
    #[inline(always)]
    pub fn port_sel(&self) -> PORT_SEL_R {
        PORT_SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - EOE"]
    #[inline(always)]
    pub fn eoe(&self) -> EOE_R {
        EOE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - PIN_SEL"]
    #[inline(always)]
    pub fn pin_sel(&mut self) -> PIN_SEL_W {
        PIN_SEL_W::new(self)
    }
    #[doc = "Bits 4:6 - PORT_SEL"]
    #[inline(always)]
    pub fn port_sel(&mut self) -> PORT_SEL_W {
        PORT_SEL_W::new(self)
    }
    #[doc = "Bit 7 - EOE"]
    #[inline(always)]
    pub fn eoe(&mut self) -> EOE_W {
        EOE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFIO_ECTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_ectrl](index.html) module"]
pub struct AFIO_ECTRL_SPEC;
impl crate::RegisterSpec for AFIO_ECTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afio_ectrl::R](R) reader structure"]
impl crate::Readable for AFIO_ECTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afio_ectrl::W](W) writer structure"]
impl crate::Writable for AFIO_ECTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFIO_ECTRL to value 0"]
impl crate::Resettable for AFIO_ECTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
