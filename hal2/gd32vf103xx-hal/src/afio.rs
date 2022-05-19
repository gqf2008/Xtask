//! Alternate Function I/Os

use crate::pac::AFIO;
use crate::rcu::{Rcu, Enable, Reset};
use crate::gpio::{Debugger, Input, Floating, Port};
use crate::gpio::gpioa::{PA13, PA14, PA15};
use crate::gpio::gpiob::{PB3, PB4};

pub trait AfioExt {
    fn constrain(self, rcu: &mut Rcu) -> Afio;
}

impl AfioExt for AFIO {
    fn constrain(self, rcu: &mut Rcu) -> Afio {
        AFIO::enable(rcu);
        AFIO::reset(rcu);

        Afio { afio: self }
    }
}

pub struct Afio {
    afio: AFIO
}

impl Afio {
    /// Disables the JTAG to free up PA13..PA15, PB3 and PB4 for normal use
    pub fn disable_jtag(
        &mut self,
        pa13: PA13<Debugger>,
        pa14: PA14<Debugger>,
        pa15: PA15<Debugger>,
        pb3: PB3<Debugger>,
        pb4: PB4<Debugger>
    ) -> (
        PA13<Input<Floating>>,
        PA14<Input<Floating>>,
        PA15<Input<Floating>>,
        PB3<Input<Floating>>,
        PB4<Input<Floating>>,
    ) {
        // Set remap to "JTAG-DP Disabled"
        self.afio.pcf0.modify(|_, w| unsafe {
            w.swj_cfg().bits(0b100)
        });

        // NOTE(unsafe) The pins are now in the good state.
        unsafe {
            (
                pa13.activate(),
                pa14.activate(),
                pa15.activate(),
                pb3.activate(),
                pb4.activate()
            )
        }
    }

    #[inline]
    pub fn extiss(&mut self, port: Port, pin: u8) {
        match pin {
            0 => self.afio.extiss0.modify(|_, w| unsafe { w.exti0_ss().bits(port as u8)}),
            1 => self.afio.extiss0.modify(|_, w| unsafe { w.exti1_ss().bits(port as u8)}),
            2 => self.afio.extiss0.modify(|_, w| unsafe { w.exti2_ss().bits(port as u8)}),
            3 => self.afio.extiss0.modify(|_, w| unsafe { w.exti3_ss().bits(port as u8)}),
            4 => self.afio.extiss1.modify(|_, w| unsafe { w.exti4_ss().bits(port as u8)}),
            5 => self.afio.extiss1.modify(|_, w| unsafe { w.exti5_ss().bits(port as u8)}),
            6 => self.afio.extiss1.modify(|_, w| unsafe { w.exti6_ss().bits(port as u8)}),
            7 => self.afio.extiss1.modify(|_, w| unsafe { w.exti7_ss().bits(port as u8)}),
            8 => self.afio.extiss2.modify(|_, w| unsafe { w.exti8_ss().bits(port as u8)}),
            9 => self.afio.extiss2.modify(|_, w| unsafe { w.exti9_ss().bits(port as u8)}),
            10 => self.afio.extiss2.modify(|_, w| unsafe { w.exti10_ss().bits(port as u8)}),
            11 => self.afio.extiss2.modify(|_, w| unsafe { w.exti11_ss().bits(port as u8)}),
            12 => self.afio.extiss3.modify(|_, w| unsafe { w.exti12_ss().bits(port as u8)}),
            13 => self.afio.extiss3.modify(|_, w| unsafe { w.exti13_ss().bits(port as u8)}),
            14 => self.afio.extiss3.modify(|_, w| unsafe { w.exti14_ss().bits(port as u8)}),
            15 => self.afio.extiss3.modify(|_, w| unsafe { w.exti15_ss().bits(port as u8)}),
            _ => {}
        }
    }
}

pub(crate) mod closed_traits {
    use crate::afio::Afio;

    pub trait Remap {
        type Variant;

        fn remap(afio: &mut Afio, variant: Self::Variant);
    }
}
pub(crate) use closed_traits::*;

macro_rules! remap_set {
    ($pcf0:ident, $field:ident, bool, $value:ident) => {
        $pcf0.write(|w| w.$field().bit($value));
    };
    ($pcf0:ident, $field:ident, $type:ty, $value:ident) => {
        $pcf0.write(|w| unsafe {
            w.$field().bits(u8::from($value))
        });
    }
}

macro_rules! remap {
    ($($PER:ident => ($field:ident, $variant:tt),)+) => {
        $(
            impl Remap for crate::pac::$PER {
                type Variant = $variant;

                #[inline(always)]
                fn remap(afio: &mut Afio, variant: $variant) {
                    let pcf0 = &afio.afio.pcf0;
                    remap_set!(pcf0, $field, $variant, variant);
                }
            }
        )+
    }
}

remap! {
    I2C0 => (i2c0_remap, bool),
    SPI0 => (spi0_remap, bool),
    SPI2 => (spi2_remap, bool),
    USART0 => (usart0_remap, bool),
    USART1 => (usart1_remap, bool),
    USART2 => (usart2_remap, u8),
    TIMER0 => (timer0_remap, u8),
    TIMER1 => (timer1_remap, u8),
    TIMER2 => (timer2_remap, u8),
    TIMER3 => (timer3_remap, bool),
    TIMER4 => (timer4ch3_iremap, bool),
}
