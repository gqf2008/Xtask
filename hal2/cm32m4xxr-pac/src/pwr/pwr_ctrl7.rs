#[doc = "Register `PWR_CTRL7` reader"]
pub struct R(crate::R<PWR_CTRL7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CTRL7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CTRL7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CTRL7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_CTRL7` writer"]
pub struct W(crate::W<PWR_CTRL7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_CTRL7_SPEC>;
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
impl From<crate::W<PWR_CTRL7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_CTRL7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PVSCSEL` reader - PVSCSEL"]
pub type PVSCSEL_R = crate::BitReader<bool>;
#[doc = "Field `PVSCSEL` writer - PVSCSEL"]
pub type PVSCSEL_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL7_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - PVSCSEL"]
    #[inline(always)]
    pub fn pvscsel(&self) -> PVSCSEL_R {
        PVSCSEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PVSCSEL"]
    #[inline(always)]
    pub fn pvscsel(&mut self) -> PVSCSEL_W {
        PVSCSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWR_CTRL7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_ctrl7](index.html) module"]
pub struct PWR_CTRL7_SPEC;
impl crate::RegisterSpec for PWR_CTRL7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_ctrl7::R](R) reader structure"]
impl crate::Readable for PWR_CTRL7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_ctrl7::W](W) writer structure"]
impl crate::Writable for PWR_CTRL7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_CTRL7 to value 0x01"]
impl crate::Resettable for PWR_CTRL7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
