#[doc = "Register `PWR_CTRL6` reader"]
pub struct R(crate::R<PWR_CTRL6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CTRL6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CTRL6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CTRL6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_CTRL6` writer"]
pub struct W(crate::W<PWR_CTRL6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_CTRL6_SPEC>;
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
impl From<crate::W<PWR_CTRL6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_CTRL6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSEEN` reader - LSEEN"]
pub type LSEEN_R = crate::BitReader<bool>;
#[doc = "Field `LSEEN` writer - LSEEN"]
pub type LSEEN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 0>;
#[doc = "Field `LSEBYPSEL` reader - LSEBYPSEL"]
pub type LSEBYPSEL_R = crate::BitReader<bool>;
#[doc = "Field `LSEBYPSEL` writer - LSEBYPSEL"]
pub type LSEBYPSEL_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 1>;
#[doc = "Field `LSEBUFFSEL` reader - LSEBUFFSEL"]
pub type LSEBUFFSEL_R = crate::BitReader<bool>;
#[doc = "Field `LSEBUFFSEL` writer - LSEBUFFSEL"]
pub type LSEBUFFSEL_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 2>;
#[doc = "Field `LSEBTRSEL` reader - LSEBTRSEL"]
pub type LSEBTRSEL_R = crate::BitReader<bool>;
#[doc = "Field `LSEBTRSEL` writer - LSEBTRSEL"]
pub type LSEBTRSEL_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 3>;
#[doc = "Field `LSEAGSEL` reader - LSEAGSEL"]
pub type LSEAGSEL_R = crate::BitReader<bool>;
#[doc = "Field `LSEAGSEL` writer - LSEAGSEL"]
pub type LSEAGSEL_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 4>;
#[doc = "Field `LSEDRVTRSEL` reader - LSEDRVTRSEL"]
pub type LSEDRVTRSEL_R = crate::BitReader<bool>;
#[doc = "Field `LSEDRVTRSEL` writer - LSEDRVTRSEL"]
pub type LSEDRVTRSEL_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 5>;
#[doc = "Field `LSIEN` reader - LSIEN"]
pub type LSIEN_R = crate::BitReader<bool>;
#[doc = "Field `LSIEN` writer - LSIEN"]
pub type LSIEN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 6>;
#[doc = "Field `LSITRSEL` reader - LSITRSEL"]
pub type LSITRSEL_R = crate::BitReader<bool>;
#[doc = "Field `LSITRSEL` writer - LSITRSEL"]
pub type LSITRSEL_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 7>;
#[doc = "Field `BKRCSEL` reader - BKRCSEL"]
pub type BKRCSEL_R = crate::BitReader<bool>;
#[doc = "Field `BKRCSEL` writer - BKRCSEL"]
pub type BKRCSEL_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 8>;
#[doc = "Field `BKRTRSEL` reader - BKRTRSEL"]
pub type BKRTRSEL_R = crate::BitReader<bool>;
#[doc = "Field `BKRTRSEL` writer - BKRTRSEL"]
pub type BKRTRSEL_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 9>;
#[doc = "Field `BKRBSTRSEL` reader - BKRBSTRSEL"]
pub type BKRBSTRSEL_R = crate::BitReader<bool>;
#[doc = "Field `BKRBSTRSEL` writer - BKRBSTRSEL"]
pub type BKRBSTRSEL_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 10>;
#[doc = "Field `BKRPORTRSEL` reader - BKRPORTRSEL"]
pub type BKRPORTRSEL_R = crate::BitReader<bool>;
#[doc = "Field `BKRPORTRSEL` writer - BKRPORTRSEL"]
pub type BKRPORTRSEL_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 11>;
#[doc = "Field `DBGENSEL` reader - DBGENSEL"]
pub type DBGENSEL_R = crate::BitReader<bool>;
#[doc = "Field `DBGENSEL` writer - DBGENSEL"]
pub type DBGENSEL_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 12>;
#[doc = "Field `PA0WPEN` reader - PA0WPEN"]
pub type PA0WPEN_R = crate::BitReader<bool>;
#[doc = "Field `PA0WPEN` writer - PA0WPEN"]
pub type PA0WPEN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 13>;
#[doc = "Field `SW2EN` reader - SW2EN"]
pub type SW2EN_R = crate::BitReader<bool>;
#[doc = "Field `SW2EN` writer - SW2EN"]
pub type SW2EN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 14>;
#[doc = "Field `SW3EN` reader - SW3EN"]
pub type SW3EN_R = crate::BitReader<bool>;
#[doc = "Field `SW3EN` writer - SW3EN"]
pub type SW3EN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 15>;
#[doc = "Field `MREN` reader - MREN"]
pub type MREN_R = crate::BitReader<bool>;
#[doc = "Field `MREN` writer - MREN"]
pub type MREN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 16>;
#[doc = "Field `MRLPEN` reader - MRLPEN"]
pub type MRLPEN_R = crate::BitReader<bool>;
#[doc = "Field `MRLPEN` writer - MRLPEN"]
pub type MRLPEN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 17>;
#[doc = "Field `BGEN` reader - BGEN"]
pub type BGEN_R = crate::BitReader<bool>;
#[doc = "Field `BGEN` writer - BGEN"]
pub type BGEN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 18>;
#[doc = "Field `PVDEN` reader - PVDEN"]
pub type PVDEN_R = crate::BitReader<bool>;
#[doc = "Field `PVDEN` writer - PVDEN"]
pub type PVDEN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 19>;
#[doc = "Field `PVDSEL` reader - PVDSEL"]
pub type PVDSEL_R = crate::BitReader<bool>;
#[doc = "Field `PVDSEL` writer - PVDSEL"]
pub type PVDSEL_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 20>;
#[doc = "Field `MRTRSEL` reader - MRTRSEL"]
pub type MRTRSEL_R = crate::BitReader<bool>;
#[doc = "Field `MRTRSEL` writer - MRTRSEL"]
pub type MRTRSEL_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 21>;
#[doc = "Field `MRLPTRSEL` reader - MRLPTRSEL"]
pub type MRLPTRSEL_R = crate::BitReader<bool>;
#[doc = "Field `MRLPTRSEL` writer - MRLPTRSEL"]
pub type MRLPTRSEL_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 22>;
#[doc = "Field `FLCSEL` reader - FLCSEL"]
pub type FLCSEL_R = crate::BitReader<bool>;
#[doc = "Field `FLCSEL` writer - FLCSEL"]
pub type FLCSEL_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 23>;
#[doc = "Field `TSCEN` reader - TSCEN"]
pub type TSCEN_R = crate::BitReader<bool>;
#[doc = "Field `TSCEN` writer - TSCEN"]
pub type TSCEN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 24>;
#[doc = "Field `TSCPADSEL` reader - TSCPADSEL"]
pub type TSCPADSEL_R = crate::BitReader<bool>;
#[doc = "Field `TSCPADSEL` writer - TSCPADSEL"]
pub type TSCPADSEL_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 25>;
#[doc = "Field `TSCIBSEL` reader - TSCIBSEL"]
pub type TSCIBSEL_R = crate::BitReader<bool>;
#[doc = "Field `TSCIBSEL` writer - TSCIBSEL"]
pub type TSCIBSEL_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 26>;
#[doc = "Field `TSCSPSEL` reader - TSCSPSEL"]
pub type TSCSPSEL_R = crate::BitReader<bool>;
#[doc = "Field `TSCSPSEL` writer - TSCSPSEL"]
pub type TSCSPSEL_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 27>;
#[doc = "Field `TSCRSEL` reader - TSCRSEL"]
pub type TSCRSEL_R = crate::BitReader<bool>;
#[doc = "Field `TSCRSEL` writer - TSCRSEL"]
pub type TSCRSEL_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 28>;
#[doc = "Field `TSC_MUXSEL` reader - TSC_MUXSEL"]
pub type TSC_MUXSEL_R = crate::BitReader<bool>;
#[doc = "Field `TSC_MUXSEL` writer - TSC_MUXSEL"]
pub type TSC_MUXSEL_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 29>;
#[doc = "Field `TSCIBTEN` reader - TSCIBTEN"]
pub type TSCIBTEN_R = crate::BitReader<bool>;
#[doc = "Field `TSCIBTEN` writer - TSCIBTEN"]
pub type TSCIBTEN_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 30>;
#[doc = "Field `TSCTSEL` reader - TSCTSEL"]
pub type TSCTSEL_R = crate::BitReader<bool>;
#[doc = "Field `TSCTSEL` writer - TSCTSEL"]
pub type TSCTSEL_W<'a> = crate::BitWriter<'a, u32, PWR_CTRL6_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - LSEEN"]
    #[inline(always)]
    pub fn lseen(&self) -> LSEEN_R {
        LSEEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSEBYPSEL"]
    #[inline(always)]
    pub fn lsebypsel(&self) -> LSEBYPSEL_R {
        LSEBYPSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LSEBUFFSEL"]
    #[inline(always)]
    pub fn lsebuffsel(&self) -> LSEBUFFSEL_R {
        LSEBUFFSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LSEBTRSEL"]
    #[inline(always)]
    pub fn lsebtrsel(&self) -> LSEBTRSEL_R {
        LSEBTRSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LSEAGSEL"]
    #[inline(always)]
    pub fn lseagsel(&self) -> LSEAGSEL_R {
        LSEAGSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LSEDRVTRSEL"]
    #[inline(always)]
    pub fn lsedrvtrsel(&self) -> LSEDRVTRSEL_R {
        LSEDRVTRSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LSIEN"]
    #[inline(always)]
    pub fn lsien(&self) -> LSIEN_R {
        LSIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LSITRSEL"]
    #[inline(always)]
    pub fn lsitrsel(&self) -> LSITRSEL_R {
        LSITRSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BKRCSEL"]
    #[inline(always)]
    pub fn bkrcsel(&self) -> BKRCSEL_R {
        BKRCSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BKRTRSEL"]
    #[inline(always)]
    pub fn bkrtrsel(&self) -> BKRTRSEL_R {
        BKRTRSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BKRBSTRSEL"]
    #[inline(always)]
    pub fn bkrbstrsel(&self) -> BKRBSTRSEL_R {
        BKRBSTRSEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BKRPORTRSEL"]
    #[inline(always)]
    pub fn bkrportrsel(&self) -> BKRPORTRSEL_R {
        BKRPORTRSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DBGENSEL"]
    #[inline(always)]
    pub fn dbgensel(&self) -> DBGENSEL_R {
        DBGENSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PA0WPEN"]
    #[inline(always)]
    pub fn pa0wpen(&self) -> PA0WPEN_R {
        PA0WPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SW2EN"]
    #[inline(always)]
    pub fn sw2en(&self) -> SW2EN_R {
        SW2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SW3EN"]
    #[inline(always)]
    pub fn sw3en(&self) -> SW3EN_R {
        SW3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MREN"]
    #[inline(always)]
    pub fn mren(&self) -> MREN_R {
        MREN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MRLPEN"]
    #[inline(always)]
    pub fn mrlpen(&self) -> MRLPEN_R {
        MRLPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - BGEN"]
    #[inline(always)]
    pub fn bgen(&self) -> BGEN_R {
        BGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PVDEN"]
    #[inline(always)]
    pub fn pvden(&self) -> PVDEN_R {
        PVDEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PVDSEL"]
    #[inline(always)]
    pub fn pvdsel(&self) -> PVDSEL_R {
        PVDSEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MRTRSEL"]
    #[inline(always)]
    pub fn mrtrsel(&self) -> MRTRSEL_R {
        MRTRSEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MRLPTRSEL"]
    #[inline(always)]
    pub fn mrlptrsel(&self) -> MRLPTRSEL_R {
        MRLPTRSEL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - FLCSEL"]
    #[inline(always)]
    pub fn flcsel(&self) -> FLCSEL_R {
        FLCSEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TSCEN"]
    #[inline(always)]
    pub fn tscen(&self) -> TSCEN_R {
        TSCEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TSCPADSEL"]
    #[inline(always)]
    pub fn tscpadsel(&self) -> TSCPADSEL_R {
        TSCPADSEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TSCIBSEL"]
    #[inline(always)]
    pub fn tscibsel(&self) -> TSCIBSEL_R {
        TSCIBSEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TSCSPSEL"]
    #[inline(always)]
    pub fn tscspsel(&self) -> TSCSPSEL_R {
        TSCSPSEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - TSCRSEL"]
    #[inline(always)]
    pub fn tscrsel(&self) -> TSCRSEL_R {
        TSCRSEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TSC_MUXSEL"]
    #[inline(always)]
    pub fn tsc_muxsel(&self) -> TSC_MUXSEL_R {
        TSC_MUXSEL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TSCIBTEN"]
    #[inline(always)]
    pub fn tscibten(&self) -> TSCIBTEN_R {
        TSCIBTEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - TSCTSEL"]
    #[inline(always)]
    pub fn tsctsel(&self) -> TSCTSEL_R {
        TSCTSEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSEEN"]
    #[inline(always)]
    pub fn lseen(&mut self) -> LSEEN_W {
        LSEEN_W::new(self)
    }
    #[doc = "Bit 1 - LSEBYPSEL"]
    #[inline(always)]
    pub fn lsebypsel(&mut self) -> LSEBYPSEL_W {
        LSEBYPSEL_W::new(self)
    }
    #[doc = "Bit 2 - LSEBUFFSEL"]
    #[inline(always)]
    pub fn lsebuffsel(&mut self) -> LSEBUFFSEL_W {
        LSEBUFFSEL_W::new(self)
    }
    #[doc = "Bit 3 - LSEBTRSEL"]
    #[inline(always)]
    pub fn lsebtrsel(&mut self) -> LSEBTRSEL_W {
        LSEBTRSEL_W::new(self)
    }
    #[doc = "Bit 4 - LSEAGSEL"]
    #[inline(always)]
    pub fn lseagsel(&mut self) -> LSEAGSEL_W {
        LSEAGSEL_W::new(self)
    }
    #[doc = "Bit 5 - LSEDRVTRSEL"]
    #[inline(always)]
    pub fn lsedrvtrsel(&mut self) -> LSEDRVTRSEL_W {
        LSEDRVTRSEL_W::new(self)
    }
    #[doc = "Bit 6 - LSIEN"]
    #[inline(always)]
    pub fn lsien(&mut self) -> LSIEN_W {
        LSIEN_W::new(self)
    }
    #[doc = "Bit 7 - LSITRSEL"]
    #[inline(always)]
    pub fn lsitrsel(&mut self) -> LSITRSEL_W {
        LSITRSEL_W::new(self)
    }
    #[doc = "Bit 8 - BKRCSEL"]
    #[inline(always)]
    pub fn bkrcsel(&mut self) -> BKRCSEL_W {
        BKRCSEL_W::new(self)
    }
    #[doc = "Bit 9 - BKRTRSEL"]
    #[inline(always)]
    pub fn bkrtrsel(&mut self) -> BKRTRSEL_W {
        BKRTRSEL_W::new(self)
    }
    #[doc = "Bit 10 - BKRBSTRSEL"]
    #[inline(always)]
    pub fn bkrbstrsel(&mut self) -> BKRBSTRSEL_W {
        BKRBSTRSEL_W::new(self)
    }
    #[doc = "Bit 11 - BKRPORTRSEL"]
    #[inline(always)]
    pub fn bkrportrsel(&mut self) -> BKRPORTRSEL_W {
        BKRPORTRSEL_W::new(self)
    }
    #[doc = "Bit 12 - DBGENSEL"]
    #[inline(always)]
    pub fn dbgensel(&mut self) -> DBGENSEL_W {
        DBGENSEL_W::new(self)
    }
    #[doc = "Bit 13 - PA0WPEN"]
    #[inline(always)]
    pub fn pa0wpen(&mut self) -> PA0WPEN_W {
        PA0WPEN_W::new(self)
    }
    #[doc = "Bit 14 - SW2EN"]
    #[inline(always)]
    pub fn sw2en(&mut self) -> SW2EN_W {
        SW2EN_W::new(self)
    }
    #[doc = "Bit 15 - SW3EN"]
    #[inline(always)]
    pub fn sw3en(&mut self) -> SW3EN_W {
        SW3EN_W::new(self)
    }
    #[doc = "Bit 16 - MREN"]
    #[inline(always)]
    pub fn mren(&mut self) -> MREN_W {
        MREN_W::new(self)
    }
    #[doc = "Bit 17 - MRLPEN"]
    #[inline(always)]
    pub fn mrlpen(&mut self) -> MRLPEN_W {
        MRLPEN_W::new(self)
    }
    #[doc = "Bit 18 - BGEN"]
    #[inline(always)]
    pub fn bgen(&mut self) -> BGEN_W {
        BGEN_W::new(self)
    }
    #[doc = "Bit 19 - PVDEN"]
    #[inline(always)]
    pub fn pvden(&mut self) -> PVDEN_W {
        PVDEN_W::new(self)
    }
    #[doc = "Bit 20 - PVDSEL"]
    #[inline(always)]
    pub fn pvdsel(&mut self) -> PVDSEL_W {
        PVDSEL_W::new(self)
    }
    #[doc = "Bit 21 - MRTRSEL"]
    #[inline(always)]
    pub fn mrtrsel(&mut self) -> MRTRSEL_W {
        MRTRSEL_W::new(self)
    }
    #[doc = "Bit 22 - MRLPTRSEL"]
    #[inline(always)]
    pub fn mrlptrsel(&mut self) -> MRLPTRSEL_W {
        MRLPTRSEL_W::new(self)
    }
    #[doc = "Bit 23 - FLCSEL"]
    #[inline(always)]
    pub fn flcsel(&mut self) -> FLCSEL_W {
        FLCSEL_W::new(self)
    }
    #[doc = "Bit 24 - TSCEN"]
    #[inline(always)]
    pub fn tscen(&mut self) -> TSCEN_W {
        TSCEN_W::new(self)
    }
    #[doc = "Bit 25 - TSCPADSEL"]
    #[inline(always)]
    pub fn tscpadsel(&mut self) -> TSCPADSEL_W {
        TSCPADSEL_W::new(self)
    }
    #[doc = "Bit 26 - TSCIBSEL"]
    #[inline(always)]
    pub fn tscibsel(&mut self) -> TSCIBSEL_W {
        TSCIBSEL_W::new(self)
    }
    #[doc = "Bit 27 - TSCSPSEL"]
    #[inline(always)]
    pub fn tscspsel(&mut self) -> TSCSPSEL_W {
        TSCSPSEL_W::new(self)
    }
    #[doc = "Bit 28 - TSCRSEL"]
    #[inline(always)]
    pub fn tscrsel(&mut self) -> TSCRSEL_W {
        TSCRSEL_W::new(self)
    }
    #[doc = "Bit 29 - TSC_MUXSEL"]
    #[inline(always)]
    pub fn tsc_muxsel(&mut self) -> TSC_MUXSEL_W {
        TSC_MUXSEL_W::new(self)
    }
    #[doc = "Bit 30 - TSCIBTEN"]
    #[inline(always)]
    pub fn tscibten(&mut self) -> TSCIBTEN_W {
        TSCIBTEN_W::new(self)
    }
    #[doc = "Bit 31 - TSCTSEL"]
    #[inline(always)]
    pub fn tsctsel(&mut self) -> TSCTSEL_W {
        TSCTSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWR_CTRL6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_ctrl6](index.html) module"]
pub struct PWR_CTRL6_SPEC;
impl crate::RegisterSpec for PWR_CTRL6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_ctrl6::R](R) reader structure"]
impl crate::Readable for PWR_CTRL6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_ctrl6::W](W) writer structure"]
impl crate::Writable for PWR_CTRL6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_CTRL6 to value 0xffff_ffff"]
impl crate::Resettable for PWR_CTRL6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
