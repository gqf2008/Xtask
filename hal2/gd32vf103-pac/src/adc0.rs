#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - status register"]
    pub stat: STAT,
    #[doc = "0x04 - control register 0"]
    pub ctl0: CTL0,
    #[doc = "0x08 - control register 1"]
    pub ctl1: CTL1,
    #[doc = "0x0c - Sample time register 0"]
    pub sampt0: SAMPT0,
    #[doc = "0x10 - Sample time register 1"]
    pub sampt1: SAMPT1,
    #[doc = "0x14 - Inserted channel data offset register 0"]
    pub ioff0: IOFF0,
    #[doc = "0x18 - Inserted channel data offset register 1"]
    pub ioff1: IOFF1,
    #[doc = "0x1c - Inserted channel data offset register 2"]
    pub ioff2: IOFF2,
    #[doc = "0x20 - Inserted channel data offset register 3"]
    pub ioff3: IOFF3,
    #[doc = "0x24 - watchdog higher threshold register"]
    pub wdht: WDHT,
    #[doc = "0x28 - watchdog lower threshold register"]
    pub wdlt: WDLT,
    #[doc = "0x2c - regular sequence register 0"]
    pub rsq0: RSQ0,
    #[doc = "0x30 - regular sequence register 1"]
    pub rsq1: RSQ1,
    #[doc = "0x34 - regular sequence register 2"]
    pub rsq2: RSQ2,
    #[doc = "0x38 - Inserted sequence register"]
    pub isq: ISQ,
    #[doc = "0x3c - Inserted data register 0"]
    pub idata0: IDATA0,
    #[doc = "0x40 - Inserted data register 1"]
    pub idata1: IDATA1,
    #[doc = "0x44 - Inserted data register 2"]
    pub idata2: IDATA2,
    #[doc = "0x48 - Inserted data register 3"]
    pub idata3: IDATA3,
    #[doc = "0x4c - regular data register"]
    pub rdata: RDATA,
    _reserved20: [u8; 48usize],
    #[doc = "0x80 - Oversample control register"]
    pub ovsampctl: OVSAMPCTL,
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "status register"]
pub mod stat;
#[doc = "control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](ctl0) module"]
pub type CTL0 = crate::Reg<u32, _CTL0>;
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
pub type CTL1 = crate::Reg<u32, _CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL1;
#[doc = "`read()` method returns [ctl1::R](ctl1::R) reader structure"]
impl crate::Readable for CTL1 {}
#[doc = "`write(|w| ..)` method takes [ctl1::W](ctl1::W) writer structure"]
impl crate::Writable for CTL1 {}
#[doc = "control register 1"]
pub mod ctl1;
#[doc = "Sample time register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sampt0](sampt0) module"]
pub type SAMPT0 = crate::Reg<u32, _SAMPT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAMPT0;
#[doc = "`read()` method returns [sampt0::R](sampt0::R) reader structure"]
impl crate::Readable for SAMPT0 {}
#[doc = "`write(|w| ..)` method takes [sampt0::W](sampt0::W) writer structure"]
impl crate::Writable for SAMPT0 {}
#[doc = "Sample time register 0"]
pub mod sampt0;
#[doc = "Sample time register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sampt1](sampt1) module"]
pub type SAMPT1 = crate::Reg<u32, _SAMPT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAMPT1;
#[doc = "`read()` method returns [sampt1::R](sampt1::R) reader structure"]
impl crate::Readable for SAMPT1 {}
#[doc = "`write(|w| ..)` method takes [sampt1::W](sampt1::W) writer structure"]
impl crate::Writable for SAMPT1 {}
#[doc = "Sample time register 1"]
pub mod sampt1;
#[doc = "Inserted channel data offset register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioff0](ioff0) module"]
pub type IOFF0 = crate::Reg<u32, _IOFF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOFF0;
#[doc = "`read()` method returns [ioff0::R](ioff0::R) reader structure"]
impl crate::Readable for IOFF0 {}
#[doc = "`write(|w| ..)` method takes [ioff0::W](ioff0::W) writer structure"]
impl crate::Writable for IOFF0 {}
#[doc = "Inserted channel data offset register 0"]
pub mod ioff0;
#[doc = "Inserted channel data offset register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioff1](ioff1) module"]
pub type IOFF1 = crate::Reg<u32, _IOFF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOFF1;
#[doc = "`read()` method returns [ioff1::R](ioff1::R) reader structure"]
impl crate::Readable for IOFF1 {}
#[doc = "`write(|w| ..)` method takes [ioff1::W](ioff1::W) writer structure"]
impl crate::Writable for IOFF1 {}
#[doc = "Inserted channel data offset register 1"]
pub mod ioff1;
#[doc = "Inserted channel data offset register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioff2](ioff2) module"]
pub type IOFF2 = crate::Reg<u32, _IOFF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOFF2;
#[doc = "`read()` method returns [ioff2::R](ioff2::R) reader structure"]
impl crate::Readable for IOFF2 {}
#[doc = "`write(|w| ..)` method takes [ioff2::W](ioff2::W) writer structure"]
impl crate::Writable for IOFF2 {}
#[doc = "Inserted channel data offset register 2"]
pub mod ioff2;
#[doc = "Inserted channel data offset register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioff3](ioff3) module"]
pub type IOFF3 = crate::Reg<u32, _IOFF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IOFF3;
#[doc = "`read()` method returns [ioff3::R](ioff3::R) reader structure"]
impl crate::Readable for IOFF3 {}
#[doc = "`write(|w| ..)` method takes [ioff3::W](ioff3::W) writer structure"]
impl crate::Writable for IOFF3 {}
#[doc = "Inserted channel data offset register 3"]
pub mod ioff3;
#[doc = "watchdog higher threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdht](wdht) module"]
pub type WDHT = crate::Reg<u32, _WDHT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDHT;
#[doc = "`read()` method returns [wdht::R](wdht::R) reader structure"]
impl crate::Readable for WDHT {}
#[doc = "`write(|w| ..)` method takes [wdht::W](wdht::W) writer structure"]
impl crate::Writable for WDHT {}
#[doc = "watchdog higher threshold register"]
pub mod wdht;
#[doc = "watchdog lower threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdlt](wdlt) module"]
pub type WDLT = crate::Reg<u32, _WDLT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDLT;
#[doc = "`read()` method returns [wdlt::R](wdlt::R) reader structure"]
impl crate::Readable for WDLT {}
#[doc = "`write(|w| ..)` method takes [wdlt::W](wdlt::W) writer structure"]
impl crate::Writable for WDLT {}
#[doc = "watchdog lower threshold register"]
pub mod wdlt;
#[doc = "regular sequence register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsq0](rsq0) module"]
pub type RSQ0 = crate::Reg<u32, _RSQ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSQ0;
#[doc = "`read()` method returns [rsq0::R](rsq0::R) reader structure"]
impl crate::Readable for RSQ0 {}
#[doc = "`write(|w| ..)` method takes [rsq0::W](rsq0::W) writer structure"]
impl crate::Writable for RSQ0 {}
#[doc = "regular sequence register 0"]
pub mod rsq0;
#[doc = "regular sequence register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsq1](rsq1) module"]
pub type RSQ1 = crate::Reg<u32, _RSQ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSQ1;
#[doc = "`read()` method returns [rsq1::R](rsq1::R) reader structure"]
impl crate::Readable for RSQ1 {}
#[doc = "`write(|w| ..)` method takes [rsq1::W](rsq1::W) writer structure"]
impl crate::Writable for RSQ1 {}
#[doc = "regular sequence register 1"]
pub mod rsq1;
#[doc = "regular sequence register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsq2](rsq2) module"]
pub type RSQ2 = crate::Reg<u32, _RSQ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSQ2;
#[doc = "`read()` method returns [rsq2::R](rsq2::R) reader structure"]
impl crate::Readable for RSQ2 {}
#[doc = "`write(|w| ..)` method takes [rsq2::W](rsq2::W) writer structure"]
impl crate::Writable for RSQ2 {}
#[doc = "regular sequence register 2"]
pub mod rsq2;
#[doc = "Inserted sequence register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isq](isq) module"]
pub type ISQ = crate::Reg<u32, _ISQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISQ;
#[doc = "`read()` method returns [isq::R](isq::R) reader structure"]
impl crate::Readable for ISQ {}
#[doc = "`write(|w| ..)` method takes [isq::W](isq::W) writer structure"]
impl crate::Writable for ISQ {}
#[doc = "Inserted sequence register"]
pub mod isq;
#[doc = "Inserted data register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idata0](idata0) module"]
pub type IDATA0 = crate::Reg<u32, _IDATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDATA0;
#[doc = "`read()` method returns [idata0::R](idata0::R) reader structure"]
impl crate::Readable for IDATA0 {}
#[doc = "Inserted data register 0"]
pub mod idata0;
#[doc = "Inserted data register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idata1](idata1) module"]
pub type IDATA1 = crate::Reg<u32, _IDATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDATA1;
#[doc = "`read()` method returns [idata1::R](idata1::R) reader structure"]
impl crate::Readable for IDATA1 {}
#[doc = "Inserted data register 1"]
pub mod idata1;
#[doc = "Inserted data register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idata2](idata2) module"]
pub type IDATA2 = crate::Reg<u32, _IDATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDATA2;
#[doc = "`read()` method returns [idata2::R](idata2::R) reader structure"]
impl crate::Readable for IDATA2 {}
#[doc = "Inserted data register 2"]
pub mod idata2;
#[doc = "Inserted data register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idata3](idata3) module"]
pub type IDATA3 = crate::Reg<u32, _IDATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDATA3;
#[doc = "`read()` method returns [idata3::R](idata3::R) reader structure"]
impl crate::Readable for IDATA3 {}
#[doc = "Inserted data register 3"]
pub mod idata3;
#[doc = "regular data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdata](rdata) module"]
pub type RDATA = crate::Reg<u32, _RDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDATA;
#[doc = "`read()` method returns [rdata::R](rdata::R) reader structure"]
impl crate::Readable for RDATA {}
#[doc = "regular data register"]
pub mod rdata;
#[doc = "Oversample control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ovsampctl](ovsampctl) module"]
pub type OVSAMPCTL = crate::Reg<u32, _OVSAMPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OVSAMPCTL;
#[doc = "`read()` method returns [ovsampctl::R](ovsampctl::R) reader structure"]
impl crate::Readable for OVSAMPCTL {}
#[doc = "`write(|w| ..)` method takes [ovsampctl::W](ovsampctl::W) writer structure"]
impl crate::Writable for OVSAMPCTL {}
#[doc = "Oversample control register"]
pub mod ovsampctl;
