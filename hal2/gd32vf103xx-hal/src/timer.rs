//! Timers

use crate::rcu::{BaseFrequency, Enable, Rcu, Reset};
use crate::time::Hertz;

use embedded_hal::timer::{CountDown, Periodic};
use gd32vf103_pac::{TIMER0, TIMER1, TIMER2, TIMER3, TIMER4, TIMER5, TIMER6};
use void::Void;

/// Hardware timer
pub struct Timer<TIM> {
    pub(crate) tim: TIM,
    pub(crate) timer_clock: Hertz,
    pub(crate) timeout: Hertz,
}

/// Interrupt events
pub enum Event {
    /// Update event. It usually happens due to the counter overflow/underflow.
    Update,
}

macro_rules! hal {
    ($($TIM:ident: $tim:ident,)+) => {
        $(
            impl Timer<$TIM> {
                pub fn $tim<T>(timer: $TIM, timeout: T, rcu: &mut Rcu) -> Self
                    where T: Into<Hertz> {
                    $TIM::enable(rcu);
                    $TIM::reset(rcu);
                    let mut t = Timer {
                        timer_clock: $TIM::base_frequency(rcu),
                        tim: timer,
                        timeout: Hertz(0),
                    };
                    t.start(timeout);

                    t
                }

                /// Starts listening for an `event`
                pub fn listen(&mut self, event: Event) {
                    match event {
                        Event::Update => self.tim.dmainten.modify(|_, w| w.upie().set_bit()),
                    }
                }

                /// Stops listening for an `event`
                pub fn unlisten(&mut self, event: Event) {
                    match event {
                        Event::Update => self.tim.dmainten.modify(|_, w| w.upie().clear_bit()),
                    }
                }

                /// Clears Update Interrupt Flag
                pub fn clear_update_interrupt_flag(&mut self) {
                    self.tim.intf.modify(|_, w| w.upif().clear_bit());
                }

                /// Releases the TIMER peripheral
                pub fn free(self) -> $TIM {
                    self.tim.ctl0.modify(|_, w| w.cen().clear_bit());
                    self.tim
                }
            }

            impl Periodic for Timer<$TIM> {}

            impl CountDown for Timer<$TIM> {
                type Time = Hertz;

                fn start<T>(&mut self, timeout: T)
                    where
                        T: Into<Hertz>,
                {
                    self.timeout = timeout.into();

                    self.tim.ctl0.modify(|_, w| w.cen().clear_bit());
                    self.tim.cnt.reset();

                    let ticks = self.timer_clock.0 / self.timeout.0;
                    let psc = ((ticks - 1) / (1 << 16)) as u16;
                    let car = (ticks / ((psc + 1) as u32)) as u16;
                    self.tim.psc.write(|w| unsafe { w.bits(psc) } );
                    self.tim.car.write(|w| unsafe { w.bits(car) } );

                    // Set UPS=1 so an UPG will *not* trigger an interrupt
                    self.tim.ctl0.modify(|_, w| w.ups().set_bit());
                    // Trigger an UPG to clear the timer counter register
                    // see user manual (v1.2) p.261 for details
                    self.tim.swevg.write(|w| w.upg().set_bit());

                    // clear any outstanding UPIF flag to ensure a clear start
                    self.tim.intf.modify(|_, w| w.upif().clear_bit());

                    self.tim.ctl0.write(|w| { w
                        .updis().clear_bit()
                        .cen().set_bit()
                    });
                }

                fn wait(&mut self) -> nb::Result<(), Void> {
                    if self.tim.intf.read().upif().bit_is_clear() {
                        Err(nb::Error::WouldBlock)
                    } else {
                        self.tim.intf.modify(|_r, w| w.upif().clear_bit());
                        Ok(())
                    }
                }
            }
        )+
    }
}

hal! {
    TIMER0: timer0,
    TIMER1: timer1,
    TIMER2: timer2,
    TIMER3: timer3,
    TIMER4: timer4,
    TIMER5: timer5,
    TIMER6: timer6,
}
