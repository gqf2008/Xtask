#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub ctl: CTL,
    #[doc = "0x04 - software trigger register"]
    pub swt: SWT,
    #[doc = "0x08 - DAC0 12-bit right-aligned data holding register"]
    pub dac0_r12dh: DAC0_R12DH,
    #[doc = "0x0c - DAC0 12-bit left-aligned data holding register"]
    pub dac0_l12dh: DAC0_L12DH,
    #[doc = "0x10 - DAC0 8-bit right aligned data holding register"]
    pub dac0_r8dh: DAC0_R8DH,
    #[doc = "0x14 - DAC1 12-bit right-aligned data holding register"]
    pub dac1_r12dh: DAC1_R12DH,
    #[doc = "0x18 - DAC1 12-bit left aligned data holding register"]
    pub dac1_l12dh: DAC1_L12DH,
    #[doc = "0x1c - DAC1 8-bit right aligned data holding register"]
    pub dac1_r8dh: DAC1_R8DH,
    #[doc = "0x20 - DAC concurrent mode 12-bit right-aligned data holding register"]
    pub dacc_r12dh: DACC_R12DH,
    #[doc = "0x24 - DAC concurrent mode 12-bit left aligned data holding register"]
    pub dacc_l12dh: DACC_L12DH,
    #[doc = "0x28 - DAC concurrent mode 8-bit right aligned data holding register"]
    pub dacc_r8dh: DACC_R8DH,
    #[doc = "0x2c - DAC0 data output register"]
    pub dac0_do: DAC0_DO,
    #[doc = "0x30 - DAC1 data output register"]
    pub dac1_do: DAC1_DO,
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "control register"]
pub mod ctl;
#[doc = "software trigger register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swt](swt) module"]
pub type SWT = crate::Reg<u32, _SWT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWT;
#[doc = "`write(|w| ..)` method takes [swt::W](swt::W) writer structure"]
impl crate::Writable for SWT {}
#[doc = "software trigger register"]
pub mod swt;
#[doc = "DAC0 12-bit right-aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac0_r12dh](dac0_r12dh) module"]
pub type DAC0_R12DH = crate::Reg<u32, _DAC0_R12DH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC0_R12DH;
#[doc = "`read()` method returns [dac0_r12dh::R](dac0_r12dh::R) reader structure"]
impl crate::Readable for DAC0_R12DH {}
#[doc = "`write(|w| ..)` method takes [dac0_r12dh::W](dac0_r12dh::W) writer structure"]
impl crate::Writable for DAC0_R12DH {}
#[doc = "DAC0 12-bit right-aligned data holding register"]
pub mod dac0_r12dh;
#[doc = "DAC0 12-bit left-aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac0_l12dh](dac0_l12dh) module"]
pub type DAC0_L12DH = crate::Reg<u32, _DAC0_L12DH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC0_L12DH;
#[doc = "`read()` method returns [dac0_l12dh::R](dac0_l12dh::R) reader structure"]
impl crate::Readable for DAC0_L12DH {}
#[doc = "`write(|w| ..)` method takes [dac0_l12dh::W](dac0_l12dh::W) writer structure"]
impl crate::Writable for DAC0_L12DH {}
#[doc = "DAC0 12-bit left-aligned data holding register"]
pub mod dac0_l12dh;
#[doc = "DAC0 8-bit right aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac0_r8dh](dac0_r8dh) module"]
pub type DAC0_R8DH = crate::Reg<u32, _DAC0_R8DH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC0_R8DH;
#[doc = "`read()` method returns [dac0_r8dh::R](dac0_r8dh::R) reader structure"]
impl crate::Readable for DAC0_R8DH {}
#[doc = "`write(|w| ..)` method takes [dac0_r8dh::W](dac0_r8dh::W) writer structure"]
impl crate::Writable for DAC0_R8DH {}
#[doc = "DAC0 8-bit right aligned data holding register"]
pub mod dac0_r8dh;
#[doc = "DAC1 12-bit right-aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac1_r12dh](dac1_r12dh) module"]
pub type DAC1_R12DH = crate::Reg<u32, _DAC1_R12DH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC1_R12DH;
#[doc = "`read()` method returns [dac1_r12dh::R](dac1_r12dh::R) reader structure"]
impl crate::Readable for DAC1_R12DH {}
#[doc = "`write(|w| ..)` method takes [dac1_r12dh::W](dac1_r12dh::W) writer structure"]
impl crate::Writable for DAC1_R12DH {}
#[doc = "DAC1 12-bit right-aligned data holding register"]
pub mod dac1_r12dh;
#[doc = "DAC1 12-bit left aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac1_l12dh](dac1_l12dh) module"]
pub type DAC1_L12DH = crate::Reg<u32, _DAC1_L12DH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC1_L12DH;
#[doc = "`read()` method returns [dac1_l12dh::R](dac1_l12dh::R) reader structure"]
impl crate::Readable for DAC1_L12DH {}
#[doc = "`write(|w| ..)` method takes [dac1_l12dh::W](dac1_l12dh::W) writer structure"]
impl crate::Writable for DAC1_L12DH {}
#[doc = "DAC1 12-bit left aligned data holding register"]
pub mod dac1_l12dh;
#[doc = "DAC1 8-bit right aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac1_r8dh](dac1_r8dh) module"]
pub type DAC1_R8DH = crate::Reg<u32, _DAC1_R8DH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC1_R8DH;
#[doc = "`read()` method returns [dac1_r8dh::R](dac1_r8dh::R) reader structure"]
impl crate::Readable for DAC1_R8DH {}
#[doc = "`write(|w| ..)` method takes [dac1_r8dh::W](dac1_r8dh::W) writer structure"]
impl crate::Writable for DAC1_R8DH {}
#[doc = "DAC1 8-bit right aligned data holding register"]
pub mod dac1_r8dh;
#[doc = "DAC concurrent mode 12-bit right-aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacc_r12dh](dacc_r12dh) module"]
pub type DACC_R12DH = crate::Reg<u32, _DACC_R12DH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACC_R12DH;
#[doc = "`read()` method returns [dacc_r12dh::R](dacc_r12dh::R) reader structure"]
impl crate::Readable for DACC_R12DH {}
#[doc = "`write(|w| ..)` method takes [dacc_r12dh::W](dacc_r12dh::W) writer structure"]
impl crate::Writable for DACC_R12DH {}
#[doc = "DAC concurrent mode 12-bit right-aligned data holding register"]
pub mod dacc_r12dh;
#[doc = "DAC concurrent mode 12-bit left aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacc_l12dh](dacc_l12dh) module"]
pub type DACC_L12DH = crate::Reg<u32, _DACC_L12DH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACC_L12DH;
#[doc = "`read()` method returns [dacc_l12dh::R](dacc_l12dh::R) reader structure"]
impl crate::Readable for DACC_L12DH {}
#[doc = "`write(|w| ..)` method takes [dacc_l12dh::W](dacc_l12dh::W) writer structure"]
impl crate::Writable for DACC_L12DH {}
#[doc = "DAC concurrent mode 12-bit left aligned data holding register"]
pub mod dacc_l12dh;
#[doc = "DAC concurrent mode 8-bit right aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacc_r8dh](dacc_r8dh) module"]
pub type DACC_R8DH = crate::Reg<u32, _DACC_R8DH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACC_R8DH;
#[doc = "`read()` method returns [dacc_r8dh::R](dacc_r8dh::R) reader structure"]
impl crate::Readable for DACC_R8DH {}
#[doc = "`write(|w| ..)` method takes [dacc_r8dh::W](dacc_r8dh::W) writer structure"]
impl crate::Writable for DACC_R8DH {}
#[doc = "DAC concurrent mode 8-bit right aligned data holding register"]
pub mod dacc_r8dh;
#[doc = "DAC0 data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac0_do](dac0_do) module"]
pub type DAC0_DO = crate::Reg<u32, _DAC0_DO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC0_DO;
#[doc = "`read()` method returns [dac0_do::R](dac0_do::R) reader structure"]
impl crate::Readable for DAC0_DO {}
#[doc = "DAC0 data output register"]
pub mod dac0_do;
#[doc = "DAC1 data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac1_do](dac1_do) module"]
pub type DAC1_DO = crate::Reg<u32, _DAC1_DO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC1_DO;
#[doc = "`read()` method returns [dac1_do::R](dac1_do::R) reader structure"]
impl crate::Readable for DAC1_DO {}
#[doc = "DAC1 data output register"]
pub mod dac1_do;
