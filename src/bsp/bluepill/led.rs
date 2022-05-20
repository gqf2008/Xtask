use stm32f1xx_hal::gpio::{Output, Pin, PushPull, CRH};

pub struct Led(pub Pin<Output<PushPull>, CRH, 'C', 13>);

impl Led {
    pub fn new(pin: Pin<Output<PushPull>, CRH, 'C', 13>) -> Self {
        Self(pin)
    }

    pub fn off(&mut self) {
        self.0.set_high()
    }

    pub fn on(&mut self) {
        self.0.set_low()
    }

    pub fn is_on(&mut self) -> bool {
        self.0.is_set_low()
    }

    pub fn is_off(&mut self) -> bool {
        self.0.is_set_high()
    }

    pub fn toggle(&mut self) {
        self.0.toggle()
    }
}
