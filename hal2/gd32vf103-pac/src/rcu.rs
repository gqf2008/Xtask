#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub ctl: CTL,
    #[doc = "0x04 - Clock configuration register 0 (RCU_CFG0)"]
    pub cfg0: CFG0,
    #[doc = "0x08 - Clock interrupt register (RCU_INT)"]
    pub int: INT,
    #[doc = "0x0c - APB2 reset register (RCU_APB2RST)"]
    pub apb2rst: APB2RST,
    #[doc = "0x10 - APB1 reset register (RCU_APB1RST)"]
    pub apb1rst: APB1RST,
    #[doc = "0x14 - AHB enable register"]
    pub ahben: AHBEN,
    #[doc = "0x18 - APB2 clock enable register (RCU_APB2EN)"]
    pub apb2en: APB2EN,
    #[doc = "0x1c - APB1 clock enable register (RCU_APB1EN)"]
    pub apb1en: APB1EN,
    #[doc = "0x20 - Backup domain control register (RCU_BDCTL)"]
    pub bdctl: BDCTL,
    #[doc = "0x24 - Reset source /clock register (RCU_RSTSCK)"]
    pub rstsck: RSTSCK,
    #[doc = "0x28 - AHB reset register"]
    pub ahbrst: AHBRST,
    #[doc = "0x2c - Clock Configuration register 1"]
    pub cfg1: CFG1,
    _reserved12: [u8; 4usize],
    #[doc = "0x34 - Deep sleep mode Voltage register"]
    pub dsv: DSV,
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
#[doc = "Clock configuration register 0 (RCU_CFG0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg0](cfg0) module"]
pub type CFG0 = crate::Reg<u32, _CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG0;
#[doc = "`read()` method returns [cfg0::R](cfg0::R) reader structure"]
impl crate::Readable for CFG0 {}
#[doc = "`write(|w| ..)` method takes [cfg0::W](cfg0::W) writer structure"]
impl crate::Writable for CFG0 {}
#[doc = "Clock configuration register 0 (RCU_CFG0)"]
pub mod cfg0;
#[doc = "Clock interrupt register (RCU_INT)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int](int) module"]
pub type INT = crate::Reg<u32, _INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT;
#[doc = "`read()` method returns [int::R](int::R) reader structure"]
impl crate::Readable for INT {}
#[doc = "`write(|w| ..)` method takes [int::W](int::W) writer structure"]
impl crate::Writable for INT {}
#[doc = "Clock interrupt register (RCU_INT)"]
pub mod int;
#[doc = "APB2 reset register (RCU_APB2RST)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2rst](apb2rst) module"]
pub type APB2RST = crate::Reg<u32, _APB2RST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2RST;
#[doc = "`read()` method returns [apb2rst::R](apb2rst::R) reader structure"]
impl crate::Readable for APB2RST {}
#[doc = "`write(|w| ..)` method takes [apb2rst::W](apb2rst::W) writer structure"]
impl crate::Writable for APB2RST {}
#[doc = "APB2 reset register (RCU_APB2RST)"]
pub mod apb2rst;
#[doc = "APB1 reset register (RCU_APB1RST)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1rst](apb1rst) module"]
pub type APB1RST = crate::Reg<u32, _APB1RST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1RST;
#[doc = "`read()` method returns [apb1rst::R](apb1rst::R) reader structure"]
impl crate::Readable for APB1RST {}
#[doc = "`write(|w| ..)` method takes [apb1rst::W](apb1rst::W) writer structure"]
impl crate::Writable for APB1RST {}
#[doc = "APB1 reset register (RCU_APB1RST)"]
pub mod apb1rst;
#[doc = "AHB enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahben](ahben) module"]
pub type AHBEN = crate::Reg<u32, _AHBEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBEN;
#[doc = "`read()` method returns [ahben::R](ahben::R) reader structure"]
impl crate::Readable for AHBEN {}
#[doc = "`write(|w| ..)` method takes [ahben::W](ahben::W) writer structure"]
impl crate::Writable for AHBEN {}
#[doc = "AHB enable register"]
pub mod ahben;
#[doc = "APB2 clock enable register (RCU_APB2EN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2en](apb2en) module"]
pub type APB2EN = crate::Reg<u32, _APB2EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2EN;
#[doc = "`read()` method returns [apb2en::R](apb2en::R) reader structure"]
impl crate::Readable for APB2EN {}
#[doc = "`write(|w| ..)` method takes [apb2en::W](apb2en::W) writer structure"]
impl crate::Writable for APB2EN {}
#[doc = "APB2 clock enable register (RCU_APB2EN)"]
pub mod apb2en;
#[doc = "APB1 clock enable register (RCU_APB1EN)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1en](apb1en) module"]
pub type APB1EN = crate::Reg<u32, _APB1EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1EN;
#[doc = "`read()` method returns [apb1en::R](apb1en::R) reader structure"]
impl crate::Readable for APB1EN {}
#[doc = "`write(|w| ..)` method takes [apb1en::W](apb1en::W) writer structure"]
impl crate::Writable for APB1EN {}
#[doc = "APB1 clock enable register (RCU_APB1EN)"]
pub mod apb1en;
#[doc = "Backup domain control register (RCU_BDCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdctl](bdctl) module"]
pub type BDCTL = crate::Reg<u32, _BDCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDCTL;
#[doc = "`read()` method returns [bdctl::R](bdctl::R) reader structure"]
impl crate::Readable for BDCTL {}
#[doc = "`write(|w| ..)` method takes [bdctl::W](bdctl::W) writer structure"]
impl crate::Writable for BDCTL {}
#[doc = "Backup domain control register (RCU_BDCTL)"]
pub mod bdctl;
#[doc = "Reset source /clock register (RCU_RSTSCK)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstsck](rstsck) module"]
pub type RSTSCK = crate::Reg<u32, _RSTSCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTSCK;
#[doc = "`read()` method returns [rstsck::R](rstsck::R) reader structure"]
impl crate::Readable for RSTSCK {}
#[doc = "`write(|w| ..)` method takes [rstsck::W](rstsck::W) writer structure"]
impl crate::Writable for RSTSCK {}
#[doc = "Reset source /clock register (RCU_RSTSCK)"]
pub mod rstsck;
#[doc = "AHB reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbrst](ahbrst) module"]
pub type AHBRST = crate::Reg<u32, _AHBRST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBRST;
#[doc = "`read()` method returns [ahbrst::R](ahbrst::R) reader structure"]
impl crate::Readable for AHBRST {}
#[doc = "`write(|w| ..)` method takes [ahbrst::W](ahbrst::W) writer structure"]
impl crate::Writable for AHBRST {}
#[doc = "AHB reset register"]
pub mod ahbrst;
#[doc = "Clock Configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg1](cfg1) module"]
pub type CFG1 = crate::Reg<u32, _CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG1;
#[doc = "`read()` method returns [cfg1::R](cfg1::R) reader structure"]
impl crate::Readable for CFG1 {}
#[doc = "`write(|w| ..)` method takes [cfg1::W](cfg1::W) writer structure"]
impl crate::Writable for CFG1 {}
#[doc = "Clock Configuration register 1"]
pub mod cfg1;
#[doc = "Deep sleep mode Voltage register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsv](dsv) module"]
pub type DSV = crate::Reg<u32, _DSV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSV;
#[doc = "`read()` method returns [dsv::R](dsv::R) reader structure"]
impl crate::Readable for DSV {}
#[doc = "`write(|w| ..)` method takes [dsv::W](dsv::W) writer structure"]
impl crate::Writable for DSV {}
#[doc = "Deep sleep mode Voltage register"]
pub mod dsv;
