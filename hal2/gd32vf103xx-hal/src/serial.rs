//! # Serial Communication (USART)
//! This module contains the functions to utilize the USART (Universal
//! synchronous asynchronous receiver transmitter)
//!
//!
//! ## Example usage:
//!  ```rust
//! // prelude: create handles to the peripherals and registers
//! let p = crate::pac::Peripherals::take().unwrap();
//! let mut afio = p.AFIO.constrain(&mut rcu);
//! let mut gpioa = p.GPIOA.split();
//!
//! // USART0 on Pins A9 and A10
//! let pin_tx = gpioa.pa9;
//! let pin_rx = gpioa.pa10;
//! // Create an interface struct for USART0 with 9600 Baud
//! let serial = Serial::new(
//!     p.USART0,
//!     (pin_tx, pin_rx),
//!     Config::default().baudrate(9_600.bps()),
//!     &mut afio,
//!     &mut rcu,
//! );
//!
//! // separate into tx and rx channels
//! let (mut tx, mut rx) = serial.split();
//!
//! // Write 'R' to the USART
//! block!(tx.write(b'R')).ok();
//! // Receive a byte from the USART and store it in "received"
//! let received = block!(rx.read()).unwrap();
//!  ```

use core::marker::PhantomData;
use core::ptr;

use nb;
use core::convert::Infallible;
use embedded_hal::serial::Write;

use crate::rcu::Rcu;
use crate::time::{U32Ext, Bps};
use crate::afio::Afio;

/// Interrupt event
pub enum Event {
    /// New data has been received
    Rxne,
    /// New data can be sent
    Txe,
}

/// Serial error
#[derive(Debug)]
pub enum Error {
    /// Framing error
    Framing,
    /// Noise error
    Noise,
    /// RX buffer overrun
    Overrun,
    /// Parity check error
    Parity,
    #[doc(hidden)]
    _Extensible,
}

mod closed_traits {
    use gd32vf103_pac::{USART0, USART1, USART2, usart0::RegisterBlock};
    use core::ops::Deref;
    use crate::rcu::{Enable, Reset, BaseFrequency};
    use crate::afio::Remap;
    use crate::gpio::{Alternate, Floating, Input, PushPull};
    use crate::gpio::gpioa::{PA10, PA2, PA3, PA9};
    use crate::gpio::gpiob::{PB6, PB7, PB10, PB11};
    use crate::gpio::gpioc::{PC10, PC11};
    use crate::gpio::gpiod::{PD5, PD6, PD8, PD9};

    pub trait UsartX : Deref<Target=RegisterBlock> + Enable + Reset + BaseFrequency + Remap {
        fn ptr() -> *const RegisterBlock;
    }

    pub trait Pins<USART: Remap> {
        const REMAP: USART::Variant;
        type Tx;
        type Rx;
        fn configure(self) -> (Self::Tx, Self::Rx);
    }

    macro_rules! pins {
        ($usart:ty, $remap_type:ty, $remap_value:expr, $tx:ident, $rx:ident) => {
            impl<TM, RM> crate::serial::Pins<$usart> for ($tx<TM>, $rx<RM>)
            where
                TM: crate::gpio::Active,
                RM: crate::gpio::Active
            {
                const REMAP: $remap_type = $remap_value;
                type Tx = $tx<Alternate<PushPull>>;
                type Rx = $rx<Input<Floating>>;

                #[inline(always)]
                fn configure(self) -> (Self::Tx, Self::Rx) {
                    let tx = self.0.into_alternate_push_pull();
                    let rx = self.1.into_floating_input();
                    (tx, rx)
                }
            }
        }
    }

    impl UsartX for USART0 {
        #[inline(always)]
        fn ptr() -> *const RegisterBlock {
            USART0::ptr()
        }
    }

    pins!(USART0, bool, false, PA9, PA10);
    pins!(USART0, bool, true, PB6, PB7);

    impl UsartX for USART1 {
        #[inline(always)]
        fn ptr() -> *const RegisterBlock {
            USART1::ptr()
        }
    }

    pins!(USART1, bool, false, PA2, PA3);
    pins!(USART1, bool, true, PD5, PD6);

    impl UsartX for USART2 {
        #[inline(always)]
        fn ptr() -> *const RegisterBlock {
            USART2::ptr()
        }
    }

    pins!(USART2, u8, 0, PB10, PB11);
    pins!(USART2, u8, 1, PC10, PC11);
    pins!(USART2, u8, 0b11, PD8, PD9);
}
use closed_traits::*;


