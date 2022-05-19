#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Backup data register 0"]
    pub data0: DATA0,
    _reserved1: [u8; 2usize],
    #[doc = "0x08 - Backup data register 1"]
    pub data1: DATA1,
    _reserved2: [u8; 2usize],
    #[doc = "0x0c - Backup data register 2"]
    pub data2: DATA2,
    _reserved3: [u8; 2usize],
    #[doc = "0x10 - Backup data register 3"]
    pub data3: DATA3,
    _reserved4: [u8; 2usize],
    #[doc = "0x14 - Backup data register 4"]
    pub data4: DATA4,
    _reserved5: [u8; 2usize],
    #[doc = "0x18 - Backup data register 5"]
    pub data5: DATA5,
    _reserved6: [u8; 2usize],
    #[doc = "0x1c - Backup data register 6"]
    pub data6: DATA6,
    _reserved7: [u8; 2usize],
    #[doc = "0x20 - Backup data register 7"]
    pub data7: DATA7,
    _reserved8: [u8; 2usize],
    #[doc = "0x24 - Backup data register 8"]
    pub data8: DATA8,
    _reserved9: [u8; 2usize],
    #[doc = "0x28 - Backup data register 9"]
    pub data9: DATA9,
    _reserved10: [u8; 2usize],
    #[doc = "0x2c - RTC signal output control register"]
    pub octl: OCTL,
    _reserved11: [u8; 2usize],
    #[doc = "0x30 - Tamper pin control register"]
    pub tpctl: TPCTL,
    _reserved12: [u8; 2usize],
    #[doc = "0x34 - Tamper control and status register"]
    pub tpcs: TPCS,
    _reserved13: [u8; 10usize],
    #[doc = "0x40 - Backup data register 10"]
    pub data10: DATA10,
    _reserved14: [u8; 2usize],
    #[doc = "0x44 - Backup data register 11"]
    pub data11: DATA11,
    _reserved15: [u8; 2usize],
    #[doc = "0x48 - Backup data register 12"]
    pub data12: DATA12,
    _reserved16: [u8; 2usize],
    #[doc = "0x4c - Backup data register 13"]
    pub data13: DATA13,
    _reserved17: [u8; 2usize],
    #[doc = "0x50 - Backup data register 14"]
    pub data14: DATA14,
    _reserved18: [u8; 2usize],
    #[doc = "0x54 - Backup data register 15"]
    pub data15: DATA15,
    _reserved19: [u8; 2usize],
    #[doc = "0x58 - Backup data register 16"]
    pub data16: DATA16,
    _reserved20: [u8; 2usize],
    #[doc = "0x5c - Backup data register 17"]
    pub data17: DATA17,
    _reserved21: [u8; 2usize],
    #[doc = "0x60 - Backup data register 18"]
    pub data18: DATA18,
    _reserved22: [u8; 2usize],
    #[doc = "0x64 - Backup data register 19"]
    pub data19: DATA19,
    _reserved23: [u8; 2usize],
    #[doc = "0x68 - Backup data register 20"]
    pub data20: DATA20,
    _reserved24: [u8; 2usize],
    #[doc = "0x6c - Backup data register 21"]
    pub data21: DATA21,
    _reserved25: [u8; 2usize],
    #[doc = "0x70 - Backup data register 22"]
    pub data22: DATA22,
    _reserved26: [u8; 2usize],
    #[doc = "0x74 - Backup data register 23"]
    pub data23: DATA23,
    _reserved27: [u8; 2usize],
    #[doc = "0x78 - Backup data register 24"]
    pub data24: DATA24,
    _reserved28: [u8; 2usize],
    #[doc = "0x7c - Backup data register 25"]
    pub data25: DATA25,
    _reserved29: [u8; 2usize],
    #[doc = "0x80 - Backup data register 26"]
    pub data26: DATA26,
    _reserved30: [u8; 2usize],
    #[doc = "0x84 - Backup data register 27"]
    pub data27: DATA27,
    _reserved31: [u8; 2usize],
    #[doc = "0x88 - Backup data register 28"]
    pub data28: DATA28,
    _reserved32: [u8; 2usize],
    #[doc = "0x8c - Backup data register 29"]
    pub data29: DATA29,
    _reserved33: [u8; 2usize],
    #[doc = "0x90 - Backup data register 30"]
    pub data30: DATA30,
    _reserved34: [u8; 2usize],
    #[doc = "0x94 - Backup data register 31"]
    pub data31: DATA31,
    _reserved35: [u8; 2usize],
    #[doc = "0x98 - Backup data register 32"]
    pub data32: DATA32,
    _reserved36: [u8; 2usize],
    #[doc = "0x9c - Backup data register 33"]
    pub data33: DATA33,
    _reserved37: [u8; 2usize],
    #[doc = "0xa0 - Backup data register 34"]
    pub data34: DATA34,
    _reserved38: [u8; 2usize],
    #[doc = "0xa4 - Backup data register 35"]
    pub data35: DATA35,
    _reserved39: [u8; 2usize],
    #[doc = "0xa8 - Backup data register 36"]
    pub data36: DATA36,
    _reserved40: [u8; 2usize],
    #[doc = "0xac - Backup data register 37"]
    pub data37: DATA37,
    _reserved41: [u8; 2usize],
    #[doc = "0xb0 - Backup data register 38"]
    pub data38: DATA38,
    _reserved42: [u8; 2usize],
    #[doc = "0xb4 - Backup data register 39"]
    pub data39: DATA39,
    _reserved43: [u8; 2usize],
    #[doc = "0xb8 - Backup data register 40"]
    pub data40: DATA40,
    _reserved44: [u8; 2usize],
    #[doc = "0xbc - Backup data register 41"]
    pub data41: DATA41,
}
#[doc = "Backup data register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data0](data0) module"]
pub type DATA0 = crate::Reg<u16, _DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA0;
#[doc = "`read()` method returns [data0::R](data0::R) reader structure"]
impl crate::Readable for DATA0 {}
#[doc = "`write(|w| ..)` method takes [data0::W](data0::W) writer structure"]
impl crate::Writable for DATA0 {}
#[doc = "Backup data register 0"]
pub mod data0;
#[doc = "Backup data register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data1](data1) module"]
pub type DATA1 = crate::Reg<u16, _DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA1;
#[doc = "`read()` method returns [data1::R](data1::R) reader structure"]
impl crate::Readable for DATA1 {}
#[doc = "`write(|w| ..)` method takes [data1::W](data1::W) writer structure"]
impl crate::Writable for DATA1 {}
#[doc = "Backup data register 1"]
pub mod data1;
#[doc = "Backup data register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data2](data2) module"]
pub type DATA2 = crate::Reg<u16, _DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA2;
#[doc = "`read()` method returns [data2::R](data2::R) reader structure"]
impl crate::Readable for DATA2 {}
#[doc = "`write(|w| ..)` method takes [data2::W](data2::W) writer structure"]
impl crate::Writable for DATA2 {}
#[doc = "Backup data register 2"]
pub mod data2;
#[doc = "Backup data register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data3](data3) module"]
pub type DATA3 = crate::Reg<u16, _DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA3;
#[doc = "`read()` method returns [data3::R](data3::R) reader structure"]
impl crate::Readable for DATA3 {}
#[doc = "`write(|w| ..)` method takes [data3::W](data3::W) writer structure"]
impl crate::Writable for DATA3 {}
#[doc = "Backup data register 3"]
pub mod data3;
#[doc = "Backup data register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data4](data4) module"]
pub type DATA4 = crate::Reg<u16, _DATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA4;
#[doc = "`read()` method returns [data4::R](data4::R) reader structure"]
impl crate::Readable for DATA4 {}
#[doc = "`write(|w| ..)` method takes [data4::W](data4::W) writer structure"]
impl crate::Writable for DATA4 {}
#[doc = "Backup data register 4"]
pub mod data4;
#[doc = "Backup data register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data5](data5) module"]
pub type DATA5 = crate::Reg<u16, _DATA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA5;
#[doc = "`read()` method returns [data5::R](data5::R) reader structure"]
impl crate::Readable for DATA5 {}
#[doc = "`write(|w| ..)` method takes [data5::W](data5::W) writer structure"]
impl crate::Writable for DATA5 {}
#[doc = "Backup data register 5"]
pub mod data5;
#[doc = "Backup data register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data6](data6) module"]
pub type DATA6 = crate::Reg<u16, _DATA6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA6;
#[doc = "`read()` method returns [data6::R](data6::R) reader structure"]
impl crate::Readable for DATA6 {}
#[doc = "`write(|w| ..)` method takes [data6::W](data6::W) writer structure"]
impl crate::Writable for DATA6 {}
#[doc = "Backup data register 6"]
pub mod data6;
#[doc = "Backup data register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data7](data7) module"]
pub type DATA7 = crate::Reg<u16, _DATA7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA7;
#[doc = "`read()` method returns [data7::R](data7::R) reader structure"]
impl crate::Readable for DATA7 {}
#[doc = "`write(|w| ..)` method takes [data7::W](data7::W) writer structure"]
impl crate::Writable for DATA7 {}
#[doc = "Backup data register 7"]
pub mod data7;
#[doc = "Backup data register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data8](data8) module"]
pub type DATA8 = crate::Reg<u16, _DATA8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA8;
#[doc = "`read()` method returns [data8::R](data8::R) reader structure"]
impl crate::Readable for DATA8 {}
#[doc = "`write(|w| ..)` method takes [data8::W](data8::W) writer structure"]
impl crate::Writable for DATA8 {}
#[doc = "Backup data register 8"]
pub mod data8;
#[doc = "Backup data register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data9](data9) module"]
pub type DATA9 = crate::Reg<u16, _DATA9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA9;
#[doc = "`read()` method returns [data9::R](data9::R) reader structure"]
impl crate::Readable for DATA9 {}
#[doc = "`write(|w| ..)` method takes [data9::W](data9::W) writer structure"]
impl crate::Writable for DATA9 {}
#[doc = "Backup data register 9"]
pub mod data9;
#[doc = "Backup data register 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data10](data10) module"]
pub type DATA10 = crate::Reg<u16, _DATA10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA10;
#[doc = "`read()` method returns [data10::R](data10::R) reader structure"]
impl crate::Readable for DATA10 {}
#[doc = "`write(|w| ..)` method takes [data10::W](data10::W) writer structure"]
impl crate::Writable for DATA10 {}
#[doc = "Backup data register 10"]
pub mod data10;
#[doc = "Backup data register 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data11](data11) module"]
pub type DATA11 = crate::Reg<u16, _DATA11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA11;
#[doc = "`read()` method returns [data11::R](data11::R) reader structure"]
impl crate::Readable for DATA11 {}
#[doc = "`write(|w| ..)` method takes [data11::W](data11::W) writer structure"]
impl crate::Writable for DATA11 {}
#[doc = "Backup data register 11"]
pub mod data11;
#[doc = "Backup data register 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data12](data12) module"]
pub type DATA12 = crate::Reg<u16, _DATA12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA12;
#[doc = "`read()` method returns [data12::R](data12::R) reader structure"]
impl crate::Readable for DATA12 {}
#[doc = "`write(|w| ..)` method takes [data12::W](data12::W) writer structure"]
impl crate::Writable for DATA12 {}
#[doc = "Backup data register 12"]
pub mod data12;
#[doc = "Backup data register 13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data13](data13) module"]
pub type DATA13 = crate::Reg<u16, _DATA13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA13;
#[doc = "`read()` method returns [data13::R](data13::R) reader structure"]
impl crate::Readable for DATA13 {}
#[doc = "`write(|w| ..)` method takes [data13::W](data13::W) writer structure"]
impl crate::Writable for DATA13 {}
#[doc = "Backup data register 13"]
pub mod data13;
#[doc = "Backup data register 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data14](data14) module"]
pub type DATA14 = crate::Reg<u16, _DATA14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA14;
#[doc = "`read()` method returns [data14::R](data14::R) reader structure"]
impl crate::Readable for DATA14 {}
#[doc = "`write(|w| ..)` method takes [data14::W](data14::W) writer structure"]
impl crate::Writable for DATA14 {}
#[doc = "Backup data register 14"]
pub mod data14;
#[doc = "Backup data register 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data15](data15) module"]
pub type DATA15 = crate::Reg<u16, _DATA15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA15;
#[doc = "`read()` method returns [data15::R](data15::R) reader structure"]
impl crate::Readable for DATA15 {}
#[doc = "`write(|w| ..)` method takes [data15::W](data15::W) writer structure"]
impl crate::Writable for DATA15 {}
#[doc = "Backup data register 15"]
pub mod data15;
#[doc = "Backup data register 16\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data16](data16) module"]
pub type DATA16 = crate::Reg<u16, _DATA16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA16;
#[doc = "`read()` method returns [data16::R](data16::R) reader structure"]
impl crate::Readable for DATA16 {}
#[doc = "`write(|w| ..)` method takes [data16::W](data16::W) writer structure"]
impl crate::Writable for DATA16 {}
#[doc = "Backup data register 16"]
pub mod data16;
#[doc = "Backup data register 17\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data17](data17) module"]
pub type DATA17 = crate::Reg<u16, _DATA17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA17;
#[doc = "`read()` method returns [data17::R](data17::R) reader structure"]
impl crate::Readable for DATA17 {}
#[doc = "`write(|w| ..)` method takes [data17::W](data17::W) writer structure"]
impl crate::Writable for DATA17 {}
#[doc = "Backup data register 17"]
pub mod data17;
#[doc = "Backup data register 18\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data18](data18) module"]
pub type DATA18 = crate::Reg<u16, _DATA18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA18;
#[doc = "`read()` method returns [data18::R](data18::R) reader structure"]
impl crate::Readable for DATA18 {}
#[doc = "`write(|w| ..)` method takes [data18::W](data18::W) writer structure"]
impl crate::Writable for DATA18 {}
#[doc = "Backup data register 18"]
pub mod data18;
#[doc = "Backup data register 19\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data19](data19) module"]
pub type DATA19 = crate::Reg<u16, _DATA19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA19;
#[doc = "`read()` method returns [data19::R](data19::R) reader structure"]
impl crate::Readable for DATA19 {}
#[doc = "`write(|w| ..)` method takes [data19::W](data19::W) writer structure"]
impl crate::Writable for DATA19 {}
#[doc = "Backup data register 19"]
pub mod data19;
#[doc = "Backup data register 20\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data20](data20) module"]
pub type DATA20 = crate::Reg<u16, _DATA20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA20;
#[doc = "`read()` method returns [data20::R](data20::R) reader structure"]
impl crate::Readable for DATA20 {}
#[doc = "`write(|w| ..)` method takes [data20::W](data20::W) writer structure"]
impl crate::Writable for DATA20 {}
#[doc = "Backup data register 20"]
pub mod data20;
#[doc = "Backup data register 21\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data21](data21) module"]
pub type DATA21 = crate::Reg<u16, _DATA21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA21;
#[doc = "`read()` method returns [data21::R](data21::R) reader structure"]
impl crate::Readable for DATA21 {}
#[doc = "`write(|w| ..)` method takes [data21::W](data21::W) writer structure"]
impl crate::Writable for DATA21 {}
#[doc = "Backup data register 21"]
pub mod data21;
#[doc = "Backup data register 22\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data22](data22) module"]
pub type DATA22 = crate::Reg<u16, _DATA22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA22;
#[doc = "`read()` method returns [data22::R](data22::R) reader structure"]
impl crate::Readable for DATA22 {}
#[doc = "`write(|w| ..)` method takes [data22::W](data22::W) writer structure"]
impl crate::Writable for DATA22 {}
#[doc = "Backup data register 22"]
pub mod data22;
#[doc = "Backup data register 23\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data23](data23) module"]
pub type DATA23 = crate::Reg<u16, _DATA23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA23;
#[doc = "`read()` method returns [data23::R](data23::R) reader structure"]
impl crate::Readable for DATA23 {}
#[doc = "`write(|w| ..)` method takes [data23::W](data23::W) writer structure"]
impl crate::Writable for DATA23 {}
#[doc = "Backup data register 23"]
pub mod data23;
#[doc = "Backup data register 24\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data24](data24) module"]
pub type DATA24 = crate::Reg<u16, _DATA24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA24;
#[doc = "`read()` method returns [data24::R](data24::R) reader structure"]
impl crate::Readable for DATA24 {}
#[doc = "`write(|w| ..)` method takes [data24::W](data24::W) writer structure"]
impl crate::Writable for DATA24 {}
#[doc = "Backup data register 24"]
pub mod data24;
#[doc = "Backup data register 25\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data25](data25) module"]
pub type DATA25 = crate::Reg<u16, _DATA25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA25;
#[doc = "`read()` method returns [data25::R](data25::R) reader structure"]
impl crate::Readable for DATA25 {}
#[doc = "`write(|w| ..)` method takes [data25::W](data25::W) writer structure"]
impl crate::Writable for DATA25 {}
#[doc = "Backup data register 25"]
pub mod data25;
#[doc = "Backup data register 26\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data26](data26) module"]
pub type DATA26 = crate::Reg<u16, _DATA26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA26;
#[doc = "`read()` method returns [data26::R](data26::R) reader structure"]
impl crate::Readable for DATA26 {}
#[doc = "`write(|w| ..)` method takes [data26::W](data26::W) writer structure"]
impl crate::Writable for DATA26 {}
#[doc = "Backup data register 26"]
pub mod data26;
#[doc = "Backup data register 27\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data27](data27) module"]
pub type DATA27 = crate::Reg<u16, _DATA27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA27;
#[doc = "`read()` method returns [data27::R](data27::R) reader structure"]
impl crate::Readable for DATA27 {}
#[doc = "`write(|w| ..)` method takes [data27::W](data27::W) writer structure"]
impl crate::Writable for DATA27 {}
#[doc = "Backup data register 27"]
pub mod data27;
#[doc = "Backup data register 28\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data28](data28) module"]
pub type DATA28 = crate::Reg<u16, _DATA28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA28;
#[doc = "`read()` method returns [data28::R](data28::R) reader structure"]
impl crate::Readable for DATA28 {}
#[doc = "`write(|w| ..)` method takes [data28::W](data28::W) writer structure"]
impl crate::Writable for DATA28 {}
#[doc = "Backup data register 28"]
pub mod data28;
#[doc = "Backup data register 29\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data29](data29) module"]
pub type DATA29 = crate::Reg<u16, _DATA29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA29;
#[doc = "`read()` method returns [data29::R](data29::R) reader structure"]
impl crate::Readable for DATA29 {}
#[doc = "`write(|w| ..)` method takes [data29::W](data29::W) writer structure"]
impl crate::Writable for DATA29 {}
#[doc = "Backup data register 29"]
pub mod data29;
#[doc = "Backup data register 30\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data30](data30) module"]
pub type DATA30 = crate::Reg<u16, _DATA30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA30;
#[doc = "`read()` method returns [data30::R](data30::R) reader structure"]
impl crate::Readable for DATA30 {}
#[doc = "`write(|w| ..)` method takes [data30::W](data30::W) writer structure"]
impl crate::Writable for DATA30 {}
#[doc = "Backup data register 30"]
pub mod data30;
#[doc = "Backup data register 31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data31](data31) module"]
pub type DATA31 = crate::Reg<u16, _DATA31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA31;
#[doc = "`read()` method returns [data31::R](data31::R) reader structure"]
impl crate::Readable for DATA31 {}
#[doc = "`write(|w| ..)` method takes [data31::W](data31::W) writer structure"]
impl crate::Writable for DATA31 {}
#[doc = "Backup data register 31"]
pub mod data31;
#[doc = "Backup data register 32\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data32](data32) module"]
pub type DATA32 = crate::Reg<u16, _DATA32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA32;
#[doc = "`read()` method returns [data32::R](data32::R) reader structure"]
impl crate::Readable for DATA32 {}
#[doc = "`write(|w| ..)` method takes [data32::W](data32::W) writer structure"]
impl crate::Writable for DATA32 {}
#[doc = "Backup data register 32"]
pub mod data32;
#[doc = "Backup data register 33\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data33](data33) module"]
pub type DATA33 = crate::Reg<u16, _DATA33>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA33;
#[doc = "`read()` method returns [data33::R](data33::R) reader structure"]
impl crate::Readable for DATA33 {}
#[doc = "`write(|w| ..)` method takes [data33::W](data33::W) writer structure"]
impl crate::Writable for DATA33 {}
#[doc = "Backup data register 33"]
pub mod data33;
#[doc = "Backup data register 34\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data34](data34) module"]
pub type DATA34 = crate::Reg<u16, _DATA34>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA34;
#[doc = "`read()` method returns [data34::R](data34::R) reader structure"]
impl crate::Readable for DATA34 {}
#[doc = "`write(|w| ..)` method takes [data34::W](data34::W) writer structure"]
impl crate::Writable for DATA34 {}
#[doc = "Backup data register 34"]
pub mod data34;
#[doc = "Backup data register 35\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data35](data35) module"]
pub type DATA35 = crate::Reg<u16, _DATA35>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA35;
#[doc = "`read()` method returns [data35::R](data35::R) reader structure"]
impl crate::Readable for DATA35 {}
#[doc = "`write(|w| ..)` method takes [data35::W](data35::W) writer structure"]
impl crate::Writable for DATA35 {}
#[doc = "Backup data register 35"]
pub mod data35;
#[doc = "Backup data register 36\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data36](data36) module"]
pub type DATA36 = crate::Reg<u16, _DATA36>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA36;
#[doc = "`read()` method returns [data36::R](data36::R) reader structure"]
impl crate::Readable for DATA36 {}
#[doc = "`write(|w| ..)` method takes [data36::W](data36::W) writer structure"]
impl crate::Writable for DATA36 {}
#[doc = "Backup data register 36"]
pub mod data36;
#[doc = "Backup data register 37\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data37](data37) module"]
pub type DATA37 = crate::Reg<u16, _DATA37>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA37;
#[doc = "`read()` method returns [data37::R](data37::R) reader structure"]
impl crate::Readable for DATA37 {}
#[doc = "`write(|w| ..)` method takes [data37::W](data37::W) writer structure"]
impl crate::Writable for DATA37 {}
#[doc = "Backup data register 37"]
pub mod data37;
#[doc = "Backup data register 38\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data38](data38) module"]
pub type DATA38 = crate::Reg<u16, _DATA38>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA38;
#[doc = "`read()` method returns [data38::R](data38::R) reader structure"]
impl crate::Readable for DATA38 {}
#[doc = "`write(|w| ..)` method takes [data38::W](data38::W) writer structure"]
impl crate::Writable for DATA38 {}
#[doc = "Backup data register 38"]
pub mod data38;
#[doc = "Backup data register 39\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data39](data39) module"]
pub type DATA39 = crate::Reg<u16, _DATA39>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA39;
#[doc = "`read()` method returns [data39::R](data39::R) reader structure"]
impl crate::Readable for DATA39 {}
#[doc = "`write(|w| ..)` method takes [data39::W](data39::W) writer structure"]
impl crate::Writable for DATA39 {}
#[doc = "Backup data register 39"]
pub mod data39;
#[doc = "Backup data register 40\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data40](data40) module"]
pub type DATA40 = crate::Reg<u16, _DATA40>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA40;
#[doc = "`read()` method returns [data40::R](data40::R) reader structure"]
impl crate::Readable for DATA40 {}
#[doc = "`write(|w| ..)` method takes [data40::W](data40::W) writer structure"]
impl crate::Writable for DATA40 {}
#[doc = "Backup data register 40"]
pub mod data40;
#[doc = "Backup data register 41\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data41](data41) module"]
pub type DATA41 = crate::Reg<u16, _DATA41>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA41;
#[doc = "`read()` method returns [data41::R](data41::R) reader structure"]
impl crate::Readable for DATA41 {}
#[doc = "`write(|w| ..)` method takes [data41::W](data41::W) writer structure"]
impl crate::Writable for DATA41 {}
#[doc = "Backup data register 41"]
pub mod data41;
#[doc = "RTC signal output control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octl](octl) module"]
pub type OCTL = crate::Reg<u16, _OCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCTL;
#[doc = "`read()` method returns [octl::R](octl::R) reader structure"]
impl crate::Readable for OCTL {}
#[doc = "`write(|w| ..)` method takes [octl::W](octl::W) writer structure"]
impl crate::Writable for OCTL {}
#[doc = "RTC signal output control register"]
pub mod octl;
#[doc = "Tamper pin control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpctl](tpctl) module"]
pub type TPCTL = crate::Reg<u16, _TPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPCTL;
#[doc = "`read()` method returns [tpctl::R](tpctl::R) reader structure"]
impl crate::Readable for TPCTL {}
#[doc = "`write(|w| ..)` method takes [tpctl::W](tpctl::W) writer structure"]
impl crate::Writable for TPCTL {}
#[doc = "Tamper pin control register"]
pub mod tpctl;
#[doc = "Tamper control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpcs](tpcs) module"]
pub type TPCS = crate::Reg<u16, _TPCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPCS;
#[doc = "`read()` method returns [tpcs::R](tpcs::R) reader structure"]
impl crate::Readable for TPCS {}
#[doc = "`write(|w| ..)` method takes [tpcs::W](tpcs::W) writer structure"]
impl crate::Writable for TPCS {}
#[doc = "Tamper control and status register"]
pub mod tpcs;
