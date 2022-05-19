//! Inter-Integrated Circuit (I2C) bus

use crate::gpio::gpiob::{PB10, PB11, PB6, PB7, PB8, PB9};
use crate::gpio::{Alternate, OpenDrain};
use crate::hal::blocking::i2c::{Read, Write, WriteRead};
use crate::pac::{I2C0, I2C1};
use crate::rcu::{Rcu, Clocks, Enable, Reset, BaseFrequency};
use crate::time::Hertz;
use crate::afio::{Afio, Remap};
use riscv::register::mcycle;
use nb::Error::{Other, WouldBlock};
use nb::{Error as NbError, Result as NbResult};

/// I2C error
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// Bus error
    Bus,
    /// Arbitration loss
    Arbitration,
    /// No ack received
    Acknowledge,
    /// Overrun/underrun
    Overrun,
    // Pec, // SMBUS mode only
    // Timeout, // SMBUS mode only
    // Alert, // SMBUS mode only
    #[doc(hidden)]
    _Extensible,
}

#[derive(Eq, PartialEq)]
pub enum DutyCycle {
    Ratio2to1,
    Ratio16to9,
}

#[derive(PartialEq)]
pub enum Mode {
    Standard {
        frequency: Hertz,
    },
    Fast {
        frequency: Hertz,
        duty_cycle: DutyCycle,
    },
    FastPlus {
        frequency: Hertz,
        duty_cycle: DutyCycle,
    }
}

impl Mode {
    pub fn standard<F: Into<Hertz>>(frequency: F) -> Self {
        Mode::Standard {
            frequency: frequency.into(),
        }
    }

    pub fn fast<F: Into<Hertz>>(frequency: F, duty_cycle: DutyCycle) -> Self {
        Mode::Fast {
            frequency: frequency.into(),
            duty_cycle,
        }
    }

    pub fn fast_plus<F: Into<Hertz>>(frequency: F, duty_cycle: DutyCycle) -> Self {
        Mode::Fast {
            frequency: frequency.into(),
            duty_cycle,
        }
    }

    pub fn get_frequency(&self) -> Hertz {
        match *self {
            Mode::Standard { frequency } => frequency,
            Mode::Fast { frequency, .. } => frequency,
            Mode::FastPlus { frequency, .. } => frequency,
        }
    }
}

/// Helper trait to ensure that the correct I2C pins are used for the corresponding interface
pub trait Pins<I2C> {
    const REMAP: bool;
}

impl Pins<I2C0> for (PB6<Alternate<OpenDrain>>, PB7<Alternate<OpenDrain>>) {
    const REMAP: bool = false;
}

impl Pins<I2C0> for (PB8<Alternate<OpenDrain>>, PB9<Alternate<OpenDrain>>) {
    const REMAP: bool = true;
}

impl Pins<I2C1> for (PB10<Alternate<OpenDrain>>, PB11<Alternate<OpenDrain>>) {
    const REMAP: bool = false;
}

/// I2C peripheral operating in master mode
pub struct I2c<I2C, PINS> {
    i2c: I2C,
    pins: PINS,
    mode: Mode,
    pclk1: u32,
}

/// embedded-hal compatible blocking I2C implementation
pub struct BlockingI2c<I2C, PINS> {
    nb: I2c<I2C, PINS>,
    start_timeout: u32,
    start_retries: u8,
    addr_timeout: u32,
    data_timeout: u32,
}

impl<PINS> I2c<I2C0, PINS> {
    /// Creates a generic I2C0 object on pins PB6 and PB7 or PB8 and PB9 (if remapped)
    pub fn i2c0(
        i2c: I2C0,
        pins: PINS,
        afio: &mut Afio,
        mode: Mode,
        rcu: &mut Rcu,
    ) -> Self
    where
        PINS: Pins<I2C0>,
    {
        I2C0::remap(afio, PINS::REMAP);
        I2c::_i2c0(i2c, pins, mode, rcu)
    }
}

