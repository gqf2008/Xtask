use embedded_hal::digital::v2::OutputPin;
use rp2040_hal::gpio::{bank0::Gpio25, Output, Pin, PushPull};
pub struct Led {
    port: Pin<Gpio25, Output<PushPull>>,
}

impl Led {
    pub fn new(pin: Pin<Gpio25, Output<PushPull>>) -> Self {
        Self {
            port: pin.into_push_pull_output(),
        }
    }
    pub fn off(&mut self) {
        self.port.set_high().ok();
    }

    pub fn on(&mut self) {
        self.port.set_low().ok();
    }
}
