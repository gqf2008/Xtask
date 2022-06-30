#[doc = "Register `DBG_ID` reader"]
pub struct R(crate::R<DBG_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBG_ID` writer"]
pub struct W(crate::W<DBG_ID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBG_ID_SPEC>;
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
impl From<crate::W<DBG_ID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBG_ID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REV_NUM_L` reader - REV_NUM_L"]
pub type REV_NUM_L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REV_NUM_L` writer - REV_NUM_L"]
pub type REV_NUM_L_W<'a> = crate::FieldWriter<'a, u32, DBG_ID_SPEC, u8, u8, 4, 0>;
#[doc = "Field `REV_NUM_H` reader - REV_NUM_H"]
pub type REV_NUM_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REV_NUM_H` writer - REV_NUM_H"]
pub type REV_NUM_H_W<'a> = crate::FieldWriter<'a, u32, DBG_ID_SPEC, u8, u8, 4, 4>;
#[doc = "Field `DEV_NUM_M` reader - DEV_NUM_M"]
pub type DEV_NUM_M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEV_NUM_M` writer - DEV_NUM_M"]
pub type DEV_NUM_M_W<'a> = crate::FieldWriter<'a, u32, DBG_ID_SPEC, u8, u8, 4, 8>;
#[doc = "Field `DEV_NUM_H` reader - DEV_NUM_H"]
pub type DEV_NUM_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEV_NUM_H` writer - DEV_NUM_H"]
pub type DEV_NUM_H_W<'a> = crate::FieldWriter<'a, u32, DBG_ID_SPEC, u8, u8, 4, 12>;
#[doc = "Field `FLASH` reader - FLASH"]
pub type FLASH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLASH` writer - FLASH"]
pub type FLASH_W<'a> = crate::FieldWriter<'a, u32, DBG_ID_SPEC, u8, u8, 4, 16>;
#[doc = "Field `DEV_NUM_L` reader - DEV_NUM_L"]
pub type DEV_NUM_L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEV_NUM_L` writer - DEV_NUM_L"]
pub type DEV_NUM_L_W<'a> = crate::FieldWriter<'a, u32, DBG_ID_SPEC, u8, u8, 4, 20>;
#[doc = "Field `SRAM` reader - SRAM"]
pub type SRAM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRAM` writer - SRAM"]
pub type SRAM_W<'a> = crate::FieldWriter<'a, u32, DBG_ID_SPEC, u8, u8, 4, 28>;
impl R {
    #[doc = "Bits 0:3 - REV_NUM_L"]
    #[inline(always)]
    pub fn rev_num_l(&self) -> REV_NUM_L_R {
        REV_NUM_L_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - REV_NUM_H"]
    #[inline(always)]
    pub fn rev_num_h(&self) -> REV_NUM_H_R {
        REV_NUM_H_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DEV_NUM_M"]
    #[inline(always)]
    pub fn dev_num_m(&self) -> DEV_NUM_M_R {
        DEV_NUM_M_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DEV_NUM_H"]
    #[inline(always)]
    pub fn dev_num_h(&self) -> DEV_NUM_H_R {
        DEV_NUM_H_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - FLASH"]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - DEV_NUM_L"]
    #[inline(always)]
    pub fn dev_num_l(&self) -> DEV_NUM_L_R {
        DEV_NUM_L_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - SRAM"]
    #[inline(always)]
    pub fn sram(&self) -> SRAM_R {
        SRAM_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - REV_NUM_L"]
    #[inline(always)]
    pub fn rev_num_l(&mut self) -> REV_NUM_L_W {
        REV_NUM_L_W::new(self)
    }
    #[doc = "Bits 4:7 - REV_NUM_H"]
    #[inline(always)]
    pub fn rev_num_h(&mut self) -> REV_NUM_H_W {
        REV_NUM_H_W::new(self)
    }
    #[doc = "Bits 8:11 - DEV_NUM_M"]
    #[inline(always)]
    pub fn dev_num_m(&mut self) -> DEV_NUM_M_W {
        DEV_NUM_M_W::new(self)
    }
    #[doc = "Bits 12:15 - DEV_NUM_H"]
    #[inline(always)]
    pub fn dev_num_h(&mut self) -> DEV_NUM_H_W {
        DEV_NUM_H_W::new(self)
    }
    #[doc = "Bits 16:19 - FLASH"]
    #[inline(always)]
    pub fn flash(&mut self) -> FLASH_W {
        FLASH_W::new(self)
    }
    #[doc = "Bits 20:23 - DEV_NUM_L"]
    #[inline(always)]
    pub fn dev_num_l(&mut self) -> DEV_NUM_L_W {
        DEV_NUM_L_W::new(self)
    }
    #[doc = "Bits 28:31 - SRAM"]
    #[inline(always)]
    pub fn sram(&mut self) -> SRAM_W {
        SRAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBG_ID\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_id](index.html) module"]
pub struct DBG_ID_SPEC;
impl crate::RegisterSpec for DBG_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_id::R](R) reader structure"]
impl crate::Readable for DBG_ID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbg_id::W](W) writer structure"]
impl crate::Writable for DBG_ID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBG_ID to value 0"]
impl crate::Resettable for DBG_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
