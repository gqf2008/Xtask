#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - port control register 0"]
    pub ctl0: CTL0,
    #[doc = "0x04 - port control register 1"]
    pub ctl1: CTL1,
    #[doc = "0x08 - Port input status register"]
    pub istat: ISTAT,
    #[doc = "0x0c - Port output control register"]
    pub octl: OCTL,
    #[doc = "0x10 - Port bit operate register"]
    pub bop: BOP,
    #[doc = "0x14 - Port bit clear register"]
    pub bc: BC,
    #[doc = "0x18 - GPIO port configuration lock register"]
    pub lock: LOCK,
}
#[doc = "port control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](ctl0) module"]
pub type CTL0 = crate::Reg<u32, _CTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL0;
#[doc = "`read()` method returns [ctl0::R](ctl0::R) reader structure"]
impl crate::Readable for CTL0 {}
#[doc = "`write(|w| ..)` method takes [ctl0::W](ctl0::W) writer structure"]
impl crate::Writable for CTL0 {}
#[doc = "port control register 0"]
pub mod ctl0;
#[doc = "port control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](ctl1) module"]
pub type CTL1 = crate::Reg<u32, _CTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL1;
#[doc = "`read()` method returns [ctl1::R](ctl1::R) reader structure"]
impl crate::Readable for CTL1 {}
#[doc = "`write(|w| ..)` method takes [ctl1::W](ctl1::W) writer structure"]
impl crate::Writable for CTL1 {}
#[doc = "port control register 1"]
pub mod ctl1;
#[doc = "Port input status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [istat](istat) module"]
pub type ISTAT = crate::Reg<u32, _ISTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISTAT;
#[doc = "`read()` method returns [istat::R](istat::R) reader structure"]
impl crate::Readable for ISTAT {}
#[doc = "Port input status register"]
pub mod istat;
#[doc = "Port output control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octl](octl) module"]
pub type OCTL = crate::Reg<u32, _OCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCTL;
#[doc = "`read()` method returns [octl::R](octl::R) reader structure"]
impl crate::Readable for OCTL {}
#[doc = "`write(|w| ..)` method takes [octl::W](octl::W) writer structure"]
impl crate::Writable for OCTL {}
#[doc = "Port output control register"]
pub mod octl;
#[doc = "Port bit operate register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bop](bop) module"]
pub type BOP = crate::Reg<u32, _BOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOP;
#[doc = "`write(|w| ..)` method takes [bop::W](bop::W) writer structure"]
impl crate::Writable for BOP {}
#[doc = "Port bit operate register"]
pub mod bop;
#[doc = "Port bit clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bc](bc) module"]
pub type BC = crate::Reg<u32, _BC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BC;
#[doc = "`write(|w| ..)` method takes [bc::W](bc::W) writer structure"]
impl crate::Writable for BC {}
#[doc = "Port bit clear register"]
pub mod bc;
#[doc = "GPIO port configuration lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lock](lock) module"]
pub type LOCK = crate::Reg<u32, _LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCK;
#[doc = "`read()` method returns [lock::R](lock::R) reader structure"]
impl crate::Readable for LOCK {}
#[doc = "`write(|w| ..)` method takes [lock::W](lock::W) writer structure"]
impl crate::Writable for LOCK {}
#[doc = "GPIO port configuration lock register"]
pub mod lock;