impl<PINS> BlockingI2c<I2C0, PINS> {
    /// Creates a blocking I2C0 object on pins PB6 and PB7 or PB8 and PB9 using the embedded-hal `BlockingI2c` trait.
    pub fn i2c0(
        i2c: I2C0,
        pins: PINS,
        afio: &mut Afio,
        mode: Mode,
        rcu: &mut Rcu,
        start_timeout_us: u32,
        start_retries: u8,
        addr_timeout_us: u32,
        data_timeout_us: u32,
    ) -> Self
    where
        PINS: Pins<I2C0>,
    {
        I2C0::remap(afio, PINS::REMAP);
        BlockingI2c::_i2c0(
            i2c,
            pins,
            mode,
            rcu,
            start_timeout_us,
            start_retries,
            addr_timeout_us,
            data_timeout_us,
        )
    }
}

impl<PINS> I2c<I2C1, PINS> {
    /// Creates a generic I2C1 object on pins PB10 and PB11 using the embedded-hal `BlockingI2c` trait.
    pub fn i2c1(
        i2c: I2C1,
        pins: PINS,
        mode: Mode,
        rcu: &mut Rcu,
    ) -> Self
    where
        PINS: Pins<I2C1>,
    {
        I2c::_i2c1(i2c, pins, mode, rcu)
    }
}

impl<PINS> BlockingI2c<I2C1, PINS> {
    /// Creates a blocking I2C1 object on pins PB10 and PB11
    pub fn i2c1(
        i2c: I2C1,
        pins: PINS,
        mode: Mode,
        rcu: &mut Rcu,
        start_timeout_us: u32,
        start_retries: u8,
        addr_timeout_us: u32,
        data_timeout_us: u32,
    ) -> Self
    where
        PINS: Pins<I2C1>,
    {
        BlockingI2c::_i2c1(
            i2c,
            pins,
            mode,
            rcu,
            start_timeout_us,
            start_retries,
            addr_timeout_us,
            data_timeout_us,
        )
    }
}

/// Generates a blocking I2C instance from a universal I2C object
fn blocking_i2c<I2C, PINS>(
    i2c: I2c<I2C, PINS>,
    clocks: Clocks,
    start_timeout_us: u32,
    start_retries: u8,
    addr_timeout_us: u32,
    data_timeout_us: u32,
) -> BlockingI2c<I2C, PINS> {
    let sysclk_mhz = clocks.sysclk().0 / 1_000_000;
    BlockingI2c {
        nb: i2c,
        start_timeout: start_timeout_us * sysclk_mhz,
        start_retries,
        addr_timeout: addr_timeout_us * sysclk_mhz,
        data_timeout: data_timeout_us * sysclk_mhz,
    }
}

macro_rules! wait_for_flag {
    ($i2c:expr, $flag:ident) => {{
        let stat0 = $i2c.stat0.read();

        if stat0.berr().bit_is_set() {
            $i2c.stat0.modify(|_, w| w.berr().clear_bit());
            Err(Other(Error::Bus))
        } else if stat0.lostarb().bit_is_set() {
            $i2c.stat0.modify(|_, w| w.lostarb().clear_bit());
            Err(Other(Error::Arbitration))
        } else if stat0.aerr().bit_is_set() {
            $i2c.stat0.modify(|_, w| w.aerr().clear_bit());
            Err(Other(Error::Acknowledge))
        } else if stat0.ouerr().bit_is_set() {
            $i2c.stat0.modify(|_, w| w.ouerr().clear_bit());
            Err(Other(Error::Overrun))
        } else if stat0.$flag().bit_is_set() {
            Ok(())
        } else {
            Err(WouldBlock)
        }
    }};
}

macro_rules! busy_wait {
    ($nb_expr:expr, $exit_cond:expr) => {{
        loop {
            let res = $nb_expr;
            if res != Err(WouldBlock) {
                break res;
            }
            if $exit_cond {
                break res;
            }
        }
    }};
}

