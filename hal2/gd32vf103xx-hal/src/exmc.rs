//! External memory controller

use crate::pac::EXMC;
use crate::rcu::{Rcu, Enable};
use crate::gpio::gpiod::{PD0, PD1, PD4, PD5, PD6, PD7, PD8, PD9, PD10, PD11, PD12, PD13, PD14, PD15};
use crate::gpio::gpioe::{PE0, PE1, PE2, PE3, PE4, PE5, PE6, PE7, PE8, PE9, PE10, PE11, PE12, PE13, PE14, PE15};
use crate::gpio::{Alternate, PushPull};
use vcell::VolatileCell;

const REGION_SIZE: usize = 64*1024*1024;
const REGION_PTR: *const () = 0x6000_0000 as *const ();

/// Extension trait that sets up the `EXMC` peripheral
pub trait ExmcExt {
    /// Configures the `EXMC` peripheral
    fn configure(self, pins: ExmcPins, conf: ExmcConfiguration, timing_conf: ExmcTimingConfiguration, rcu: &mut Rcu) -> Exmc;
}

impl ExmcExt for EXMC {
    fn configure(self, pins: ExmcPins, conf: ExmcConfiguration, timing_conf: ExmcTimingConfiguration, rcu: &mut Rcu) -> Exmc {
        Exmc::new(self, pins, conf, timing_conf, rcu)
    }
}

pub struct Exmc {
    regs: EXMC,
    pins: ExmcPins,
}

impl Exmc {
    /// Configures the `EXMC` peripheral
    pub fn new(regs: EXMC, pins: ExmcPins, conf: ExmcConfiguration, timing_conf: ExmcTimingConfiguration, rcu: &mut Rcu) -> Self {
        EXMC::enable(rcu);

        regs.snctl0.write(|w| unsafe {
            w.nrmux().bit(conf.address_data_mux_enabled);
            w.nrtp().bits(conf.memory_type as u8);
            w.nrw().bits(conf.databus_width as u8);
            w.nren().bit(conf.memory_type == MemoryType::NORFlash);
            w.nrwtpol().bit(conf.nwait_polarity == NwaitPolarity::ActiveHigh);
            w.wren().bit(conf.memory_write_enabled);
            w.nrwten().bit(conf.nwait_signal_enabled);
            w.asyncwait().bit(conf.async_wait_enabled);
            w
        });

        regs.sntcfg0.write(|w| unsafe {
            w.aset().bits(timing_conf.address_setup_time);
            w.ahld().bits(timing_conf.address_hold_time);
            w.dset().bits(timing_conf.data_setup_time);
            w.buslat().bits(timing_conf.bus_latency)
        });

        // Enable memory bank
        regs.snctl0.modify(|_, w| w.nrbken().set_bit());

        Exmc {
            regs,
            pins,
        }
    }

    pub fn release(self) -> (EXMC, ExmcPins) {
        // Disable memory bank
        self.regs.snctl0.modify(|_, w| w.nrbken().clear_bit());

        (self.regs, self.pins)
    }

    pub fn as_u8_slice(&self) -> &[VolatileCell<u8>] {
        let ptr = REGION_PTR as *const VolatileCell<u8>;
        unsafe { core::slice::from_raw_parts(ptr, REGION_SIZE) }
    }

    pub fn as_u16_slice(&self) -> &[VolatileCell<u16>] {
        let ptr = REGION_PTR as *const VolatileCell<u16>;
        unsafe { core::slice::from_raw_parts(ptr, REGION_SIZE / 2) }
    }

    pub fn as_u32_slice(&self) -> &[VolatileCell<u32>] {
        let ptr = REGION_PTR as *const VolatileCell<u32>;
        unsafe { core::slice::from_raw_parts(ptr, REGION_SIZE / 4) }
    }
}

/// EXMC configuration
pub struct ExmcConfiguration {
    /// Asynchronous wait feature
    pub async_wait_enabled: bool,

    /// For Flash memory access in burst mode, this flag
    /// enables/disables wait-state insertion via the NWAIT signal
    pub nwait_signal_enabled: bool,

    /// Enables/disables write in the bank
    pub memory_write_enabled: bool,

