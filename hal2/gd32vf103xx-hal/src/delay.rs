//! Delays

use crate::timer::Timer;
use crate::time::U32Ext;

use cast::u32;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::timer::CountDown;
use gd32vf103_pac::{TIMER0, TIMER1, TIMER2, TIMER3, TIMER4, TIMER5, TIMER6};
use crate::rcu::Clocks;

/// Machine mode cycle counter (`mcycle`) as a delay provider
#[derive(Copy, Clone)]
pub struct McycleDelay {
    core_frequency: u32
}

impl McycleDelay {
    /// Constructs the delay provider
    pub fn new(clocks: &Clocks) -> Self {
        Self {
            core_frequency: clocks.sysclk().0
        }
    }
}

impl DelayUs<u64> for McycleDelay {
    fn delay_us(&mut self, us: u64) {
        let t0 = riscv::register::mcycle::read64();
        let clocks = (us * (self.core_frequency as u64)) / 1_000_000;
        while riscv::register::mcycle::read64().wrapping_sub(t0) <= clocks { }
    }
}

impl DelayUs<u32> for McycleDelay {
    #[inline(always)]
    fn delay_us(&mut self, us: u32) {
        self.delay_us(us as u64)
    }
}

// Implemented for constructions like `delay.delay_us(50_000);`
impl DelayUs<i32> for McycleDelay {
    #[inline(always)]
    fn delay_us(&mut self, us: i32) {
        assert!(us >= 0);
        self.delay_us(us as u32);
    }
}

impl DelayUs<u16> for McycleDelay {
    #[inline(always)]
    fn delay_us(&mut self, us: u16) {
        self.delay_us(us as u32)
    }
}

impl DelayUs<u8> for McycleDelay {
    #[inline(always)]
    fn delay_us(&mut self, us: u8) {
        self.delay_us(us as u32)
    }
}

impl DelayMs<u32> for McycleDelay {
    fn delay_ms(&mut self, ms: u32) {
        self.delay_us((ms as u64) * 1000)
    }
}

// Implemented for constructions like `delay.delay_ms(50_000);`
impl DelayMs<i32> for McycleDelay {
    #[inline(always)]
    fn delay_ms(&mut self, ms: i32) {
        assert!(ms >= 0);
        self.delay_ms(ms as u32);
    }
}

impl DelayMs<u16> for McycleDelay {
    #[inline(always)]
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(ms as u32)
    }
}

impl DelayMs<u8> for McycleDelay {
    #[inline(always)]
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(ms as u32)
    }
}

/// TIMER as a delay provider
pub struct Delay<TIMER> where Timer<TIMER>: CountDown {
    timer: Timer<TIMER>,
}

macro_rules! delay {
    ($($TIMER:ident,)+) => {
        $(
            impl Delay<$TIMER> {
                /// Configures the timer as a delay provider
                pub fn new(timer: Timer<$TIMER>) -> Self {

                    Delay { timer, }
                }

                /// Releases the timer resource
                pub fn free(self) -> Timer<$TIMER> {
                    self.timer
                }
            }

            impl DelayMs<u32> for Delay<$TIMER> {
                fn delay_ms(&mut self, ms: u32) {
                    self.delay_us(ms * 1_000);
                }
            }

            impl DelayMs<u16> for Delay<$TIMER> {
                fn delay_ms(&mut self, ms: u16) {
                    self.delay_ms(u32(ms));
                }
            }

            impl DelayMs<u8> for Delay<$TIMER> {
                fn delay_ms(&mut self, ms: u8) {
                    self.delay_ms(u32(ms));
                }
            }

            impl DelayUs<u32> for Delay<$TIMER> {
                fn delay_us(&mut self, us: u32) {
                    let freq = 1_000_000 / us;
                    self.timer.start(freq.hz());
                    while let Err(_) = self.timer.wait() { }
                    self.timer.tim.ctl0.modify(|_, w| w.cen().clear_bit());
                }
            }

            impl DelayUs<u16> for Delay<$TIMER> {
                fn delay_us(&mut self, us: u16) {
                    self.delay_us(u32(us))
                }
            }

            impl DelayUs<u8> for Delay<$TIMER> {
                fn delay_us(&mut self, us: u8) {
                    self.delay_us(u32(us))
                }
            }
        )+
    }
}

delay! {
    TIMER0,
    TIMER1,
    TIMER2,
    TIMER3,
    TIMER4,
    TIMER5,
    TIMER6,
}
