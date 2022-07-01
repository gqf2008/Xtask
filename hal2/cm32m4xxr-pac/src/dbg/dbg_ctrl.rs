#[doc = "Register `DBG_CTRL` reader"]
pub struct R(crate::R<DBG_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBG_CTRL` writer"]
pub struct W(crate::W<DBG_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBG_CTRL_SPEC>;
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
impl From<crate::W<DBG_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBG_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEEP` reader - SLEEP"]
pub type SLEEP_R = crate::BitReader<bool>;
#[doc = "Field `SLEEP` writer - SLEEP"]
pub type SLEEP_W<'a> = crate::BitWriter<'a, u32, DBG_CTRL_SPEC, bool, 0>;
#[doc = "Field `STOP` reader - STOP"]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - STOP"]
pub type STOP_W<'a> = crate::BitWriter<'a, u32, DBG_CTRL_SPEC, bool, 1>;
#[doc = "Field `STDBY` reader - STDBY"]
pub type STDBY_R = crate::BitReader<bool>;
#[doc = "Field `STDBY` writer - STDBY"]
pub type STDBY_W<'a> = crate::BitWriter<'a, u32, DBG_CTRL_SPEC, bool, 2>;
#[doc = "Field `IWDG_STOP` reader - IWDG_STOP"]
pub type IWDG_STOP_R = crate::BitReader<bool>;
#[doc = "Field `IWDG_STOP` writer - IWDG_STOP"]
pub type IWDG_STOP_W<'a> = crate::BitWriter<'a, u32, DBG_CTRL_SPEC, bool, 8>;
#[doc = "Field `WWDG_STOP` reader - WWDG_STOP"]
pub type WWDG_STOP_R = crate::BitReader<bool>;
#[doc = "Field `WWDG_STOP` writer - WWDG_STOP"]
pub type WWDG_STOP_W<'a> = crate::BitWriter<'a, u32, DBG_CTRL_SPEC, bool, 9>;
#[doc = "Field `TIM1_STOP` reader - TIM1_STOP"]
pub type TIM1_STOP_R = crate::BitReader<bool>;
#[doc = "Field `TIM1_STOP` writer - TIM1_STOP"]
pub type TIM1_STOP_W<'a> = crate::BitWriter<'a, u32, DBG_CTRL_SPEC, bool, 10>;
#[doc = "Field `TIM2_STOP` reader - TIM2_STOP"]
pub type TIM2_STOP_R = crate::BitReader<bool>;
#[doc = "Field `TIM2_STOP` writer - TIM2_STOP"]
pub type TIM2_STOP_W<'a> = crate::BitWriter<'a, u32, DBG_CTRL_SPEC, bool, 11>;
#[doc = "Field `TIM3_STOP` reader - TIM3_STOP"]
pub type TIM3_STOP_R = crate::BitReader<bool>;
#[doc = "Field `TIM3_STOP` writer - TIM3_STOP"]
pub type TIM3_STOP_W<'a> = crate::BitWriter<'a, u32, DBG_CTRL_SPEC, bool, 12>;
#[doc = "Field `TIM4_STOP` reader - TIM4_STOP"]
pub type TIM4_STOP_R = crate::BitReader<bool>;
#[doc = "Field `TIM4_STOP` writer - TIM4_STOP"]
pub type TIM4_STOP_W<'a> = crate::BitWriter<'a, u32, DBG_CTRL_SPEC, bool, 13>;
#[doc = "Field `CAN1_STOP` reader - CAN1_STOP"]
pub type CAN1_STOP_R = crate::BitReader<bool>;
#[doc = "Field `CAN1_STOP` writer - CAN1_STOP"]
pub type CAN1_STOP_W<'a> = crate::BitWriter<'a, u32, DBG_CTRL_SPEC, bool, 14>;
#[doc = "Field `I2C1SMBUS_TIMEOUT` reader - I2C1SMBUS_TIMEOUT"]
pub type I2C1SMBUS_TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `I2C1SMBUS_TIMEOUT` writer - I2C1SMBUS_TIMEOUT"]
pub type I2C1SMBUS_TIMEOUT_W<'a> = crate::BitWriter<'a, u32, DBG_CTRL_SPEC, bool, 15>;
#[doc = "Field `I2C2SMBUS_TIMEOUT` reader - I2C2SMBUS_TIMEOUT"]
pub type I2C2SMBUS_TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `I2C2SMBUS_TIMEOUT` writer - I2C2SMBUS_TIMEOUT"]
pub type I2C2SMBUS_TIMEOUT_W<'a> = crate::BitWriter<'a, u32, DBG_CTRL_SPEC, bool, 16>;
#[doc = "Field `TIM8_STOP` reader - TIM8_STOP"]
pub type TIM8_STOP_R = crate::BitReader<bool>;
#[doc = "Field `TIM8_STOP` writer - TIM8_STOP"]
pub type TIM8_STOP_W<'a> = crate::BitWriter<'a, u32, DBG_CTRL_SPEC, bool, 17>;
#[doc = "Field `TIM5_STOP` reader - TIM5_STOP"]
pub type TIM5_STOP_R = crate::BitReader<bool>;
#[doc = "Field `TIM5_STOP` writer - TIM5_STOP"]
pub type TIM5_STOP_W<'a> = crate::BitWriter<'a, u32, DBG_CTRL_SPEC, bool, 18>;
#[doc = "Field `TIM6_STOP` reader - TIM6_STOP"]
pub type TIM6_STOP_R = crate::BitReader<bool>;
#[doc = "Field `TIM6_STOP` writer - TIM6_STOP"]
pub type TIM6_STOP_W<'a> = crate::BitWriter<'a, u32, DBG_CTRL_SPEC, bool, 19>;
#[doc = "Field `TIM7_STOP` reader - TIM7_STOP"]
pub type TIM7_STOP_R = crate::BitReader<bool>;
#[doc = "Field `TIM7_STOP` writer - TIM7_STOP"]
pub type TIM7_STOP_W<'a> = crate::BitWriter<'a, u32, DBG_CTRL_SPEC, bool, 20>;
#[doc = "Field `CAN2_STOP` reader - CAN2_STOP"]
pub type CAN2_STOP_R = crate::BitReader<bool>;
#[doc = "Field `CAN2_STOP` writer - CAN2_STOP"]
pub type CAN2_STOP_W<'a> = crate::BitWriter<'a, u32, DBG_CTRL_SPEC, bool, 21>;
#[doc = "Field `DWJTAG_BYPASS` reader - DWJTAG_BYPASS"]
pub type DWJTAG_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `DWJTAG_BYPASS` writer - DWJTAG_BYPASS"]
pub type DWJTAG_BYPASS_W<'a> = crate::BitWriter<'a, u32, DBG_CTRL_SPEC, bool, 22>;
#[doc = "Field `DBG_DSLEEP_EN` reader - DBG_DSLEEP_EN"]
pub type DBG_DSLEEP_EN_R = crate::BitReader<bool>;
#[doc = "Field `DBG_DSLEEP_EN` writer - DBG_DSLEEP_EN"]
pub type DBG_DSLEEP_EN_W<'a> = crate::BitWriter<'a, u32, DBG_CTRL_SPEC, bool, 23>;
impl R {
    #[doc = "Bit 0 - SLEEP"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STOP"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STDBY"]
    #[inline(always)]
    pub fn stdby(&self) -> STDBY_R {
        STDBY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - IWDG_STOP"]
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WWDG_STOP"]
    #[inline(always)]
    pub fn wwdg_stop(&self) -> WWDG_STOP_R {
        WWDG_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TIM1_STOP"]
    #[inline(always)]
    pub fn tim1_stop(&self) -> TIM1_STOP_R {
        TIM1_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM2_STOP"]
    #[inline(always)]
    pub fn tim2_stop(&self) -> TIM2_STOP_R {
        TIM2_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TIM3_STOP"]
    #[inline(always)]
    pub fn tim3_stop(&self) -> TIM3_STOP_R {
        TIM3_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM4_STOP"]
    #[inline(always)]
    pub fn tim4_stop(&self) -> TIM4_STOP_R {
        TIM4_STOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CAN1_STOP"]
    #[inline(always)]
    pub fn can1_stop(&self) -> CAN1_STOP_R {
        CAN1_STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C1SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn i2c1smbus_timeout(&self) -> I2C1SMBUS_TIMEOUT_R {
        I2C1SMBUS_TIMEOUT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - I2C2SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn i2c2smbus_timeout(&self) -> I2C2SMBUS_TIMEOUT_R {
        I2C2SMBUS_TIMEOUT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM8_STOP"]
    #[inline(always)]
    pub fn tim8_stop(&self) -> TIM8_STOP_R {
        TIM8_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM5_STOP"]
    #[inline(always)]
    pub fn tim5_stop(&self) -> TIM5_STOP_R {
        TIM5_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TIM6_STOP"]
    #[inline(always)]
    pub fn tim6_stop(&self) -> TIM6_STOP_R {
        TIM6_STOP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TIM7_STOP"]
    #[inline(always)]
    pub fn tim7_stop(&self) -> TIM7_STOP_R {
        TIM7_STOP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CAN2_STOP"]
    #[inline(always)]
    pub fn can2_stop(&self) -> CAN2_STOP_R {
        CAN2_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DWJTAG_BYPASS"]
    #[inline(always)]
    pub fn dwjtag_bypass(&self) -> DWJTAG_BYPASS_R {
        DWJTAG_BYPASS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DBG_DSLEEP_EN"]
    #[inline(always)]
    pub fn dbg_dsleep_en(&self) -> DBG_DSLEEP_EN_R {
        DBG_DSLEEP_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SLEEP"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W {
        SLEEP_W::new(self)
    }
    #[doc = "Bit 1 - STOP"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W::new(self)
    }
    #[doc = "Bit 2 - STDBY"]
    #[inline(always)]
    pub fn stdby(&mut self) -> STDBY_W {
        STDBY_W::new(self)
    }
    #[doc = "Bit 8 - IWDG_STOP"]
    #[inline(always)]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W {
        IWDG_STOP_W::new(self)
    }
    #[doc = "Bit 9 - WWDG_STOP"]
    #[inline(always)]
    pub fn wwdg_stop(&mut self) -> WWDG_STOP_W {
        WWDG_STOP_W::new(self)
    }
    #[doc = "Bit 10 - TIM1_STOP"]
    #[inline(always)]
    pub fn tim1_stop(&mut self) -> TIM1_STOP_W {
        TIM1_STOP_W::new(self)
    }
    #[doc = "Bit 11 - TIM2_STOP"]
    #[inline(always)]
    pub fn tim2_stop(&mut self) -> TIM2_STOP_W {
        TIM2_STOP_W::new(self)
    }
    #[doc = "Bit 12 - TIM3_STOP"]
    #[inline(always)]
    pub fn tim3_stop(&mut self) -> TIM3_STOP_W {
        TIM3_STOP_W::new(self)
    }
    #[doc = "Bit 13 - TIM4_STOP"]
    #[inline(always)]
    pub fn tim4_stop(&mut self) -> TIM4_STOP_W {
        TIM4_STOP_W::new(self)
    }
    #[doc = "Bit 14 - CAN1_STOP"]
    #[inline(always)]
    pub fn can1_stop(&mut self) -> CAN1_STOP_W {
        CAN1_STOP_W::new(self)
    }
    #[doc = "Bit 15 - I2C1SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn i2c1smbus_timeout(&mut self) -> I2C1SMBUS_TIMEOUT_W {
        I2C1SMBUS_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 16 - I2C2SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn i2c2smbus_timeout(&mut self) -> I2C2SMBUS_TIMEOUT_W {
        I2C2SMBUS_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 17 - TIM8_STOP"]
    #[inline(always)]
    pub fn tim8_stop(&mut self) -> TIM8_STOP_W {
        TIM8_STOP_W::new(self)
    }
    #[doc = "Bit 18 - TIM5_STOP"]
    #[inline(always)]
    pub fn tim5_stop(&mut self) -> TIM5_STOP_W {
        TIM5_STOP_W::new(self)
    }
    #[doc = "Bit 19 - TIM6_STOP"]
    #[inline(always)]
    pub fn tim6_stop(&mut self) -> TIM6_STOP_W {
        TIM6_STOP_W::new(self)
    }
    #[doc = "Bit 20 - TIM7_STOP"]
    #[inline(always)]
    pub fn tim7_stop(&mut self) -> TIM7_STOP_W {
        TIM7_STOP_W::new(self)
    }
    #[doc = "Bit 21 - CAN2_STOP"]
    #[inline(always)]
    pub fn can2_stop(&mut self) -> CAN2_STOP_W {
        CAN2_STOP_W::new(self)
    }
    #[doc = "Bit 22 - DWJTAG_BYPASS"]
    #[inline(always)]
    pub fn dwjtag_bypass(&mut self) -> DWJTAG_BYPASS_W {
        DWJTAG_BYPASS_W::new(self)
    }
    #[doc = "Bit 23 - DBG_DSLEEP_EN"]
    #[inline(always)]
    pub fn dbg_dsleep_en(&mut self) -> DBG_DSLEEP_EN_W {
        DBG_DSLEEP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBG_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_ctrl](index.html) module"]
pub struct DBG_CTRL_SPEC;
impl crate::RegisterSpec for DBG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_ctrl::R](R) reader structure"]
impl crate::Readable for DBG_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbg_ctrl::W](W) writer structure"]
impl crate::Writable for DBG_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBG_CTRL to value 0"]
impl crate::Resettable for DBG_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