    /// NWAIT signal polarity
    pub nwait_polarity: NwaitPolarity,

    /// NOR region memory data bus width
    pub databus_width: DataBusWidth,

    /// NOR region memory type
    pub memory_type: MemoryType,

    /// NOR region memory address/data multiplexing
    pub address_data_mux_enabled: bool,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum NwaitPolarity {
    ActiveLow = 0,
    ActiveHigh = 1,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum DataBusWidth {
    Width8Bits = 0,
    Width16Bits = 1,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum MemoryType {
    SRAM = 0,
    PSRAM = 1,
    NORFlash = 2,
}

/// EXMC timing configuration
#[derive(Copy, Clone)]
pub struct ExmcTimingConfiguration {
    address_setup_time: u8,
    address_hold_time: u8,
    data_setup_time: u8,
    bus_latency: u8,
}

impl Default for ExmcTimingConfiguration {
    fn default() -> Self {
        Self {
            address_setup_time: 0xf,
            address_hold_time: 0xf,
            data_setup_time: 0xff,
            bus_latency: 0xf,
        }
    }
}

impl ExmcTimingConfiguration {
    pub fn address_setup_time(&mut self, hclk_periods: u32) -> &mut Self {
        assert!(1 <= hclk_periods && hclk_periods < 16);
        self.address_setup_time = (hclk_periods - 1) as u8;
        self
    }

    pub fn address_hold_time(&mut self, hclk_periods: u32) -> &mut Self {
        assert!(2 <= hclk_periods && hclk_periods < 16);
        self.address_hold_time = (hclk_periods - 1) as u8;
        self
    }

    pub fn data_setup_time(&mut self, hclk_periods: u32) -> &mut Self {
        assert!(2 <= hclk_periods && hclk_periods < 256);
        self.data_setup_time = (hclk_periods - 1) as u8;
        self
    }

    pub fn bus_latency(&mut self, hclk_periods: u32) -> &mut Self {
        assert!(1 <= hclk_periods && hclk_periods < 16);
        self.bus_latency = (hclk_periods - 1) as u8;
        self
    }
}

#[allow(dead_code)]
pub struct ExmcPins {
    pub d0: PD14<Alternate<PushPull>>,
    pub d1: PD15<Alternate<PushPull>>,
    pub d2: PD0<Alternate<PushPull>>,
    pub d3: PD1<Alternate<PushPull>>,
    pub d4: PE7<Alternate<PushPull>>,
    pub d5: PE8<Alternate<PushPull>>,
    pub d6: PE9<Alternate<PushPull>>,
    pub d7: PE10<Alternate<PushPull>>,
    pub d8: Option<PE11<Alternate<PushPull>>>,
    pub d9: Option<PE12<Alternate<PushPull>>>,
    pub d10: Option<PE13<Alternate<PushPull>>>,
    pub d11: Option<PE14<Alternate<PushPull>>>,
    pub d12: Option<PE15<Alternate<PushPull>>>,
    pub d13: Option<PD8<Alternate<PushPull>>>,
    pub d14: Option<PD9<Alternate<PushPull>>>,
    pub d15: Option<PD10<Alternate<PushPull>>>,
    pub a16: Option<PD11<Alternate<PushPull>>>,
    pub a17: Option<PD12<Alternate<PushPull>>>,
    pub a18: Option<PD13<Alternate<PushPull>>>,
    pub a19: Option<PE3<Alternate<PushPull>>>,
    pub a20: Option<PE4<Alternate<PushPull>>>,
    pub a21: Option<PE5<Alternate<PushPull>>>,
    pub a22: Option<PE6<Alternate<PushPull>>>,
    pub a23: Option<PE2<Alternate<PushPull>>>,
    pub noe: Option<PD4<Alternate<PushPull>>>,
    pub nwe: Option<PD5<Alternate<PushPull>>>,
    pub nwait: Option<PD6<Alternate<PushPull>>>,
    pub ne0: Option<PD7<Alternate<PushPull>>>,
    pub nbl0: Option<PE0<Alternate<PushPull>>>,
    pub nbl1: Option<PE1<Alternate<PushPull>>>,
}
