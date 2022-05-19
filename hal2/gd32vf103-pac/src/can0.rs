#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub ctl: CTL,
    #[doc = "0x04 - Status register"]
    pub stat: STAT,
    #[doc = "0x08 - Transmit status register"]
    pub tstat: TSTAT,
    #[doc = "0x0c - Receive message FIFO0 register"]
    pub rfifo0: RFIFO0,
    #[doc = "0x10 - Receive message FIFO1 register"]
    pub rfifo1: RFIFO1,
    #[doc = "0x14 - Interrupt enable register"]
    pub inten: INTEN,
    #[doc = "0x18 - Error register"]
    pub err: ERR,
    #[doc = "0x1c - Bit timing register"]
    pub bt: BT,
    _reserved8: [u8; 352usize],
    #[doc = "0x180 - Transmit mailbox identifier register 0"]
    pub tmi0: TMI0,
    #[doc = "0x184 - Transmit mailbox property register 0"]
    pub tmp0: TMP0,
    #[doc = "0x188 - Transmit mailbox data0 register"]
    pub tmdata00: TMDATA00,
    #[doc = "0x18c - Transmit mailbox data1 register"]
    pub tmdata10: TMDATA10,
    #[doc = "0x190 - Transmit mailbox identifier register 1"]
    pub tmi1: TMI1,
    #[doc = "0x194 - Transmit mailbox property register 1"]
    pub tmp1: TMP1,
    #[doc = "0x198 - Transmit mailbox data0 register"]
    pub tmdata01: TMDATA01,
    #[doc = "0x19c - Transmit mailbox data1 register"]
    pub tmdata11: TMDATA11,
    #[doc = "0x1a0 - Transmit mailbox identifier register 2"]
    pub tmi2: TMI2,
    #[doc = "0x1a4 - Transmit mailbox property register 2"]
    pub tmp2: TMP2,
    #[doc = "0x1a8 - Transmit mailbox data0 register"]
    pub tmdata02: TMDATA02,
    #[doc = "0x1ac - Transmit mailbox data1 register"]
    pub tmdata12: TMDATA12,
    #[doc = "0x1b0 - Receive FIFO mailbox identifier register"]
    pub rfifomi0: RFIFOMI0,
    #[doc = "0x1b4 - Receive FIFO0 mailbox property register"]
    pub rfifomp0: RFIFOMP0,
    #[doc = "0x1b8 - Receive FIFO0 mailbox data0 register"]
    pub rfifomdata00: RFIFOMDATA00,
    #[doc = "0x1bc - Receive FIFO0 mailbox data1 register"]
    pub rfifomdata10: RFIFOMDATA10,
    #[doc = "0x1c0 - Receive FIFO1 mailbox identifier register"]
    pub rfifomi1: RFIFOMI1,
    #[doc = "0x1c4 - Receive FIFO1 mailbox property register"]
    pub rfifomp1: RFIFOMP1,
    #[doc = "0x1c8 - Receive FIFO1 mailbox data0 register"]
    pub rfifomdata01: RFIFOMDATA01,
    #[doc = "0x1cc - Receive FIFO1 mailbox data1 register"]
    pub rfifomdata11: RFIFOMDATA11,
    _reserved28: [u8; 48usize],
    #[doc = "0x200 - Filter control register"]
    pub fctl: FCTL,
    #[doc = "0x204 - Filter mode configuration register"]
    pub fmcfg: FMCFG,
    _reserved30: [u8; 4usize],
    #[doc = "0x20c - Filter scale configuration register"]
    pub fscfg: FSCFG,
    _reserved31: [u8; 4usize],
    #[doc = "0x214 - Filter associated FIFO register"]
    pub fafifo: FAFIFO,
    _reserved32: [u8; 4usize],
    #[doc = "0x21c - Filter working register"]
    pub fw: FW,
    _reserved33: [u8; 32usize],
    #[doc = "0x240 - Filter 0 data 0 register"]
    pub f0data0: F0DATA0,
    #[doc = "0x244 - Filter 0 data 1 register"]
    pub f0data1: F0DATA1,
    #[doc = "0x248 - Filter 1 data 0 register"]
    pub f1data0: F1DATA0,
    #[doc = "0x24c - Filter 1 data 1 register"]
    pub f1data1: F1DATA1,
    #[doc = "0x250 - Filter 2 data 0 register"]
    pub f2data0: F2DATA0,
    #[doc = "0x254 - Filter 2 data 1 register"]
    pub f2data1: F2DATA1,
    #[doc = "0x258 - Filter 3 data 0 register"]
    pub f3data0: F3DATA0,
    #[doc = "0x25c - Filter 3 data 1 register"]
    pub f3data1: F3DATA1,
    #[doc = "0x260 - Filter 4 data 0 register"]
    pub f4data0: F4DATA0,
    #[doc = "0x264 - Filter 4 data 1 register"]
    pub f4data1: F4DATA1,
    #[doc = "0x268 - Filter 5 data 0 register"]
    pub f5data0: F5DATA0,
    #[doc = "0x26c - Filter 5 data 1 register"]
    pub f5data1: F5DATA1,
    #[doc = "0x270 - Filter 6 data 0 register"]
    pub f6data0: F6DATA0,
    #[doc = "0x274 - Filter 6 data 1 register"]
    pub f6data1: F6DATA1,
    #[doc = "0x278 - Filter 7 data 0 register"]
    pub f7data0: F7DATA0,
    #[doc = "0x27c - Filter 7 data 1 register"]
    pub f7data1: F7DATA1,
    #[doc = "0x280 - Filter 8 data 0 register"]
    pub f8data0: F8DATA0,
    #[doc = "0x284 - Filter 8 data 1 register"]
    pub f8data1: F8DATA1,
    #[doc = "0x288 - Filter 9 data 0 register"]
    pub f9data0: F9DATA0,
    #[doc = "0x28c - Filter 9 data 1 register"]
    pub f9data1: F9DATA1,
    #[doc = "0x290 - Filter 10 data 0 register"]
    pub f10data0: F10DATA0,
    #[doc = "0x294 - Filter 10 data 1 register"]
    pub f10data1: F10DATA1,
    #[doc = "0x298 - Filter 11 data 0 register"]
    pub f11data0: F11DATA0,
    #[doc = "0x29c - Filter 11 data 1 register"]
    pub f11data1: F11DATA1,
    #[doc = "0x2a0 - Filter 12 data 0 register"]
    pub f12data0: F12DATA0,
    #[doc = "0x2a4 - Filter 12 data 1 register"]
    pub f12data1: F12DATA1,
    #[doc = "0x2a8 - Filter 13 data 0 register"]
    pub f13data0: F13DATA0,
    #[doc = "0x2ac - Filter 13 data 1 register"]
    pub f13data1: F13DATA1,
    #[doc = "0x2b0 - Filter 14 data 0 register"]
    pub f14data0: F14DATA0,
    #[doc = "0x2b4 - Filter 14 data 1 register"]
    pub f14data1: F14DATA1,
    #[doc = "0x2b8 - Filter 15 data 0 register"]
    pub f15data0: F15DATA0,
    #[doc = "0x2bc - Filter 15 data 1 register"]
    pub f15data1: F15DATA1,
    #[doc = "0x2c0 - Filter 16 data 0 register"]
    pub f16data0: F16DATA0,
    #[doc = "0x2c4 - Filter 16 data 1 register"]
    pub f16data1: F16DATA1,
    #[doc = "0x2c8 - Filter 17 data 0 register"]
    pub f17data0: F17DATA0,
    #[doc = "0x2cc - Filter 17 data 1 register"]
    pub f17data1: F17DATA1,
    #[doc = "0x2d0 - Filter 18 data 0 register"]
    pub f18data0: F18DATA0,
    #[doc = "0x2d4 - Filter 18 data 1 register"]
    pub f18data1: F18DATA1,
    #[doc = "0x2d8 - Filter 19 data 0 register"]
    pub f19data0: F19DATA0,
    #[doc = "0x2dc - Filter 19 data 1 register"]
    pub f19data1: F19DATA1,
    #[doc = "0x2e0 - Filter 20 data 0 register"]
    pub f20data0: F20DATA0,
    #[doc = "0x2e4 - Filter 20 data 1 register"]
    pub f20data1: F20DATA1,
    #[doc = "0x2e8 - Filter 21 data 0 register"]
    pub f21data0: F21DATA0,
    #[doc = "0x2ec - Filter 21 data 1 register"]
    pub f21data1: F21DATA1,
    #[doc = "0x2f0 - Filter 22 data 0 register"]
    pub f22data0: F22DATA0,
    #[doc = "0x2f4 - Filter 22 data 1 register"]
    pub f22data1: F22DATA1,
    #[doc = "0x2f8 - Filter 23 data 0 register"]
    pub f23data0: F23DATA0,
    #[doc = "0x2fc - Filter 23 data 1 register"]
    pub f23data1: F23DATA1,
    #[doc = "0x300 - Filter 24 data 0 register"]
    pub f24data0: F24DATA0,
    #[doc = "0x304 - Filter 24 data 1 register"]
    pub f24data1: F24DATA1,
    #[doc = "0x308 - Filter 25 data 0 register"]
    pub f25data0: F25DATA0,
    #[doc = "0x30c - Filter 25 data 1 register"]
    pub f25data1: F25DATA1,
    #[doc = "0x310 - Filter 26 data 0 register"]
    pub f26data0: F26DATA0,
    #[doc = "0x314 - Filter 26 data 1 register"]
    pub f26data1: F26DATA1,
    #[doc = "0x318 - Filter 27 data 0 register"]
    pub f27data0: F27DATA0,
    #[doc = "0x31c - Filter 27 data 1 register"]
    pub f27data1: F27DATA1,
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Control register"]
pub mod ctl;
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "Status register"]
pub mod stat;
#[doc = "Transmit status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tstat](tstat) module"]
pub type TSTAT = crate::Reg<u32, _TSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSTAT;
#[doc = "`read()` method returns [tstat::R](tstat::R) reader structure"]
impl crate::Readable for TSTAT {}
#[doc = "`write(|w| ..)` method takes [tstat::W](tstat::W) writer structure"]
impl crate::Writable for TSTAT {}
#[doc = "Transmit status register"]
pub mod tstat;
#[doc = "Receive message FIFO0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfifo0](rfifo0) module"]
pub type RFIFO0 = crate::Reg<u32, _RFIFO0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFIFO0;
#[doc = "`read()` method returns [rfifo0::R](rfifo0::R) reader structure"]
impl crate::Readable for RFIFO0 {}
#[doc = "`write(|w| ..)` method takes [rfifo0::W](rfifo0::W) writer structure"]
impl crate::Writable for RFIFO0 {}
#[doc = "Receive message FIFO0 register"]
pub mod rfifo0;
#[doc = "Receive message FIFO1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfifo1](rfifo1) module"]
pub type RFIFO1 = crate::Reg<u32, _RFIFO1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFIFO1;
#[doc = "`read()` method returns [rfifo1::R](rfifo1::R) reader structure"]
impl crate::Readable for RFIFO1 {}
#[doc = "`write(|w| ..)` method takes [rfifo1::W](rfifo1::W) writer structure"]
impl crate::Writable for RFIFO1 {}
#[doc = "Receive message FIFO1 register"]
pub mod rfifo1;
#[doc = "Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "Interrupt enable register"]
pub mod inten;
#[doc = "Error register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err](err) module"]
pub type ERR = crate::Reg<u32, _ERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERR;
#[doc = "`read()` method returns [err::R](err::R) reader structure"]
impl crate::Readable for ERR {}
#[doc = "`write(|w| ..)` method takes [err::W](err::W) writer structure"]
impl crate::Writable for ERR {}
#[doc = "Error register"]
pub mod err;
#[doc = "Bit timing register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bt](bt) module"]
pub type BT = crate::Reg<u32, _BT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BT;
#[doc = "`read()` method returns [bt::R](bt::R) reader structure"]
impl crate::Readable for BT {}
#[doc = "`write(|w| ..)` method takes [bt::W](bt::W) writer structure"]
impl crate::Writable for BT {}
#[doc = "Bit timing register"]
pub mod bt;
#[doc = "Transmit mailbox identifier register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmi0](tmi0) module"]
pub type TMI0 = crate::Reg<u32, _TMI0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMI0;
#[doc = "`read()` method returns [tmi0::R](tmi0::R) reader structure"]
impl crate::Readable for TMI0 {}
#[doc = "`write(|w| ..)` method takes [tmi0::W](tmi0::W) writer structure"]
impl crate::Writable for TMI0 {}
#[doc = "Transmit mailbox identifier register 0"]
pub mod tmi0;
#[doc = "Transmit mailbox property register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmp0](tmp0) module"]
pub type TMP0 = crate::Reg<u32, _TMP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMP0;
#[doc = "`read()` method returns [tmp0::R](tmp0::R) reader structure"]
impl crate::Readable for TMP0 {}
#[doc = "`write(|w| ..)` method takes [tmp0::W](tmp0::W) writer structure"]
impl crate::Writable for TMP0 {}
#[doc = "Transmit mailbox property register 0"]
pub mod tmp0;
#[doc = "Transmit mailbox data0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmdata00](tmdata00) module"]
pub type TMDATA00 = crate::Reg<u32, _TMDATA00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMDATA00;
#[doc = "`read()` method returns [tmdata00::R](tmdata00::R) reader structure"]
impl crate::Readable for TMDATA00 {}
#[doc = "`write(|w| ..)` method takes [tmdata00::W](tmdata00::W) writer structure"]
impl crate::Writable for TMDATA00 {}
#[doc = "Transmit mailbox data0 register"]
pub mod tmdata00;
#[doc = "Transmit mailbox data1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmdata10](tmdata10) module"]
pub type TMDATA10 = crate::Reg<u32, _TMDATA10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMDATA10;
#[doc = "`read()` method returns [tmdata10::R](tmdata10::R) reader structure"]
impl crate::Readable for TMDATA10 {}
#[doc = "`write(|w| ..)` method takes [tmdata10::W](tmdata10::W) writer structure"]
impl crate::Writable for TMDATA10 {}
#[doc = "Transmit mailbox data1 register"]
pub mod tmdata10;
#[doc = "Transmit mailbox identifier register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmi1](tmi1) module"]
pub type TMI1 = crate::Reg<u32, _TMI1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMI1;
#[doc = "`read()` method returns [tmi1::R](tmi1::R) reader structure"]
impl crate::Readable for TMI1 {}
#[doc = "`write(|w| ..)` method takes [tmi1::W](tmi1::W) writer structure"]
impl crate::Writable for TMI1 {}
#[doc = "Transmit mailbox identifier register 1"]
pub mod tmi1;
#[doc = "Transmit mailbox property register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmp1](tmp1) module"]
pub type TMP1 = crate::Reg<u32, _TMP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMP1;
#[doc = "`read()` method returns [tmp1::R](tmp1::R) reader structure"]
impl crate::Readable for TMP1 {}
#[doc = "`write(|w| ..)` method takes [tmp1::W](tmp1::W) writer structure"]
impl crate::Writable for TMP1 {}
#[doc = "Transmit mailbox property register 1"]
pub mod tmp1;
#[doc = "Transmit mailbox data0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmdata01](tmdata01) module"]
pub type TMDATA01 = crate::Reg<u32, _TMDATA01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMDATA01;
#[doc = "`read()` method returns [tmdata01::R](tmdata01::R) reader structure"]
impl crate::Readable for TMDATA01 {}
#[doc = "`write(|w| ..)` method takes [tmdata01::W](tmdata01::W) writer structure"]
impl crate::Writable for TMDATA01 {}
#[doc = "Transmit mailbox data0 register"]
pub mod tmdata01;
#[doc = "Transmit mailbox data1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmdata11](tmdata11) module"]
pub type TMDATA11 = crate::Reg<u32, _TMDATA11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMDATA11;
#[doc = "`read()` method returns [tmdata11::R](tmdata11::R) reader structure"]
impl crate::Readable for TMDATA11 {}
#[doc = "`write(|w| ..)` method takes [tmdata11::W](tmdata11::W) writer structure"]
impl crate::Writable for TMDATA11 {}
#[doc = "Transmit mailbox data1 register"]
pub mod tmdata11;
#[doc = "Transmit mailbox identifier register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmi2](tmi2) module"]
pub type TMI2 = crate::Reg<u32, _TMI2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMI2;
#[doc = "`read()` method returns [tmi2::R](tmi2::R) reader structure"]
impl crate::Readable for TMI2 {}
#[doc = "`write(|w| ..)` method takes [tmi2::W](tmi2::W) writer structure"]
impl crate::Writable for TMI2 {}
#[doc = "Transmit mailbox identifier register 2"]
pub mod tmi2;
#[doc = "Transmit mailbox property register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmp2](tmp2) module"]
pub type TMP2 = crate::Reg<u32, _TMP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMP2;
#[doc = "`read()` method returns [tmp2::R](tmp2::R) reader structure"]
impl crate::Readable for TMP2 {}
#[doc = "`write(|w| ..)` method takes [tmp2::W](tmp2::W) writer structure"]
impl crate::Writable for TMP2 {}
#[doc = "Transmit mailbox property register 2"]
pub mod tmp2;
#[doc = "Transmit mailbox data0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmdata02](tmdata02) module"]
pub type TMDATA02 = crate::Reg<u32, _TMDATA02>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMDATA02;
#[doc = "`read()` method returns [tmdata02::R](tmdata02::R) reader structure"]
impl crate::Readable for TMDATA02 {}
#[doc = "`write(|w| ..)` method takes [tmdata02::W](tmdata02::W) writer structure"]
impl crate::Writable for TMDATA02 {}
#[doc = "Transmit mailbox data0 register"]
pub mod tmdata02;
#[doc = "Transmit mailbox data1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmdata12](tmdata12) module"]
pub type TMDATA12 = crate::Reg<u32, _TMDATA12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMDATA12;
#[doc = "`read()` method returns [tmdata12::R](tmdata12::R) reader structure"]
impl crate::Readable for TMDATA12 {}
#[doc = "`write(|w| ..)` method takes [tmdata12::W](tmdata12::W) writer structure"]
impl crate::Writable for TMDATA12 {}
#[doc = "Transmit mailbox data1 register"]
pub mod tmdata12;
#[doc = "Receive FIFO mailbox identifier register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfifomi0](rfifomi0) module"]
pub type RFIFOMI0 = crate::Reg<u32, _RFIFOMI0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFIFOMI0;
#[doc = "`read()` method returns [rfifomi0::R](rfifomi0::R) reader structure"]
impl crate::Readable for RFIFOMI0 {}
#[doc = "Receive FIFO mailbox identifier register"]
pub mod rfifomi0;
#[doc = "Receive FIFO0 mailbox property register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfifomp0](rfifomp0) module"]
pub type RFIFOMP0 = crate::Reg<u32, _RFIFOMP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFIFOMP0;
#[doc = "`read()` method returns [rfifomp0::R](rfifomp0::R) reader structure"]
impl crate::Readable for RFIFOMP0 {}
#[doc = "Receive FIFO0 mailbox property register"]
pub mod rfifomp0;
#[doc = "Receive FIFO0 mailbox data0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfifomdata00](rfifomdata00) module"]
pub type RFIFOMDATA00 = crate::Reg<u32, _RFIFOMDATA00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFIFOMDATA00;
#[doc = "`read()` method returns [rfifomdata00::R](rfifomdata00::R) reader structure"]
impl crate::Readable for RFIFOMDATA00 {}
#[doc = "Receive FIFO0 mailbox data0 register"]
pub mod rfifomdata00;
#[doc = "Receive FIFO0 mailbox data1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfifomdata10](rfifomdata10) module"]
pub type RFIFOMDATA10 = crate::Reg<u32, _RFIFOMDATA10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFIFOMDATA10;
#[doc = "`read()` method returns [rfifomdata10::R](rfifomdata10::R) reader structure"]
impl crate::Readable for RFIFOMDATA10 {}
#[doc = "Receive FIFO0 mailbox data1 register"]
pub mod rfifomdata10;
#[doc = "Receive FIFO1 mailbox identifier register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfifomi1](rfifomi1) module"]
pub type RFIFOMI1 = crate::Reg<u32, _RFIFOMI1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFIFOMI1;
#[doc = "`read()` method returns [rfifomi1::R](rfifomi1::R) reader structure"]
impl crate::Readable for RFIFOMI1 {}
#[doc = "Receive FIFO1 mailbox identifier register"]
pub mod rfifomi1;
#[doc = "Receive FIFO1 mailbox property register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfifomp1](rfifomp1) module"]
pub type RFIFOMP1 = crate::Reg<u32, _RFIFOMP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFIFOMP1;
#[doc = "`read()` method returns [rfifomp1::R](rfifomp1::R) reader structure"]
impl crate::Readable for RFIFOMP1 {}
#[doc = "Receive FIFO1 mailbox property register"]
pub mod rfifomp1;
#[doc = "Receive FIFO1 mailbox data0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfifomdata01](rfifomdata01) module"]
pub type RFIFOMDATA01 = crate::Reg<u32, _RFIFOMDATA01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFIFOMDATA01;
#[doc = "`read()` method returns [rfifomdata01::R](rfifomdata01::R) reader structure"]
impl crate::Readable for RFIFOMDATA01 {}
#[doc = "Receive FIFO1 mailbox data0 register"]
pub mod rfifomdata01;
#[doc = "Receive FIFO1 mailbox data1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfifomdata11](rfifomdata11) module"]
pub type RFIFOMDATA11 = crate::Reg<u32, _RFIFOMDATA11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFIFOMDATA11;
#[doc = "`read()` method returns [rfifomdata11::R](rfifomdata11::R) reader structure"]
impl crate::Readable for RFIFOMDATA11 {}
#[doc = "Receive FIFO1 mailbox data1 register"]
pub mod rfifomdata11;
#[doc = "Filter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fctl](fctl) module"]
pub type FCTL = crate::Reg<u32, _FCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCTL;
#[doc = "`read()` method returns [fctl::R](fctl::R) reader structure"]
impl crate::Readable for FCTL {}
#[doc = "`write(|w| ..)` method takes [fctl::W](fctl::W) writer structure"]
impl crate::Writable for FCTL {}
#[doc = "Filter control register"]
pub mod fctl;
#[doc = "Filter mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmcfg](fmcfg) module"]
pub type FMCFG = crate::Reg<u32, _FMCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMCFG;
#[doc = "`read()` method returns [fmcfg::R](fmcfg::R) reader structure"]
impl crate::Readable for FMCFG {}
#[doc = "`write(|w| ..)` method takes [fmcfg::W](fmcfg::W) writer structure"]
impl crate::Writable for FMCFG {}
#[doc = "Filter mode configuration register"]
pub mod fmcfg;
#[doc = "Filter scale configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fscfg](fscfg) module"]
pub type FSCFG = crate::Reg<u32, _FSCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSCFG;
#[doc = "`read()` method returns [fscfg::R](fscfg::R) reader structure"]
impl crate::Readable for FSCFG {}
#[doc = "`write(|w| ..)` method takes [fscfg::W](fscfg::W) writer structure"]
impl crate::Writable for FSCFG {}
#[doc = "Filter scale configuration register"]
pub mod fscfg;
#[doc = "Filter associated FIFO register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fafifo](fafifo) module"]
pub type FAFIFO = crate::Reg<u32, _FAFIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FAFIFO;
#[doc = "`read()` method returns [fafifo::R](fafifo::R) reader structure"]
impl crate::Readable for FAFIFO {}
#[doc = "`write(|w| ..)` method takes [fafifo::W](fafifo::W) writer structure"]
impl crate::Writable for FAFIFO {}
#[doc = "Filter associated FIFO register"]
pub mod fafifo;
#[doc = "Filter working register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fw](fw) module"]
pub type FW = crate::Reg<u32, _FW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FW;
#[doc = "`read()` method returns [fw::R](fw::R) reader structure"]
impl crate::Readable for FW {}
#[doc = "`write(|w| ..)` method takes [fw::W](fw::W) writer structure"]
impl crate::Writable for FW {}
#[doc = "Filter working register"]
pub mod fw;
#[doc = "Filter 0 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f0data0](f0data0) module"]
pub type F0DATA0 = crate::Reg<u32, _F0DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F0DATA0;
#[doc = "`read()` method returns [f0data0::R](f0data0::R) reader structure"]
impl crate::Readable for F0DATA0 {}
#[doc = "`write(|w| ..)` method takes [f0data0::W](f0data0::W) writer structure"]
impl crate::Writable for F0DATA0 {}
#[doc = "Filter 0 data 0 register"]
pub mod f0data0;
#[doc = "Filter 0 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f0data1](f0data1) module"]
pub type F0DATA1 = crate::Reg<u32, _F0DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F0DATA1;
#[doc = "`read()` method returns [f0data1::R](f0data1::R) reader structure"]
impl crate::Readable for F0DATA1 {}
#[doc = "`write(|w| ..)` method takes [f0data1::W](f0data1::W) writer structure"]
impl crate::Writable for F0DATA1 {}
#[doc = "Filter 0 data 1 register"]
pub mod f0data1;
#[doc = "Filter 1 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f1data0](f1data0) module"]
pub type F1DATA0 = crate::Reg<u32, _F1DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F1DATA0;
#[doc = "`read()` method returns [f1data0::R](f1data0::R) reader structure"]
impl crate::Readable for F1DATA0 {}
#[doc = "`write(|w| ..)` method takes [f1data0::W](f1data0::W) writer structure"]
impl crate::Writable for F1DATA0 {}
#[doc = "Filter 1 data 0 register"]
pub mod f1data0;
#[doc = "Filter 1 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f1data1](f1data1) module"]
pub type F1DATA1 = crate::Reg<u32, _F1DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F1DATA1;
#[doc = "`read()` method returns [f1data1::R](f1data1::R) reader structure"]
impl crate::Readable for F1DATA1 {}
#[doc = "`write(|w| ..)` method takes [f1data1::W](f1data1::W) writer structure"]
impl crate::Writable for F1DATA1 {}
#[doc = "Filter 1 data 1 register"]
pub mod f1data1;
#[doc = "Filter 2 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f2data0](f2data0) module"]
pub type F2DATA0 = crate::Reg<u32, _F2DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F2DATA0;
#[doc = "`read()` method returns [f2data0::R](f2data0::R) reader structure"]
impl crate::Readable for F2DATA0 {}
#[doc = "`write(|w| ..)` method takes [f2data0::W](f2data0::W) writer structure"]
impl crate::Writable for F2DATA0 {}
#[doc = "Filter 2 data 0 register"]
pub mod f2data0;
#[doc = "Filter 2 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f2data1](f2data1) module"]
pub type F2DATA1 = crate::Reg<u32, _F2DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F2DATA1;
#[doc = "`read()` method returns [f2data1::R](f2data1::R) reader structure"]
impl crate::Readable for F2DATA1 {}
#[doc = "`write(|w| ..)` method takes [f2data1::W](f2data1::W) writer structure"]
impl crate::Writable for F2DATA1 {}
#[doc = "Filter 2 data 1 register"]
pub mod f2data1;
#[doc = "Filter 3 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f3data0](f3data0) module"]
pub type F3DATA0 = crate::Reg<u32, _F3DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F3DATA0;
#[doc = "`read()` method returns [f3data0::R](f3data0::R) reader structure"]
impl crate::Readable for F3DATA0 {}
#[doc = "`write(|w| ..)` method takes [f3data0::W](f3data0::W) writer structure"]
impl crate::Writable for F3DATA0 {}
#[doc = "Filter 3 data 0 register"]
pub mod f3data0;
#[doc = "Filter 3 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f3data1](f3data1) module"]
pub type F3DATA1 = crate::Reg<u32, _F3DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F3DATA1;
#[doc = "`read()` method returns [f3data1::R](f3data1::R) reader structure"]
impl crate::Readable for F3DATA1 {}
#[doc = "`write(|w| ..)` method takes [f3data1::W](f3data1::W) writer structure"]
impl crate::Writable for F3DATA1 {}
#[doc = "Filter 3 data 1 register"]
pub mod f3data1;
#[doc = "Filter 4 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f4data0](f4data0) module"]
pub type F4DATA0 = crate::Reg<u32, _F4DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F4DATA0;
#[doc = "`read()` method returns [f4data0::R](f4data0::R) reader structure"]
impl crate::Readable for F4DATA0 {}
#[doc = "`write(|w| ..)` method takes [f4data0::W](f4data0::W) writer structure"]
impl crate::Writable for F4DATA0 {}
#[doc = "Filter 4 data 0 register"]
pub mod f4data0;
#[doc = "Filter 4 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f4data1](f4data1) module"]
pub type F4DATA1 = crate::Reg<u32, _F4DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F4DATA1;
#[doc = "`read()` method returns [f4data1::R](f4data1::R) reader structure"]
impl crate::Readable for F4DATA1 {}
#[doc = "`write(|w| ..)` method takes [f4data1::W](f4data1::W) writer structure"]
impl crate::Writable for F4DATA1 {}
#[doc = "Filter 4 data 1 register"]
pub mod f4data1;
#[doc = "Filter 5 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f5data0](f5data0) module"]
pub type F5DATA0 = crate::Reg<u32, _F5DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F5DATA0;
#[doc = "`read()` method returns [f5data0::R](f5data0::R) reader structure"]
impl crate::Readable for F5DATA0 {}
#[doc = "`write(|w| ..)` method takes [f5data0::W](f5data0::W) writer structure"]
impl crate::Writable for F5DATA0 {}
#[doc = "Filter 5 data 0 register"]
pub mod f5data0;
#[doc = "Filter 5 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f5data1](f5data1) module"]
pub type F5DATA1 = crate::Reg<u32, _F5DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F5DATA1;
#[doc = "`read()` method returns [f5data1::R](f5data1::R) reader structure"]
impl crate::Readable for F5DATA1 {}
#[doc = "`write(|w| ..)` method takes [f5data1::W](f5data1::W) writer structure"]
impl crate::Writable for F5DATA1 {}
#[doc = "Filter 5 data 1 register"]
pub mod f5data1;
#[doc = "Filter 6 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f6data0](f6data0) module"]
pub type F6DATA0 = crate::Reg<u32, _F6DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F6DATA0;
#[doc = "`read()` method returns [f6data0::R](f6data0::R) reader structure"]
impl crate::Readable for F6DATA0 {}
#[doc = "`write(|w| ..)` method takes [f6data0::W](f6data0::W) writer structure"]
impl crate::Writable for F6DATA0 {}
#[doc = "Filter 6 data 0 register"]
pub mod f6data0;
#[doc = "Filter 6 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f6data1](f6data1) module"]
pub type F6DATA1 = crate::Reg<u32, _F6DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F6DATA1;
#[doc = "`read()` method returns [f6data1::R](f6data1::R) reader structure"]
impl crate::Readable for F6DATA1 {}
#[doc = "`write(|w| ..)` method takes [f6data1::W](f6data1::W) writer structure"]
impl crate::Writable for F6DATA1 {}
#[doc = "Filter 6 data 1 register"]
pub mod f6data1;
#[doc = "Filter 7 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f7data0](f7data0) module"]
pub type F7DATA0 = crate::Reg<u32, _F7DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F7DATA0;
#[doc = "`read()` method returns [f7data0::R](f7data0::R) reader structure"]
impl crate::Readable for F7DATA0 {}
#[doc = "`write(|w| ..)` method takes [f7data0::W](f7data0::W) writer structure"]
impl crate::Writable for F7DATA0 {}
#[doc = "Filter 7 data 0 register"]
pub mod f7data0;
#[doc = "Filter 7 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f7data1](f7data1) module"]
pub type F7DATA1 = crate::Reg<u32, _F7DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F7DATA1;
#[doc = "`read()` method returns [f7data1::R](f7data1::R) reader structure"]
impl crate::Readable for F7DATA1 {}
#[doc = "`write(|w| ..)` method takes [f7data1::W](f7data1::W) writer structure"]
impl crate::Writable for F7DATA1 {}
#[doc = "Filter 7 data 1 register"]
pub mod f7data1;
#[doc = "Filter 8 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f8data0](f8data0) module"]
pub type F8DATA0 = crate::Reg<u32, _F8DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F8DATA0;
#[doc = "`read()` method returns [f8data0::R](f8data0::R) reader structure"]
impl crate::Readable for F8DATA0 {}
#[doc = "`write(|w| ..)` method takes [f8data0::W](f8data0::W) writer structure"]
impl crate::Writable for F8DATA0 {}
#[doc = "Filter 8 data 0 register"]
pub mod f8data0;
#[doc = "Filter 8 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f8data1](f8data1) module"]
pub type F8DATA1 = crate::Reg<u32, _F8DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F8DATA1;
#[doc = "`read()` method returns [f8data1::R](f8data1::R) reader structure"]
impl crate::Readable for F8DATA1 {}
#[doc = "`write(|w| ..)` method takes [f8data1::W](f8data1::W) writer structure"]
impl crate::Writable for F8DATA1 {}
#[doc = "Filter 8 data 1 register"]
pub mod f8data1;
#[doc = "Filter 9 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f9data0](f9data0) module"]
pub type F9DATA0 = crate::Reg<u32, _F9DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F9DATA0;
#[doc = "`read()` method returns [f9data0::R](f9data0::R) reader structure"]
impl crate::Readable for F9DATA0 {}
#[doc = "`write(|w| ..)` method takes [f9data0::W](f9data0::W) writer structure"]
impl crate::Writable for F9DATA0 {}
#[doc = "Filter 9 data 0 register"]
pub mod f9data0;
#[doc = "Filter 9 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f9data1](f9data1) module"]
pub type F9DATA1 = crate::Reg<u32, _F9DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F9DATA1;
#[doc = "`read()` method returns [f9data1::R](f9data1::R) reader structure"]
impl crate::Readable for F9DATA1 {}
#[doc = "`write(|w| ..)` method takes [f9data1::W](f9data1::W) writer structure"]
impl crate::Writable for F9DATA1 {}
#[doc = "Filter 9 data 1 register"]
pub mod f9data1;
#[doc = "Filter 10 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f10data0](f10data0) module"]
pub type F10DATA0 = crate::Reg<u32, _F10DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F10DATA0;
#[doc = "`read()` method returns [f10data0::R](f10data0::R) reader structure"]
impl crate::Readable for F10DATA0 {}
#[doc = "`write(|w| ..)` method takes [f10data0::W](f10data0::W) writer structure"]
impl crate::Writable for F10DATA0 {}
#[doc = "Filter 10 data 0 register"]
pub mod f10data0;
#[doc = "Filter 10 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f10data1](f10data1) module"]
pub type F10DATA1 = crate::Reg<u32, _F10DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F10DATA1;
#[doc = "`read()` method returns [f10data1::R](f10data1::R) reader structure"]
impl crate::Readable for F10DATA1 {}
#[doc = "`write(|w| ..)` method takes [f10data1::W](f10data1::W) writer structure"]
impl crate::Writable for F10DATA1 {}
#[doc = "Filter 10 data 1 register"]
pub mod f10data1;
#[doc = "Filter 11 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f11data0](f11data0) module"]
pub type F11DATA0 = crate::Reg<u32, _F11DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F11DATA0;
#[doc = "`read()` method returns [f11data0::R](f11data0::R) reader structure"]
impl crate::Readable for F11DATA0 {}
#[doc = "`write(|w| ..)` method takes [f11data0::W](f11data0::W) writer structure"]
impl crate::Writable for F11DATA0 {}
#[doc = "Filter 11 data 0 register"]
pub mod f11data0;
#[doc = "Filter 11 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f11data1](f11data1) module"]
pub type F11DATA1 = crate::Reg<u32, _F11DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F11DATA1;
#[doc = "`read()` method returns [f11data1::R](f11data1::R) reader structure"]
impl crate::Readable for F11DATA1 {}
#[doc = "`write(|w| ..)` method takes [f11data1::W](f11data1::W) writer structure"]
impl crate::Writable for F11DATA1 {}
#[doc = "Filter 11 data 1 register"]
pub mod f11data1;
#[doc = "Filter 12 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f12data0](f12data0) module"]
pub type F12DATA0 = crate::Reg<u32, _F12DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F12DATA0;
#[doc = "`read()` method returns [f12data0::R](f12data0::R) reader structure"]
impl crate::Readable for F12DATA0 {}
#[doc = "`write(|w| ..)` method takes [f12data0::W](f12data0::W) writer structure"]
impl crate::Writable for F12DATA0 {}
#[doc = "Filter 12 data 0 register"]
pub mod f12data0;
#[doc = "Filter 12 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f12data1](f12data1) module"]
pub type F12DATA1 = crate::Reg<u32, _F12DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F12DATA1;
#[doc = "`read()` method returns [f12data1::R](f12data1::R) reader structure"]
impl crate::Readable for F12DATA1 {}
#[doc = "`write(|w| ..)` method takes [f12data1::W](f12data1::W) writer structure"]
impl crate::Writable for F12DATA1 {}
#[doc = "Filter 12 data 1 register"]
pub mod f12data1;
#[doc = "Filter 13 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f13data0](f13data0) module"]
pub type F13DATA0 = crate::Reg<u32, _F13DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F13DATA0;
#[doc = "`read()` method returns [f13data0::R](f13data0::R) reader structure"]
impl crate::Readable for F13DATA0 {}
#[doc = "`write(|w| ..)` method takes [f13data0::W](f13data0::W) writer structure"]
impl crate::Writable for F13DATA0 {}
#[doc = "Filter 13 data 0 register"]
pub mod f13data0;
#[doc = "Filter 13 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f13data1](f13data1) module"]
pub type F13DATA1 = crate::Reg<u32, _F13DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F13DATA1;
#[doc = "`read()` method returns [f13data1::R](f13data1::R) reader structure"]
impl crate::Readable for F13DATA1 {}
#[doc = "`write(|w| ..)` method takes [f13data1::W](f13data1::W) writer structure"]
impl crate::Writable for F13DATA1 {}
#[doc = "Filter 13 data 1 register"]
pub mod f13data1;
#[doc = "Filter 14 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f14data0](f14data0) module"]
pub type F14DATA0 = crate::Reg<u32, _F14DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F14DATA0;
#[doc = "`read()` method returns [f14data0::R](f14data0::R) reader structure"]
impl crate::Readable for F14DATA0 {}
#[doc = "`write(|w| ..)` method takes [f14data0::W](f14data0::W) writer structure"]
impl crate::Writable for F14DATA0 {}
#[doc = "Filter 14 data 0 register"]
pub mod f14data0;
#[doc = "Filter 14 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f14data1](f14data1) module"]
pub type F14DATA1 = crate::Reg<u32, _F14DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F14DATA1;
#[doc = "`read()` method returns [f14data1::R](f14data1::R) reader structure"]
impl crate::Readable for F14DATA1 {}
#[doc = "`write(|w| ..)` method takes [f14data1::W](f14data1::W) writer structure"]
impl crate::Writable for F14DATA1 {}
#[doc = "Filter 14 data 1 register"]
pub mod f14data1;
#[doc = "Filter 15 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f15data0](f15data0) module"]
pub type F15DATA0 = crate::Reg<u32, _F15DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F15DATA0;
#[doc = "`read()` method returns [f15data0::R](f15data0::R) reader structure"]
impl crate::Readable for F15DATA0 {}
#[doc = "`write(|w| ..)` method takes [f15data0::W](f15data0::W) writer structure"]
impl crate::Writable for F15DATA0 {}
#[doc = "Filter 15 data 0 register"]
pub mod f15data0;
#[doc = "Filter 15 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f15data1](f15data1) module"]
pub type F15DATA1 = crate::Reg<u32, _F15DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F15DATA1;
#[doc = "`read()` method returns [f15data1::R](f15data1::R) reader structure"]
impl crate::Readable for F15DATA1 {}
#[doc = "`write(|w| ..)` method takes [f15data1::W](f15data1::W) writer structure"]
impl crate::Writable for F15DATA1 {}
#[doc = "Filter 15 data 1 register"]
pub mod f15data1;
#[doc = "Filter 16 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f16data0](f16data0) module"]
pub type F16DATA0 = crate::Reg<u32, _F16DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F16DATA0;
#[doc = "`read()` method returns [f16data0::R](f16data0::R) reader structure"]
impl crate::Readable for F16DATA0 {}
#[doc = "`write(|w| ..)` method takes [f16data0::W](f16data0::W) writer structure"]
impl crate::Writable for F16DATA0 {}
#[doc = "Filter 16 data 0 register"]
pub mod f16data0;
#[doc = "Filter 16 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f16data1](f16data1) module"]
pub type F16DATA1 = crate::Reg<u32, _F16DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F16DATA1;
#[doc = "`read()` method returns [f16data1::R](f16data1::R) reader structure"]
impl crate::Readable for F16DATA1 {}
#[doc = "`write(|w| ..)` method takes [f16data1::W](f16data1::W) writer structure"]
impl crate::Writable for F16DATA1 {}
#[doc = "Filter 16 data 1 register"]
pub mod f16data1;
#[doc = "Filter 17 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f17data0](f17data0) module"]
pub type F17DATA0 = crate::Reg<u32, _F17DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F17DATA0;
#[doc = "`read()` method returns [f17data0::R](f17data0::R) reader structure"]
impl crate::Readable for F17DATA0 {}
#[doc = "`write(|w| ..)` method takes [f17data0::W](f17data0::W) writer structure"]
impl crate::Writable for F17DATA0 {}
#[doc = "Filter 17 data 0 register"]
pub mod f17data0;
#[doc = "Filter 17 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f17data1](f17data1) module"]
pub type F17DATA1 = crate::Reg<u32, _F17DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F17DATA1;
#[doc = "`read()` method returns [f17data1::R](f17data1::R) reader structure"]
impl crate::Readable for F17DATA1 {}
#[doc = "`write(|w| ..)` method takes [f17data1::W](f17data1::W) writer structure"]
impl crate::Writable for F17DATA1 {}
#[doc = "Filter 17 data 1 register"]
pub mod f17data1;
#[doc = "Filter 18 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f18data0](f18data0) module"]
pub type F18DATA0 = crate::Reg<u32, _F18DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F18DATA0;
#[doc = "`read()` method returns [f18data0::R](f18data0::R) reader structure"]
impl crate::Readable for F18DATA0 {}
#[doc = "`write(|w| ..)` method takes [f18data0::W](f18data0::W) writer structure"]
impl crate::Writable for F18DATA0 {}
#[doc = "Filter 18 data 0 register"]
pub mod f18data0;
#[doc = "Filter 18 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f18data1](f18data1) module"]
pub type F18DATA1 = crate::Reg<u32, _F18DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F18DATA1;
#[doc = "`read()` method returns [f18data1::R](f18data1::R) reader structure"]
impl crate::Readable for F18DATA1 {}
#[doc = "`write(|w| ..)` method takes [f18data1::W](f18data1::W) writer structure"]
impl crate::Writable for F18DATA1 {}
#[doc = "Filter 18 data 1 register"]
pub mod f18data1;
#[doc = "Filter 19 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f19data0](f19data0) module"]
pub type F19DATA0 = crate::Reg<u32, _F19DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F19DATA0;
#[doc = "`read()` method returns [f19data0::R](f19data0::R) reader structure"]
impl crate::Readable for F19DATA0 {}
#[doc = "`write(|w| ..)` method takes [f19data0::W](f19data0::W) writer structure"]
impl crate::Writable for F19DATA0 {}
#[doc = "Filter 19 data 0 register"]
pub mod f19data0;
#[doc = "Filter 19 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f19data1](f19data1) module"]
pub type F19DATA1 = crate::Reg<u32, _F19DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F19DATA1;
#[doc = "`read()` method returns [f19data1::R](f19data1::R) reader structure"]
impl crate::Readable for F19DATA1 {}
#[doc = "`write(|w| ..)` method takes [f19data1::W](f19data1::W) writer structure"]
impl crate::Writable for F19DATA1 {}
#[doc = "Filter 19 data 1 register"]
pub mod f19data1;
#[doc = "Filter 20 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f20data0](f20data0) module"]
pub type F20DATA0 = crate::Reg<u32, _F20DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F20DATA0;
#[doc = "`read()` method returns [f20data0::R](f20data0::R) reader structure"]
impl crate::Readable for F20DATA0 {}
#[doc = "`write(|w| ..)` method takes [f20data0::W](f20data0::W) writer structure"]
impl crate::Writable for F20DATA0 {}
#[doc = "Filter 20 data 0 register"]
pub mod f20data0;
#[doc = "Filter 20 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f20data1](f20data1) module"]
pub type F20DATA1 = crate::Reg<u32, _F20DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F20DATA1;
#[doc = "`read()` method returns [f20data1::R](f20data1::R) reader structure"]
impl crate::Readable for F20DATA1 {}
#[doc = "`write(|w| ..)` method takes [f20data1::W](f20data1::W) writer structure"]
impl crate::Writable for F20DATA1 {}
#[doc = "Filter 20 data 1 register"]
pub mod f20data1;
#[doc = "Filter 21 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f21data0](f21data0) module"]
pub type F21DATA0 = crate::Reg<u32, _F21DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F21DATA0;
#[doc = "`read()` method returns [f21data0::R](f21data0::R) reader structure"]
impl crate::Readable for F21DATA0 {}
#[doc = "`write(|w| ..)` method takes [f21data0::W](f21data0::W) writer structure"]
impl crate::Writable for F21DATA0 {}
#[doc = "Filter 21 data 0 register"]
pub mod f21data0;
#[doc = "Filter 21 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f21data1](f21data1) module"]
pub type F21DATA1 = crate::Reg<u32, _F21DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F21DATA1;
#[doc = "`read()` method returns [f21data1::R](f21data1::R) reader structure"]
impl crate::Readable for F21DATA1 {}
#[doc = "`write(|w| ..)` method takes [f21data1::W](f21data1::W) writer structure"]
impl crate::Writable for F21DATA1 {}
#[doc = "Filter 21 data 1 register"]
pub mod f21data1;
#[doc = "Filter 22 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f22data0](f22data0) module"]
pub type F22DATA0 = crate::Reg<u32, _F22DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F22DATA0;
#[doc = "`read()` method returns [f22data0::R](f22data0::R) reader structure"]
impl crate::Readable for F22DATA0 {}
#[doc = "`write(|w| ..)` method takes [f22data0::W](f22data0::W) writer structure"]
impl crate::Writable for F22DATA0 {}
#[doc = "Filter 22 data 0 register"]
pub mod f22data0;
#[doc = "Filter 22 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f22data1](f22data1) module"]
pub type F22DATA1 = crate::Reg<u32, _F22DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F22DATA1;
#[doc = "`read()` method returns [f22data1::R](f22data1::R) reader structure"]
impl crate::Readable for F22DATA1 {}
#[doc = "`write(|w| ..)` method takes [f22data1::W](f22data1::W) writer structure"]
impl crate::Writable for F22DATA1 {}
#[doc = "Filter 22 data 1 register"]
pub mod f22data1;
#[doc = "Filter 23 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f23data0](f23data0) module"]
pub type F23DATA0 = crate::Reg<u32, _F23DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F23DATA0;
#[doc = "`read()` method returns [f23data0::R](f23data0::R) reader structure"]
impl crate::Readable for F23DATA0 {}
#[doc = "`write(|w| ..)` method takes [f23data0::W](f23data0::W) writer structure"]
impl crate::Writable for F23DATA0 {}
#[doc = "Filter 23 data 0 register"]
pub mod f23data0;
#[doc = "Filter 23 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f23data1](f23data1) module"]
pub type F23DATA1 = crate::Reg<u32, _F23DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F23DATA1;
#[doc = "`read()` method returns [f23data1::R](f23data1::R) reader structure"]
impl crate::Readable for F23DATA1 {}
#[doc = "`write(|w| ..)` method takes [f23data1::W](f23data1::W) writer structure"]
impl crate::Writable for F23DATA1 {}
#[doc = "Filter 23 data 1 register"]
pub mod f23data1;
#[doc = "Filter 24 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f24data0](f24data0) module"]
pub type F24DATA0 = crate::Reg<u32, _F24DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F24DATA0;
#[doc = "`read()` method returns [f24data0::R](f24data0::R) reader structure"]
impl crate::Readable for F24DATA0 {}
#[doc = "`write(|w| ..)` method takes [f24data0::W](f24data0::W) writer structure"]
impl crate::Writable for F24DATA0 {}
#[doc = "Filter 24 data 0 register"]
pub mod f24data0;
#[doc = "Filter 24 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f24data1](f24data1) module"]
pub type F24DATA1 = crate::Reg<u32, _F24DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F24DATA1;
#[doc = "`read()` method returns [f24data1::R](f24data1::R) reader structure"]
impl crate::Readable for F24DATA1 {}
#[doc = "`write(|w| ..)` method takes [f24data1::W](f24data1::W) writer structure"]
impl crate::Writable for F24DATA1 {}
#[doc = "Filter 24 data 1 register"]
pub mod f24data1;
#[doc = "Filter 25 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f25data0](f25data0) module"]
pub type F25DATA0 = crate::Reg<u32, _F25DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F25DATA0;
#[doc = "`read()` method returns [f25data0::R](f25data0::R) reader structure"]
impl crate::Readable for F25DATA0 {}
#[doc = "`write(|w| ..)` method takes [f25data0::W](f25data0::W) writer structure"]
impl crate::Writable for F25DATA0 {}
#[doc = "Filter 25 data 0 register"]
pub mod f25data0;
#[doc = "Filter 25 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f25data1](f25data1) module"]
pub type F25DATA1 = crate::Reg<u32, _F25DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F25DATA1;
#[doc = "`read()` method returns [f25data1::R](f25data1::R) reader structure"]
impl crate::Readable for F25DATA1 {}
#[doc = "`write(|w| ..)` method takes [f25data1::W](f25data1::W) writer structure"]
impl crate::Writable for F25DATA1 {}
#[doc = "Filter 25 data 1 register"]
pub mod f25data1;
#[doc = "Filter 26 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f26data0](f26data0) module"]
pub type F26DATA0 = crate::Reg<u32, _F26DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F26DATA0;
#[doc = "`read()` method returns [f26data0::R](f26data0::R) reader structure"]
impl crate::Readable for F26DATA0 {}
#[doc = "`write(|w| ..)` method takes [f26data0::W](f26data0::W) writer structure"]
impl crate::Writable for F26DATA0 {}
#[doc = "Filter 26 data 0 register"]
pub mod f26data0;
#[doc = "Filter 26 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f26data1](f26data1) module"]
pub type F26DATA1 = crate::Reg<u32, _F26DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F26DATA1;
#[doc = "`read()` method returns [f26data1::R](f26data1::R) reader structure"]
impl crate::Readable for F26DATA1 {}
#[doc = "`write(|w| ..)` method takes [f26data1::W](f26data1::W) writer structure"]
impl crate::Writable for F26DATA1 {}
#[doc = "Filter 26 data 1 register"]
pub mod f26data1;
#[doc = "Filter 27 data 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f27data0](f27data0) module"]
pub type F27DATA0 = crate::Reg<u32, _F27DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F27DATA0;
#[doc = "`read()` method returns [f27data0::R](f27data0::R) reader structure"]
impl crate::Readable for F27DATA0 {}
#[doc = "`write(|w| ..)` method takes [f27data0::W](f27data0::W) writer structure"]
impl crate::Writable for F27DATA0 {}
#[doc = "Filter 27 data 0 register"]
pub mod f27data0;
#[doc = "Filter 27 data 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [f27data1](f27data1) module"]
pub type F27DATA1 = crate::Reg<u32, _F27DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _F27DATA1;
#[doc = "`read()` method returns [f27data1::R](f27data1::R) reader structure"]
impl crate::Readable for F27DATA1 {}
#[doc = "`write(|w| ..)` method takes [f27data1::W](f27data1::W) writer structure"]
impl crate::Writable for F27DATA1 {}
#[doc = "Filter 27 data 1 register"]
pub mod f27data1;