pub enum Parity {
    ParityNone,
    ParityEven,
    ParityOdd,
}

pub enum StopBits {
    #[doc = "1 stop bit"]
    STOP1,
    #[doc = "0.5 stop bits"]
    STOP0P5,
    #[doc = "2 stop bits"]
    STOP2,
    #[doc = "1.5 stop bits"]
    STOP1P5,
}

pub struct Config {
    pub baudrate: Bps,
    pub parity: Parity,
    pub stopbits: StopBits,
}

impl Config {
    pub fn baudrate(mut self, baudrate: Bps) -> Self {
        self.baudrate = baudrate;
        self
    }

    pub fn parity_none(mut self) -> Self {
        self.parity = Parity::ParityNone;
        self
    }

    pub fn parity_even(mut self) -> Self {
        self.parity = Parity::ParityEven;
        self
    }

    pub fn parity_odd(mut self) -> Self {
        self.parity = Parity::ParityOdd;
        self
    }

    pub fn stopbits(mut self, stopbits: StopBits) -> Self {
        self.stopbits = stopbits;
        self
    }
}

impl Default for Config {
    fn default() -> Config {
        let baudrate = 115_200_u32.bps();
        Config {
            baudrate,
            parity: Parity::ParityNone,
            stopbits: StopBits::STOP1,
        }
    }
}

/// Serial abstraction
pub struct Serial<USART, TX, RX> {
    usart: USART,
    tx: TX,
    rx: RX,
}

/// Serial receiver
pub struct Rx<USART> {
    _usart: PhantomData<USART>,
}

/// Serial transmitter
pub struct Tx<USART> {
    _usart: PhantomData<USART>,
}

impl<USART: UsartX, TX, RX> Serial<USART, TX, RX>
{
    /// Configures the serial interface and creates the interface
    /// struct.
    ///
    /// `pins` is a tuple specifying transmit and receive pins. Current mode of
    /// these pins does not matter, as they are reconfigured during USART
    /// initialization.
    ///
    /// `config` holds UART configuration such as the baud rate of the interface.
    ///
    /// The `Serial` struct takes ownership over the `USARTx` device
    /// registers and the specified `PINS`
    ///
    /// `afio` and `rcu` are register handles which are passed for
    /// configuration. (`afio` is used to map the USART to the
    /// corresponding pins. `rcu` is used to reset the USART.)
    pub fn new<PINS>(
        usart: USART,
        pins: PINS,
        config: Config,
        afio: &mut Afio,
        rcu: &mut Rcu
    ) -> Self
    where PINS: Pins<USART, Tx=TX, Rx=RX>
    {
        // enable and reset USART
        USART::enable(rcu);
        USART::reset(rcu);

        // Pin configuration must happen after configuring USART clock in order
        // to avoid junk being transmitted during initialization
        let (tx, rx) = pins.configure();

        // Remap pins
        USART::remap(afio, PINS::REMAP);

        // enable DMA transfers
        usart.ctl2.write(|w| w.dent().set_bit().denr().set_bit());

        // Configure baud rate
        let brr = USART::base_frequency(rcu).0 / config.baudrate.0;
        assert!(brr >= 16, "impossible baud rate");
        usart.baud.write(|w| unsafe { w.bits(brr) });

        // Configure parity and word length
        // Unlike most uart devices, the "word length" of this usart device refers to
        // the size of the data plus the parity bit. I.e. "word length"=8, parity=even
        // results in 7 bits of data. Therefore, in order to get 8 bits and one parity
        // bit, we need to set the "word" length to 9 when using parity bits.
        let (word_length, parity_control_enable, parity) = match config.parity {
            Parity::ParityNone => (false, false, false),
            Parity::ParityEven => (true, true, false),
            Parity::ParityOdd => (true, true, true),
        };
        usart.ctl0.modify(|_r, w| {
            w.wl().bit(word_length);
            w.pm().bit(parity);
            w.pcen().bit(parity_control_enable)
        });

        // Configure stop bits
        let stop_bits = match config.stopbits {
            StopBits::STOP1 => 0b00,
            StopBits::STOP0P5 => 0b01,
            StopBits::STOP2 => 0b10,
            StopBits::STOP1P5 => 0b11,
        };
        usart.ctl1.modify(|_r, w| unsafe {
            w.stb().bits(stop_bits)
        });

        // UE: enable USART
        // RE: enable receiver
        // TE: enable transceiver
        usart.ctl0.modify(|_r, w| {
            w.uen().set_bit();
            w.ren().set_bit();
            w.ten().set_bit()
        });

        Serial { usart, tx, rx }
    }

