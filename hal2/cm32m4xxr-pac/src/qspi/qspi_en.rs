#[doc = "Register `QSPI_EN` reader"]
pub struct R(crate::R<QSPI_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QSPI_EN` writer"]
pub struct W(crate::W<QSPI_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QSPI_EN_SPEC>;
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
impl From<crate::W<QSPI_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QSPI_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QEN` reader - QEN"]
pub type QEN_R = crate::BitReader<bool>;
#[doc = "Field `QEN` writer - QEN"]
pub type QEN_W<'a> = crate::BitWriter<'a, u32, QSPI_EN_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - QEN"]
    #[inline(always)]
    pub fn qen(&self) -> QEN_R {
        QEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - QEN"]
    #[inline(always)]
    pub fn qen(&mut self) -> QEN_W {
        QEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_en](index.html) module"]
pub struct QSPI_EN_SPEC;
impl crate::RegisterSpec for QSPI_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_en::R](R) reader structure"]
impl crate::Readable for QSPI_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qspi_en::W](W) writer structure"]
impl crate::Writable for QSPI_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QSPI_EN to value 0"]
impl crate::Resettable for QSPI_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
