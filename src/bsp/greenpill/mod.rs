pub use stm32f4xx_hal as hal;

pub mod led;
pub mod stdout;

static mut TAKEN: bool = false;

pub fn take() -> Option<(cortex_m::Peripherals, stm32f4xx_hal::pac::Peripherals)> {
    cortex_m::interrupt::free(|_| {
        if unsafe { TAKEN } {
            None
        } else {
            unsafe { TAKEN = true };
            let cp = cortex_m::Peripherals::take().unwrap();
            let dp = stm32f4xx_hal::pac::Peripherals::take().unwrap();
            Some((cp, dp))
        }
    })
}
pub fn enable_interrupt(interrupt: hal::pac::Interrupt) {
    unsafe {
        cortex_m::peripheral::NVIC::unmask(interrupt);
    }
}

pub fn disable_interrupt(interrupt: hal::pac::Interrupt) {
    cortex_m::peripheral::NVIC::mask(interrupt);
}
