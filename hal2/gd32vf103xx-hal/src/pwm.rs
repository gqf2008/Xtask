//! Pulse width modulation

use embedded_hal::Pwm;
use gd32vf103_pac::{TIMER0, TIMER1, TIMER2, TIMER3, TIMER4};

use crate::afio::{Afio, Remap};
use crate::gpio::gpioa::*;
use crate::gpio::gpiob::*;
use crate::gpio::gpioc::*;
use crate::gpio::gpiod::*;
use crate::gpio::gpioe::*;
use crate::gpio::{Alternate, PushPull};
use crate::rcu::{BaseFrequency, Enable, Rcu, Reset};
use crate::time::{Hertz, U32Ext};

use core::marker::PhantomData;

pub trait Pins<TIMER, Remap> {
    fn remap(&self) -> Remap;
}

macro_rules! pwm_pin {
    ($timer:ty, $($remap:ident: [
        ch0:$ch0_pin:ident, ch1:$ch1_pin:ident,
        ch2:$ch2_pin:ident, ch3:$ch3_pin:ident
    ],)+ ) => {
    $(
        impl Pins<$timer, $remap> for (Option<&$ch0_pin<Alternate<PushPull>>>,
                                       Option<&$ch1_pin<Alternate<PushPull>>>,
                                       Option<&$ch2_pin<Alternate<PushPull>>>,
                                       Option<&$ch3_pin<Alternate<PushPull>>>)
        {
            fn remap(&self) -> $remap { $remap {} }
        } )+
    }
}

pwm_pin! {
    TIMER0,
        NoRemap: [ ch0: PA8, ch1: PA9, ch2: PA10, ch3: PA11 ],
        PartialRemap1: [ ch0: PA8, ch1: PA9, ch2: PA10, ch3: PA11 ],
        FullRemap: [ ch0: PE9, ch1: PE11, ch2: PE13, ch3: PE14 ],
}

pwm_pin! {
    TIMER1,
        NoRemap: [ ch0: PA0, ch1: PA1, ch2: PA2, ch3: PA3 ],
        PartialRemap1: [ ch0: PA15, ch1: PB3, ch2: PA2, ch3: PA3 ],
        PartialRemap2: [ ch0: PA0, ch1: PA1, ch2: PB10, ch3: PB11 ],
        FullRemap: [ ch0: PA15, ch1: PB3, ch2: PB10, ch3: PB11 ],
}

pwm_pin! {
    TIMER2,
        NoRemap: [ ch0: PA6, ch1: PA7, ch2: PB0, ch3: PB1 ],
        PartialRemap2: [ ch0: PB4, ch1: PB5, ch2: PB0, ch3: PB1 ],
        FullRemap: [ ch0: PC6, ch1: PC7, ch2: PC8, ch3: PC9 ],
}

pwm_pin! {
    TIMER3,
        NoRemap: [ ch0: PB6, ch1: PB7, ch2: PB8, ch3: PB9 ],
        FullRemap: [ ch0: PD12, ch1: PD13, ch2: PD14, ch3: PD15 ],
}

pwm_pin! {
    TIMER4,
        NoRemap: [ ch0: PA0, ch1: PA1, ch2: PA2, ch3: PA3 ],
}

/// Type [`NoRemap`] represent the timers *no remap mode*, which correspond for
/// TIMER0, TIMER1 and TIMER2 to `0b00`; For TIMER3 and TIMER4 correspond
/// to `0b0`.
pub struct NoRemap;

/// Type [`PartialRemap1`] represent the timers *partial remap* mode, which
/// correspond for TIMER0, TIMER1 to `0b01`.
pub struct PartialRemap1;

/// Type [`PartialRemap2`] represent the timers *partial remap* mode, which
/// correspond for TIMER1 and TIMER2 to `0b10`,
pub struct PartialRemap2;

/// Type [`FullRemap`] represent the timers *full remap* mode, which correspond
/// for TIMER0, TIMER1, TIMER2 to `0b11`; For TIMER3 and TIMER4 to `0b1`.
pub struct FullRemap;

impl From<NoRemap> for bool {
    fn from(_v: NoRemap) -> Self {
        false
    }
}

impl From<NoRemap> for u8 {
    fn from(_v: NoRemap) -> Self {
        0b00
    }
}

