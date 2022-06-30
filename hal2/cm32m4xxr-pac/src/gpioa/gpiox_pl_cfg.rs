#[doc = "Register `GPIOx_PL_CFG` reader"]
pub struct R(crate::R<GPIOX_PL_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOX_PL_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOX_PL_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOX_PL_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOx_PL_CFG` writer"]
pub struct W(crate::W<GPIOX_PL_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOX_PL_CFG_SPEC>;
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
impl From<crate::W<GPIOX_PL_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOX_PL_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMODE0` reader - PMODE0"]
pub type PMODE0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PMODE0` writer - PMODE0"]
pub type PMODE0_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PL_CFG_SPEC, u8, u8, 2, 0>;
#[doc = "Field `PCFG0` reader - PCFG0"]
pub type PCFG0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCFG0` writer - PCFG0"]
pub type PCFG0_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PL_CFG_SPEC, u8, u8, 2, 2>;
#[doc = "Field `PMODE1` reader - PMODE1"]
pub type PMODE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PMODE1` writer - PMODE1"]
pub type PMODE1_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PL_CFG_SPEC, u8, u8, 2, 4>;
#[doc = "Field `PCFG1` reader - PCFG1"]
pub type PCFG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCFG1` writer - PCFG1"]
pub type PCFG1_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PL_CFG_SPEC, u8, u8, 2, 6>;
#[doc = "Field `PMODE2` reader - PMODE2"]
pub type PMODE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PMODE2` writer - PMODE2"]
pub type PMODE2_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PL_CFG_SPEC, u8, u8, 2, 8>;
#[doc = "Field `PCFG2` reader - PCFG2"]
pub type PCFG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCFG2` writer - PCFG2"]
pub type PCFG2_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PL_CFG_SPEC, u8, u8, 2, 10>;
#[doc = "Field `PMODE3` reader - PMODE3"]
pub type PMODE3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PMODE3` writer - PMODE3"]
pub type PMODE3_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PL_CFG_SPEC, u8, u8, 2, 12>;
#[doc = "Field `PCFG3` reader - PCFG3"]
pub type PCFG3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCFG3` writer - PCFG3"]
pub type PCFG3_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PL_CFG_SPEC, u8, u8, 2, 14>;
#[doc = "Field `PMODE4` reader - PMODE4"]
pub type PMODE4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PMODE4` writer - PMODE4"]
pub type PMODE4_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PL_CFG_SPEC, u8, u8, 2, 16>;
#[doc = "Field `PCFG4` reader - PCFG4"]
pub type PCFG4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCFG4` writer - PCFG4"]
pub type PCFG4_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PL_CFG_SPEC, u8, u8, 2, 18>;
#[doc = "Field `PMODE5` reader - PMODE5"]
pub type PMODE5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PMODE5` writer - PMODE5"]
pub type PMODE5_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PL_CFG_SPEC, u8, u8, 2, 20>;
#[doc = "Field `PCFG5` reader - PCFG5"]
pub type PCFG5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCFG5` writer - PCFG5"]
pub type PCFG5_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PL_CFG_SPEC, u8, u8, 2, 22>;
#[doc = "Field `PMODE6` reader - PMODE6"]
pub type PMODE6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PMODE6` writer - PMODE6"]
pub type PMODE6_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PL_CFG_SPEC, u8, u8, 2, 24>;
#[doc = "Field `PCFG6` reader - PCFG6"]
pub type PCFG6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCFG6` writer - PCFG6"]
pub type PCFG6_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PL_CFG_SPEC, u8, u8, 2, 26>;
#[doc = "Field `PMODE7` reader - PMODE7"]
pub type PMODE7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PMODE7` writer - PMODE7"]
pub type PMODE7_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PL_CFG_SPEC, u8, u8, 2, 28>;
#[doc = "Field `PCFG7` reader - PCFG7"]
pub type PCFG7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCFG7` writer - PCFG7"]
pub type PCFG7_W<'a> = crate::FieldWriter<'a, u32, GPIOX_PL_CFG_SPEC, u8, u8, 2, 30>;
impl R {
    #[doc = "Bits 0:1 - PMODE0"]
    #[inline(always)]
    pub fn pmode0(&self) -> PMODE0_R {
        PMODE0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PCFG0"]
    #[inline(always)]
    pub fn pcfg0(&self) -> PCFG0_R {
        PCFG0_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PMODE1"]
    #[inline(always)]
    pub fn pmode1(&self) -> PMODE1_R {
        PMODE1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PCFG1"]
    #[inline(always)]
    pub fn pcfg1(&self) -> PCFG1_R {
        PCFG1_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PMODE2"]
    #[inline(always)]
    pub fn pmode2(&self) -> PMODE2_R {
        PMODE2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PCFG2"]
    #[inline(always)]
    pub fn pcfg2(&self) -> PCFG2_R {
        PCFG2_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PMODE3"]
    #[inline(always)]
    pub fn pmode3(&self) -> PMODE3_R {
        PMODE3_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PCFG3"]
    #[inline(always)]
    pub fn pcfg3(&self) -> PCFG3_R {
        PCFG3_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PMODE4"]
    #[inline(always)]
    pub fn pmode4(&self) -> PMODE4_R {
        PMODE4_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PCFG4"]
    #[inline(always)]
    pub fn pcfg4(&self) -> PCFG4_R {
        PCFG4_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PMODE5"]
    #[inline(always)]
    pub fn pmode5(&self) -> PMODE5_R {
        PMODE5_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PCFG5"]
    #[inline(always)]
    pub fn pcfg5(&self) -> PCFG5_R {
        PCFG5_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PMODE6"]
    #[inline(always)]
    pub fn pmode6(&self) -> PMODE6_R {
        PMODE6_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PCFG6"]
    #[inline(always)]
    pub fn pcfg6(&self) -> PCFG6_R {
        PCFG6_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PMODE7"]
    #[inline(always)]
    pub fn pmode7(&self) -> PMODE7_R {
        PMODE7_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PCFG7"]
    #[inline(always)]
    pub fn pcfg7(&self) -> PCFG7_R {
        PCFG7_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PMODE0"]
    #[inline(always)]
    pub fn pmode0(&mut self) -> PMODE0_W {
        PMODE0_W::new(self)
    }
    #[doc = "Bits 2:3 - PCFG0"]
    #[inline(always)]
    pub fn pcfg0(&mut self) -> PCFG0_W {
        PCFG0_W::new(self)
    }
    #[doc = "Bits 4:5 - PMODE1"]
    #[inline(always)]
    pub fn pmode1(&mut self) -> PMODE1_W {
        PMODE1_W::new(self)
    }
    #[doc = "Bits 6:7 - PCFG1"]
    #[inline(always)]
    pub fn pcfg1(&mut self) -> PCFG1_W {
        PCFG1_W::new(self)
    }
    #[doc = "Bits 8:9 - PMODE2"]
    #[inline(always)]
    pub fn pmode2(&mut self) -> PMODE2_W {
        PMODE2_W::new(self)
    }
    #[doc = "Bits 10:11 - PCFG2"]
    #[inline(always)]
    pub fn pcfg2(&mut self) -> PCFG2_W {
        PCFG2_W::new(self)
    }
    #[doc = "Bits 12:13 - PMODE3"]
    #[inline(always)]
    pub fn pmode3(&mut self) -> PMODE3_W {
        PMODE3_W::new(self)
    }
    #[doc = "Bits 14:15 - PCFG3"]
    #[inline(always)]
    pub fn pcfg3(&mut self) -> PCFG3_W {
        PCFG3_W::new(self)
    }
    #[doc = "Bits 16:17 - PMODE4"]
    #[inline(always)]
    pub fn pmode4(&mut self) -> PMODE4_W {
        PMODE4_W::new(self)
    }
    #[doc = "Bits 18:19 - PCFG4"]
    #[inline(always)]
    pub fn pcfg4(&mut self) -> PCFG4_W {
        PCFG4_W::new(self)
    }
    #[doc = "Bits 20:21 - PMODE5"]
    #[inline(always)]
    pub fn pmode5(&mut self) -> PMODE5_W {
        PMODE5_W::new(self)
    }
    #[doc = "Bits 22:23 - PCFG5"]
    #[inline(always)]
    pub fn pcfg5(&mut self) -> PCFG5_W {
        PCFG5_W::new(self)
    }
    #[doc = "Bits 24:25 - PMODE6"]
    #[inline(always)]
    pub fn pmode6(&mut self) -> PMODE6_W {
        PMODE6_W::new(self)
    }
    #[doc = "Bits 26:27 - PCFG6"]
    #[inline(always)]
    pub fn pcfg6(&mut self) -> PCFG6_W {
        PCFG6_W::new(self)
    }
    #[doc = "Bits 28:29 - PMODE7"]
    #[inline(always)]
    pub fn pmode7(&mut self) -> PMODE7_W {
        PMODE7_W::new(self)
    }
    #[doc = "Bits 30:31 - PCFG7"]
    #[inline(always)]
    pub fn pcfg7(&mut self) -> PCFG7_W {
        PCFG7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOx_PL_CFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiox_pl_cfg](index.html) module"]
pub struct GPIOX_PL_CFG_SPEC;
impl crate::RegisterSpec for GPIOX_PL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiox_pl_cfg::R](R) reader structure"]
impl crate::Readable for GPIOX_PL_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiox_pl_cfg::W](W) writer structure"]
impl crate::Writable for GPIOX_PL_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOx_PL_CFG to value 0x0008_0800"]
impl crate::Resettable for GPIOX_PL_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0008_0800
    }
}
