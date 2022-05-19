#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - cliccfg Register"]
    pub cliccfg: CLICCFG,
    _reserved1: [u8; 3usize],
    #[doc = "0x04 - clicinfo Register"]
    pub clicinfo: CLICINFO,
    _reserved2: [u8; 3usize],
    #[doc = "0x0b - MTH Register"]
    pub mth: MTH,
    _reserved3: [u8; 4084usize],
    #[doc = "0x1000 - Core-local Interrupt Controller Interrupt Registers"]
    pub clicints: [CLICINTS; 87],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CLICINTS {
    #[doc = "0x00 - clicintip Register"]
    pub clicintip: self::clicints::CLICINTIP,
    #[doc = "0x01 - clicintie Register"]
    pub clicintie: self::clicints::CLICINTIE,
    #[doc = "0x02 - clicintattr Register"]
    pub clicintattr: self::clicints::CLICINTATTR,
    #[doc = "0x03 - clicintctl Register"]
    pub clicintctl: self::clicints::CLICINTCTL,
}
#[doc = r"Register block"]
#[doc = "Core-local Interrupt Controller Interrupt Registers"]
pub mod clicints;
#[doc = "cliccfg Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cliccfg](cliccfg) module"]
pub type CLICCFG = crate::Reg<u8, _CLICCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLICCFG;
#[doc = "`read()` method returns [cliccfg::R](cliccfg::R) reader structure"]
impl crate::Readable for CLICCFG {}
#[doc = "`write(|w| ..)` method takes [cliccfg::W](cliccfg::W) writer structure"]
impl crate::Writable for CLICCFG {}
#[doc = "cliccfg Register"]
pub mod cliccfg;
#[doc = "clicinfo Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clicinfo](clicinfo) module"]
pub type CLICINFO = crate::Reg<u32, _CLICINFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLICINFO;
#[doc = "`read()` method returns [clicinfo::R](clicinfo::R) reader structure"]
impl crate::Readable for CLICINFO {}
#[doc = "clicinfo Register"]
pub mod clicinfo;
#[doc = "MTH Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mth](mth) module"]
pub type MTH = crate::Reg<u8, _MTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTH;
#[doc = "`read()` method returns [mth::R](mth::R) reader structure"]
impl crate::Readable for MTH {}
#[doc = "`write(|w| ..)` method takes [mth::W](mth::W) writer structure"]
impl crate::Writable for MTH {}
#[doc = "MTH Register"]
pub mod mth;