macro_rules! busy_wait_cycles {
    ($nb_expr:expr, $cycles:expr) => {{
        let started = mcycle::read();
        let cycles = $cycles as usize;
        busy_wait!(
            $nb_expr,
            mcycle::read().wrapping_sub(started) >= cycles
        )
    }};
}

// Generate the same code for both I2Cs
macro_rules! hal {
    ($($I2CX:ident: ($i2cX:ident),)+) => {
        $(
            impl<PINS> I2c<$I2CX, PINS> {
                /// Configures the I2C peripheral to work in master mode
                fn $i2cX(
                    i2c: $I2CX,
                    pins: PINS,
                    mode: Mode,
                    rcu: &mut Rcu,
                ) -> Self {
                    $I2CX::enable(rcu);
                    $I2CX::reset(rcu);

                    let pclk1 = $I2CX::base_frequency(rcu).0;

                    assert!(mode.get_frequency().0 <= 1_000_000);

                    let mut i2c = I2c { i2c, pins, mode, pclk1 };
                    i2c.init();
                    i2c
                }

                /// Initializes I2C.
                fn init(&mut self) {
                    let freq = self.mode.get_frequency();
                    let pclk1_mhz = (self.pclk1 / 1000000) as u16;

                    self.i2c.ctl1.write(|w| unsafe {
                        w.i2cclk().bits(pclk1_mhz as u8)
                    });
                    self.i2c.ctl0.write(|w| w.i2cen().clear_bit());

                    match self.mode {
                        Mode::Standard { .. } => {
                            self.i2c.rt.write(|w| unsafe {
                                w.risetime().bits((pclk1_mhz + 1) as u8)
                            });
                            self.i2c.ckcfg.write(|w| unsafe {
                                w.clkc().bits(((self.pclk1 / (freq.0 * 2)) as u16).max(4))
                            });
                        },
                        Mode::Fast { ref duty_cycle, .. } => {
                            self.configure_fast_mode(pclk1_mhz, freq, duty_cycle)
                        }
                        Mode::FastPlus { ref duty_cycle, .. } => {
                            self.configure_fast_mode(pclk1_mhz, freq, duty_cycle);

                            self.i2c.fmpcfg.write(|w| w.fmpen().set_bit())
                        }
                    };

                    self.i2c.ctl0.modify(|_, w| w.i2cen().set_bit());
                }

                fn configure_fast_mode(&self, pclk1_mhz: u16, freq: Hertz, duty_cycle: &DutyCycle) {
                    self.i2c.rt.write(|w| unsafe {
                        w.risetime().bits((pclk1_mhz * 300 / 1000 + 1) as u8)
                    });

                    self.i2c.ckcfg.write(|w| {
                        let (freq, duty) = match duty_cycle {
                            DutyCycle::Ratio2to1 => (((self.pclk1 / (freq.0 * 3)) as u16).max(1), false),
                            DutyCycle::Ratio16to9 => (((self.pclk1 / (freq.0 * 25)) as u16).max(1), true)
                        };

                        unsafe {
                            w.clkc().bits(freq).dtcy().bit(duty).fast().set_bit()
                        }
                    });
                }

                /// Perform an I2C software reset
                fn reset(&mut self) {
                    self.i2c.ctl0.write(|w| w.i2cen().set_bit().sreset().set_bit());
                    self.i2c.ctl0.reset();
                    self.init();
                }

                /// Generate START condition
                fn send_start(&mut self) {
                    self.i2c.ctl0.modify(|_, w| w.start().set_bit());
                }

                /// Check if START condition is generated. If the condition is not generated, this
                /// method returns `WouldBlock` so the program can act accordingly
                /// (busy wait, async, ...)
                fn wait_after_sent_start(&mut self) -> NbResult<(), Error> {
                    wait_for_flag!(self.i2c, sbsend)
                }

                /// Check if STOP condition is generated. If the condition is not generated, this
                /// method returns `WouldBlock` so the program can act accordingly
                /// (busy wait, async, ...)
                fn wait_for_stop(&mut self) -> NbResult<(), Error> {
                    if self.i2c.ctl0.read().stop().bit_is_set() {
                        Ok(())
                    } else {
                        Err(WouldBlock)
                    }
                }

                /// Sends the (7-Bit) address on the I2C bus. The 8th bit on the bus is set
                /// depending on wether it is a read or write transfer.
                fn send_addr(&self, addr: u8, read: bool) {
                    self.i2c.data.write(|w| unsafe { w.trb().bits(addr << 1 | (if read {1} else {0})) });
                }

                /// Generate STOP condition
                fn send_stop(&self) {
                    self.i2c.ctl0.modify(|_, w| w.stop().set_bit());
                }

                /// Releases the I2C peripheral and associated pins
                pub fn free(self) -> ($I2CX, PINS) {
                    (self.i2c, self.pins)
                }
            }

            impl<PINS> BlockingI2c<$I2CX, PINS> {
                fn $i2cX(
                    i2c: $I2CX,
                    pins: PINS,
                    mode: Mode,
                    rcu: &mut Rcu,
                    start_timeout_us: u32,
                    start_retries: u8,
                    addr_timeout_us: u32,
                    data_timeout_us: u32
                ) -> Self {
                    blocking_i2c(I2c::$i2cX(i2c, pins, mode, rcu),
                        rcu.clocks, start_timeout_us, start_retries,
                        addr_timeout_us, data_timeout_us)
                }

                fn send_start_and_wait(&mut self) -> NbResult<(), Error> {
                    let mut retries_left = self.start_retries;
                    let mut last_ret: NbResult<(), Error> = Err(WouldBlock);
                    while retries_left > 0 {
                        self.nb.send_start();
                        last_ret = busy_wait_cycles!(self.nb.wait_after_sent_start(), self.start_timeout);
                        if last_ret.is_err() {
                            self.nb.reset();
                        } else {
                            break;
                        }
                        retries_left -= 1;
                    }
                    last_ret
                }

                fn send_addr_and_wait(&mut self, addr: u8, read: bool) -> NbResult<(), Error> {
                    self.nb.i2c.stat0.read();
                    self.nb.send_addr(addr, read);
                    let ret = busy_wait_cycles!(wait_for_flag!(self.nb.i2c, addsend), self.addr_timeout);

                    if ret == Err(Other(Error::Acknowledge)) {
                        self.nb.send_stop();
                    }
                    ret
                }

                fn write_bytes_and_wait(&mut self, bytes: &[u8]) -> NbResult<(), Error> {
                    self.nb.i2c.stat0.read();
                    self.nb.i2c.stat1.read();

                    self.nb.i2c.data.write(|w| unsafe { w.trb().bits(bytes[0]) });

                    for byte in &bytes[1..] {
                        busy_wait_cycles!(wait_for_flag!(self.nb.i2c, tbe), self.data_timeout)?;
                        self.nb.i2c.data.write(|w| unsafe { w.trb().bits(*byte) });
                    }
                    busy_wait_cycles!(wait_for_flag!(self.nb.i2c, btc), self.data_timeout)?;

                    Ok(())
                }

                fn write_without_stop(&mut self, addr: u8, bytes: &[u8]) -> NbResult<(), Error> {
                    self.send_start_and_wait()?;
                    self.send_addr_and_wait(addr, false)?;

                    let ret = self.write_bytes_and_wait(bytes);
                    if ret == Err(Other(Error::Acknowledge)) {
                        self.nb.send_stop();
                    }
                    ret
                }
            }

            impl<PINS> Write for BlockingI2c<$I2CX, PINS> {
                type Error = NbError<Error>;

                fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
                    self.write_without_stop(addr, bytes)?;
                    self.nb.send_stop();
                    busy_wait_cycles!(self.nb.wait_for_stop(), self.data_timeout)?;

                    Ok(())
                }
            }

            impl<PINS> Read for BlockingI2c<$I2CX, PINS> {
                type Error = NbError<Error>;

                fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
                    self.send_start_and_wait()?;
                    self.send_addr_and_wait(addr, true)?;

                    match buffer.len() {
                        1 => {
                            self.nb.i2c.ctl0.modify(|_, w| w.acken().clear_bit());
                            self.nb.i2c.stat0.read();
                            self.nb.i2c.stat1.read();
                            self.nb.send_stop();

                            busy_wait_cycles!(wait_for_flag!(self.nb.i2c, rbne), self.data_timeout)?;
                            buffer[0] = self.nb.i2c.data.read().trb().bits();

                            busy_wait_cycles!(self.nb.wait_for_stop(), self.data_timeout)?;
                            self.nb.i2c.ctl0.modify(|_, w| w.acken().set_bit());
                        }
                        2 => {
                            self.nb.i2c.ctl0.modify(|_, w| w.pecen().set_bit().acken().set_bit());
                            self.nb.i2c.stat0.read();
                            self.nb.i2c.stat1.read();
                            self.nb.i2c.ctl0.modify(|_, w| w.acken().clear_bit());

                            busy_wait_cycles!(wait_for_flag!(self.nb.i2c, btc), self.data_timeout)?;
                            self.nb.send_stop();
                            buffer[0] = self.nb.i2c.data.read().trb().bits();
                            buffer[1] = self.nb.i2c.data.read().trb().bits();

                            busy_wait_cycles!(self.nb.wait_for_stop(), self.data_timeout)?;
                            self.nb.i2c.ctl0.modify(|_, w| w.pecen().clear_bit().acken().clear_bit());
                            self.nb.i2c.ctl0.modify(|_, w| w.acken().set_bit());
                        }
                        buffer_len => {
                            self.nb.i2c.ctl0.modify(|_, w| w.acken().set_bit());
                            self.nb.i2c.stat0.read();
                            self.nb.i2c.stat1.read();

                            let (first_bytes, last_two_bytes) = buffer.split_at_mut(buffer_len - 3);
                            for byte in first_bytes {
                                busy_wait_cycles!(wait_for_flag!(self.nb.i2c, rbne), self.data_timeout)?;
                                *byte = self.nb.i2c.data.read().trb().bits();
                            }

                            busy_wait_cycles!(wait_for_flag!(self.nb.i2c, btc), self.data_timeout)?;
                            self.nb.i2c.ctl0.modify(|_, w| w.acken().clear_bit());
                            last_two_bytes[0] = self.nb.i2c.data.read().trb().bits();
                            self.nb.send_stop();
                            last_two_bytes[1] = self.nb.i2c.data.read().trb().bits();
                            busy_wait_cycles!(wait_for_flag!(self.nb.i2c, rbne), self.data_timeout)?;
                            last_two_bytes[2] = self.nb.i2c.data.read().trb().bits();

                            busy_wait_cycles!(self.nb.wait_for_stop(), self.data_timeout)?;
                            self.nb.i2c.ctl0.modify(|_, w| w.acken().set_bit());
                        }
                    }

                    Ok(())
                }
            }

            impl<PINS> WriteRead for BlockingI2c<$I2CX, PINS> {
                type Error = NbError<Error>;

                fn write_read(
                    &mut self,
                    addr: u8,
                    bytes: &[u8],
                    buffer: &mut [u8],
                ) -> Result<(), Self::Error> {
                    if !bytes.is_empty() {
                        self.write_without_stop(addr, bytes)?;
                    }

                    if !buffer.is_empty() {
                        self.read(addr, buffer)?;
                    } else if !bytes.is_empty() {
                        self.nb.send_stop();
                        busy_wait_cycles!(self.nb.wait_for_stop(), self.data_timeout)?;
                    }

                    Ok(())
                }
            }
        )+
    }
}

hal! {
    I2C0: (_i2c0),
    I2C1: (_i2c1),
}