    /// Starts listening to the USART by enabling the _Received data
    /// ready to be read (RXNE)_ interrupt and _Transmit data
    /// register empty (TXE)_ interrupt
    pub fn listen(&mut self, event: Event) {
        match event {
            Event::Rxne => self.usart.ctl0.modify(|_, w| w.rbneie().set_bit()),
            Event::Txe => self.usart.ctl0.modify(|_, w| w.tbeie().set_bit()),
        }
    }

    /// Stops listening to the USART by disabling the _Received data
    /// ready to be read (RXNE)_ interrupt and _Transmit data
    /// register empty (TXE)_ interrupt
    pub fn unlisten(&mut self, event: Event) {
        match event {
            Event::Rxne => self.usart.ctl0.modify(|_, w| w.rbneie().clear_bit()),
            Event::Txe => self.usart.ctl0.modify(|_, w| w.tbeie().clear_bit()),
        }
    }

    /// Returns ownership of the borrowed register handles
    pub fn release(self) -> (USART, TX, RX) {
        (self.usart, self.tx, self.rx)
    }

    /// Separates the serial struct into separate channel objects for sending (Tx) and
    /// receiving (Rx)
    pub fn split(self) -> (Tx<USART>, Rx<USART>) {
        (
            Tx {
                _usart: PhantomData,
            },
            Rx {
                _usart: PhantomData,
            },
        )
    }
}

impl<USART: UsartX> Tx<USART> {
    pub fn listen(&mut self) {
        unsafe { (*USART::ptr()).ctl0.modify(|_, w| w.tbeie().set_bit()) };
    }

    pub fn unlisten(&mut self) {
        unsafe { (*USART::ptr()).ctl0.modify(|_, w| w.tbeie().clear_bit()) };
    }
}

impl<USART: UsartX> Rx<USART> {
    pub fn listen(&mut self) {
        unsafe { (*USART::ptr()).ctl0.modify(|_, w| w.rbneie().set_bit()) };
    }

    pub fn unlisten(&mut self) {
        unsafe { (*USART::ptr()).ctl0.modify(|_, w| w.rbneie().clear_bit()) };
    }
}

impl<USART: UsartX> crate::hal::serial::Read<u8> for Rx<USART> {
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*USART::ptr()).stat.read() };

        // Check for any errors
        let err = if sr.perr().bit_is_set() {
            Some(Error::Parity)
        } else if sr.ferr().bit_is_set() {
            Some(Error::Framing)
        } else if sr.nerr().bit_is_set() {
            Some(Error::Noise)
        } else if sr.orerr().bit_is_set() {
            Some(Error::Overrun)
        } else {
            None
        };

        if let Some(err) = err {
            // Some error occurred. In order to clear that error flag, you have to
            // do a read from the sr register followed by a read from the dr
            // register
            // NOTE(read_volatile) see `write_volatile` below
            unsafe {
                ptr::read_volatile(&(*USART::ptr()).stat as *const _ as *const _);
                ptr::read_volatile(&(*USART::ptr()).data as *const _ as *const _);
            }
            Err(nb::Error::Other(err))
        } else {
            // Check if a byte is available
            if sr.rbne().bit_is_set() {
                // Read the received byte
                // NOTE(read_volatile) see `write_volatile` below
                Ok(unsafe {
                    ptr::read_volatile(&(*USART::ptr()).data as *const _ as *const _)
                })
            } else {
                Err(nb::Error::WouldBlock)
            }
        }
    }
}

impl<USART: UsartX> crate::hal::serial::Write<u8> for Tx<USART> {
    type Error = Infallible;

    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*USART::ptr()).stat.read() };

        if sr.tbe().bit_is_set() {
            // NOTE(unsafe) atomic write to stateless register
            // NOTE(write_volatile) 8-bit write that's not possible through the svd2rust API
            unsafe {
                ptr::write_volatile(&(*USART::ptr()).data as *const _ as *mut _, byte)
            }
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        // NOTE(unsafe) atomic read with no side effects
        let sr = unsafe { (*USART::ptr()).stat.read() };

        if sr.tc().bit_is_set() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<USART> core::fmt::Write for Tx<USART>
where
    Tx<USART>: embedded_hal::serial::Write<u8>,
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        s.as_bytes()
            .iter()
            .try_for_each(|c| nb::block!(self.write(*c)))
            .map_err(|_| core::fmt::Error)
    }
}
