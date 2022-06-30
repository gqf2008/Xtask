#[doc = "Register `USART_CTRL2` reader"]
pub struct R(crate::R<USART_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USART_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USART_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USART_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USART_CTRL2` writer"]
pub struct W(crate::W<USART_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USART_CTRL2_SPEC>;
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
impl From<crate::W<USART_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USART_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - ADDR"]
pub type ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR` writer - ADDR"]
pub type ADDR_W<'a> = crate::FieldWriter<'a, u32, USART_CTRL2_SPEC, u8, u8, 4, 0>;
#[doc = "Field `LINBDL` reader - LINBDL"]
pub type LINBDL_R = crate::BitReader<bool>;
#[doc = "Field `LINBDL` writer - LINBDL"]
pub type LINBDL_W<'a> = crate::BitWriter<'a, u32, USART_CTRL2_SPEC, bool, 5>;
#[doc = "Field `LINBDIEN` reader - LINBDIEN"]
pub type LINBDIEN_R = crate::BitReader<bool>;
#[doc = "Field `LINBDIEN` writer - LINBDIEN"]
pub type LINBDIEN_W<'a> = crate::BitWriter<'a, u32, USART_CTRL2_SPEC, bool, 6>;
#[doc = "Field `LBCLK` reader - LBCLK"]
pub type LBCLK_R = crate::BitReader<bool>;
#[doc = "Field `LBCLK` writer - LBCLK"]
pub type LBCLK_W<'a> = crate::BitWriter<'a, u32, USART_CTRL2_SPEC, bool, 8>;
#[doc = "Field `CLKPHA` reader - CLKPHA"]
pub type CLKPHA_R = crate::BitReader<bool>;
#[doc = "Field `CLKPHA` writer - CLKPHA"]
pub type CLKPHA_W<'a> = crate::BitWriter<'a, u32, USART_CTRL2_SPEC, bool, 9>;
#[doc = "Field `CLKPOL` reader - CLKPOL"]
pub type CLKPOL_R = crate::BitReader<bool>;
#[doc = "Field `CLKPOL` writer - CLKPOL"]
pub type CLKPOL_W<'a> = crate::BitWriter<'a, u32, USART_CTRL2_SPEC, bool, 10>;
#[doc = "Field `CLKEN` reader - CLKEN"]
pub type CLKEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKEN` writer - CLKEN"]
pub type CLKEN_W<'a> = crate::BitWriter<'a, u32, USART_CTRL2_SPEC, bool, 11>;
#[doc = "Field `STPB` reader - STPB"]
pub type STPB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STPB` writer - STPB"]
pub type STPB_W<'a> = crate::FieldWriter<'a, u32, USART_CTRL2_SPEC, u8, u8, 2, 12>;
#[doc = "Field `LINMEN` reader - LINMEN"]
pub type LINMEN_R = crate::BitReader<bool>;
#[doc = "Field `LINMEN` writer - LINMEN"]
pub type LINMEN_W<'a> = crate::BitWriter<'a, u32, USART_CTRL2_SPEC, bool, 14>;
impl R {
    #[doc = "Bits 0:3 - ADDR"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - LINBDL"]
    #[inline(always)]
    pub fn linbdl(&self) -> LINBDL_R {
        LINBDL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LINBDIEN"]
    #[inline(always)]
    pub fn linbdien(&self) -> LINBDIEN_R {
        LINBDIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - LBCLK"]
    #[inline(always)]
    pub fn lbclk(&self) -> LBCLK_R {
        LBCLK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CLKPHA"]
    #[inline(always)]
    pub fn clkpha(&self) -> CLKPHA_R {
        CLKPHA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CLKPOL"]
    #[inline(always)]
    pub fn clkpol(&self) -> CLKPOL_R {
        CLKPOL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CLKEN"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STPB"]
    #[inline(always)]
    pub fn stpb(&self) -> STPB_R {
        STPB_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - LINMEN"]
    #[inline(always)]
    pub fn linmen(&self) -> LINMEN_R {
        LINMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDR"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W::new(self)
    }
    #[doc = "Bit 5 - LINBDL"]
    #[inline(always)]
    pub fn linbdl(&mut self) -> LINBDL_W {
        LINBDL_W::new(self)
    }
    #[doc = "Bit 6 - LINBDIEN"]
    #[inline(always)]
    pub fn linbdien(&mut self) -> LINBDIEN_W {
        LINBDIEN_W::new(self)
    }
    #[doc = "Bit 8 - LBCLK"]
    #[inline(always)]
    pub fn lbclk(&mut self) -> LBCLK_W {
        LBCLK_W::new(self)
    }
    #[doc = "Bit 9 - CLKPHA"]
    #[inline(always)]
    pub fn clkpha(&mut self) -> CLKPHA_W {
        CLKPHA_W::new(self)
    }
    #[doc = "Bit 10 - CLKPOL"]
    #[inline(always)]
    pub fn clkpol(&mut self) -> CLKPOL_W {
        CLKPOL_W::new(self)
    }
    #[doc = "Bit 11 - CLKEN"]
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W {
        CLKEN_W::new(self)
    }
    #[doc = "Bits 12:13 - STPB"]
    #[inline(always)]
    pub fn stpb(&mut self) -> STPB_W {
        STPB_W::new(self)
    }
    #[doc = "Bit 14 - LINMEN"]
    #[inline(always)]
    pub fn linmen(&mut self) -> LINMEN_W {
        LINMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART_CTRL2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_ctrl2](index.html) module"]
pub struct USART_CTRL2_SPEC;
impl crate::RegisterSpec for USART_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usart_ctrl2::R](R) reader structure"]
impl crate::Readable for USART_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usart_ctrl2::W](W) writer structure"]
impl crate::Writable for USART_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USART_CTRL2 to value 0"]
impl crate::Resettable for USART_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
