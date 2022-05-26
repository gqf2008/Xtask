use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::timer::CountDown;
use gd32vf103xx_hal::delay::Delay;
use gd32vf103xx_hal::rcu::Rcu;
use gd32vf103xx_hal::time::*;
use gd32vf103xx_hal::{pac::TIMER0, timer::Timer};

pub fn time1(timer0: TIMER0, hz: u32, rcu: &mut Rcu) {
    let timer: Timer<TIMER0> = Timer::timer0(timer0, hz.hz(), rcu);
    let delay = Delay::timer0(timer);
}
