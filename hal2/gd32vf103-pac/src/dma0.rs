#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt flag register"]
    pub intf: INTF,
    #[doc = "0x04 - Interrupt flag clear register"]
    pub intc: INTC,
    #[doc = "0x08 - Channel 0 control register"]
    pub ch0ctl: CH0CTL,
    #[doc = "0x0c - Channel 0 counter register"]
    pub ch0cnt: CH0CNT,
    #[doc = "0x10 - Channel 0 peripheral base address register"]
    pub ch0paddr: CH0PADDR,
    #[doc = "0x14 - Channel 0 memory base address register"]
    pub ch0maddr: CH0MADDR,
    _reserved6: [u8; 4usize],
    #[doc = "0x1c - Channel 1 control register"]
    pub ch1ctl: CH1CTL,
    #[doc = "0x20 - Channel 1 counter register"]
    pub ch1cnt: CH1CNT,
    #[doc = "0x24 - Channel 1 peripheral base address register"]
    pub ch1paddr: CH1PADDR,
    #[doc = "0x28 - Channel 1 memory base address register"]
    pub ch1maddr: CH1MADDR,
    _reserved10: [u8; 4usize],
    #[doc = "0x30 - Channel 2 control register"]
    pub ch2ctl: CH2CTL,
    #[doc = "0x34 - Channel 2 counter register"]
    pub ch2cnt: CH2CNT,
    #[doc = "0x38 - Channel 2 peripheral base address register"]
    pub ch2paddr: CH2PADDR,
    #[doc = "0x3c - Channel 2 memory base address register"]
    pub ch2maddr: CH2MADDR,
    _reserved14: [u8; 4usize],
    #[doc = "0x44 - Channel 3 control register"]
    pub ch3ctl: CH3CTL,
    #[doc = "0x48 - Channel 3 counter register"]
    pub ch3cnt: CH3CNT,
    #[doc = "0x4c - Channel 3 peripheral base address register"]
    pub ch3paddr: CH3PADDR,
    #[doc = "0x50 - Channel 3 memory base address register"]
    pub ch3maddr: CH3MADDR,
    _reserved18: [u8; 4usize],
    #[doc = "0x58 - Channel 4 control register"]
    pub ch4ctl: CH4CTL,
    #[doc = "0x5c - Channel 4 counter register"]
    pub ch4cnt: CH4CNT,
    #[doc = "0x60 - Channel 4 peripheral base address register"]
    pub ch4paddr: CH4PADDR,
    #[doc = "0x64 - Channel 4 memory base address register"]
    pub ch4maddr: CH4MADDR,
    _reserved22: [u8; 4usize],
    #[doc = "0x6c - Channel 5 control register"]
    pub ch5ctl: CH5CTL,
    #[doc = "0x70 - Channel 5 counter register"]
    pub ch5cnt: CH5CNT,
    #[doc = "0x74 - Channel 5 peripheral base address register"]
    pub ch5paddr: CH5PADDR,
    #[doc = "0x78 - Channel 5 memory base address register"]
    pub ch5maddr: CH5MADDR,
    _reserved26: [u8; 4usize],
    #[doc = "0x80 - Channel 6 control register"]
    pub ch6ctl: CH6CTL,
    #[doc = "0x84 - Channel 6 counter register"]
    pub ch6cnt: CH6CNT,
    #[doc = "0x88 - Channel 6 peripheral base address register"]
    pub ch6paddr: CH6PADDR,
    #[doc = "0x8c - Channel 6 memory base address register"]
    pub ch6maddr: CH6MADDR,
}
#[doc = "Interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intf](intf) module"]
pub type INTF = crate::Reg<u32, _INTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTF;
#[doc = "`read()` method returns [intf::R](intf::R) reader structure"]
impl crate::Readable for INTF {}
#[doc = "Interrupt flag register"]
pub mod intf;
#[doc = "Interrupt flag clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intc](intc) module"]
pub type INTC = crate::Reg<u32, _INTC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTC;
#[doc = "`write(|w| ..)` method takes [intc::W](intc::W) writer structure"]
impl crate::Writable for INTC {}
#[doc = "Interrupt flag clear register"]
pub mod intc;
#[doc = "Channel 0 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0ctl](ch0ctl) module"]
pub type CH0CTL = crate::Reg<u32, _CH0CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0CTL;
#[doc = "`read()` method returns [ch0ctl::R](ch0ctl::R) reader structure"]
impl crate::Readable for CH0CTL {}
#[doc = "`write(|w| ..)` method takes [ch0ctl::W](ch0ctl::W) writer structure"]
impl crate::Writable for CH0CTL {}
#[doc = "Channel 0 control register"]
pub mod ch0ctl;
#[doc = "Channel 0 counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0cnt](ch0cnt) module"]
pub type CH0CNT = crate::Reg<u32, _CH0CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0CNT;
#[doc = "`read()` method returns [ch0cnt::R](ch0cnt::R) reader structure"]
impl crate::Readable for CH0CNT {}
#[doc = "`write(|w| ..)` method takes [ch0cnt::W](ch0cnt::W) writer structure"]
impl crate::Writable for CH0CNT {}
#[doc = "Channel 0 counter register"]
pub mod ch0cnt;
#[doc = "Channel 0 peripheral base address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0paddr](ch0paddr) module"]
pub type CH0PADDR = crate::Reg<u32, _CH0PADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0PADDR;
#[doc = "`read()` method returns [ch0paddr::R](ch0paddr::R) reader structure"]
impl crate::Readable for CH0PADDR {}
#[doc = "`write(|w| ..)` method takes [ch0paddr::W](ch0paddr::W) writer structure"]
impl crate::Writable for CH0PADDR {}
#[doc = "Channel 0 peripheral base address register"]
pub mod ch0paddr;
#[doc = "Channel 0 memory base address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0maddr](ch0maddr) module"]
pub type CH0MADDR = crate::Reg<u32, _CH0MADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0MADDR;
#[doc = "`read()` method returns [ch0maddr::R](ch0maddr::R) reader structure"]
impl crate::Readable for CH0MADDR {}
#[doc = "`write(|w| ..)` method takes [ch0maddr::W](ch0maddr::W) writer structure"]
impl crate::Writable for CH0MADDR {}
#[doc = "Channel 0 memory base address register"]
pub mod ch0maddr;
#[doc = "Channel 1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1ctl](ch1ctl) module"]
pub type CH1CTL = crate::Reg<u32, _CH1CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1CTL;
#[doc = "`read()` method returns [ch1ctl::R](ch1ctl::R) reader structure"]
impl crate::Readable for CH1CTL {}
#[doc = "`write(|w| ..)` method takes [ch1ctl::W](ch1ctl::W) writer structure"]
impl crate::Writable for CH1CTL {}
#[doc = "Channel 1 control register"]
pub mod ch1ctl;
#[doc = "Channel 1 counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1cnt](ch1cnt) module"]
pub type CH1CNT = crate::Reg<u32, _CH1CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1CNT;
#[doc = "`read()` method returns [ch1cnt::R](ch1cnt::R) reader structure"]
impl crate::Readable for CH1CNT {}
#[doc = "`write(|w| ..)` method takes [ch1cnt::W](ch1cnt::W) writer structure"]
impl crate::Writable for CH1CNT {}
#[doc = "Channel 1 counter register"]
pub mod ch1cnt;
#[doc = "Channel 1 peripheral base address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1paddr](ch1paddr) module"]
pub type CH1PADDR = crate::Reg<u32, _CH1PADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1PADDR;
#[doc = "`read()` method returns [ch1paddr::R](ch1paddr::R) reader structure"]
impl crate::Readable for CH1PADDR {}
#[doc = "`write(|w| ..)` method takes [ch1paddr::W](ch1paddr::W) writer structure"]
impl crate::Writable for CH1PADDR {}
#[doc = "Channel 1 peripheral base address register"]
pub mod ch1paddr;
#[doc = "Channel 1 memory base address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1maddr](ch1maddr) module"]
pub type CH1MADDR = crate::Reg<u32, _CH1MADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1MADDR;
#[doc = "`read()` method returns [ch1maddr::R](ch1maddr::R) reader structure"]
impl crate::Readable for CH1MADDR {}
#[doc = "`write(|w| ..)` method takes [ch1maddr::W](ch1maddr::W) writer structure"]
impl crate::Writable for CH1MADDR {}
#[doc = "Channel 1 memory base address register"]
pub mod ch1maddr;
#[doc = "Channel 2 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2ctl](ch2ctl) module"]
pub type CH2CTL = crate::Reg<u32, _CH2CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2CTL;
#[doc = "`read()` method returns [ch2ctl::R](ch2ctl::R) reader structure"]
impl crate::Readable for CH2CTL {}
#[doc = "`write(|w| ..)` method takes [ch2ctl::W](ch2ctl::W) writer structure"]
impl crate::Writable for CH2CTL {}
#[doc = "Channel 2 control register"]
pub mod ch2ctl;
#[doc = "Channel 2 counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2cnt](ch2cnt) module"]
pub type CH2CNT = crate::Reg<u32, _CH2CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2CNT;
#[doc = "`read()` method returns [ch2cnt::R](ch2cnt::R) reader structure"]
impl crate::Readable for CH2CNT {}
#[doc = "`write(|w| ..)` method takes [ch2cnt::W](ch2cnt::W) writer structure"]
impl crate::Writable for CH2CNT {}
#[doc = "Channel 2 counter register"]
pub mod ch2cnt;
#[doc = "Channel 2 peripheral base address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2paddr](ch2paddr) module"]
pub type CH2PADDR = crate::Reg<u32, _CH2PADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2PADDR;
#[doc = "`read()` method returns [ch2paddr::R](ch2paddr::R) reader structure"]
impl crate::Readable for CH2PADDR {}
#[doc = "`write(|w| ..)` method takes [ch2paddr::W](ch2paddr::W) writer structure"]
impl crate::Writable for CH2PADDR {}
#[doc = "Channel 2 peripheral base address register"]
pub mod ch2paddr;
#[doc = "Channel 2 memory base address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2maddr](ch2maddr) module"]
pub type CH2MADDR = crate::Reg<u32, _CH2MADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2MADDR;
#[doc = "`read()` method returns [ch2maddr::R](ch2maddr::R) reader structure"]
impl crate::Readable for CH2MADDR {}
#[doc = "`write(|w| ..)` method takes [ch2maddr::W](ch2maddr::W) writer structure"]
impl crate::Writable for CH2MADDR {}
#[doc = "Channel 2 memory base address register"]
pub mod ch2maddr;
#[doc = "Channel 3 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3ctl](ch3ctl) module"]
pub type CH3CTL = crate::Reg<u32, _CH3CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3CTL;
#[doc = "`read()` method returns [ch3ctl::R](ch3ctl::R) reader structure"]
impl crate::Readable for CH3CTL {}
#[doc = "`write(|w| ..)` method takes [ch3ctl::W](ch3ctl::W) writer structure"]
impl crate::Writable for CH3CTL {}
#[doc = "Channel 3 control register"]
pub mod ch3ctl;
#[doc = "Channel 3 counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3cnt](ch3cnt) module"]
pub type CH3CNT = crate::Reg<u32, _CH3CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3CNT;
#[doc = "`read()` method returns [ch3cnt::R](ch3cnt::R) reader structure"]
impl crate::Readable for CH3CNT {}
#[doc = "`write(|w| ..)` method takes [ch3cnt::W](ch3cnt::W) writer structure"]
impl crate::Writable for CH3CNT {}
#[doc = "Channel 3 counter register"]
pub mod ch3cnt;
#[doc = "Channel 3 peripheral base address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3paddr](ch3paddr) module"]
pub type CH3PADDR = crate::Reg<u32, _CH3PADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3PADDR;
#[doc = "`read()` method returns [ch3paddr::R](ch3paddr::R) reader structure"]
impl crate::Readable for CH3PADDR {}
#[doc = "`write(|w| ..)` method takes [ch3paddr::W](ch3paddr::W) writer structure"]
impl crate::Writable for CH3PADDR {}
#[doc = "Channel 3 peripheral base address register"]
pub mod ch3paddr;
#[doc = "Channel 3 memory base address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3maddr](ch3maddr) module"]
pub type CH3MADDR = crate::Reg<u32, _CH3MADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3MADDR;
#[doc = "`read()` method returns [ch3maddr::R](ch3maddr::R) reader structure"]
impl crate::Readable for CH3MADDR {}
#[doc = "`write(|w| ..)` method takes [ch3maddr::W](ch3maddr::W) writer structure"]
impl crate::Writable for CH3MADDR {}
#[doc = "Channel 3 memory base address register"]
pub mod ch3maddr;
#[doc = "Channel 4 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4ctl](ch4ctl) module"]
pub type CH4CTL = crate::Reg<u32, _CH4CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4CTL;
#[doc = "`read()` method returns [ch4ctl::R](ch4ctl::R) reader structure"]
impl crate::Readable for CH4CTL {}
#[doc = "`write(|w| ..)` method takes [ch4ctl::W](ch4ctl::W) writer structure"]
impl crate::Writable for CH4CTL {}
#[doc = "Channel 4 control register"]
pub mod ch4ctl;
#[doc = "Channel 4 counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4cnt](ch4cnt) module"]
pub type CH4CNT = crate::Reg<u32, _CH4CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4CNT;
#[doc = "`read()` method returns [ch4cnt::R](ch4cnt::R) reader structure"]
impl crate::Readable for CH4CNT {}
#[doc = "`write(|w| ..)` method takes [ch4cnt::W](ch4cnt::W) writer structure"]
impl crate::Writable for CH4CNT {}
#[doc = "Channel 4 counter register"]
pub mod ch4cnt;
#[doc = "Channel 4 peripheral base address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4paddr](ch4paddr) module"]
pub type CH4PADDR = crate::Reg<u32, _CH4PADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4PADDR;
#[doc = "`read()` method returns [ch4paddr::R](ch4paddr::R) reader structure"]
impl crate::Readable for CH4PADDR {}
#[doc = "`write(|w| ..)` method takes [ch4paddr::W](ch4paddr::W) writer structure"]
impl crate::Writable for CH4PADDR {}
#[doc = "Channel 4 peripheral base address register"]
pub mod ch4paddr;
#[doc = "Channel 4 memory base address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4maddr](ch4maddr) module"]
pub type CH4MADDR = crate::Reg<u32, _CH4MADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4MADDR;
#[doc = "`read()` method returns [ch4maddr::R](ch4maddr::R) reader structure"]
impl crate::Readable for CH4MADDR {}
#[doc = "`write(|w| ..)` method takes [ch4maddr::W](ch4maddr::W) writer structure"]
impl crate::Writable for CH4MADDR {}
#[doc = "Channel 4 memory base address register"]
pub mod ch4maddr;
#[doc = "Channel 5 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5ctl](ch5ctl) module"]
pub type CH5CTL = crate::Reg<u32, _CH5CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5CTL;
#[doc = "`read()` method returns [ch5ctl::R](ch5ctl::R) reader structure"]
impl crate::Readable for CH5CTL {}
#[doc = "`write(|w| ..)` method takes [ch5ctl::W](ch5ctl::W) writer structure"]
impl crate::Writable for CH5CTL {}
#[doc = "Channel 5 control register"]
pub mod ch5ctl;
#[doc = "Channel 5 counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5cnt](ch5cnt) module"]
pub type CH5CNT = crate::Reg<u32, _CH5CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5CNT;
#[doc = "`read()` method returns [ch5cnt::R](ch5cnt::R) reader structure"]
impl crate::Readable for CH5CNT {}
#[doc = "`write(|w| ..)` method takes [ch5cnt::W](ch5cnt::W) writer structure"]
impl crate::Writable for CH5CNT {}
#[doc = "Channel 5 counter register"]
pub mod ch5cnt;
#[doc = "Channel 5 peripheral base address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5paddr](ch5paddr) module"]
pub type CH5PADDR = crate::Reg<u32, _CH5PADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5PADDR;
#[doc = "`read()` method returns [ch5paddr::R](ch5paddr::R) reader structure"]
impl crate::Readable for CH5PADDR {}
#[doc = "`write(|w| ..)` method takes [ch5paddr::W](ch5paddr::W) writer structure"]
impl crate::Writable for CH5PADDR {}
#[doc = "Channel 5 peripheral base address register"]
pub mod ch5paddr;
#[doc = "Channel 5 memory base address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5maddr](ch5maddr) module"]
pub type CH5MADDR = crate::Reg<u32, _CH5MADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5MADDR;
#[doc = "`read()` method returns [ch5maddr::R](ch5maddr::R) reader structure"]
impl crate::Readable for CH5MADDR {}
#[doc = "`write(|w| ..)` method takes [ch5maddr::W](ch5maddr::W) writer structure"]
impl crate::Writable for CH5MADDR {}
#[doc = "Channel 5 memory base address register"]
pub mod ch5maddr;
#[doc = "Channel 6 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6ctl](ch6ctl) module"]
pub type CH6CTL = crate::Reg<u32, _CH6CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6CTL;
#[doc = "`read()` method returns [ch6ctl::R](ch6ctl::R) reader structure"]
impl crate::Readable for CH6CTL {}
#[doc = "`write(|w| ..)` method takes [ch6ctl::W](ch6ctl::W) writer structure"]
impl crate::Writable for CH6CTL {}
#[doc = "Channel 6 control register"]
pub mod ch6ctl;
#[doc = "Channel 6 counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6cnt](ch6cnt) module"]
pub type CH6CNT = crate::Reg<u32, _CH6CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6CNT;
#[doc = "`read()` method returns [ch6cnt::R](ch6cnt::R) reader structure"]
impl crate::Readable for CH6CNT {}
#[doc = "`write(|w| ..)` method takes [ch6cnt::W](ch6cnt::W) writer structure"]
impl crate::Writable for CH6CNT {}
#[doc = "Channel 6 counter register"]
pub mod ch6cnt;
#[doc = "Channel 6 peripheral base address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6paddr](ch6paddr) module"]
pub type CH6PADDR = crate::Reg<u32, _CH6PADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6PADDR;
#[doc = "`read()` method returns [ch6paddr::R](ch6paddr::R) reader structure"]
impl crate::Readable for CH6PADDR {}
#[doc = "`write(|w| ..)` method takes [ch6paddr::W](ch6paddr::W) writer structure"]
impl crate::Writable for CH6PADDR {}
#[doc = "Channel 6 peripheral base address register"]
pub mod ch6paddr;
#[doc = "Channel 6 memory base address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6maddr](ch6maddr) module"]
pub type CH6MADDR = crate::Reg<u32, _CH6MADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6MADDR;
#[doc = "`read()` method returns [ch6maddr::R](ch6maddr::R) reader structure"]
impl crate::Readable for CH6MADDR {}
#[doc = "`write(|w| ..)` method takes [ch6maddr::W](ch6maddr::W) writer structure"]
impl crate::Writable for CH6MADDR {}
#[doc = "Channel 6 memory base address register"]
pub mod ch6maddr;
