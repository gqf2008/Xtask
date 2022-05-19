/*!
  Real time clock

  A continuously running clock that counts seconds. It is part of the backup domain which means
  that the counter is not affected by system resets or standby mode. If Vbat is connected, it is
  not reset even if the rest of the device is powered off. This allows it to be used to wake the
  CPU when it is in low power mode.

  Since it is part of the backup domain, write access to it must be enabled before the RTC can be
  used.
*/

use crate::pac::{RCU, RTC};

use crate::backup_domain::BackupDomain;
use crate::time::Hertz;

use core::convert::Infallible;

const LXTAL_HERTZ: u32 = 32_768;

/**
  Interface to the real time clock
*/
pub struct Rtc {
    regs: RTC,
}

impl Rtc {
    /**
      Initialises the RTC.
    */
    pub fn rtc(regs: RTC, bkp: &mut BackupDomain) -> Self {
        let mut result = Rtc { regs };

        Rtc::enable_rtc(bkp);

        // Set the prescaler to make it count up once every second.
        let prl = LXTAL_HERTZ - 1;
        assert!(prl < 1 << 20);
        result.perform_write(|s| {
            s.regs.psch.write(|w| unsafe { w.bits(prl >> 16) });
            s.regs.pscl.write(|w| unsafe { w.bits(prl as u16 as u32) });
        });

        result
    }

    /// Enables the RTC device with the LXTAL as the clock
    fn enable_rtc(_bkp: &mut BackupDomain) {
        let rcu = unsafe { &*RCU::ptr() };

        // Enable LXTAL
        rcu.bdctl
            .modify(|_, w| w.lxtalen().set_bit().lxtalbps().clear_bit());

        // Wait for stable LXTAL
        while !rcu.bdctl.read().lxtalstb().bit() {}

        rcu.bdctl.modify(|_, w| {
            unsafe {
                w
                    // Set the source of the RTC to LXTAL
                    .rtcsrc()
                    .bits(0b01)
                    // Enable the RTC
                    .rtcen()
                    .set_bit()
            }
        })
    }

    /// Selects the frequency of the RTC Timer
    pub fn select_frequency(&mut self, timeout: impl Into<Hertz>) {
        let frequency = timeout.into().0;

        assert!(frequency <= LXTAL_HERTZ);

        let prescaler = LXTAL_HERTZ / frequency - 1;
        self.perform_write(|s| {
            s.regs.psch.write(|w| unsafe { w.bits(prescaler >> 16) });
            s.regs
                .pscl
                .write(|w| unsafe { w.bits(prescaler as u16 as u32) });
        });
    }

    /// Set the current RTC counter value to the specified amount
    pub fn set_time(&mut self, counter_value: u32) {
        self.perform_write(|s| {
            s.regs
                .cnth
                .write(|w| unsafe { w.bits(counter_value >> 16) });
            s.regs
                .cntl
                .write(|w| unsafe { w.bits(counter_value as u16 as u32) });
        });
    }

    /**
      Sets the time at which an alarm will be triggered

      This also clears the alarm flag if it is set
    */
    pub fn set_alarm(&mut self, counter_value: u32) {
        // Set alarm time
        // See section 14.3.4 for explanation
        let alarm_value = counter_value - 1;

        self.perform_write(|s| {
            s.regs
                .alrmh
                .write(|w| unsafe { w.alrm().bits((alarm_value >> 16) as u16) });
            s.regs
                .alrml
                .write(|w| unsafe { w.alrm().bits(alarm_value as u16) });
        });

        self.clear_alarm_flag();
    }

    /// Enables the RTC_ALARM interrupt
    pub fn listen_alarm(&mut self) {
        // Enable alarm interrupt
        self.perform_write(|s| {
            s.regs.inten.modify(|_, w| w.alrmie().set_bit());
        })
    }

    /// Disables the RTC_ALARM interrupt
    pub fn unlisten_alarm(&mut self) {
        // Disable alarm interrupt
        self.perform_write(|s| {
            s.regs.inten.modify(|_, w| w.alrmie().clear_bit());
        })
    }

    /// Reads the current counter
    pub fn current_time(&self) -> u32 {
        // Wait for the APB1 interface to be ready
        while !self.regs.ctl.read().rsynf().bit() {}

        self.regs.cnth.read().bits() << 16 | self.regs.cntl.read().bits()
    }

    /// Enables the RTC second interrupt
    pub fn listen_seconds(&mut self) {
        self.perform_write(|s| s.regs.inten.modify(|_, w| w.scie().set_bit()))
    }

    /// Disables the RTC second interrupt
    pub fn unlisten_seconds(&mut self) {
        self.perform_write(|s| s.regs.inten.modify(|_, w| w.scie().clear_bit()))
    }

    /// Clears the RTC second interrupt flag
    pub fn clear_second_flag(&mut self) {
        self.perform_write(|s| s.regs.ctl.modify(|_, w| w.scif().clear_bit()))
    }

    /// Clears the RTC alarm interrupt flag
    pub fn clear_alarm_flag(&mut self) {
        self.perform_write(|s| s.regs.ctl.modify(|_, w| w.alrmif().clear_bit()))
    }

    /// Return `Ok(())` if the second flag is set, `Err(nb::WouldBlock)` otherwise.
    pub fn wait_second(&mut self) -> nb::Result<(), Infallible> {
        if self.regs.ctl.read().scif().bit() {
            self.regs.ctl.modify(|_, w| w.scif().clear_bit());
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    /**
      Return `Ok(())` if the alarm flag is set, `Err(nb::WouldBlock)` otherwise.

      ```rust
      use nb::block;

      rtc.set_alarm(rtc.current_time() + 5);
      // NOTE: Safe unwrap because Infallible can't be returned
      block!(rtc.wait_alarm()).unwrap();
      ```
    */
    pub fn wait_alarm(&mut self) -> nb::Result<(), Infallible> {
        if self.regs.ctl.read().alrmif().bit() {
            self.regs.ctl.modify(|_, w| w.alrmif().clear_bit());
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    /**
      The RTC registers can not be written to at any time as documented in section
      14.3.3 of the manual. Performing writes using this function ensures that
      the writes are done correctly.
    */
    fn perform_write(&mut self, func: impl Fn(&mut Self)) {
        // Wait for the last write operation to be done
        while !self.regs.ctl.read().lwoff().bit() {}
        // Put the clock into config mode
        self.regs.ctl.modify(|_, w| w.cmf().set_bit());

        // Perform the write operation
        func(self);

        // Take the device out of config mode
        self.regs.ctl.modify(|_, w| w.cmf().clear_bit());
        // Wait for the write to be done
        while !self.regs.ctl.read().lwoff().bit() {}
    }
}
