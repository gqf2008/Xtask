#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 0"]
    pub ctl0: CTL0,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - control register 1"]
    pub ctl1: CTL1,
    _reserved2: [u8; 2usize],
    #[doc = "0x08 - status register"]
    pub stat: STAT,
    _reserved3: [u8; 2usize],
    #[doc = "0x0c - data register"]
    pub data: DATA,
    _reserved4: [u8; 2usize],
    #[doc = "0x10 - CRC polynomial register"]
    pub crcpoly: CRCPOLY,
    _reserved5: [u8; 2usize],
    #[doc = "0x14 - RX CRC register"]
    pub rcrc: RCRC,
    _reserved6: [u8; 2usize],
    #[doc = "0x18 - TX CRC register"]
    pub tcrc: TCRC,
    _reserved7: [u8; 2usize],
    #[doc = "0x1c - I2S control register"]
    pub i2sctl: I2SCTL,
    _reserved8: [u8; 2usize],
    #[doc = "0x20 - I2S prescaler register"]
    pub i2spsc: I2SPSC,
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
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u16, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "status register"]
pub mod stat;
#[doc = "data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](data) module"]
pub type DATA = crate::Reg<u16, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "data register"]
pub mod data;
#[doc = "CRC polynomial register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcpoly](crcpoly) module"]
pub type CRCPOLY = crate::Reg<u16, _CRCPOLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCPOLY;
#[doc = "`read()` method returns [crcpoly::R](crcpoly::R) reader structure"]
impl crate::Readable for CRCPOLY {}
#[doc = "`write(|w| ..)` method takes [crcpoly::W](crcpoly::W) writer structure"]
impl crate::Writable for CRCPOLY {}
#[doc = "CRC polynomial register"]
pub mod crcpoly;
#[doc = "RX CRC register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcrc](rcrc) module"]
pub type RCRC = crate::Reg<u16, _RCRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCRC;
#[doc = "`read()` method returns [rcrc::R](rcrc::R) reader structure"]
impl crate::Readable for RCRC {}
#[doc = "RX CRC register"]
pub mod rcrc;
#[doc = "TX CRC register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcrc](tcrc) module"]
pub type TCRC = crate::Reg<u16, _TCRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCRC;
#[doc = "`read()` method returns [tcrc::R](tcrc::R) reader structure"]
impl crate::Readable for TCRC {}
#[doc = "TX CRC register"]
pub mod tcrc;
#[doc = "I2S control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sctl](i2sctl) module"]
pub type I2SCTL = crate::Reg<u16, _I2SCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SCTL;
#[doc = "`read()` method returns [i2sctl::R](i2sctl::R) reader structure"]
impl crate::Readable for I2SCTL {}
#[doc = "`write(|w| ..)` method takes [i2sctl::W](i2sctl::W) writer structure"]
impl crate::Writable for I2SCTL {}
#[doc = "I2S control register"]
pub mod i2sctl;
#[doc = "I2S prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2spsc](i2spsc) module"]
pub type I2SPSC = crate::Reg<u16, _I2SPSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SPSC;
#[doc = "`read()` method returns [i2spsc::R](i2spsc::R) reader structure"]
impl crate::Readable for I2SPSC {}
#[doc = "`write(|w| ..)` method takes [i2spsc::W](i2spsc::W) writer structure"]
impl crate::Writable for I2SPSC {}
#[doc = "I2S prescaler register"]
pub mod i2spsc;