impl From<FullRemap> for bool {
    fn from(_v: FullRemap) -> Self {
        true
    }
}

impl From<FullRemap> for u8 {
    fn from(_v: FullRemap) -> Self {
        0b11
    }
}

impl From<PartialRemap1> for u8 {
    fn from(_v: PartialRemap1) -> Self {
        0b01
    }
}

impl From<PartialRemap2> for u8 {
    fn from(_v: PartialRemap2) -> Self {
        0b11
    }
}

/// PWM TIMER configuration.
///
/// To create a new PWM TIMER you should choose a timer and a REMAP mode from
/// [`NoRemap`], [`PartialRemap1`], [`PartialRemap2`] or [`FullRemap`].
/// The tuple provided should contains only the pins you will use.
///
/// ```no_run
/// use gd32vf103xx_hal as hal;
/// use hal::pac::{Peripherals, TIMER0};
/// use hal::gpio::GpioExt;
/// use hal::rcu::RcuExt;
/// use hal::afio::AfioExt;
/// use hal::pwm::{PwmTimer, Channel, NoRemap};
/// use embedded_hal::Pwm;
///
/// // ...
/// let dp = Peripherals::take().unwrap();
/// let mut rcu = dp.RCU.configure().freeze();
///
/// let gpioa = dp.GPIOA.split(&mut rcu);
/// let pa9 = gpioa.pa9.into_alternate_push_pull();
/// let pa10 = gpioa.pa10.into_alternate_push_pull();
///
/// let mut afio = dp.AFIO.constrain(&mut rcu);
/// let timer0 = dp.TIMER0;

/// let mut pwm_t0 = PwmTimer::<TIMER0, NoRemap>::new(
///     timer0, (None, Some(&pa9), Some(&pa10), None), &mut rcu, &mut afio);
///
/// let max = pwm_t0.get_max_duty();
/// pwm_t0.set_period(100.hz());
/// pwm_t0.set_duty(Channel::CH0, max / 4); // 25% duty cycle
/// pwm_t0.enable(Channel::CH0);
///
/// ```
pub struct PwmTimer<TIMER, REMAP> {
    timer: TIMER,
    timer_clock: Hertz,
    max_duty_cycle: u16,
    period: Hertz,
    duty: [u16; 4],
    _remap: PhantomData<REMAP>,
}

#[derive(Copy, Clone)]
pub enum Channel {
    CH0,
    CH1,
    CH2,
    CH3,
}

macro_rules! advanced_pwm_timer {
    ($TIM:ident: $tim:ident) => {
        impl<REMAP: Into<u8>> PwmTimer<$TIM, REMAP> {
            pub fn new(
                timer: $TIM,
                pins: impl Pins<$TIM, REMAP>,
                rcu: &mut Rcu,
                afio: &mut Afio,
            ) -> PwmTimer<$TIM, REMAP> {
                <$TIM>::remap(afio, pins.remap().into());

                $TIM::enable(rcu);
                $TIM::reset(rcu);

                /* Advanced TIMER implements a BREAK function that deactivates
                 * the outputs. This bit automatically activates the output when
                 * no break input is present */
                timer.cchp.modify(|_, w| w.oaen().set_bit());

                PwmTimer {
                    timer,
                    timer_clock: $TIM::base_frequency(rcu),
                    max_duty_cycle: u16::MAX,
                    period: 0.hz(),
                    duty: [0u16; 4],
                    _remap: PhantomData::<REMAP>,
                }
            }
        }

        pwm_timer!($TIM: $tim);
    };
}

macro_rules! general_pwm_timer {
    ($TIM:ident: $tim:ident) => {
        impl<REMAP: Into<u8> + Into<bool>> PwmTimer<$TIM, REMAP> {
            pub fn new(
                timer: $TIM,
                pins: impl Pins<$TIM, REMAP>,
                rcu: &mut Rcu,
                afio: &mut Afio,
            ) -> PwmTimer<$TIM, REMAP> {
                <$TIM>::remap(afio, pins.remap().into());

                $TIM::enable(rcu);
                $TIM::reset(rcu);

                PwmTimer {
                    timer,
                    timer_clock: $TIM::base_frequency(rcu),
                    max_duty_cycle: u16::MAX,
                    period: 0.hz(),
                    duty: [0u16; 4],
                    _remap: PhantomData::<REMAP>,
                }
            }
        }

        pwm_timer!($TIM: $tim);
    };
}

