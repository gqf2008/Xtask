use embedded_hal::digital::v2::OutputPin;
use rp2040_hal::gpio::{bank0::Gpio25, FunctionSio, Pin, SioOutput};

/// 板载 LED(GPIO25,RP2040 Pico 焊在无线模块上为高电平点亮……实际是
/// 低电平点亮——on=low/off=high,与 Nucleo 习惯相反)
pub struct Led {
    port: Pin<Gpio25, FunctionSio<SioOutput>, rp2040_hal::gpio::PullDown>,
}

impl Led {
    pub fn new(pin: Pin<Gpio25, FunctionSio<SioOutput>, rp2040_hal::gpio::PullDown>) -> Self {
        Self { port: pin }
    }
    pub fn off(&mut self) {
        self.port.set_high().ok();
    }

    pub fn on(&mut self) {
        self.port.set_low().ok();
    }
}
