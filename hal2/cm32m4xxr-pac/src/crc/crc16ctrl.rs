#[doc = "Register `CRC16CTRL` reader"]
pub struct R(crate::R<CRC16CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC16CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC16CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC16CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC16CTRL` writer"]
pub struct W(crate::W<CRC16CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC16CTRL_SPEC>;
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
impl From<crate::W<CRC16CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC16CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENDHL` reader - ENDHL"]
pub type ENDHL_R = crate::BitReader<bool>;
#[doc = "Field `ENDHL` writer - ENDHL"]
pub type ENDHL_W<'a> = crate::BitWriter<'a, u32, CRC16CTRL_SPEC, bool, 1>;
#[doc = "Field `CLR` reader - CLR"]
pub type CLR_R = crate::BitReader<bool>;
#[doc = "Field `CLR` writer - CLR"]
pub type CLR_W<'a> = crate::BitWriter<'a, u32, CRC16CTRL_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 1 - ENDHL"]
    #[inline(always)]
    pub fn endhl(&self) -> ENDHL_R {
        ENDHL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CLR"]
    #[inline(always)]
    pub fn clr(&self) -> CLR_R {
        CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - ENDHL"]
    #[inline(always)]
    pub fn endhl(&mut self) -> ENDHL_W {
        ENDHL_W::new(self)
    }
    #[doc = "Bit 2 - CLR"]
    #[inline(always)]
    pub fn clr(&mut self) -> CLR_W {
        CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC16CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc16ctrl](index.html) module"]
pub struct CRC16CTRL_SPEC;
impl crate::RegisterSpec for CRC16CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc16ctrl::R](R) reader structure"]
impl crate::Readable for CRC16CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc16ctrl::W](W) writer structure"]
impl crate::Writable for CRC16CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC16CTRL to value 0"]
impl crate::Resettable for CRC16CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
