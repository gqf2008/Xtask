#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C_CTRL1"]
    pub i2c_ctrl1: crate::Reg<i2c_ctrl1::I2C_CTRL1_SPEC>,
    #[doc = "0x04 - I2C_CTRL2"]
    pub i2c_ctrl2: crate::Reg<i2c_ctrl2::I2C_CTRL2_SPEC>,
    #[doc = "0x08 - I2C_OADDR1"]
    pub i2c_oaddr1: crate::Reg<i2c_oaddr1::I2C_OADDR1_SPEC>,
    #[doc = "0x0c - I2C_OADDR2"]
    pub i2c_oaddr2: crate::Reg<i2c_oaddr2::I2C_OADDR2_SPEC>,
    #[doc = "0x10 - I2C_DAT"]
    pub i2c_dat: crate::Reg<i2c_dat::I2C_DAT_SPEC>,
    #[doc = "0x14 - I2C_STS1"]
    pub i2c_sts1: crate::Reg<i2c_sts1::I2C_STS1_SPEC>,
    #[doc = "0x18 - I2C_STS2"]
    pub i2c_sts2: crate::Reg<i2c_sts2::I2C_STS2_SPEC>,
    #[doc = "0x1c - I2C_CLKCTRL"]
    pub i2c_clkctrl: crate::Reg<i2c_clkctrl::I2C_CLKCTRL_SPEC>,
    #[doc = "0x20 - I2C_TMRISE"]
    pub i2c_tmrise: crate::Reg<i2c_tmrise::I2C_TMRISE_SPEC>,
}
#[doc = "I2C_CTRL1 register accessor: an alias for `Reg<I2C_CTRL1_SPEC>`"]
pub type I2C_CTRL1 = crate::Reg<i2c_ctrl1::I2C_CTRL1_SPEC>;
#[doc = "I2C_CTRL1"]
pub mod i2c_ctrl1;
#[doc = "I2C_CTRL2 register accessor: an alias for `Reg<I2C_CTRL2_SPEC>`"]
pub type I2C_CTRL2 = crate::Reg<i2c_ctrl2::I2C_CTRL2_SPEC>;
#[doc = "I2C_CTRL2"]
pub mod i2c_ctrl2;
#[doc = "I2C_OADDR1 register accessor: an alias for `Reg<I2C_OADDR1_SPEC>`"]
pub type I2C_OADDR1 = crate::Reg<i2c_oaddr1::I2C_OADDR1_SPEC>;
#[doc = "I2C_OADDR1"]
pub mod i2c_oaddr1;
#[doc = "I2C_OADDR2 register accessor: an alias for `Reg<I2C_OADDR2_SPEC>`"]
pub type I2C_OADDR2 = crate::Reg<i2c_oaddr2::I2C_OADDR2_SPEC>;
#[doc = "I2C_OADDR2"]
pub mod i2c_oaddr2;
#[doc = "I2C_DAT register accessor: an alias for `Reg<I2C_DAT_SPEC>`"]
pub type I2C_DAT = crate::Reg<i2c_dat::I2C_DAT_SPEC>;
#[doc = "I2C_DAT"]
pub mod i2c_dat;
#[doc = "I2C_STS1 register accessor: an alias for `Reg<I2C_STS1_SPEC>`"]
pub type I2C_STS1 = crate::Reg<i2c_sts1::I2C_STS1_SPEC>;
#[doc = "I2C_STS1"]
pub mod i2c_sts1;
#[doc = "I2C_STS2 register accessor: an alias for `Reg<I2C_STS2_SPEC>`"]
pub type I2C_STS2 = crate::Reg<i2c_sts2::I2C_STS2_SPEC>;
#[doc = "I2C_STS2"]
pub mod i2c_sts2;
#[doc = "I2C_CLKCTRL register accessor: an alias for `Reg<I2C_CLKCTRL_SPEC>`"]
pub type I2C_CLKCTRL = crate::Reg<i2c_clkctrl::I2C_CLKCTRL_SPEC>;
#[doc = "I2C_CLKCTRL"]
pub mod i2c_clkctrl;
#[doc = "I2C_TMRISE register accessor: an alias for `Reg<I2C_TMRISE_SPEC>`"]
pub type I2C_TMRISE = crate::Reg<i2c_tmrise::I2C_TMRISE_SPEC>;
#[doc = "I2C_TMRISE"]
pub mod i2c_tmrise;
