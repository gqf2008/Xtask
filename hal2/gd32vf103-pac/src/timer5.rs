#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 0"]
    pub ctl0: CTL0,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - control register 1"]
    pub ctl1: CTL1,
    _reserved2: [u8; 6usize],
    #[doc = "0x0c - DMA/Interrupt enable register"]
    pub dmainten: DMAINTEN,
    _reserved3: [u8; 2usize],
    #[doc = "0x10 - Interrupt flag register"]
    pub intf: INTF,
    _reserved4: [u8; 2usize],
    #[doc = "0x14 - event generation register"]
    pub swevg: SWEVG,
    _reserved5: [u8; 14usize],
    #[doc = "0x24 - Counter register"]
    pub cnt: CNT,
    _reserved6: [u8; 2usize],
    #[doc = "0x28 - Prescaler register"]
    pub psc: PSC,
    _reserved7: [u8; 2usize],
    #[doc = "0x2c - Counter auto reload register"]
    pub car: CAR,
}
#[doc = "control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](ctl0) module"]
pub type CTL0 = crate::Reg<u16, _CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL0;
#[doc = "`read()` method returns [ctl0::R](ctl0::R) reader structure"]
impl crate::Readable for CTL0 {}
#[doc = "`write(|w| ..)` method takes [ctl0::W](ctl0::W) writer structure"]
impl crate::Writable for CTL0 {}
#[doc = "control register 0"]
pub mod ctl0;
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](ctl1) module"]
pub type CTL1 = crate::Reg<u16, _CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL1;
#[doc = "`read()` method returns [ctl1::R](ctl1::R) reader structure"]
impl crate::Readable for CTL1 {}
#[doc = "`write(|w| ..)` method takes [ctl1::W](ctl1::W) writer structure"]
impl crate::Writable for CTL1 {}
#[doc = "control register 1"]
pub mod ctl1;
#[doc = "DMA/Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmainten](dmainten) module"]
pub type DMAINTEN = crate::Reg<u16, _DMAINTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAINTEN;
#[doc = "`read()` method returns [dmainten::R](dmainten::R) reader structure"]
impl crate::Readable for DMAINTEN {}
#[doc = "`write(|w| ..)` method takes [dmainten::W](dmainten::W) writer structure"]
impl crate::Writable for DMAINTEN {}
#[doc = "DMA/Interrupt enable register"]
pub mod dmainten;
#[doc = "Interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intf](intf) module"]
pub type INTF = crate::Reg<u16, _INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTF;
#[doc = "`read()` method returns [intf::R](intf::R) reader structure"]
impl crate::Readable for INTF {}
#[doc = "`write(|w| ..)` method takes [intf::W](intf::W) writer structure"]
impl crate::Writable for INTF {}
#[doc = "Interrupt flag register"]
pub mod intf;
#[doc = "event generation register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swevg](swevg) module"]
pub type SWEVG = crate::Reg<u16, _SWEVG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWEVG;
#[doc = "`write(|w| ..)` method takes [swevg::W](swevg::W) writer structure"]
impl crate::Writable for SWEVG {}
#[doc = "event generation register"]
pub mod swevg;
#[doc = "Counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt](cnt) module"]
pub type CNT = crate::Reg<u16, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
#[doc = "`read()` method returns [cnt::R](cnt::R) reader structure"]
impl crate::Readable for CNT {}
#[doc = "`write(|w| ..)` method takes [cnt::W](cnt::W) writer structure"]
impl crate::Writable for CNT {}
#[doc = "Counter register"]
pub mod cnt;
#[doc = "Prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psc](psc) module"]
pub type PSC = crate::Reg<u16, _PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSC;
#[doc = "`read()` method returns [psc::R](psc::R) reader structure"]
impl crate::Readable for PSC {}
#[doc = "`write(|w| ..)` method takes [psc::W](psc::W) writer structure"]
impl crate::Writable for PSC {}
#[doc = "Prescaler register"]
pub mod psc;
#[doc = "Counter auto reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [car](car) module"]
pub type CAR = crate::Reg<u16, _CAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAR;
#[doc = "`read()` method returns [car::R](car::R) reader structure"]
impl crate::Readable for CAR {}
#[doc = "`write(|w| ..)` method takes [car::W](car::W) writer structure"]
impl crate::Writable for CAR {}
#[doc = "Counter auto reload register"]
pub mod car;
