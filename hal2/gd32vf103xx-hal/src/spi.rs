//! # Serial Peripheral Interface

use nb;

pub use crate::hal::spi::{Mode, Phase, Polarity, MODE_0, MODE_1, MODE_2, MODE_3};
use crate::pac::{spi0, SPI0, SPI1};
use crate::gpio::gpioa::{PA5, PA6, PA7};
use crate::gpio::gpiob::{PB13, PB14, PB15, PB3, PB4, PB5};
use crate::gpio::{Alternate, Floating, Input, PushPull};
use crate::rcu::{Rcu, Enable, Reset, BaseFrequency};
use crate::time::Hertz;
use crate::afio::{Afio, Remap};
use core::ops::Deref;

/// SPI error
#[derive(Debug)]
pub enum Error {
    /// Overrun occurred
    Overrun,
    /// Mode fault occurred
    ModeFault,
    /// CRC error
    Crc,
    #[doc(hidden)]
    _Extensible,
}

#[doc(hidden)]
pub trait SpiX: Deref<Target = spi0::RegisterBlock> {}
impl SpiX for SPI0 {}
impl SpiX for SPI1 {}

pub trait Pins<SPI> {
    const REMAP: bool;
}

impl Pins<SPI0>
    for (
        PA5<Alternate<PushPull>>,
        PA6<Input<Floating>>,
        PA7<Alternate<PushPull>>,
    )
{
    const REMAP: bool = false;
}

impl Pins<SPI0>
    for (
        PB3<Alternate<PushPull>>,
        PB4<Input<Floating>>,
        PB5<Alternate<PushPull>>,
    )
{
    const REMAP: bool = true;
}

impl Pins<SPI1>
    for (
        PB13<Alternate<PushPull>>,
        PB14<Input<Floating>>,
        PB15<Alternate<PushPull>>,
    )
{
    const REMAP: bool = false;
}

pub struct Spi<SPI, PINS> {
    spi: SPI,
    pins: PINS,
    base_freq: Hertz,
}

impl<PINS: Pins<SPI0>> Spi<SPI0, PINS> {
    pub fn spi0(
        spi: SPI0,
        pins: PINS,
        afio: &mut Afio,
        mode: Mode,
        freq: impl Into<Hertz>,
        rcu: &mut Rcu
    ) -> Self
    {
        SPI0::remap(afio, PINS::REMAP);
        Spi::new(spi, pins, mode, freq, rcu)
    }
}

impl<PINS: Pins<SPI1>> Spi<SPI1, PINS> {
    pub fn spi1(
        spi: SPI1,
        pins: PINS,
        mode: Mode,
        freq: impl Into<Hertz>,
        rcu: &mut Rcu
    ) -> Self
    {
        Spi::new(spi, pins, mode, freq, rcu)
    }
}

impl<SPI, PINS> Spi<SPI, PINS> where SPI: SpiX
{
    fn new(
        spi: SPI,
        pins: PINS,
        mode: Mode,
        freq: impl Into<Hertz>,
        rcu: &mut Rcu
    ) -> Self where SPI: Enable + Reset + BaseFrequency {
        SPI::enable(rcu);
        SPI::reset(rcu);

        // disable SS output
        spi.ctl1.write(|w| w.nssdrv().clear_bit());

        let base_freq = SPI::base_frequency(rcu);

        let br = match base_freq.0 / freq.into().0 {
            0 => panic!("Requested SPI frequency is too high"),
            1..=2 => 0b000,
            3..=5 => 0b001,
            6..=11 => 0b010,
            12..=23 => 0b011,
            24..=47 => 0b100,
            48..=95 => 0b101,
            96..=191 => 0b110,
            _ => 0b111,
        };

        // mstr: master configuration
        // lsbfirst: MSB first
        // ssm: enable software slave management (NSS pin free for other uses)
        // ssi: set nss high = master mode
        spi.ctl0.write(|w| unsafe { w
            .ckph().bit(mode.phase == Phase::CaptureOnSecondTransition)
            .ckpl().bit(mode.polarity == Polarity::IdleHigh)
            .mstmod().set_bit()     // Master mode
            .psc().bits(br)         // Master clock prescaler selection
            .lf().clear_bit()       // Transmit MSB first
            .swnss().set_bit()      // NSS pin is pulled high
            .swnssen().set_bit()    // NSS software mode. The NSS level depends on SWNSS bit.
            .ro().clear_bit()       // Full-duplex mode
            .ff16().clear_bit()     // 8-bit data frame format
            .bden().clear_bit()     // 2-line unidirectional mode
            .spien().set_bit()      // Enable SPI peripheral
        });

        Spi { spi, pins, base_freq }
    }

    /// Change the frequency of operation of the SPI bus.
    /// The maximum frequency for the SPI bus is the frequency
    /// of the APB1 bus which is half the system frequency configured
    /// with RCU.configure().sysclk(). Specifying a higher frequency causes panic.
    pub fn change_clock_freq(&mut self, freq: impl Into<Hertz>) {
        let br = match self.base_freq.0 / freq.into().0 {
            0 => panic!("Requested SPI frequency is too high"),
            1..=2 => 0b000,
            3..=5 => 0b001,
            6..=11 => 0b010,
            12..=23 => 0b011,
            24..=47 => 0b100,
            48..=95 => 0b101,
            96..=191 => 0b110,
            _ => 0b111,
        };

        // Disable SPI
        self.spi.ctl0.modify(|_, w| { w.spien().clear_bit()});

        // Restore config, change frequency and re-enable SPI
        self.spi.ctl0.modify( |_, w| unsafe { w
            .psc().bits(br)
            .spien().set_bit()
        });
    }

    pub fn free(self) -> (SPI, PINS) {
        (self.spi, self.pins)
    }
}

impl<SPI: SpiX, PINS> crate::hal::spi::FullDuplex<u8> for Spi<SPI, PINS> {
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Error> {
        let sr = self.spi.stat.read();

        Err(if sr.rxorerr().bit_is_set() {
            nb::Error::Other(Error::Overrun)
        } else if sr.conferr().bit_is_set() {
            nb::Error::Other(Error::ModeFault)
        } else if sr.crcerr().bit_is_set() {
            nb::Error::Other(Error::Crc)
        } else if sr.rbne().bit_is_set() {
            let byte = (self.spi.data.read().bits() & 0xff) as u8;
            return Ok(byte);
        } else {
            nb::Error::WouldBlock
        })
    }

    fn send(&mut self, byte: u8) -> nb::Result<(), Error> {
        let sr = self.spi.stat.read();

        Err(if sr.rxorerr().bit_is_set() {
            nb::Error::Other(Error::Overrun)
        } else if sr.conferr().bit_is_set() {
            nb::Error::Other(Error::ModeFault)
        } else if sr.crcerr().bit_is_set() {
            nb::Error::Other(Error::Crc)
        } else if sr.tbe().bit_is_set() {
            self.spi.data.write(|w| unsafe { w.bits(byte as u16) });
            return Ok(());
        } else {
            nb::Error::WouldBlock
        })
    }

}

impl<SPI: SpiX, PINS> crate::hal::blocking::spi::transfer::Default<u8> for Spi<SPI, PINS> {}

impl<SPI: SpiX, PINS> crate::hal::blocking::spi::write::Default<u8> for Spi<SPI, PINS> {}
