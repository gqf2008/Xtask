#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - device configuration register (DCFG)"]
    pub dcfg: DCFG,
    #[doc = "0x04 - device control register (DCTL)"]
    pub dctl: DCTL,
    #[doc = "0x08 - device status register (DSTAT)"]
    pub dstat: DSTAT,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - device IN endpoint common interrupt mask register (DIEPINTEN)"]
    pub diepinten: DIEPINTEN,
    #[doc = "0x14 - device OUT endpoint common interrupt enable register (DOEPINTEN)"]
    pub doepinten: DOEPINTEN,
    #[doc = "0x18 - device all endpoints interrupt register (DAEPINT)"]
    pub daepint: DAEPINT,
    #[doc = "0x1c - Device all endpoints interrupt enable register (DAEPINTEN)"]
    pub daepinten: DAEPINTEN,
    _reserved7: [u8; 8usize],
    #[doc = "0x28 - device VBUS discharge time register"]
    pub dvbusdt: DVBUSDT,
    #[doc = "0x2c - device VBUS pulsing time register"]
    pub dvbuspt: DVBUSPT,
    _reserved9: [u8; 4usize],
    #[doc = "0x34 - device IN endpoint FIFO empty interrupt enable register"]
    pub diepfeinten: DIEPFEINTEN,
    _reserved10: [u8; 200usize],
    #[doc = "0x100 - device IN endpoint 0 control register (DIEP0CTL)"]
    pub diep0ctl: DIEP0CTL,
    _reserved11: [u8; 4usize],
    #[doc = "0x108 - device endpoint-0 interrupt register"]
    pub diep0intf: DIEP0INTF,
    _reserved12: [u8; 4usize],
    #[doc = "0x110 - device IN endpoint-0 transfer length register"]
    pub diep0len: DIEP0LEN,
    _reserved13: [u8; 4usize],
    #[doc = "0x118 - device IN endpoint 0 transmit FIFO status register"]
    pub diep0tfstat: DIEP0TFSTAT,
    _reserved14: [u8; 4usize],
    #[doc = "0x120 - device in endpoint-1 control register"]
    pub diep1ctl: DIEP1CTL,
    _reserved15: [u8; 4usize],
    #[doc = "0x128 - device endpoint-1 interrupt register"]
    pub diep1intf: DIEP1INTF,
    _reserved16: [u8; 4usize],
    #[doc = "0x130 - device IN endpoint-1 transfer length register"]
    pub diep1len: DIEP1LEN,
    _reserved17: [u8; 4usize],
    #[doc = "0x138 - device IN endpoint 1 transmit FIFO status register"]
    pub diep1tfstat: DIEP1TFSTAT,
    _reserved18: [u8; 4usize],
    #[doc = "0x140 - device endpoint-2 control register"]
    pub diep2ctl: DIEP2CTL,
    _reserved19: [u8; 4usize],
    #[doc = "0x148 - device endpoint-2 interrupt register"]
    pub diep2intf: DIEP2INTF,
    _reserved20: [u8; 4usize],
    #[doc = "0x150 - device IN endpoint-2 transfer length register"]
    pub diep2len: DIEP2LEN,
    _reserved21: [u8; 4usize],
    #[doc = "0x158 - device IN endpoint 2 transmit FIFO status register"]
    pub diep2tfstat: DIEP2TFSTAT,
    _reserved22: [u8; 4usize],
    #[doc = "0x160 - device endpoint-3 control register"]
    pub diep3ctl: DIEP3CTL,
    _reserved23: [u8; 4usize],
    #[doc = "0x168 - device endpoint-3 interrupt register"]
    pub diep3intf: DIEP3INTF,
    _reserved24: [u8; 4usize],
    #[doc = "0x170 - device IN endpoint-3 transfer length register"]
    pub diep3len: DIEP3LEN,
    _reserved25: [u8; 4usize],
    #[doc = "0x178 - device IN endpoint 3 transmit FIFO status register"]
    pub diep3tfstat: DIEP3TFSTAT,
    _reserved26: [u8; 388usize],
    #[doc = "0x300 - device endpoint-0 control register"]
    pub doep0ctl: DOEP0CTL,
    _reserved27: [u8; 4usize],
    #[doc = "0x308 - device out endpoint-0 interrupt flag register"]
    pub doep0intf: DOEP0INTF,
    _reserved28: [u8; 4usize],
    #[doc = "0x310 - device OUT endpoint-0 transfer length register"]
    pub doep0len: DOEP0LEN,
    _reserved29: [u8; 12usize],
    #[doc = "0x320 - device endpoint-1 control register"]
    pub doep1ctl: DOEP1CTL,
    _reserved30: [u8; 4usize],
    #[doc = "0x328 - device out endpoint-1 interrupt flag register"]
    pub doep1intf: DOEP1INTF,
    _reserved31: [u8; 4usize],
    #[doc = "0x330 - device OUT endpoint-1 transfer length register"]
    pub doep1len: DOEP1LEN,
    _reserved32: [u8; 12usize],
    #[doc = "0x340 - device endpoint-2 control register"]
    pub doep2ctl: DOEP2CTL,
    _reserved33: [u8; 4usize],
    #[doc = "0x348 - device out endpoint-2 interrupt flag register"]
    pub doep2intf: DOEP2INTF,
    _reserved34: [u8; 4usize],
    #[doc = "0x350 - device OUT endpoint-2 transfer length register"]
    pub doep2len: DOEP2LEN,
    _reserved35: [u8; 12usize],
    #[doc = "0x360 - device endpoint-3 control register"]
    pub doep3ctl: DOEP3CTL,
    _reserved36: [u8; 4usize],
    #[doc = "0x368 - device out endpoint-3 interrupt flag register"]
    pub doep3intf: DOEP3INTF,
    _reserved37: [u8; 4usize],
    #[doc = "0x370 - device OUT endpoint-3 transfer length register"]
    pub doep3len: DOEP3LEN,
}
#[doc = "device configuration register (DCFG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcfg](dcfg) module"]
pub type DCFG = crate::Reg<u32, _DCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCFG;
#[doc = "`read()` method returns [dcfg::R](dcfg::R) reader structure"]
impl crate::Readable for DCFG {}
#[doc = "`write(|w| ..)` method takes [dcfg::W](dcfg::W) writer structure"]
impl crate::Writable for DCFG {}
#[doc = "device configuration register (DCFG)"]
pub mod dcfg;
#[doc = "device control register (DCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dctl](dctl) module"]
pub type DCTL = crate::Reg<u32, _DCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCTL;
#[doc = "`read()` method returns [dctl::R](dctl::R) reader structure"]
impl crate::Readable for DCTL {}
#[doc = "`write(|w| ..)` method takes [dctl::W](dctl::W) writer structure"]
impl crate::Writable for DCTL {}
#[doc = "device control register (DCTL)"]
pub mod dctl;
#[doc = "device status register (DSTAT)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dstat](dstat) module"]
pub type DSTAT = crate::Reg<u32, _DSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSTAT;
#[doc = "`read()` method returns [dstat::R](dstat::R) reader structure"]
impl crate::Readable for DSTAT {}
#[doc = "device status register (DSTAT)"]
pub mod dstat;
#[doc = "device IN endpoint common interrupt mask register (DIEPINTEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepinten](diepinten) module"]
pub type DIEPINTEN = crate::Reg<u32, _DIEPINTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPINTEN;
#[doc = "`read()` method returns [diepinten::R](diepinten::R) reader structure"]
impl crate::Readable for DIEPINTEN {}
#[doc = "`write(|w| ..)` method takes [diepinten::W](diepinten::W) writer structure"]
impl crate::Writable for DIEPINTEN {}
#[doc = "device IN endpoint common interrupt mask register (DIEPINTEN)"]
pub mod diepinten;
#[doc = "device OUT endpoint common interrupt enable register (DOEPINTEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepinten](doepinten) module"]
pub type DOEPINTEN = crate::Reg<u32, _DOEPINTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPINTEN;
#[doc = "`read()` method returns [doepinten::R](doepinten::R) reader structure"]
impl crate::Readable for DOEPINTEN {}
#[doc = "`write(|w| ..)` method takes [doepinten::W](doepinten::W) writer structure"]
impl crate::Writable for DOEPINTEN {}
#[doc = "device OUT endpoint common interrupt enable register (DOEPINTEN)"]
pub mod doepinten;
#[doc = "device all endpoints interrupt register (DAEPINT)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daepint](daepint) module"]
pub type DAEPINT = crate::Reg<u32, _DAEPINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAEPINT;
#[doc = "`read()` method returns [daepint::R](daepint::R) reader structure"]
impl crate::Readable for DAEPINT {}
#[doc = "device all endpoints interrupt register (DAEPINT)"]
pub mod daepint;
#[doc = "Device all endpoints interrupt enable register (DAEPINTEN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daepinten](daepinten) module"]
pub type DAEPINTEN = crate::Reg<u32, _DAEPINTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAEPINTEN;
#[doc = "`read()` method returns [daepinten::R](daepinten::R) reader structure"]
impl crate::Readable for DAEPINTEN {}
#[doc = "`write(|w| ..)` method takes [daepinten::W](daepinten::W) writer structure"]
impl crate::Writable for DAEPINTEN {}
#[doc = "Device all endpoints interrupt enable register (DAEPINTEN)"]
pub mod daepinten;
#[doc = "device VBUS discharge time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvbusdt](dvbusdt) module"]
pub type DVBUSDT = crate::Reg<u32, _DVBUSDT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DVBUSDT;
#[doc = "`read()` method returns [dvbusdt::R](dvbusdt::R) reader structure"]
impl crate::Readable for DVBUSDT {}
#[doc = "`write(|w| ..)` method takes [dvbusdt::W](dvbusdt::W) writer structure"]
impl crate::Writable for DVBUSDT {}
#[doc = "device VBUS discharge time register"]
pub mod dvbusdt;
#[doc = "device VBUS pulsing time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvbuspt](dvbuspt) module"]
pub type DVBUSPT = crate::Reg<u32, _DVBUSPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DVBUSPT;
#[doc = "`read()` method returns [dvbuspt::R](dvbuspt::R) reader structure"]
impl crate::Readable for DVBUSPT {}
#[doc = "`write(|w| ..)` method takes [dvbuspt::W](dvbuspt::W) writer structure"]
impl crate::Writable for DVBUSPT {}
#[doc = "device VBUS pulsing time register"]
pub mod dvbuspt;
#[doc = "device IN endpoint FIFO empty interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepfeinten](diepfeinten) module"]
pub type DIEPFEINTEN = crate::Reg<u32, _DIEPFEINTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPFEINTEN;
#[doc = "`read()` method returns [diepfeinten::R](diepfeinten::R) reader structure"]
impl crate::Readable for DIEPFEINTEN {}
#[doc = "`write(|w| ..)` method takes [diepfeinten::W](diepfeinten::W) writer structure"]
impl crate::Writable for DIEPFEINTEN {}
#[doc = "device IN endpoint FIFO empty interrupt enable register"]
pub mod diepfeinten;
#[doc = "device IN endpoint 0 control register (DIEP0CTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep0ctl](diep0ctl) module"]
pub type DIEP0CTL = crate::Reg<u32, _DIEP0CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0CTL;
#[doc = "`read()` method returns [diep0ctl::R](diep0ctl::R) reader structure"]
impl crate::Readable for DIEP0CTL {}
#[doc = "`write(|w| ..)` method takes [diep0ctl::W](diep0ctl::W) writer structure"]
impl crate::Writable for DIEP0CTL {}
#[doc = "device IN endpoint 0 control register (DIEP0CTL)"]
pub mod diep0ctl;
#[doc = "device in endpoint-1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep1ctl](diep1ctl) module"]
pub type DIEP1CTL = crate::Reg<u32, _DIEP1CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP1CTL;
#[doc = "`read()` method returns [diep1ctl::R](diep1ctl::R) reader structure"]
impl crate::Readable for DIEP1CTL {}
#[doc = "`write(|w| ..)` method takes [diep1ctl::W](diep1ctl::W) writer structure"]
impl crate::Writable for DIEP1CTL {}
#[doc = "device in endpoint-1 control register"]
pub mod diep1ctl;
#[doc = "device endpoint-2 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep2ctl](diep2ctl) module"]
pub type DIEP2CTL = crate::Reg<u32, _DIEP2CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP2CTL;
#[doc = "`read()` method returns [diep2ctl::R](diep2ctl::R) reader structure"]
impl crate::Readable for DIEP2CTL {}
#[doc = "`write(|w| ..)` method takes [diep2ctl::W](diep2ctl::W) writer structure"]
impl crate::Writable for DIEP2CTL {}
#[doc = "device endpoint-2 control register"]
pub mod diep2ctl;
#[doc = "device endpoint-3 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep3ctl](diep3ctl) module"]
pub type DIEP3CTL = crate::Reg<u32, _DIEP3CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP3CTL;
#[doc = "`read()` method returns [diep3ctl::R](diep3ctl::R) reader structure"]
impl crate::Readable for DIEP3CTL {}
#[doc = "`write(|w| ..)` method takes [diep3ctl::W](diep3ctl::W) writer structure"]
impl crate::Writable for DIEP3CTL {}
#[doc = "device endpoint-3 control register"]
pub mod diep3ctl;
#[doc = "device endpoint-0 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep0ctl](doep0ctl) module"]
pub type DOEP0CTL = crate::Reg<u32, _DOEP0CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP0CTL;
#[doc = "`read()` method returns [doep0ctl::R](doep0ctl::R) reader structure"]
impl crate::Readable for DOEP0CTL {}
#[doc = "`write(|w| ..)` method takes [doep0ctl::W](doep0ctl::W) writer structure"]
impl crate::Writable for DOEP0CTL {}
#[doc = "device endpoint-0 control register"]
pub mod doep0ctl;
#[doc = "device endpoint-1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep1ctl](doep1ctl) module"]
pub type DOEP1CTL = crate::Reg<u32, _DOEP1CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP1CTL;
#[doc = "`read()` method returns [doep1ctl::R](doep1ctl::R) reader structure"]
impl crate::Readable for DOEP1CTL {}
#[doc = "`write(|w| ..)` method takes [doep1ctl::W](doep1ctl::W) writer structure"]
impl crate::Writable for DOEP1CTL {}
#[doc = "device endpoint-1 control register"]
pub mod doep1ctl;
#[doc = "device endpoint-2 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep2ctl](doep2ctl) module"]
pub type DOEP2CTL = crate::Reg<u32, _DOEP2CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP2CTL;
#[doc = "`read()` method returns [doep2ctl::R](doep2ctl::R) reader structure"]
impl crate::Readable for DOEP2CTL {}
#[doc = "`write(|w| ..)` method takes [doep2ctl::W](doep2ctl::W) writer structure"]
impl crate::Writable for DOEP2CTL {}
#[doc = "device endpoint-2 control register"]
pub mod doep2ctl;
#[doc = "device endpoint-3 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep3ctl](doep3ctl) module"]
pub type DOEP3CTL = crate::Reg<u32, _DOEP3CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP3CTL;
#[doc = "`read()` method returns [doep3ctl::R](doep3ctl::R) reader structure"]
impl crate::Readable for DOEP3CTL {}
#[doc = "`write(|w| ..)` method takes [doep3ctl::W](doep3ctl::W) writer structure"]
impl crate::Writable for DOEP3CTL {}
#[doc = "device endpoint-3 control register"]
pub mod doep3ctl;
#[doc = "device endpoint-0 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep0intf](diep0intf) module"]
pub type DIEP0INTF = crate::Reg<u32, _DIEP0INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0INTF;
#[doc = "`read()` method returns [diep0intf::R](diep0intf::R) reader structure"]
impl crate::Readable for DIEP0INTF {}
#[doc = "`write(|w| ..)` method takes [diep0intf::W](diep0intf::W) writer structure"]
impl crate::Writable for DIEP0INTF {}
#[doc = "device endpoint-0 interrupt register"]
pub mod diep0intf;
#[doc = "device endpoint-1 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep1intf](diep1intf) module"]
pub type DIEP1INTF = crate::Reg<u32, _DIEP1INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP1INTF;
#[doc = "`read()` method returns [diep1intf::R](diep1intf::R) reader structure"]
impl crate::Readable for DIEP1INTF {}
#[doc = "`write(|w| ..)` method takes [diep1intf::W](diep1intf::W) writer structure"]
impl crate::Writable for DIEP1INTF {}
#[doc = "device endpoint-1 interrupt register"]
pub mod diep1intf;
#[doc = "device endpoint-2 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep2intf](diep2intf) module"]
pub type DIEP2INTF = crate::Reg<u32, _DIEP2INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP2INTF;
#[doc = "`read()` method returns [diep2intf::R](diep2intf::R) reader structure"]
impl crate::Readable for DIEP2INTF {}
#[doc = "`write(|w| ..)` method takes [diep2intf::W](diep2intf::W) writer structure"]
impl crate::Writable for DIEP2INTF {}
#[doc = "device endpoint-2 interrupt register"]
pub mod diep2intf;
#[doc = "device endpoint-3 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep3intf](diep3intf) module"]
pub type DIEP3INTF = crate::Reg<u32, _DIEP3INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP3INTF;
#[doc = "`read()` method returns [diep3intf::R](diep3intf::R) reader structure"]
impl crate::Readable for DIEP3INTF {}
#[doc = "`write(|w| ..)` method takes [diep3intf::W](diep3intf::W) writer structure"]
impl crate::Writable for DIEP3INTF {}
#[doc = "device endpoint-3 interrupt register"]
pub mod diep3intf;
#[doc = "device out endpoint-0 interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep0intf](doep0intf) module"]
pub type DOEP0INTF = crate::Reg<u32, _DOEP0INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP0INTF;
#[doc = "`read()` method returns [doep0intf::R](doep0intf::R) reader structure"]
impl crate::Readable for DOEP0INTF {}
#[doc = "`write(|w| ..)` method takes [doep0intf::W](doep0intf::W) writer structure"]
impl crate::Writable for DOEP0INTF {}
#[doc = "device out endpoint-0 interrupt flag register"]
pub mod doep0intf;
#[doc = "device out endpoint-1 interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep1intf](doep1intf) module"]
pub type DOEP1INTF = crate::Reg<u32, _DOEP1INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP1INTF;
#[doc = "`read()` method returns [doep1intf::R](doep1intf::R) reader structure"]
impl crate::Readable for DOEP1INTF {}
#[doc = "`write(|w| ..)` method takes [doep1intf::W](doep1intf::W) writer structure"]
impl crate::Writable for DOEP1INTF {}
#[doc = "device out endpoint-1 interrupt flag register"]
pub mod doep1intf;
#[doc = "device out endpoint-2 interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep2intf](doep2intf) module"]
pub type DOEP2INTF = crate::Reg<u32, _DOEP2INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP2INTF;
#[doc = "`read()` method returns [doep2intf::R](doep2intf::R) reader structure"]
impl crate::Readable for DOEP2INTF {}
#[doc = "`write(|w| ..)` method takes [doep2intf::W](doep2intf::W) writer structure"]
impl crate::Writable for DOEP2INTF {}
#[doc = "device out endpoint-2 interrupt flag register"]
pub mod doep2intf;
#[doc = "device out endpoint-3 interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep3intf](doep3intf) module"]
pub type DOEP3INTF = crate::Reg<u32, _DOEP3INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP3INTF;
#[doc = "`read()` method returns [doep3intf::R](doep3intf::R) reader structure"]
impl crate::Readable for DOEP3INTF {}
#[doc = "`write(|w| ..)` method takes [doep3intf::W](doep3intf::W) writer structure"]
impl crate::Writable for DOEP3INTF {}
#[doc = "device out endpoint-3 interrupt flag register"]
pub mod doep3intf;
#[doc = "device IN endpoint-0 transfer length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep0len](diep0len) module"]
pub type DIEP0LEN = crate::Reg<u32, _DIEP0LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0LEN;
#[doc = "`read()` method returns [diep0len::R](diep0len::R) reader structure"]
impl crate::Readable for DIEP0LEN {}
#[doc = "`write(|w| ..)` method takes [diep0len::W](diep0len::W) writer structure"]
impl crate::Writable for DIEP0LEN {}
#[doc = "device IN endpoint-0 transfer length register"]
pub mod diep0len;
#[doc = "device OUT endpoint-0 transfer length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep0len](doep0len) module"]
pub type DOEP0LEN = crate::Reg<u32, _DOEP0LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP0LEN;
#[doc = "`read()` method returns [doep0len::R](doep0len::R) reader structure"]
impl crate::Readable for DOEP0LEN {}
#[doc = "`write(|w| ..)` method takes [doep0len::W](doep0len::W) writer structure"]
impl crate::Writable for DOEP0LEN {}
#[doc = "device OUT endpoint-0 transfer length register"]
pub mod doep0len;
#[doc = "device IN endpoint-1 transfer length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep1len](diep1len) module"]
pub type DIEP1LEN = crate::Reg<u32, _DIEP1LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP1LEN;
#[doc = "`read()` method returns [diep1len::R](diep1len::R) reader structure"]
impl crate::Readable for DIEP1LEN {}
#[doc = "`write(|w| ..)` method takes [diep1len::W](diep1len::W) writer structure"]
impl crate::Writable for DIEP1LEN {}
#[doc = "device IN endpoint-1 transfer length register"]
pub mod diep1len;
#[doc = "device IN endpoint-2 transfer length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep2len](diep2len) module"]
pub type DIEP2LEN = crate::Reg<u32, _DIEP2LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP2LEN;
#[doc = "`read()` method returns [diep2len::R](diep2len::R) reader structure"]
impl crate::Readable for DIEP2LEN {}
#[doc = "`write(|w| ..)` method takes [diep2len::W](diep2len::W) writer structure"]
impl crate::Writable for DIEP2LEN {}
#[doc = "device IN endpoint-2 transfer length register"]
pub mod diep2len;
#[doc = "device IN endpoint-3 transfer length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep3len](diep3len) module"]
pub type DIEP3LEN = crate::Reg<u32, _DIEP3LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP3LEN;
#[doc = "`read()` method returns [diep3len::R](diep3len::R) reader structure"]
impl crate::Readable for DIEP3LEN {}
#[doc = "`write(|w| ..)` method takes [diep3len::W](diep3len::W) writer structure"]
impl crate::Writable for DIEP3LEN {}
#[doc = "device IN endpoint-3 transfer length register"]
pub mod diep3len;
#[doc = "device OUT endpoint-1 transfer length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep1len](doep1len) module"]
pub type DOEP1LEN = crate::Reg<u32, _DOEP1LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP1LEN;
#[doc = "`read()` method returns [doep1len::R](doep1len::R) reader structure"]
impl crate::Readable for DOEP1LEN {}
#[doc = "`write(|w| ..)` method takes [doep1len::W](doep1len::W) writer structure"]
impl crate::Writable for DOEP1LEN {}
#[doc = "device OUT endpoint-1 transfer length register"]
pub mod doep1len;
#[doc = "device OUT endpoint-2 transfer length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep2len](doep2len) module"]
pub type DOEP2LEN = crate::Reg<u32, _DOEP2LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP2LEN;
#[doc = "`read()` method returns [doep2len::R](doep2len::R) reader structure"]
impl crate::Readable for DOEP2LEN {}
#[doc = "`write(|w| ..)` method takes [doep2len::W](doep2len::W) writer structure"]
impl crate::Writable for DOEP2LEN {}
#[doc = "device OUT endpoint-2 transfer length register"]
pub mod doep2len;
#[doc = "device OUT endpoint-3 transfer length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep3len](doep3len) module"]
pub type DOEP3LEN = crate::Reg<u32, _DOEP3LEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP3LEN;
#[doc = "`read()` method returns [doep3len::R](doep3len::R) reader structure"]
impl crate::Readable for DOEP3LEN {}
#[doc = "`write(|w| ..)` method takes [doep3len::W](doep3len::W) writer structure"]
impl crate::Writable for DOEP3LEN {}
#[doc = "device OUT endpoint-3 transfer length register"]
pub mod doep3len;
#[doc = "device IN endpoint 0 transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep0tfstat](diep0tfstat) module"]
pub type DIEP0TFSTAT = crate::Reg<u32, _DIEP0TFSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0TFSTAT;
#[doc = "`read()` method returns [diep0tfstat::R](diep0tfstat::R) reader structure"]
impl crate::Readable for DIEP0TFSTAT {}
#[doc = "device IN endpoint 0 transmit FIFO status register"]
pub mod diep0tfstat;
#[doc = "device IN endpoint 1 transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep1tfstat](diep1tfstat) module"]
pub type DIEP1TFSTAT = crate::Reg<u32, _DIEP1TFSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP1TFSTAT;
#[doc = "`read()` method returns [diep1tfstat::R](diep1tfstat::R) reader structure"]
impl crate::Readable for DIEP1TFSTAT {}
#[doc = "device IN endpoint 1 transmit FIFO status register"]
pub mod diep1tfstat;
#[doc = "device IN endpoint 2 transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep2tfstat](diep2tfstat) module"]
pub type DIEP2TFSTAT = crate::Reg<u32, _DIEP2TFSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP2TFSTAT;
#[doc = "`read()` method returns [diep2tfstat::R](diep2tfstat::R) reader structure"]
impl crate::Readable for DIEP2TFSTAT {}
#[doc = "device IN endpoint 2 transmit FIFO status register"]
pub mod diep2tfstat;
#[doc = "device IN endpoint 3 transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep3tfstat](diep3tfstat) module"]
pub type DIEP3TFSTAT = crate::Reg<u32, _DIEP3TFSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP3TFSTAT;
#[doc = "`read()` method returns [diep3tfstat::R](diep3tfstat::R) reader structure"]
impl crate::Readable for DIEP3TFSTAT {}
#[doc = "device IN endpoint 3 transmit FIFO status register"]
pub mod diep3tfstat;
