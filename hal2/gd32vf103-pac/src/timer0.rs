#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 0"]
    pub ctl0: CTL0,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - control register 1"]
    pub ctl1: CTL1,
    _reserved2: [u8; 2usize],
    #[doc = "0x08 - slave mode configuration register"]
    pub smcfg: SMCFG,
    _reserved3: [u8; 2usize],
    #[doc = "0x0c - DMA/Interrupt enable register"]
    pub dmainten: DMAINTEN,
    _reserved4: [u8; 2usize],
    #[doc = "0x10 - Interrupt flag register"]
    pub intf: INTF,
    _reserved5: [u8; 2usize],
    #[doc = "0x14 - Software event generation register"]
    pub swevg: SWEVG,
    _reserved6: [u8; 2usize],
    _reserved_6_chctl0: [u8; 2usize],
    _reserved7: [u8; 2usize],
    _reserved_7_chctl1: [u8; 2usize],
    _reserved8: [u8; 2usize],
    #[doc = "0x20 - Channel control register 2"]
    pub chctl2: CHCTL2,
    _reserved9: [u8; 2usize],
    #[doc = "0x24 - counter"]
    pub cnt: CNT,
    _reserved10: [u8; 2usize],
    #[doc = "0x28 - prescaler"]
    pub psc: PSC,
    _reserved11: [u8; 2usize],
    #[doc = "0x2c - Counter auto reload register"]
    pub car: CAR,
    _reserved12: [u8; 2usize],
    #[doc = "0x30 - Counter repetition register"]
    pub crep: CREP,
    _reserved13: [u8; 2usize],
    #[doc = "0x34 - Channel 0 capture/compare value register"]
    pub ch0cv: CH0CV,
    _reserved14: [u8; 2usize],
    #[doc = "0x38 - Channel 1 capture/compare value register"]
    pub ch1cv: CH1CV,
    _reserved15: [u8; 2usize],
    #[doc = "0x3c - Channel 2 capture/compare value register"]
    pub ch2cv: CH2CV,
    _reserved16: [u8; 2usize],
    #[doc = "0x40 - Channel 3 capture/compare value register"]
    pub ch3cv: CH3CV,
    _reserved17: [u8; 2usize],
    #[doc = "0x44 - channel complementary protection register"]
    pub cchp: CCHP,
    _reserved18: [u8; 2usize],
    #[doc = "0x48 - DMA configuration register"]
    pub dmacfg: DMACFG,
    _reserved19: [u8; 2usize],
    #[doc = "0x4c - DMA transfer buffer register"]
    pub dmatb: DMATB,
}
impl RegisterBlock {
    #[doc = "0x18 - Channel control register 0 (input mode)"]
    #[inline(always)]
    pub fn chctl0_input(&self) -> &CHCTL0_INPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const CHCTL0_INPUT) }
    }
    #[doc = "0x18 - Channel control register 0 (input mode)"]
    #[inline(always)]
    pub fn chctl0_input_mut(&self) -> &mut CHCTL0_INPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut CHCTL0_INPUT) }
    }
    #[doc = "0x18 - Channel control register 0 (output mode)"]
    #[inline(always)]
    pub fn chctl0_output(&self) -> &CHCTL0_OUTPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const CHCTL0_OUTPUT) }
    }
    #[doc = "0x18 - Channel control register 0 (output mode)"]
    #[inline(always)]
    pub fn chctl0_output_mut(&self) -> &mut CHCTL0_OUTPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut CHCTL0_OUTPUT) }
    }
    #[doc = "0x1c - Channel control register 1 (input mode)"]
    #[inline(always)]
    pub fn chctl1_input(&self) -> &CHCTL1_INPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const CHCTL1_INPUT) }
    }
    #[doc = "0x1c - Channel control register 1 (input mode)"]
    #[inline(always)]
    pub fn chctl1_input_mut(&self) -> &mut CHCTL1_INPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut CHCTL1_INPUT) }
    }
    #[doc = "0x1c - Channel control register 1 (output mode)"]
    #[inline(always)]
    pub fn chctl1_output(&self) -> &CHCTL1_OUTPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const CHCTL1_OUTPUT) }
    }
    #[doc = "0x1c - Channel control register 1 (output mode)"]
    #[inline(always)]
    pub fn chctl1_output_mut(&self) -> &mut CHCTL1_OUTPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut CHCTL1_OUTPUT) }
    }
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
#[doc = "slave mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcfg](smcfg) module"]
pub type SMCFG = crate::Reg<u16, _SMCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMCFG;
#[doc = "`read()` method returns [smcfg::R](smcfg::R) reader structure"]
impl crate::Readable for SMCFG {}
#[doc = "`write(|w| ..)` method takes [smcfg::W](smcfg::W) writer structure"]
impl crate::Writable for SMCFG {}
#[doc = "slave mode configuration register"]
pub mod smcfg;
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
#[doc = "Software event generation register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swevg](swevg) module"]
pub type SWEVG = crate::Reg<u16, _SWEVG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWEVG;
#[doc = "`write(|w| ..)` method takes [swevg::W](swevg::W) writer structure"]
impl crate::Writable for SWEVG {}
#[doc = "Software event generation register"]
pub mod swevg;
#[doc = "Channel control register 0 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctl0_output](chctl0_output) module"]
pub type CHCTL0_OUTPUT = crate::Reg<u16, _CHCTL0_OUTPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCTL0_OUTPUT;
#[doc = "`read()` method returns [chctl0_output::R](chctl0_output::R) reader structure"]
impl crate::Readable for CHCTL0_OUTPUT {}
#[doc = "`write(|w| ..)` method takes [chctl0_output::W](chctl0_output::W) writer structure"]
impl crate::Writable for CHCTL0_OUTPUT {}
#[doc = "Channel control register 0 (output mode)"]
pub mod chctl0_output;
#[doc = "Channel control register 0 (input mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctl0_input](chctl0_input) module"]
pub type CHCTL0_INPUT = crate::Reg<u16, _CHCTL0_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCTL0_INPUT;
#[doc = "`read()` method returns [chctl0_input::R](chctl0_input::R) reader structure"]
impl crate::Readable for CHCTL0_INPUT {}
#[doc = "`write(|w| ..)` method takes [chctl0_input::W](chctl0_input::W) writer structure"]
impl crate::Writable for CHCTL0_INPUT {}
#[doc = "Channel control register 0 (input mode)"]
pub mod chctl0_input;
#[doc = "Channel control register 1 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctl1_output](chctl1_output) module"]
pub type CHCTL1_OUTPUT = crate::Reg<u16, _CHCTL1_OUTPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCTL1_OUTPUT;
#[doc = "`read()` method returns [chctl1_output::R](chctl1_output::R) reader structure"]
impl crate::Readable for CHCTL1_OUTPUT {}
#[doc = "`write(|w| ..)` method takes [chctl1_output::W](chctl1_output::W) writer structure"]
impl crate::Writable for CHCTL1_OUTPUT {}
#[doc = "Channel control register 1 (output mode)"]
pub mod chctl1_output;
#[doc = "Channel control register 1 (input mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctl1_input](chctl1_input) module"]
pub type CHCTL1_INPUT = crate::Reg<u16, _CHCTL1_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCTL1_INPUT;
#[doc = "`read()` method returns [chctl1_input::R](chctl1_input::R) reader structure"]
impl crate::Readable for CHCTL1_INPUT {}
#[doc = "`write(|w| ..)` method takes [chctl1_input::W](chctl1_input::W) writer structure"]
impl crate::Writable for CHCTL1_INPUT {}
#[doc = "Channel control register 1 (input mode)"]
pub mod chctl1_input;
#[doc = "Channel control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctl2](chctl2) module"]
pub type CHCTL2 = crate::Reg<u16, _CHCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHCTL2;
#[doc = "`read()` method returns [chctl2::R](chctl2::R) reader structure"]
impl crate::Readable for CHCTL2 {}
#[doc = "`write(|w| ..)` method takes [chctl2::W](chctl2::W) writer structure"]
impl crate::Writable for CHCTL2 {}
#[doc = "Channel control register 2"]
pub mod chctl2;
#[doc = "counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt](cnt) module"]
pub type CNT = crate::Reg<u16, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
#[doc = "`read()` method returns [cnt::R](cnt::R) reader structure"]
impl crate::Readable for CNT {}
#[doc = "`write(|w| ..)` method takes [cnt::W](cnt::W) writer structure"]
impl crate::Writable for CNT {}
#[doc = "counter"]
pub mod cnt;
#[doc = "prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psc](psc) module"]
pub type PSC = crate::Reg<u16, _PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSC;
#[doc = "`read()` method returns [psc::R](psc::R) reader structure"]
impl crate::Readable for PSC {}
#[doc = "`write(|w| ..)` method takes [psc::W](psc::W) writer structure"]
impl crate::Writable for PSC {}
#[doc = "prescaler"]
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
#[doc = "Counter repetition register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crep](crep) module"]
pub type CREP = crate::Reg<u16, _CREP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CREP;
#[doc = "`read()` method returns [crep::R](crep::R) reader structure"]
impl crate::Readable for CREP {}
#[doc = "`write(|w| ..)` method takes [crep::W](crep::W) writer structure"]
impl crate::Writable for CREP {}
#[doc = "Counter repetition register"]
pub mod crep;
#[doc = "Channel 0 capture/compare value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0cv](ch0cv) module"]
pub type CH0CV = crate::Reg<u16, _CH0CV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0CV;
#[doc = "`read()` method returns [ch0cv::R](ch0cv::R) reader structure"]
impl crate::Readable for CH0CV {}
#[doc = "`write(|w| ..)` method takes [ch0cv::W](ch0cv::W) writer structure"]
impl crate::Writable for CH0CV {}
#[doc = "Channel 0 capture/compare value register"]
pub mod ch0cv;
#[doc = "Channel 1 capture/compare value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1cv](ch1cv) module"]
pub type CH1CV = crate::Reg<u16, _CH1CV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1CV;
#[doc = "`read()` method returns [ch1cv::R](ch1cv::R) reader structure"]
impl crate::Readable for CH1CV {}
#[doc = "`write(|w| ..)` method takes [ch1cv::W](ch1cv::W) writer structure"]
impl crate::Writable for CH1CV {}
#[doc = "Channel 1 capture/compare value register"]
pub mod ch1cv;
#[doc = "Channel 2 capture/compare value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2cv](ch2cv) module"]
pub type CH2CV = crate::Reg<u16, _CH2CV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2CV;
#[doc = "`read()` method returns [ch2cv::R](ch2cv::R) reader structure"]
impl crate::Readable for CH2CV {}
#[doc = "`write(|w| ..)` method takes [ch2cv::W](ch2cv::W) writer structure"]
impl crate::Writable for CH2CV {}
#[doc = "Channel 2 capture/compare value register"]
pub mod ch2cv;
#[doc = "Channel 3 capture/compare value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3cv](ch3cv) module"]
pub type CH3CV = crate::Reg<u16, _CH3CV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3CV;
#[doc = "`read()` method returns [ch3cv::R](ch3cv::R) reader structure"]
impl crate::Readable for CH3CV {}
#[doc = "`write(|w| ..)` method takes [ch3cv::W](ch3cv::W) writer structure"]
impl crate::Writable for CH3CV {}
#[doc = "Channel 3 capture/compare value register"]
pub mod ch3cv;
#[doc = "channel complementary protection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cchp](cchp) module"]
pub type CCHP = crate::Reg<u16, _CCHP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCHP;
#[doc = "`read()` method returns [cchp::R](cchp::R) reader structure"]
impl crate::Readable for CCHP {}
#[doc = "`write(|w| ..)` method takes [cchp::W](cchp::W) writer structure"]
impl crate::Writable for CCHP {}
#[doc = "channel complementary protection register"]
pub mod cchp;
#[doc = "DMA configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacfg](dmacfg) module"]
pub type DMACFG = crate::Reg<u16, _DMACFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACFG;
#[doc = "`read()` method returns [dmacfg::R](dmacfg::R) reader structure"]
impl crate::Readable for DMACFG {}
#[doc = "`write(|w| ..)` method takes [dmacfg::W](dmacfg::W) writer structure"]
impl crate::Writable for DMACFG {}
#[doc = "DMA configuration register"]
pub mod dmacfg;
#[doc = "DMA transfer buffer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmatb](dmatb) module"]
pub type DMATB = crate::Reg<u16, _DMATB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMATB;
#[doc = "`read()` method returns [dmatb::R](dmatb::R) reader structure"]
impl crate::Readable for DMATB {}
#[doc = "`write(|w| ..)` method takes [dmatb::W](dmatb::W) writer structure"]
impl crate::Writable for DMATB {}
#[doc = "DMA transfer buffer register"]
pub mod dmatb;
