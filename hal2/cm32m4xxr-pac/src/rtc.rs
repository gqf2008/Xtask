#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC_TSH"]
    pub rtc_tsh: crate::Reg<rtc_tsh::RTC_TSH_SPEC>,
    #[doc = "0x04 - RTC_DATE"]
    pub rtc_date: crate::Reg<rtc_date::RTC_DATE_SPEC>,
    #[doc = "0x08 - RTC_CTRL"]
    pub rtc_ctrl: crate::Reg<rtc_ctrl::RTC_CTRL_SPEC>,
    #[doc = "0x0c - RTC_INITSTS"]
    pub rtc_initsts: crate::Reg<rtc_initsts::RTC_INITSTS_SPEC>,
    #[doc = "0x10 - RTC_PRE"]
    pub rtc_pre: crate::Reg<rtc_pre::RTC_PRE_SPEC>,
    #[doc = "0x14 - RTC_WKUPT"]
    pub rtc_wkupt: crate::Reg<rtc_wkupt::RTC_WKUPT_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - RTC_ALARMA"]
    pub rtc_alarma: crate::Reg<rtc_alarma::RTC_ALARMA_SPEC>,
    #[doc = "0x20 - RTC_ALARMB"]
    pub rtc_alarmb: crate::Reg<rtc_alarmb::RTC_ALARMB_SPEC>,
    #[doc = "0x24 - RTC_WRP"]
    pub rtc_wrp: crate::Reg<rtc_wrp::RTC_WRP_SPEC>,
    #[doc = "0x28 - RTC_SUBS"]
    pub rtc_subs: crate::Reg<rtc_subs::RTC_SUBS_SPEC>,
    #[doc = "0x2c - RTC_SCTRL"]
    pub rtc_sctrl: crate::Reg<rtc_sctrl::RTC_SCTRL_SPEC>,
    #[doc = "0x30 - RTC_TST"]
    pub rtc_tst: crate::Reg<rtc_tst::RTC_TST_SPEC>,
    #[doc = "0x34 - RTC_TSD"]
    pub rtc_tsd: crate::Reg<rtc_tsd::RTC_TSD_SPEC>,
    #[doc = "0x38 - RTC_TSSS"]
    pub rtc_tsss: crate::Reg<rtc_tsss::RTC_TSSS_SPEC>,
    #[doc = "0x3c - RTC_CALIB"]
    pub rtc_calib: crate::Reg<rtc_calib::RTC_CALIB_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x44 - RTC_ALRMASS"]
    pub rtc_alrmass: crate::Reg<rtc_alrmass::RTC_ALRMASS_SPEC>,
    #[doc = "0x48 - RTC_ALRMBSS"]
    pub rtc_alrmbss: crate::Reg<rtc_alrmbss::RTC_ALRMBSS_SPEC>,
    #[doc = "0x4c - RTC_OPT"]
    pub rtc_opt: crate::Reg<rtc_opt::RTC_OPT_SPEC>,
    _reserved18: [u8; 0x14],
    #[doc = "0x64 - RTC_TSCWKUPCTRL"]
    pub rtc_tscwkupctrl: crate::Reg<rtc_tscwkupctrl::RTC_TSCWKUPCTRL_SPEC>,
    #[doc = "0x68 - RTC_TSCWKUPCNT"]
    pub rtc_tscwkupcnt: crate::Reg<rtc_tscwkupcnt::RTC_TSCWKUPCNT_SPEC>,
}
#[doc = "RTC_TSH register accessor: an alias for `Reg<RTC_TSH_SPEC>`"]
pub type RTC_TSH = crate::Reg<rtc_tsh::RTC_TSH_SPEC>;
#[doc = "RTC_TSH"]
pub mod rtc_tsh;
#[doc = "RTC_DATE register accessor: an alias for `Reg<RTC_DATE_SPEC>`"]
pub type RTC_DATE = crate::Reg<rtc_date::RTC_DATE_SPEC>;
#[doc = "RTC_DATE"]
pub mod rtc_date;
#[doc = "RTC_CTRL register accessor: an alias for `Reg<RTC_CTRL_SPEC>`"]
pub type RTC_CTRL = crate::Reg<rtc_ctrl::RTC_CTRL_SPEC>;
#[doc = "RTC_CTRL"]
pub mod rtc_ctrl;
#[doc = "RTC_INITSTS register accessor: an alias for `Reg<RTC_INITSTS_SPEC>`"]
pub type RTC_INITSTS = crate::Reg<rtc_initsts::RTC_INITSTS_SPEC>;
#[doc = "RTC_INITSTS"]
pub mod rtc_initsts;
#[doc = "RTC_PRE register accessor: an alias for `Reg<RTC_PRE_SPEC>`"]
pub type RTC_PRE = crate::Reg<rtc_pre::RTC_PRE_SPEC>;
#[doc = "RTC_PRE"]
pub mod rtc_pre;
#[doc = "RTC_WKUPT register accessor: an alias for `Reg<RTC_WKUPT_SPEC>`"]
pub type RTC_WKUPT = crate::Reg<rtc_wkupt::RTC_WKUPT_SPEC>;
#[doc = "RTC_WKUPT"]
pub mod rtc_wkupt;
#[doc = "RTC_ALARMA register accessor: an alias for `Reg<RTC_ALARMA_SPEC>`"]
pub type RTC_ALARMA = crate::Reg<rtc_alarma::RTC_ALARMA_SPEC>;
#[doc = "RTC_ALARMA"]
pub mod rtc_alarma;
#[doc = "RTC_ALARMB register accessor: an alias for `Reg<RTC_ALARMB_SPEC>`"]
pub type RTC_ALARMB = crate::Reg<rtc_alarmb::RTC_ALARMB_SPEC>;
#[doc = "RTC_ALARMB"]
pub mod rtc_alarmb;
#[doc = "RTC_WRP register accessor: an alias for `Reg<RTC_WRP_SPEC>`"]
pub type RTC_WRP = crate::Reg<rtc_wrp::RTC_WRP_SPEC>;
#[doc = "RTC_WRP"]
pub mod rtc_wrp;
#[doc = "RTC_SUBS register accessor: an alias for `Reg<RTC_SUBS_SPEC>`"]
pub type RTC_SUBS = crate::Reg<rtc_subs::RTC_SUBS_SPEC>;
#[doc = "RTC_SUBS"]
pub mod rtc_subs;
#[doc = "RTC_SCTRL register accessor: an alias for `Reg<RTC_SCTRL_SPEC>`"]
pub type RTC_SCTRL = crate::Reg<rtc_sctrl::RTC_SCTRL_SPEC>;
#[doc = "RTC_SCTRL"]
pub mod rtc_sctrl;
#[doc = "RTC_TST register accessor: an alias for `Reg<RTC_TST_SPEC>`"]
pub type RTC_TST = crate::Reg<rtc_tst::RTC_TST_SPEC>;
#[doc = "RTC_TST"]
pub mod rtc_tst;
#[doc = "RTC_TSD register accessor: an alias for `Reg<RTC_TSD_SPEC>`"]
pub type RTC_TSD = crate::Reg<rtc_tsd::RTC_TSD_SPEC>;
#[doc = "RTC_TSD"]
pub mod rtc_tsd;
#[doc = "RTC_TSSS register accessor: an alias for `Reg<RTC_TSSS_SPEC>`"]
pub type RTC_TSSS = crate::Reg<rtc_tsss::RTC_TSSS_SPEC>;
#[doc = "RTC_TSSS"]
pub mod rtc_tsss;
#[doc = "RTC_CALIB register accessor: an alias for `Reg<RTC_CALIB_SPEC>`"]
pub type RTC_CALIB = crate::Reg<rtc_calib::RTC_CALIB_SPEC>;
#[doc = "RTC_CALIB"]
pub mod rtc_calib;
#[doc = "RTC_ALRMASS register accessor: an alias for `Reg<RTC_ALRMASS_SPEC>`"]
pub type RTC_ALRMASS = crate::Reg<rtc_alrmass::RTC_ALRMASS_SPEC>;
#[doc = "RTC_ALRMASS"]
pub mod rtc_alrmass;
#[doc = "RTC_ALRMBSS register accessor: an alias for `Reg<RTC_ALRMBSS_SPEC>`"]
pub type RTC_ALRMBSS = crate::Reg<rtc_alrmbss::RTC_ALRMBSS_SPEC>;
#[doc = "RTC_ALRMBSS"]
pub mod rtc_alrmbss;
#[doc = "RTC_OPT register accessor: an alias for `Reg<RTC_OPT_SPEC>`"]
pub type RTC_OPT = crate::Reg<rtc_opt::RTC_OPT_SPEC>;
#[doc = "RTC_OPT"]
pub mod rtc_opt;
#[doc = "RTC_TSCWKUPCTRL register accessor: an alias for `Reg<RTC_TSCWKUPCTRL_SPEC>`"]
pub type RTC_TSCWKUPCTRL = crate::Reg<rtc_tscwkupctrl::RTC_TSCWKUPCTRL_SPEC>;
#[doc = "RTC_TSCWKUPCTRL"]
pub mod rtc_tscwkupctrl;
#[doc = "RTC_TSCWKUPCNT register accessor: an alias for `Reg<RTC_TSCWKUPCNT_SPEC>`"]
pub type RTC_TSCWKUPCNT = crate::Reg<rtc_tscwkupcnt::RTC_TSCWKUPCNT_SPEC>;
#[doc = "RTC_TSCWKUPCNT"]
pub mod rtc_tscwkupcnt;
