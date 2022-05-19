//! # Direct Memory Access
#![allow(dead_code)]

use core::{
    marker::PhantomData,
    sync::atomic::{compiler_fence, Ordering},
};
use embedded_dma::{StaticReadBuffer, StaticWriteBuffer};

use crate::rcu::Rcu;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Overrun,
}

pub enum Event {
    HalfTransfer,
    TransferComplete,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Half {
    First,
    Second,
}

pub enum Direction {
    PeripherialToMemory,
    MemoryToPeripherial,
    MemoryToMemory,
}

pub struct CircBuffer<BUFFER, PAYLOAD>
where
    BUFFER: 'static,
{
    buffer: &'static mut [BUFFER; 2],
    payload: PAYLOAD,
    readable_half: Half,
}

impl<BUFFER, PAYLOAD> CircBuffer<BUFFER, PAYLOAD>
where
    &'static mut [BUFFER; 2]: StaticWriteBuffer,
    BUFFER: 'static,
{
    pub(crate) fn new(buf: &'static mut [BUFFER; 2], payload: PAYLOAD) -> Self {
        CircBuffer {
            buffer: buf,
            payload,
            readable_half: Half::Second,
        }
    }
}

pub trait DmaExt {
    type Channels;

    fn split(self, rcu: &mut Rcu) -> Self::Channels;
}

pub trait TransferPayload {
    fn start(&mut self);
    fn stop(&mut self);
}

pub struct Transfer<MODE, BUFFER, PAYLOAD>
where
    PAYLOAD: TransferPayload,
{
    _mode: PhantomData<MODE>,
    buffer: BUFFER,
    payload: PAYLOAD,
}

impl<BUFFER, PAYLOAD> Transfer<R, BUFFER, PAYLOAD>
where
    PAYLOAD: TransferPayload,
{
    pub(crate) fn r(buffer: BUFFER, payload: PAYLOAD) -> Self {
        Transfer {
            _mode: PhantomData,
            buffer,
            payload,
        }
    }
}

impl<BUFFER, PAYLOAD> Transfer<W, BUFFER, PAYLOAD>
where
    PAYLOAD: TransferPayload,
{
    pub(crate) fn w(buffer: BUFFER, payload: PAYLOAD) -> Self {
        Transfer {
            _mode: PhantomData,
            buffer,
            payload,
        }
    }
}

impl<MODE, BUFFER, PAYLOAD> Drop for Transfer<MODE, BUFFER, PAYLOAD>
where
    PAYLOAD: TransferPayload,
{
    fn drop(&mut self) {
        self.payload.stop();
        compiler_fence(Ordering::SeqCst);
    }
}

/// Read transfer
pub struct R;

/// Write transfer
pub struct W;

macro_rules! dma {
    ($($DMAX:ident: ($dmaX:ident, {
        $($CX:ident: (
            $CTLX:ident,
            $CNTX:ident,
            $PADDRX:ident,
            $MADDRX:ident,
            $ctlX:ident,
            $cntX:ident,
            $paddrX:ident,
            $maddrX:ident,
            $htfifX:ident,
            $ftfifX:ident,
            $htfifcX:ident,
            $ftfifcX:ident,
            $gifcX:ident
        ),)+
    }),)+) => {
        $(
            pub mod $dmaX {
                use core::{sync::atomic::{self, Ordering}, ptr, mem};

                use crate::pac::{$DMAX, $dmaX};
                use crate::rcu::{Rcu, Enable};

                use crate::dma::{CircBuffer, Direction, DmaExt, Error, Event, Half, Transfer, W, RxDma, TxDma, TransferPayload};

                #[allow(clippy::manual_non_exhaustive)]
                pub struct Channels((), $(pub $CX),+);

                $(
                    /// A singleton that represents a single DMAx channel (channel X in this case)
                    ///
                    /// This singleton has exclusive access to the registers of the DMAx channel X
                    pub struct $CX { _0: () }

                    impl $CX {
                        /// Associated peripheral `address`
                        ///
                        /// `inc` indicates whether the address will be incremented after every byte transfer
                        pub unsafe fn set_peripheral_address(&mut self, address: u32, inc: bool) {
                            self.paddr().write(|w| w.bits(address) );
                            self.ctl().modify(|_, w| w.pnaga().bit(inc) );
                        }

                        /// `address` where from/to data will be read/write
                        ///
                        /// `inc` indicates whether the address will be incremented after every byte transfer
                        pub unsafe fn set_memory_address(&mut self, address: u32, inc: bool) {
                            self.maddr().write(|w| w.bits(address) );
                            self.ctl().modify(|_, w| w.mnaga().bit(inc) );
                        }

                        /// Number of bytes to transfer
                        pub fn set_transfer_length(&mut self, len: usize) {
                            self.cnt().write(|w| unsafe { w.cnt().bits(cast::u16(len).unwrap()) });
                        }

                        pub fn set_direction(&mut self, dir: Direction) {
                            match dir {
                                Direction::PeripherialToMemory => {
                                    self.ctl().modify(|_, w| w.m2m().clear_bit());
                                    self.ctl().modify(|_, w| w.dir().clear_bit())
                                }
                                Direction::MemoryToPeripherial => {
                                    self.ctl().modify(|_, w| w.m2m().clear_bit());
                                    self.ctl().modify(|_, w| w.dir().set_bit())
                                }
                                Direction::MemoryToMemory => {
                                    self.ctl().modify(|_, w| w.m2m().set_bit())
                                }
                            };
                        }

                        /// Starts the DMA transfer
                        pub fn start(&mut self) {
                            self.ctl().modify(|_, w| w.chen().set_bit() );
                        }

                        /// Stops the DMA transfer
                        pub fn stop(&mut self) {
                            self.intc().write(|w| w.$gifcX().set_bit());
                            self.ctl().modify(|_, w| w.chen().clear_bit() );
                        }

                        /// Returns `true` if there's a transfer in progress
                        pub fn in_progress(&self) -> bool {
                            self.intf().$ftfifX().bit_is_clear()
                        }
                    }

                    impl $CX {
                        pub fn listen(&mut self, event: Event) {
                            match event {
                                Event::HalfTransfer => self.ctl().modify(|_, w| w.htfie().set_bit()),
                                Event::TransferComplete => {
                                    self.ctl().modify(|_, w| w.ftfie().set_bit())
                                }
                            }
                        }

                        pub fn unlisten(&mut self, event: Event) {
                            match event {
                                Event::HalfTransfer => {
                                    self.ctl().modify(|_, w| w.htfie().clear_bit())
                                },
                                Event::TransferComplete => {
                                    self.ctl().modify(|_, w| w.ftfie().clear_bit())
                                }
                            }
                        }

                        pub fn intf(&self) -> $dmaX::intf::R {
                            // NOTE(unsafe) atomic read with no side effects
                            unsafe { (*$DMAX::ptr()).intf.read() }
                        }

                        pub fn intc(&self) -> &$dmaX::INTC {
                            unsafe { &(*$DMAX::ptr()).intc }
                        }

                        pub fn ctl(&self) -> &$dmaX::$CTLX {
                            unsafe { &(*$DMAX::ptr()).$ctlX }
                        }

                        pub fn cnt(&self) -> &$dmaX::$CNTX {
                            unsafe { &(*$DMAX::ptr()).$cntX }
                        }

                        pub fn paddr(&self) -> &$dmaX::$PADDRX {
                            unsafe { &(*$DMAX::ptr()).$paddrX }
                        }

                        pub fn maddr(&self) -> &$dmaX::$MADDRX {
                            unsafe { &(*$DMAX::ptr()).$maddrX }
                        }
                    }

                    impl<B, PAYLOAD> CircBuffer<B, RxDma<PAYLOAD, $CX>>
                    where
                        RxDma<PAYLOAD, $CX>: TransferPayload,
                    {
                        /// Peeks into the readable half of the buffer
                        pub fn peek<R, F>(&mut self, f: F) -> Result<R, Error>
                            where
                            F: FnOnce(&B, Half) -> R,
                        {
                            let half_being_read = self.readable_half()?;

                            let buf = match half_being_read {
                                Half::First => &self.buffer[0],
                                Half::Second => &self.buffer[1],
                            };

                            // XXX does this need a compiler barrier?
                            let ret = f(buf, half_being_read);


                            let intf = self.payload.channel.intf();
                            let first_half_is_done = intf.$htfifX().bit_is_set();
                            let second_half_is_done = intf.$ftfifX().bit_is_set();

                            if (half_being_read == Half::First && second_half_is_done) ||
                                (half_being_read == Half::Second && first_half_is_done) {
                                Err(Error::Overrun)
                            } else {
                                Ok(ret)
                            }
                        }

                        /// Returns the `Half` of the buffer that can be read
                        pub fn readable_half(&mut self) -> Result<Half, Error> {
                            let intf = self.payload.channel.intf();
                            let first_half_is_done = intf.$htfifX().bit_is_set();
                            let second_half_is_done = intf.$ftfifX().bit_is_set();

                            if first_half_is_done && second_half_is_done {
                                return Err(Error::Overrun);
                            }

                            let last_read_half = self.readable_half;

                            Ok(match last_read_half {
                                Half::First => {
                                    if second_half_is_done {
                                        self.payload.channel.intc().write(|w| w.$ftfifcX().set_bit());

                                        self.readable_half = Half::Second;
                                        Half::Second
                                    } else {
                                        last_read_half
                                    }
                                }
                                Half::Second => {
                                    if first_half_is_done {
                                        self.payload.channel.intc().write(|w| w.$htfifcX().set_bit());

                                        self.readable_half = Half::First;
                                        Half::First
                                    } else {
                                        last_read_half
                                    }
                                }
                            })
                        }

                        /// Stops the transfer and returns the underlying buffer and RxDma
                        pub fn stop(mut self) -> (&'static mut [B; 2], RxDma<PAYLOAD, $CX>) {
                            self.payload.stop();

                            (self.buffer, self.payload)
                        }
                    }

                    impl<BUFFER, PAYLOAD, MODE> Transfer<MODE, BUFFER, RxDma<PAYLOAD, $CX>>
                    where
                        RxDma<PAYLOAD, $CX>: TransferPayload,
                    {
                        pub fn is_done(&self) -> bool {
                            !self.payload.channel.in_progress()
                        }

                        pub fn wait(mut self) -> (BUFFER, RxDma<PAYLOAD, $CX>) {
                            while !self.is_done() {}

                            atomic::compiler_fence(Ordering::Acquire);

                            self.payload.stop();

                            // we need a read here to make the Acquire fence effective
                            // we do *not* need this if `dma.stop` does a RMW operation
                            unsafe { ptr::read_volatile(&0); }

                            // we need a fence here for the same reason we need one in `Transfer.wait`
                            atomic::compiler_fence(Ordering::Acquire);

                            // `Transfer` needs to have a `Drop` implementation, because we accept
                            // managed buffers that can free their memory on drop. Because of that
                            // we can't move out of the `Transfer`'s fields, so we use `ptr::read`
                            // and `mem::forget`.
                            //
                            // NOTE(unsafe) There is no panic branch between getting the resources
                            // and forgetting `self`.
                            unsafe {
                                let buffer = ptr::read(&self.buffer);
                                let payload = ptr::read(&self.payload);
                                mem::forget(self);
                                (buffer, payload)
                            }
                        }
                    }

                    impl<BUFFER, PAYLOAD, MODE> Transfer<MODE, BUFFER, TxDma<PAYLOAD, $CX>>
                    where
                        TxDma<PAYLOAD, $CX>: TransferPayload,
                    {
                        pub fn is_done(&self) -> bool {
                            !self.payload.channel.in_progress()
                        }

                        pub fn wait(mut self) -> (BUFFER, TxDma<PAYLOAD, $CX>) {
                            while !self.is_done() {}

                            atomic::compiler_fence(Ordering::Acquire);

                            self.payload.stop();

                            // we need a read here to make the Acquire fence effective
                            // we do *not* need this if `dma.stop` does a RMW operation
                            unsafe { ptr::read_volatile(&0); }

                            // we need a fence here for the same reason we need one in `Transfer.wait`
                            atomic::compiler_fence(Ordering::Acquire);

                            // `Transfer` needs to have a `Drop` implementation, because we accept
                            // managed buffers that can free their memory on drop. Because of that
                            // we can't move out of the `Transfer`'s fields, so we use `ptr::read`
                            // and `mem::forget`.
                            //
                            // NOTE(unsafe) There is no panic branch between getting the resources
                            // and forgetting `self`.
                            unsafe {
                                let buffer = ptr::read(&self.buffer);
                                let payload = ptr::read(&self.payload);
                                mem::forget(self);
                                (buffer, payload)
                            }
                        }
                    }

                    impl<BUFFER, PAYLOAD> Transfer<W, BUFFER, RxDma<PAYLOAD, $CX>>
                    where
                        RxDma<PAYLOAD, $CX>: TransferPayload,
                    {
                        pub fn peek<T>(&self) -> &[T]
                        where
                            BUFFER: AsRef<[T]>,
                        {
                            let pending = self.payload.channel.cnt().read().bits() as usize;

                            let slice = self.buffer.as_ref();
                            let capacity = slice.len();

                            &slice[..(capacity - pending)]
                        }
                    }
                )+

                impl DmaExt for $DMAX {
                    type Channels = Channels;

                    fn split(self, rcu: &mut Rcu) -> Channels {
                        $DMAX::enable(rcu);

                        // reset the DMA control registers (stops all on-going transfers)
                        $(
                            self.$ctlX.reset();
                        )+

                        Channels((), $($CX { _0: () }),+)
                    }
                }
            }
        )+
    }
}

dma! {
    DMA0: (dma0, {
        C0: (
            CH0CTL, CH0CNT, CH0PADDR, CH0MADDR,
            ch0ctl, ch0cnt, ch0paddr, ch0maddr,
            htfif0, ftfif0,
            htfifc0, ftfifc0, gifc0
        ),
        C1: (
            CH1CTL, CH1CNT, CH1PADDR, CH1MADDR,
            ch1ctl, ch1cnt, ch1paddr, ch1maddr,
            htfif1, ftfif1,
            htfifc1, ftfifc1, gifc1
        ),
        C2: (
            CH2CTL, CH2CNT, CH2PADDR, CH2MADDR,
            ch2ctl, ch2cnt, ch2paddr, ch2maddr,
            htfif2, ftfif2,
            htfifc2, ftfifc2, gifc2
        ),
        C3: (
            CH3CTL, CH3CNT, CH3PADDR, CH3MADDR,
            ch3ctl, ch3cnt, ch3paddr, ch3maddr,
            htfif3, ftfif3,
            htfifc3, ftfifc3, gifc3
        ),
        C4: (
            CH4CTL, CH4CNT, CH4PADDR, CH4MADDR,
            ch4ctl, ch4cnt, ch4paddr, ch4maddr,
            htfif4, ftfif4,
            htfifc4, ftfifc4, gifc4
        ),
        C5: (
            CH5CTL, CH5CNT, CH5PADDR, CH5MADDR,
            ch5ctl, ch5cnt, ch5paddr, ch5maddr,
            htfif5, ftfif5,
            htfifc5, ftfifc5, gifc5
        ),
        C6: (
            CH6CTL, CH6CNT, CH6PADDR, CH6MADDR,
            ch6ctl, ch6cnt, ch6paddr, ch6maddr,
            htfif6, ftfif6,
            htfifc6, ftfifc6, gifc6
        ),
    }),

    DMA1: (dma1, {
        C0: (
            CH0CTL, CH0CNT, CH0PADDR, CH0MADDR,
            ch0ctl, ch0cnt, ch0paddr, ch0maddr,
            htfif0, ftfif0,
            htfifc0, ftfifc0, gifc0
        ),
        C1: (
            CH1CTL, CH1CNT, CH1PADDR, CH1MADDR,
            ch1ctl, ch1cnt, ch1paddr, ch1maddr,
            htfif1, ftfif1,
            htfifc1, ftfifc1, gifc1
        ),
        C2: (
            CH2CTL, CH2CNT, CH2PADDR, CH2MADDR,
            ch2ctl, ch2cnt, ch2paddr, ch2maddr,
            htfif2, ftfif2,
            htfifc2, ftfifc2, gifc2
        ),
        C3: (
            CH3CTL, CH3CNT, CH3PADDR, CH3MADDR,
            ch3ctl, ch3cnt, ch3paddr, ch3maddr,
            htfif3, ftfif3,
            htfifc3, ftfifc3, gifc3
        ),
        C4: (
            CH4CTL, CH4CNT, CH4PADDR, CH4MADDR,
            ch4ctl, ch4cnt, ch4paddr, ch4maddr,
            htfif4, ftfif4,
            htfifc4, ftfifc4, gifc4
        ),
    }),
}

/// DMA Receiver
pub struct RxDma<PAYLOAD, RXCH> {
    pub(crate) payload: PAYLOAD,
    pub channel: RXCH,
}

/// DMA Transmitter
pub struct TxDma<PAYLOAD, TXCH> {
    pub(crate) payload: PAYLOAD,
    pub channel: TXCH,
}

/// DMA Receiver/Transmitter
pub struct RxTxDma<PAYLOAD, RXCH, TXCH> {
    pub(crate) payload: PAYLOAD,
    pub rxchannel: RXCH,
    pub txchannel: TXCH,
}

pub trait Receive {
    type RxChannel;
    type TransmittedWord;
}

pub trait Transmit {
    type TxChannel;
    type ReceivedWord;
}

/// Trait for circular DMA readings from peripheral to memory.
pub trait CircReadDma<B, RS>: Receive
where
    &'static mut [B; 2]: StaticWriteBuffer<Word = RS>,
    B: 'static,
    Self: core::marker::Sized,
{
    fn circ_read(self, buffer: &'static mut [B; 2]) -> CircBuffer<B, Self>;
}

/// Trait for DMA readings from peripheral to memory.
pub trait ReadDma<B, RS>: Receive
where
    B: StaticWriteBuffer<Word = RS>,
    Self: core::marker::Sized + TransferPayload,
{
    fn read(self, buffer: B) -> Transfer<W, B, Self>;
}

/// Trait for DMA writing from memory to peripheral.
pub trait WriteDma<B, TS>: Transmit
where
    B: StaticReadBuffer<Word = TS>,
    Self: core::marker::Sized + TransferPayload,
{
    fn write(self, buffer: B) -> Transfer<R, B, Self>;
}
