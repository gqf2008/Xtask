//! Watchdog peripherals

use crate::{
    hal::watchdog::{Watchdog, WatchdogEnable},
    pac::{DBG, FWDGT},
    time::MilliSeconds,
};

/// Wraps the Free Watchdog Timer (FWDGT) peripheral
pub struct FreeWatchdog {
    fwdgt: FWDGT,
}

const IRC40K_KHZ: u32 = 40;

const MAX_PR: u8 = 8;
const MAX_RL: u16 = 0xFFF;

const CMD_ACCESS: u16 = 0x5555;
const CMD_RELOAD: u16 = 0xAAAA;
const CMD_START: u16 = 0xCCCC;

impl FreeWatchdog {
    /// Wrap and start the watchdog
    pub fn new(fwdgt: FWDGT) -> Self {
        FreeWatchdog { fwdgt }
    }

    /// Free watchdog stopped when core is halted
    pub fn stop_on_debug(&self, dbg: &DBG, stop: bool) {
        dbg.ctl.modify(|_, w| w.fwdgt_hold().bit(stop));
    }

    fn setup(&self, timeout_ms: u32) {
        let mut pr = 0;
        while pr < MAX_PR && Self::timeout_period(pr, MAX_RL) < timeout_ms {
            pr += 1;
        }

        let max_period = Self::timeout_period(pr, MAX_RL);
        let max_rl = u32::from(MAX_RL);
        let rl = (timeout_ms * max_rl / max_period).min(max_rl) as u16;

        self.access_registers(|fwdgt| {
            fwdgt.psc.modify(|_, w| unsafe { w.psc().bits(pr) });
            fwdgt.rld.modify(|_, w| unsafe { w.rld().bits(rl) });
        });
    }

    fn is_pr_updating(&self) -> bool {
        self.fwdgt.stat.read().pud().bit()
    }

    /// Returns the interval in ms
    pub fn interval(&self) -> MilliSeconds {
        while self.is_pr_updating() {}

        let pr = self.fwdgt.psc.read().psc().bits();
        let rl = self.fwdgt.rld.read().rld().bits();
        let ms = Self::timeout_period(pr, rl);

        MilliSeconds(ms)
    }

    /// pr: Prescaler divider bits, rl: reload value
    ///
    /// Returns ms
    fn timeout_period(pr: u8, rl: u16) -> u32 {
        let divider: u32 = match pr {
            0b000 => 4,
            0b001 => 8,
            0b010 => 16,
            0b011 => 32,
            0b100 => 64,
            0b101 => 128,
            0b110 => 256,
            0b111 => 256,
            _ => panic!("Invalid FWDGT prescaler divider"),
        };
        (u32::from(rl) + 1) * divider / IRC40K_KHZ
    }

    fn access_registers<A, F: FnMut(&FWDGT) -> A>(&self, mut f: F) -> A {
        // Unprotect write access to registers
        self.fwdgt
            .ctl
            .write(|w| unsafe { w.cmd().bits(CMD_ACCESS) });
        let a = f(&self.fwdgt);

        // Protect again
        self.fwdgt
            .ctl
            .write(|w| unsafe { w.cmd().bits(CMD_RELOAD) });
        a
    }
}

impl WatchdogEnable for FreeWatchdog {
    type Time = MilliSeconds;

    fn start<T: Into<Self::Time>>(&mut self, period: T) {
        self.setup(period.into().0);

        self.fwdgt.ctl.write(|w| unsafe { w.cmd().bits(CMD_START) });
    }
}

impl Watchdog for FreeWatchdog {
    fn feed(&mut self) {
        self.fwdgt
            .ctl
            .write(|w| unsafe { w.cmd().bits(CMD_RELOAD) });
    }
}
