#[doc = "Register `CAN_INTE` reader"]
pub struct R(crate::R<CAN_INTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_INTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAN_INTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAN_INTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_INTE` writer"]
pub struct W(crate::W<CAN_INTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_INTE_SPEC>;
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
impl From<crate::W<CAN_INTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAN_INTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMEITE` reader - TMEITE"]
pub type TMEITE_R = crate::BitReader<bool>;
#[doc = "Field `TMEITE` writer - TMEITE"]
pub type TMEITE_W<'a> = crate::BitWriter<'a, u32, CAN_INTE_SPEC, bool, 0>;
#[doc = "Field `FMPITE0` reader - FMPITE0"]
pub type FMPITE0_R = crate::BitReader<bool>;
#[doc = "Field `FMPITE0` writer - FMPITE0"]
pub type FMPITE0_W<'a> = crate::BitWriter<'a, u32, CAN_INTE_SPEC, bool, 1>;
#[doc = "Field `FFITE0` reader - FFITE0"]
pub type FFITE0_R = crate::BitReader<bool>;
#[doc = "Field `FFITE0` writer - FFITE0"]
pub type FFITE0_W<'a> = crate::BitWriter<'a, u32, CAN_INTE_SPEC, bool, 2>;
#[doc = "Field `FOVITE0` reader - FOVITE0"]
pub type FOVITE0_R = crate::BitReader<bool>;
#[doc = "Field `FOVITE0` writer - FOVITE0"]
pub type FOVITE0_W<'a> = crate::BitWriter<'a, u32, CAN_INTE_SPEC, bool, 3>;
#[doc = "Field `FMPITE1` reader - FMPITE1"]
pub type FMPITE1_R = crate::BitReader<bool>;
#[doc = "Field `FMPITE1` writer - FMPITE1"]
pub type FMPITE1_W<'a> = crate::BitWriter<'a, u32, CAN_INTE_SPEC, bool, 4>;
#[doc = "Field `FFITE1` reader - FFITE1"]
pub type FFITE1_R = crate::BitReader<bool>;
#[doc = "Field `FFITE1` writer - FFITE1"]
pub type FFITE1_W<'a> = crate::BitWriter<'a, u32, CAN_INTE_SPEC, bool, 5>;
#[doc = "Field `FOVITE1` reader - FOVITE1"]
pub type FOVITE1_R = crate::BitReader<bool>;
#[doc = "Field `FOVITE1` writer - FOVITE1"]
pub type FOVITE1_W<'a> = crate::BitWriter<'a, u32, CAN_INTE_SPEC, bool, 6>;
#[doc = "Field `EWGITE` reader - EWGITE"]
pub type EWGITE_R = crate::BitReader<bool>;
#[doc = "Field `EWGITE` writer - EWGITE"]
pub type EWGITE_W<'a> = crate::BitWriter<'a, u32, CAN_INTE_SPEC, bool, 8>;
#[doc = "Field `EPVITE` reader - EPVITE"]
pub type EPVITE_R = crate::BitReader<bool>;
#[doc = "Field `EPVITE` writer - EPVITE"]
pub type EPVITE_W<'a> = crate::BitWriter<'a, u32, CAN_INTE_SPEC, bool, 9>;
#[doc = "Field `BOFITE` reader - BOFITE"]
pub type BOFITE_R = crate::BitReader<bool>;
#[doc = "Field `BOFITE` writer - BOFITE"]
pub type BOFITE_W<'a> = crate::BitWriter<'a, u32, CAN_INTE_SPEC, bool, 10>;
#[doc = "Field `LECITE` reader - LECITE"]
pub type LECITE_R = crate::BitReader<bool>;
#[doc = "Field `LECITE` writer - LECITE"]
pub type LECITE_W<'a> = crate::BitWriter<'a, u32, CAN_INTE_SPEC, bool, 11>;
#[doc = "Field `ERRITE` reader - ERRITE"]
pub type ERRITE_R = crate::BitReader<bool>;
#[doc = "Field `ERRITE` writer - ERRITE"]
pub type ERRITE_W<'a> = crate::BitWriter<'a, u32, CAN_INTE_SPEC, bool, 15>;
#[doc = "Field `WKUITE` reader - WKUITE"]
pub type WKUITE_R = crate::BitReader<bool>;
#[doc = "Field `WKUITE` writer - WKUITE"]
pub type WKUITE_W<'a> = crate::BitWriter<'a, u32, CAN_INTE_SPEC, bool, 16>;
#[doc = "Field `SLKITE` reader - SLKITE"]
pub type SLKITE_R = crate::BitReader<bool>;
#[doc = "Field `SLKITE` writer - SLKITE"]
pub type SLKITE_W<'a> = crate::BitWriter<'a, u32, CAN_INTE_SPEC, bool, 17>;
impl R {
    #[doc = "Bit 0 - TMEITE"]
    #[inline(always)]
    pub fn tmeite(&self) -> TMEITE_R {
        TMEITE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FMPITE0"]
    #[inline(always)]
    pub fn fmpite0(&self) -> FMPITE0_R {
        FMPITE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FFITE0"]
    #[inline(always)]
    pub fn ffite0(&self) -> FFITE0_R {
        FFITE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FOVITE0"]
    #[inline(always)]
    pub fn fovite0(&self) -> FOVITE0_R {
        FOVITE0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FMPITE1"]
    #[inline(always)]
    pub fn fmpite1(&self) -> FMPITE1_R {
        FMPITE1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FFITE1"]
    #[inline(always)]
    pub fn ffite1(&self) -> FFITE1_R {
        FFITE1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FOVITE1"]
    #[inline(always)]
    pub fn fovite1(&self) -> FOVITE1_R {
        FOVITE1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - EWGITE"]
    #[inline(always)]
    pub fn ewgite(&self) -> EWGITE_R {
        EWGITE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EPVITE"]
    #[inline(always)]
    pub fn epvite(&self) -> EPVITE_R {
        EPVITE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BOFITE"]
    #[inline(always)]
    pub fn bofite(&self) -> BOFITE_R {
        BOFITE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LECITE"]
    #[inline(always)]
    pub fn lecite(&self) -> LECITE_R {
        LECITE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - ERRITE"]
    #[inline(always)]
    pub fn errite(&self) -> ERRITE_R {
        ERRITE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - WKUITE"]
    #[inline(always)]
    pub fn wkuite(&self) -> WKUITE_R {
        WKUITE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SLKITE"]
    #[inline(always)]
    pub fn slkite(&self) -> SLKITE_R {
        SLKITE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TMEITE"]
    #[inline(always)]
    pub fn tmeite(&mut self) -> TMEITE_W {
        TMEITE_W::new(self)
    }
    #[doc = "Bit 1 - FMPITE0"]
    #[inline(always)]
    pub fn fmpite0(&mut self) -> FMPITE0_W {
        FMPITE0_W::new(self)
    }
    #[doc = "Bit 2 - FFITE0"]
    #[inline(always)]
    pub fn ffite0(&mut self) -> FFITE0_W {
        FFITE0_W::new(self)
    }
    #[doc = "Bit 3 - FOVITE0"]
    #[inline(always)]
    pub fn fovite0(&mut self) -> FOVITE0_W {
        FOVITE0_W::new(self)
    }
    #[doc = "Bit 4 - FMPITE1"]
    #[inline(always)]
    pub fn fmpite1(&mut self) -> FMPITE1_W {
        FMPITE1_W::new(self)
    }
    #[doc = "Bit 5 - FFITE1"]
    #[inline(always)]
    pub fn ffite1(&mut self) -> FFITE1_W {
        FFITE1_W::new(self)
    }
    #[doc = "Bit 6 - FOVITE1"]
    #[inline(always)]
    pub fn fovite1(&mut self) -> FOVITE1_W {
        FOVITE1_W::new(self)
    }
    #[doc = "Bit 8 - EWGITE"]
    #[inline(always)]
    pub fn ewgite(&mut self) -> EWGITE_W {
        EWGITE_W::new(self)
    }
    #[doc = "Bit 9 - EPVITE"]
    #[inline(always)]
    pub fn epvite(&mut self) -> EPVITE_W {
        EPVITE_W::new(self)
    }
    #[doc = "Bit 10 - BOFITE"]
    #[inline(always)]
    pub fn bofite(&mut self) -> BOFITE_W {
        BOFITE_W::new(self)
    }
    #[doc = "Bit 11 - LECITE"]
    #[inline(always)]
    pub fn lecite(&mut self) -> LECITE_W {
        LECITE_W::new(self)
    }
    #[doc = "Bit 15 - ERRITE"]
    #[inline(always)]
    pub fn errite(&mut self) -> ERRITE_W {
        ERRITE_W::new(self)
    }
    #[doc = "Bit 16 - WKUITE"]
    #[inline(always)]
    pub fn wkuite(&mut self) -> WKUITE_W {
        WKUITE_W::new(self)
    }
    #[doc = "Bit 17 - SLKITE"]
    #[inline(always)]
    pub fn slkite(&mut self) -> SLKITE_W {
        SLKITE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN_INTE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_inte](index.html) module"]
pub struct CAN_INTE_SPEC;
impl crate::RegisterSpec for CAN_INTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_inte::R](R) reader structure"]
impl crate::Readable for CAN_INTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_inte::W](W) writer structure"]
impl crate::Writable for CAN_INTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_INTE to value 0"]
impl crate::Resettable for CAN_INTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