macro_rules! pwm_timer {
    ($TIM:ident: $tim:ident) => {
        impl<REMAP> Pwm for PwmTimer<$TIM, REMAP> {
            type Channel = Channel;
            type Time = Hertz;
            type Duty = u16;

            fn disable(&mut self, channel: Self::Channel) {
                match channel {
                    Channel::CH0 => self.timer.chctl2.modify(|_r, w| w.ch0en().clear_bit()),
                    Channel::CH1 => self.timer.chctl2.modify(|_r, w| w.ch1en().clear_bit()),
                    Channel::CH2 => self.timer.chctl2.modify(|_r, w| w.ch2en().clear_bit()),
                    Channel::CH3 => self.timer.chctl2.modify(|_r, w| w.ch3en().clear_bit()),
                }
            }

            fn enable(&mut self, channel: Self::Channel) {
                match channel {
                    Channel::CH0 => self.timer.chctl2.modify(|_r, w| w.ch0en().set_bit()),
                    Channel::CH1 => self.timer.chctl2.modify(|_r, w| w.ch1en().set_bit()),
                    Channel::CH2 => self.timer.chctl2.modify(|_r, w| w.ch2en().set_bit()),
                    Channel::CH3 => self.timer.chctl2.modify(|_r, w| w.ch3en().set_bit()),
                }
            }

            fn get_period(&self) -> Self::Time {
                return self.period;
            }

            fn get_duty(&self, channel: Self::Channel) -> Self::Duty {
                self.duty[channel as usize]
            }

            fn get_max_duty(&self) -> Self::Duty {
                self.max_duty_cycle
            }

            fn set_duty(&mut self, channel: Self::Channel, duty: Self::Duty) {
                let mut duty = duty;
                if duty > self.max_duty_cycle {
                    duty = self.max_duty_cycle
                }
                self.duty[channel as usize] = duty;
                match channel {
                    Channel::CH0 => self.timer.ch0cv.write(|w| unsafe { w.bits(duty) }),
                    Channel::CH1 => self.timer.ch1cv.write(|w| unsafe { w.bits(duty) }),
                    Channel::CH2 => self.timer.ch2cv.write(|w| unsafe { w.bits(duty) }),
                    Channel::CH3 => self.timer.ch3cv.write(|w| unsafe { w.bits(duty) }),
                }
            }

            fn set_period<P>(&mut self, period: P)
            where
                P: Into<Self::Time>,
            {
                self.timer.ctl0.modify(|_, w| w.cen().clear_bit());
                self.timer.cnt.reset();

                let freq = period.into();

                let ticks = self.timer_clock.0 / freq.0;
                let psc = ((ticks - 1) / (1 << 16)) as u16;
                let car = (ticks / ((psc + 1) as u32)) as u16;

                self.max_duty_cycle = car;
                self.period = freq;

                self.timer.psc.write(|w| unsafe { w.bits(psc) });
                self.timer.car.write(|w| unsafe { w.bits(car) });

                self.timer.chctl0_output().modify(|_r, w| unsafe {
                    w
                        // Enable PWM Mode 0 for channel 0 and 1
                        .ch0comctl().bits(0b110)
                        .ch1comctl().bits(0b110)

                        // Output mode for channel 0 and 1
                        .ch0ms().bits(0b00)
                        .ch1ms().bits(0b00)
                });
                self.timer.chctl1_output().modify(|_r, w| unsafe {
                    w
                        // Enable PWM Mode 0 for channel 2 and 3
                        .ch2comctl().bits(0b110)
                        .ch3comctl().bits(0b110)

                        // Output mode for channel 2 and 3
                        .ch2ms().bits(0b00)
                        .ch3ms().bits(0b00)
                });

                // Enable the timer
                self.timer.ctl0.write(|w| {
                    w
                        .updis().clear_bit()
                        .cen().set_bit()
                });
            }
        }
    };
}

advanced_pwm_timer! {TIMER0: timer0}
general_pwm_timer! {TIMER1: timer1}
general_pwm_timer! {TIMER2: timer2}
general_pwm_timer! {TIMER3: timer3}
general_pwm_timer! {TIMER4: timer4}
