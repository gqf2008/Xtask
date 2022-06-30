#[doc = "Register `USART_CTRL1` reader"]
pub struct R(crate::R<USART_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USART_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USART_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USART_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USART_CTRL1` writer"]
pub struct W(crate::W<USART_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USART_CTRL1_SPEC>;
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
impl From<crate::W<USART_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USART_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDBRK` reader - SDBRK"]
pub type SDBRK_R = crate::BitReader<bool>;
#[doc = "Field `SDBRK` writer - SDBRK"]
pub type SDBRK_W<'a> = crate::BitWriter<'a, u32, USART_CTRL1_SPEC, bool, 0>;
#[doc = "Field `RCVWU` reader - RCVWU"]
pub type RCVWU_R = crate::BitReader<bool>;
#[doc = "Field `RCVWU` writer - RCVWU"]
pub type RCVWU_W<'a> = crate::BitWriter<'a, u32, USART_CTRL1_SPEC, bool, 1>;
#[doc = "Field `RXEN` reader - RXEN"]
pub type RXEN_R = crate::BitReader<bool>;
#[doc = "Field `RXEN` writer - RXEN"]
pub type RXEN_W<'a> = crate::BitWriter<'a, u32, USART_CTRL1_SPEC, bool, 2>;
#[doc = "Field `TXEN` reader - TXEN"]
pub type TXEN_R = crate::BitReader<bool>;
#[doc = "Field `TXEN` writer - TXEN"]
pub type TXEN_W<'a> = crate::BitWriter<'a, u32, USART_CTRL1_SPEC, bool, 3>;
#[doc = "Field `IDLEIEN` reader - IDLEIEN"]
pub type IDLEIEN_R = crate::BitReader<bool>;
#[doc = "Field `IDLEIEN` writer - IDLEIEN"]
pub type IDLEIEN_W<'a> = crate::BitWriter<'a, u32, USART_CTRL1_SPEC, bool, 4>;
#[doc = "Field `RXDNEIEN` reader - RXDNEIEN"]
pub type RXDNEIEN_R = crate::BitReader<bool>;
#[doc = "Field `RXDNEIEN` writer - RXDNEIEN"]
pub type RXDNEIEN_W<'a> = crate::BitWriter<'a, u32, USART_CTRL1_SPEC, bool, 5>;
#[doc = "Field `TXCIEN` reader - TXCIEN"]
pub type TXCIEN_R = crate::BitReader<bool>;
#[doc = "Field `TXCIEN` writer - TXCIEN"]
pub type TXCIEN_W<'a> = crate::BitWriter<'a, u32, USART_CTRL1_SPEC, bool, 6>;
#[doc = "Field `TXDEIEN` reader - TXDEIEN"]
pub type TXDEIEN_R = crate::BitReader<bool>;
#[doc = "Field `TXDEIEN` writer - TXDEIEN"]
pub type TXDEIEN_W<'a> = crate::BitWriter<'a, u32, USART_CTRL1_SPEC, bool, 7>;
#[doc = "Field `PEIEN` reader - PEIEN"]
pub type PEIEN_R = crate::BitReader<bool>;
#[doc = "Field `PEIEN` writer - PEIEN"]
pub type PEIEN_W<'a> = crate::BitWriter<'a, u32, USART_CTRL1_SPEC, bool, 8>;
#[doc = "Field `PSEL` reader - PSEL"]
pub type PSEL_R = crate::BitReader<bool>;
#[doc = "Field `PSEL` writer - PSEL"]
pub type PSEL_W<'a> = crate::BitWriter<'a, u32, USART_CTRL1_SPEC, bool, 9>;
#[doc = "Field `PCEN` reader - PCEN"]
pub type PCEN_R = crate::BitReader<bool>;
#[doc = "Field `PCEN` writer - PCEN"]
pub type PCEN_W<'a> = crate::BitWriter<'a, u32, USART_CTRL1_SPEC, bool, 10>;
#[doc = "Field `WUM` reader - WUM"]
pub type WUM_R = crate::BitReader<bool>;
#[doc = "Field `WUM` writer - WUM"]
pub type WUM_W<'a> = crate::BitWriter<'a, u32, USART_CTRL1_SPEC, bool, 11>;
#[doc = "Field `WL` reader - WL"]
pub type WL_R = crate::BitReader<bool>;
#[doc = "Field `WL` writer - WL"]
pub type WL_W<'a> = crate::BitWriter<'a, u32, USART_CTRL1_SPEC, bool, 12>;
#[doc = "Field `UEN` reader - UEN"]
pub type UEN_R = crate::BitReader<bool>;
#[doc = "Field `UEN` writer - UEN"]
pub type UEN_W<'a> = crate::BitWriter<'a, u32, USART_CTRL1_SPEC, bool, 13>;
impl R {
    #[doc = "Bit 0 - SDBRK"]
    #[inline(always)]
    pub fn sdbrk(&self) -> SDBRK_R {
        SDBRK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RCVWU"]
    #[inline(always)]
    pub fn rcvwu(&self) -> RCVWU_R {
        RCVWU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXEN"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TXEN"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLEIEN"]
    #[inline(always)]
    pub fn idleien(&self) -> IDLEIEN_R {
        IDLEIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXDNEIEN"]
    #[inline(always)]
    pub fn rxdneien(&self) -> RXDNEIEN_R {
        RXDNEIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXCIEN"]
    #[inline(always)]
    pub fn txcien(&self) -> TXCIEN_R {
        TXCIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXDEIEN"]
    #[inline(always)]
    pub fn txdeien(&self) -> TXDEIEN_R {
        TXDEIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PEIEN"]
    #[inline(always)]
    pub fn peien(&self) -> PEIEN_R {
        PEIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PSEL"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PCEN"]
    #[inline(always)]
    pub fn pcen(&self) -> PCEN_R {
        PCEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WUM"]
    #[inline(always)]
    pub fn wum(&self) -> WUM_R {
        WUM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - WL"]
    #[inline(always)]
    pub fn wl(&self) -> WL_R {
        WL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - UEN"]
    #[inline(always)]
    pub fn uen(&self) -> UEN_R {
        UEN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDBRK"]
    #[inline(always)]
    pub fn sdbrk(&mut self) -> SDBRK_W {
        SDBRK_W::new(self)
    }
    #[doc = "Bit 1 - RCVWU"]
    #[inline(always)]
    pub fn rcvwu(&mut self) -> RCVWU_W {
        RCVWU_W::new(self)
    }
    #[doc = "Bit 2 - RXEN"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RXEN_W {
        RXEN_W::new(self)
    }
    #[doc = "Bit 3 - TXEN"]
    #[inline(always)]
    pub fn txen(&mut self) -> TXEN_W {
        TXEN_W::new(self)
    }
    #[doc = "Bit 4 - IDLEIEN"]
    #[inline(always)]
    pub fn idleien(&mut self) -> IDLEIEN_W {
        IDLEIEN_W::new(self)
    }
    #[doc = "Bit 5 - RXDNEIEN"]
    #[inline(always)]
    pub fn rxdneien(&mut self) -> RXDNEIEN_W {
        RXDNEIEN_W::new(self)
    }
    #[doc = "Bit 6 - TXCIEN"]
    #[inline(always)]
    pub fn txcien(&mut self) -> TXCIEN_W {
        TXCIEN_W::new(self)
    }
    #[doc = "Bit 7 - TXDEIEN"]
    #[inline(always)]
    pub fn txdeien(&mut self) -> TXDEIEN_W {
        TXDEIEN_W::new(self)
    }
    #[doc = "Bit 8 - PEIEN"]
    #[inline(always)]
    pub fn peien(&mut self) -> PEIEN_W {
        PEIEN_W::new(self)
    }
    #[doc = "Bit 9 - PSEL"]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W {
        PSEL_W::new(self)
    }
    #[doc = "Bit 10 - PCEN"]
    #[inline(always)]
    pub fn pcen(&mut self) -> PCEN_W {
        PCEN_W::new(self)
    }
    #[doc = "Bit 11 - WUM"]
    #[inline(always)]
    pub fn wum(&mut self) -> WUM_W {
        WUM_W::new(self)
    }
    #[doc = "Bit 12 - WL"]
    #[inline(always)]
    pub fn wl(&mut self) -> WL_W {
        WL_W::new(self)
    }
    #[doc = "Bit 13 - UEN"]
    #[inline(always)]
    pub fn uen(&mut self) -> UEN_W {
        UEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART_CTRL1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_ctrl1](index.html) module"]
pub struct USART_CTRL1_SPEC;
impl crate::RegisterSpec for USART_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usart_ctrl1::R](R) reader structure"]
impl crate::Readable for USART_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usart_ctrl1::W](W) writer structure"]
impl crate::Writable for USART_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USART_CTRL1 to value 0"]
impl crate::Resettable for USART_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
