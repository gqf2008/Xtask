#[doc = "Register `FLASH_CTRL` reader"]
pub struct R(crate::R<FLASH_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_CTRL` writer"]
pub struct W(crate::W<FLASH_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_CTRL_SPEC>;
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
impl From<crate::W<FLASH_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PG` reader - PG"]
pub type PG_R = crate::BitReader<bool>;
#[doc = "Field `PG` writer - PG"]
pub type PG_W<'a> = crate::BitWriter<'a, u32, FLASH_CTRL_SPEC, bool, 0>;
#[doc = "Field `PER` reader - PER"]
pub type PER_R = crate::BitReader<bool>;
#[doc = "Field `PER` writer - PER"]
pub type PER_W<'a> = crate::BitWriter<'a, u32, FLASH_CTRL_SPEC, bool, 1>;
#[doc = "Field `MER` reader - MER"]
pub type MER_R = crate::BitReader<bool>;
#[doc = "Field `MER` writer - MER"]
pub type MER_W<'a> = crate::BitWriter<'a, u32, FLASH_CTRL_SPEC, bool, 2>;
#[doc = "Field `OPTPG` reader - OPTPG"]
pub type OPTPG_R = crate::BitReader<bool>;
#[doc = "Field `OPTPG` writer - OPTPG"]
pub type OPTPG_W<'a> = crate::BitWriter<'a, u32, FLASH_CTRL_SPEC, bool, 4>;
#[doc = "Field `OPTER` reader - OPTER"]
pub type OPTER_R = crate::BitReader<bool>;
#[doc = "Field `OPTER` writer - OPTER"]
pub type OPTER_W<'a> = crate::BitWriter<'a, u32, FLASH_CTRL_SPEC, bool, 5>;
#[doc = "Field `START` reader - START"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - START"]
pub type START_W<'a> = crate::BitWriter<'a, u32, FLASH_CTRL_SPEC, bool, 6>;
#[doc = "Field `LOCK` reader - LOCK"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - LOCK"]
pub type LOCK_W<'a> = crate::BitWriter<'a, u32, FLASH_CTRL_SPEC, bool, 7>;
#[doc = "Field `SMPSEL` reader - SMPSEL"]
pub type SMPSEL_R = crate::BitReader<bool>;
#[doc = "Field `SMPSEL` writer - SMPSEL"]
pub type SMPSEL_W<'a> = crate::BitWriter<'a, u32, FLASH_CTRL_SPEC, bool, 8>;
#[doc = "Field `OPTWE` reader - OPTWE"]
pub type OPTWE_R = crate::BitReader<bool>;
#[doc = "Field `OPTWE` writer - OPTWE"]
pub type OPTWE_W<'a> = crate::BitWriter<'a, u32, FLASH_CTRL_SPEC, bool, 9>;
#[doc = "Field `ERRITE` reader - ERRITE"]
pub type ERRITE_R = crate::BitReader<bool>;
#[doc = "Field `ERRITE` writer - ERRITE"]
pub type ERRITE_W<'a> = crate::BitWriter<'a, u32, FLASH_CTRL_SPEC, bool, 10>;
#[doc = "Field `FERRITE` reader - FERRITE"]
pub type FERRITE_R = crate::BitReader<bool>;
#[doc = "Field `FERRITE` writer - FERRITE"]
pub type FERRITE_W<'a> = crate::BitWriter<'a, u32, FLASH_CTRL_SPEC, bool, 11>;
#[doc = "Field `EOPITE` reader - EOPITE"]
pub type EOPITE_R = crate::BitReader<bool>;
#[doc = "Field `EOPITE` writer - EOPITE"]
pub type EOPITE_W<'a> = crate::BitWriter<'a, u32, FLASH_CTRL_SPEC, bool, 12>;
#[doc = "Field `ECCERRITE` reader - ECCERRITE"]
pub type ECCERRITE_R = crate::BitReader<bool>;
#[doc = "Field `ECCERRITE` writer - ECCERRITE"]
pub type ECCERRITE_W<'a> = crate::BitWriter<'a, u32, FLASH_CTRL_SPEC, bool, 13>;
impl R {
    #[doc = "Bit 0 - PG"]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PER"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MER"]
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - OPTPG"]
    #[inline(always)]
    pub fn optpg(&self) -> OPTPG_R {
        OPTPG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OPTER"]
    #[inline(always)]
    pub fn opter(&self) -> OPTER_R {
        OPTER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - START"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SMPSEL"]
    #[inline(always)]
    pub fn smpsel(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OPTWE"]
    #[inline(always)]
    pub fn optwe(&self) -> OPTWE_R {
        OPTWE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ERRITE"]
    #[inline(always)]
    pub fn errite(&self) -> ERRITE_R {
        ERRITE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FERRITE"]
    #[inline(always)]
    pub fn ferrite(&self) -> FERRITE_R {
        FERRITE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EOPITE"]
    #[inline(always)]
    pub fn eopite(&self) -> EOPITE_R {
        EOPITE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ECCERRITE"]
    #[inline(always)]
    pub fn eccerrite(&self) -> ECCERRITE_R {
        ECCERRITE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PG"]
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W {
        PG_W::new(self)
    }
    #[doc = "Bit 1 - PER"]
    #[inline(always)]
    pub fn per(&mut self) -> PER_W {
        PER_W::new(self)
    }
    #[doc = "Bit 2 - MER"]
    #[inline(always)]
    pub fn mer(&mut self) -> MER_W {
        MER_W::new(self)
    }
    #[doc = "Bit 4 - OPTPG"]
    #[inline(always)]
    pub fn optpg(&mut self) -> OPTPG_W {
        OPTPG_W::new(self)
    }
    #[doc = "Bit 5 - OPTER"]
    #[inline(always)]
    pub fn opter(&mut self) -> OPTER_W {
        OPTER_W::new(self)
    }
    #[doc = "Bit 6 - START"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W::new(self)
    }
    #[doc = "Bit 7 - LOCK"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W::new(self)
    }
    #[doc = "Bit 8 - SMPSEL"]
    #[inline(always)]
    pub fn smpsel(&mut self) -> SMPSEL_W {
        SMPSEL_W::new(self)
    }
    #[doc = "Bit 9 - OPTWE"]
    #[inline(always)]
    pub fn optwe(&mut self) -> OPTWE_W {
        OPTWE_W::new(self)
    }
    #[doc = "Bit 10 - ERRITE"]
    #[inline(always)]
    pub fn errite(&mut self) -> ERRITE_W {
        ERRITE_W::new(self)
    }
    #[doc = "Bit 11 - FERRITE"]
    #[inline(always)]
    pub fn ferrite(&mut self) -> FERRITE_W {
        FERRITE_W::new(self)
    }
    #[doc = "Bit 12 - EOPITE"]
    #[inline(always)]
    pub fn eopite(&mut self) -> EOPITE_W {
        EOPITE_W::new(self)
    }
    #[doc = "Bit 13 - ECCERRITE"]
    #[inline(always)]
    pub fn eccerrite(&mut self) -> ECCERRITE_W {
        ECCERRITE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLASH_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ctrl](index.html) module"]
pub struct FLASH_CTRL_SPEC;
impl crate::RegisterSpec for FLASH_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_ctrl::R](R) reader structure"]
impl crate::Readable for FLASH_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_ctrl::W](W) writer structure"]
impl crate::Writable for FLASH_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_CTRL to value 0x80"]
impl crate::Resettable for FLASH_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
