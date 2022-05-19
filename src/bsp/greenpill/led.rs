use stm32f4xx_hal::gpio::{Output, Pin, PushPull};

pub struct Led {
    port: Pin<'C', 13, Output<PushPull>>,
}

impl Led {
    pub fn new(port: Pin<'C', 13>) -> Self {
        Self {
            port: port.into_push_pull_output(),
        }
    }
    pub fn off(&mut self) {
        self.port.set_high();
    }

    pub fn on(&mut self) {
        self.port.set_low();
    }
}
