#[doc = "Register `XIP_SLAVE_EN` reader"]
pub struct R(crate::R<XIP_SLAVE_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XIP_SLAVE_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XIP_SLAVE_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XIP_SLAVE_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XIP_SLAVE_EN` writer"]
pub struct W(crate::W<XIP_SLAVE_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XIP_SLAVE_EN_SPEC>;
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
impl From<crate::W<XIP_SLAVE_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XIP_SLAVE_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEN` reader - SEN"]
pub type SEN_R = crate::BitReader<bool>;
#[doc = "Field `SEN` writer - SEN"]
pub type SEN_W<'a> = crate::BitWriter<'a, u32, XIP_SLAVE_EN_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - SEN"]
    #[inline(always)]
    pub fn sen(&self) -> SEN_R {
        SEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SEN"]
    #[inline(always)]
    pub fn sen(&mut self) -> SEN_W {
        SEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XIP_SLAVE_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xip_slave_en](index.html) module"]
pub struct XIP_SLAVE_EN_SPEC;
impl crate::RegisterSpec for XIP_SLAVE_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xip_slave_en::R](R) reader structure"]
impl crate::Readable for XIP_SLAVE_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xip_slave_en::W](W) writer structure"]
impl crate::Writable for XIP_SLAVE_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XIP_SLAVE_EN to value 0"]
impl crate::Resettable for XIP_SLAVE_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
