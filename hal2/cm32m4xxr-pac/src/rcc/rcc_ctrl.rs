#[doc = "Register `RCC_CTRL` reader"]
pub struct R(crate::R<RCC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_CTRL` writer"]
pub struct W(crate::W<RCC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CTRL_SPEC>;
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
impl From<crate::W<RCC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSIEN` reader - HSIEN"]
pub type HSIEN_R = crate::BitReader<bool>;
#[doc = "Field `HSIEN` writer - HSIEN"]
pub type HSIEN_W<'a> = crate::BitWriter<'a, u32, RCC_CTRL_SPEC, bool, 0>;
#[doc = "Field `HSIRDF` reader - HSIRDF"]
pub type HSIRDF_R = crate::BitReader<bool>;
#[doc = "Field `HSIRDF` writer - HSIRDF"]
pub type HSIRDF_W<'a> = crate::BitWriter<'a, u32, RCC_CTRL_SPEC, bool, 1>;
#[doc = "Field `HSITRIM` reader - HSITRIM"]
pub type HSITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSITRIM` writer - HSITRIM"]
pub type HSITRIM_W<'a> = crate::FieldWriter<'a, u32, RCC_CTRL_SPEC, u8, u8, 5, 3>;
#[doc = "Field `HSICAL` reader - HSICAL"]
pub type HSICAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSICAL` writer - HSICAL"]
pub type HSICAL_W<'a> = crate::FieldWriter<'a, u32, RCC_CTRL_SPEC, u8, u8, 8, 8>;
#[doc = "Field `HSEEN` reader - HSEEN"]
pub type HSEEN_R = crate::BitReader<bool>;
#[doc = "Field `HSEEN` writer - HSEEN"]
pub type HSEEN_W<'a> = crate::BitWriter<'a, u32, RCC_CTRL_SPEC, bool, 16>;
#[doc = "Field `HSERDF` reader - HSERDF"]
pub type HSERDF_R = crate::BitReader<bool>;
#[doc = "Field `HSERDF` writer - HSERDF"]
pub type HSERDF_W<'a> = crate::BitWriter<'a, u32, RCC_CTRL_SPEC, bool, 17>;
#[doc = "Field `HSEBP` reader - HSEBP"]
pub type HSEBP_R = crate::BitReader<bool>;
#[doc = "Field `HSEBP` writer - HSEBP"]
pub type HSEBP_W<'a> = crate::BitWriter<'a, u32, RCC_CTRL_SPEC, bool, 18>;
#[doc = "Field `CLKSSEN` reader - CLKSSEN"]
pub type CLKSSEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKSSEN` writer - CLKSSEN"]
pub type CLKSSEN_W<'a> = crate::BitWriter<'a, u32, RCC_CTRL_SPEC, bool, 19>;
#[doc = "Field `PLLEN` reader - PLLEN"]
pub type PLLEN_R = crate::BitReader<bool>;
#[doc = "Field `PLLEN` writer - PLLEN"]
pub type PLLEN_W<'a> = crate::BitWriter<'a, u32, RCC_CTRL_SPEC, bool, 24>;
#[doc = "Field `PLLRDF` reader - PLLRDF"]
pub type PLLRDF_R = crate::BitReader<bool>;
#[doc = "Field `PLLRDF` writer - PLLRDF"]
pub type PLLRDF_W<'a> = crate::BitWriter<'a, u32, RCC_CTRL_SPEC, bool, 25>;
impl R {
    #[doc = "Bit 0 - HSIEN"]
    #[inline(always)]
    pub fn hsien(&self) -> HSIEN_R {
        HSIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSIRDF"]
    #[inline(always)]
    pub fn hsirdf(&self) -> HSIRDF_R {
        HSIRDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:7 - HSITRIM"]
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - HSICAL"]
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - HSEEN"]
    #[inline(always)]
    pub fn hseen(&self) -> HSEEN_R {
        HSEEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSERDF"]
    #[inline(always)]
    pub fn hserdf(&self) -> HSERDF_R {
        HSERDF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSEBP"]
    #[inline(always)]
    pub fn hsebp(&self) -> HSEBP_R {
        HSEBP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CLKSSEN"]
    #[inline(always)]
    pub fn clkssen(&self) -> CLKSSEN_R {
        CLKSSEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - PLLEN"]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PLLRDF"]
    #[inline(always)]
    pub fn pllrdf(&self) -> PLLRDF_R {
        PLLRDF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSIEN"]
    #[inline(always)]
    pub fn hsien(&mut self) -> HSIEN_W {
        HSIEN_W::new(self)
    }
    #[doc = "Bit 1 - HSIRDF"]
    #[inline(always)]
    pub fn hsirdf(&mut self) -> HSIRDF_W {
        HSIRDF_W::new(self)
    }
    #[doc = "Bits 3:7 - HSITRIM"]
    #[inline(always)]
    pub fn hsitrim(&mut self) -> HSITRIM_W {
        HSITRIM_W::new(self)
    }
    #[doc = "Bits 8:15 - HSICAL"]
    #[inline(always)]
    pub fn hsical(&mut self) -> HSICAL_W {
        HSICAL_W::new(self)
    }
    #[doc = "Bit 16 - HSEEN"]
    #[inline(always)]
    pub fn hseen(&mut self) -> HSEEN_W {
        HSEEN_W::new(self)
    }
    #[doc = "Bit 17 - HSERDF"]
    #[inline(always)]
    pub fn hserdf(&mut self) -> HSERDF_W {
        HSERDF_W::new(self)
    }
    #[doc = "Bit 18 - HSEBP"]
    #[inline(always)]
    pub fn hsebp(&mut self) -> HSEBP_W {
        HSEBP_W::new(self)
    }
    #[doc = "Bit 19 - CLKSSEN"]
    #[inline(always)]
    pub fn clkssen(&mut self) -> CLKSSEN_W {
        CLKSSEN_W::new(self)
    }
    #[doc = "Bit 24 - PLLEN"]
    #[inline(always)]
    pub fn pllen(&mut self) -> PLLEN_W {
        PLLEN_W::new(self)
    }
    #[doc = "Bit 25 - PLLRDF"]
    #[inline(always)]
    pub fn pllrdf(&mut self) -> PLLRDF_W {
        PLLRDF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_ctrl](index.html) module"]
pub struct RCC_CTRL_SPEC;
impl crate::RegisterSpec for RCC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_ctrl::R](R) reader structure"]
impl crate::Readable for RCC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_ctrl::W](W) writer structure"]
impl crate::Writable for RCC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_CTRL to value 0x83"]
impl crate::Resettable for RCC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x83
    }
}
