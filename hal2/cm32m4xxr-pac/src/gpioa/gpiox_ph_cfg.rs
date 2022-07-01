#[doc = "Register `GPIOx_PH_CFG` reader"]
pub struct R(crate::R<GPIOX_PH_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOX_PH_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOX_PH_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOX_PH_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOx_PH_CFG` writer"]
pub struct W(crate::W<GPIOX_PH_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOX_PH_CFG_SPEC>;
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
impl From<crate::W<GPIOX_PH_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOX_PH_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMODE8` reader - PMODE8"]
pub type PMODE8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PMODE8` writer - PMODE8"]
pub type PMODE8_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PH_CFG_SPEC, u8, u8, 2, 0>;
#[doc = "Field `PCFG8` reader - PCFG8"]
pub type PCFG8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCFG8` writer - PCFG8"]
pub type PCFG8_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PH_CFG_SPEC, u8, u8, 2, 2>;
#[doc = "Field `PMODE9` reader - PMODE9"]
pub type PMODE9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PMODE9` writer - PMODE9"]
pub type PMODE9_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PH_CFG_SPEC, u8, u8, 2, 4>;
#[doc = "Field `PCFG9` reader - PCFG9"]
pub type PCFG9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCFG9` writer - PCFG9"]
pub type PCFG9_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PH_CFG_SPEC, u8, u8, 2, 6>;
#[doc = "Field `PMODE10` reader - PMODE10"]
pub type PMODE10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PMODE10` writer - PMODE10"]
pub type PMODE10_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PH_CFG_SPEC, u8, u8, 2, 8>;
#[doc = "Field `PCFG10` reader - PCFG10"]
pub type PCFG10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCFG10` writer - PCFG10"]
pub type PCFG10_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PH_CFG_SPEC, u8, u8, 2, 10>;
#[doc = "Field `PMODE11` reader - PMODE11"]
pub type PMODE11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PMODE11` writer - PMODE11"]
pub type PMODE11_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PH_CFG_SPEC, u8, u8, 2, 12>;
#[doc = "Field `PCFG11` reader - PCFG11"]
pub type PCFG11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCFG11` writer - PCFG11"]
pub type PCFG11_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PH_CFG_SPEC, u8, u8, 2, 14>;
#[doc = "Field `PMODE12` reader - PMODE12"]
pub type PMODE12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PMODE12` writer - PMODE12"]
pub type PMODE12_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PH_CFG_SPEC, u8, u8, 2, 16>;
#[doc = "Field `PCFG12` reader - PCFG12"]
pub type PCFG12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCFG12` writer - PCFG12"]
pub type PCFG12_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PH_CFG_SPEC, u8, u8, 2, 18>;
#[doc = "Field `PMODE13` reader - PMODE13"]
pub type PMODE13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PMODE13` writer - PMODE13"]
pub type PMODE13_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PH_CFG_SPEC, u8, u8, 2, 20>;
#[doc = "Field `PCFG13` reader - PCFG13"]
pub type PCFG13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCFG13` writer - PCFG13"]
pub type PCFG13_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PH_CFG_SPEC, u8, u8, 2, 22>;
#[doc = "Field `PMODE14` reader - PMODE14"]
pub type PMODE14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PMODE14` writer - PMODE14"]
pub type PMODE14_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PH_CFG_SPEC, u8, u8, 2, 24>;
#[doc = "Field `PCFG14` reader - PCFG14"]
pub type PCFG14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCFG14` writer - PCFG14"]
pub type PCFG14_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PH_CFG_SPEC, u8, u8, 2, 26>;
#[doc = "Field `PMODE15` reader - PMODE15"]
pub type PMODE15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PMODE15` writer - PMODE15"]
pub type PMODE15_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PH_CFG_SPEC, u8, u8, 2, 28>;
#[doc = "Field `PCFG15` reader - PCFG15"]
pub type PCFG15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCFG15` writer - PCFG15"]
pub type PCFG15_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PH_CFG_SPEC, u8, u8, 2, 30>;
impl R {
    #[doc = "Bits 0:1 - PMODE8"]
    #[inline(always)]
    pub fn pmode8(&self) -> PMODE8_R {
        PMODE8_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PCFG8"]
    #[inline(always)]
    pub fn pcfg8(&self) -> PCFG8_R {
        PCFG8_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PMODE9"]
    #[inline(always)]
    pub fn pmode9(&self) -> PMODE9_R {
        PMODE9_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PCFG9"]
    #[inline(always)]
    pub fn pcfg9(&self) -> PCFG9_R {
        PCFG9_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PMODE10"]
    #[inline(always)]
    pub fn pmode10(&self) -> PMODE10_R {
        PMODE10_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PCFG10"]
    #[inline(always)]
    pub fn pcfg10(&self) -> PCFG10_R {
        PCFG10_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PMODE11"]
    #[inline(always)]
    pub fn pmode11(&self) -> PMODE11_R {
        PMODE11_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PCFG11"]
    #[inline(always)]
    pub fn pcfg11(&self) -> PCFG11_R {
        PCFG11_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PMODE12"]
    #[inline(always)]
    pub fn pmode12(&self) -> PMODE12_R {
        PMODE12_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PCFG12"]
    #[inline(always)]
    pub fn pcfg12(&self) -> PCFG12_R {
        PCFG12_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PMODE13"]
    #[inline(always)]
    pub fn pmode13(&self) -> PMODE13_R {
        PMODE13_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PCFG13"]
    #[inline(always)]
    pub fn pcfg13(&self) -> PCFG13_R {
        PCFG13_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PMODE14"]
    #[inline(always)]
    pub fn pmode14(&self) -> PMODE14_R {
        PMODE14_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PCFG14"]
    #[inline(always)]
    pub fn pcfg14(&self) -> PCFG14_R {
        PCFG14_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PMODE15"]
    #[inline(always)]
    pub fn pmode15(&self) -> PMODE15_R {
        PMODE15_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PCFG15"]
    #[inline(always)]
    pub fn pcfg15(&self) -> PCFG15_R {
        PCFG15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PMODE8"]
    #[inline(always)]
    pub fn pmode8(&mut self) -> PMODE8_W {
        PMODE8_W::new(self)
    }
    #[doc = "Bits 2:3 - PCFG8"]
    #[inline(always)]
    pub fn pcfg8(&mut self) -> PCFG8_W {
        PCFG8_W::new(self)
    }
    #[doc = "Bits 4:5 - PMODE9"]
    #[inline(always)]
    pub fn pmode9(&mut self) -> PMODE9_W {
        PMODE9_W::new(self)
    }
    #[doc = "Bits 6:7 - PCFG9"]
    #[inline(always)]
    pub fn pcfg9(&mut self) -> PCFG9_W {
        PCFG9_W::new(self)
    }
    #[doc = "Bits 8:9 - PMODE10"]
    #[inline(always)]
    pub fn pmode10(&mut self) -> PMODE10_W {
        PMODE10_W::new(self)
    }
    #[doc = "Bits 10:11 - PCFG10"]
    #[inline(always)]
    pub fn pcfg10(&mut self) -> PCFG10_W {
        PCFG10_W::new(self)
    }
    #[doc = "Bits 12:13 - PMODE11"]
    #[inline(always)]
    pub fn pmode11(&mut self) -> PMODE11_W {
        PMODE11_W::new(self)
    }
    #[doc = "Bits 14:15 - PCFG11"]
    #[inline(always)]
    pub fn pcfg11(&mut self) -> PCFG11_W {
        PCFG11_W::new(self)
    }
    #[doc = "Bits 16:17 - PMODE12"]
    #[inline(always)]
    pub fn pmode12(&mut self) -> PMODE12_W {
        PMODE12_W::new(self)
    }
    #[doc = "Bits 18:19 - PCFG12"]
    #[inline(always)]
    pub fn pcfg12(&mut self) -> PCFG12_W {
        PCFG12_W::new(self)
    }
    #[doc = "Bits 20:21 - PMODE13"]
    #[inline(always)]
    pub fn pmode13(&mut self) -> PMODE13_W {
        PMODE13_W::new(self)
    }
    #[doc = "Bits 22:23 - PCFG13"]
    #[inline(always)]
    pub fn pcfg13(&mut self) -> PCFG13_W {
        PCFG13_W::new(self)
    }
    #[doc = "Bits 24:25 - PMODE14"]
    #[inline(always)]
    pub fn pmode14(&mut self) -> PMODE14_W {
        PMODE14_W::new(self)
    }
    #[doc = "Bits 26:27 - PCFG14"]
    #[inline(always)]
    pub fn pcfg14(&mut self) -> PCFG14_W {
        PCFG14_W::new(self)
    }
    #[doc = "Bits 28:29 - PMODE15"]
    #[inline(always)]
    pub fn pmode15(&mut self) -> PMODE15_W {
        PMODE15_W::new(self)
    }
    #[doc = "Bits 30:31 - PCFG15"]
    #[inline(always)]
    pub fn pcfg15(&mut self) -> PCFG15_W {
        PCFG15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOx_PH_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiox_ph_cfg](index.html) module"]
pub struct GPIOX_PH_CFG_SPEC;
impl crate::RegisterSpec for GPIOX_PH_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiox_ph_cfg::R](R) reader structure"]
impl crate::Readable for GPIOX_PH_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiox_ph_cfg::W](W) writer structure"]
impl crate::Writable for GPIOX_PH_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOx_PH_CFG to value 0x8880_0000"]
impl crate::Resettable for GPIOX_PH_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8880_0000
    }
}
