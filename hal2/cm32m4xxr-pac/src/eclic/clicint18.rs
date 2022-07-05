#[doc = "Register `clicint18` reader"]
pub struct R(crate::R<CLICINT18_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLICINT18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLICINT18_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLICINT18_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clicint18` writer"]
pub struct W(crate::W<CLICINT18_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLICINT18_SPEC>;
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
impl From<crate::W<CLICINT18_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLICINT18_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IP` reader - Interrupt source pending flag"]
pub type IP_R = crate::BitReader<bool>;
#[doc = "Field `IP` writer - Interrupt source pending flag"]
pub type IP_W<'a> = crate::BitWriter<'a, u32, CLICINT18_SPEC, bool, 0>;
#[doc = "Field `IE` reader - Interrupt enable bit"]
pub type IE_R = crate::BitReader<bool>;
#[doc = "Field `IE` writer - Interrupt enable bit"]
pub type IE_W<'a> = crate::BitWriter<'a, u32, CLICINT18_SPEC, bool, 8>;
#[doc = "Field `TRIG` reader - set the level or edge triggered attribute of the interrupt source"]
pub type TRIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIG` writer - set the level or edge triggered attribute of the interrupt source"]
pub type TRIG_W<'a> = crate::FieldWriter<'a, u32, CLICINT18_SPEC, u8, u8, 3, 17>;
#[doc = "Field `SHV` reader - set whether the interrupt is vectored or non-vectored"]
pub type SHV_R = crate::BitReader<bool>;
#[doc = "Field `SHV` writer - set whether the interrupt is vectored or non-vectored"]
pub type SHV_W<'a> = crate::BitWriter<'a, u32, CLICINT18_SPEC, bool, 16>;
#[doc = "Field `LEVEL/PRIORITY` reader - set interrupt level and priority"]
pub type LEVELPRIORITY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEVEL/PRIORITY` writer - set interrupt level and priority"]
pub type LEVELPRIORITY_W<'a> = crate::FieldWriter<'a, u32, CLICINT18_SPEC, u8, u8, 8, 24>;
impl R {
    #[doc = "Bit 0 - Interrupt source pending flag"]
    #[inline(always)]
    pub fn ip(&self) -> IP_R {
        IP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt enable bit"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 17:19 - set the level or edge triggered attribute of the interrupt source"]
    #[inline(always)]
    pub fn trig(&self) -> TRIG_R {
        TRIG_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 16 - set whether the interrupt is vectored or non-vectored"]
    #[inline(always)]
    pub fn shv(&self) -> SHV_R {
        SHV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - set interrupt level and priority"]
    #[inline(always)]
    pub fn levelpriority(&self) -> LEVELPRIORITY_R {
        LEVELPRIORITY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt source pending flag"]
    #[inline(always)]
    pub fn ip(&mut self) -> IP_W {
        IP_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt enable bit"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W::new(self)
    }
    #[doc = "Bits 17:19 - set the level or edge triggered attribute of the interrupt source"]
    #[inline(always)]
    pub fn trig(&mut self) -> TRIG_W {
        TRIG_W::new(self)
    }
    #[doc = "Bit 16 - set whether the interrupt is vectored or non-vectored"]
    #[inline(always)]
    pub fn shv(&mut self) -> SHV_W {
        SHV_W::new(self)
    }
    #[doc = "Bits 24:31 - set interrupt level and priority"]
    #[inline(always)]
    pub fn levelpriority(&mut self) -> LEVELPRIORITY_W {
        LEVELPRIORITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt control and state\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clicint18](index.html) module"]
pub struct CLICINT18_SPEC;
impl crate::RegisterSpec for CLICINT18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clicint18::R](R) reader structure"]
impl crate::Readable for CLICINT18_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clicint18::W](W) writer structure"]
impl crate::Writable for CLICINT18_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets clicint18 to value 0"]
impl crate::Resettable for CLICINT18_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
