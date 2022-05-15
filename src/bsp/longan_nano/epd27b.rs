//! 电子墨水屏

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::spi::Write;
use embedded_hal::digital::v2::*;
use gd32vf103xx_hal::gpio::gpioa::PA4;
use gd32vf103xx_hal::gpio::gpioa::PA5;
use gd32vf103xx_hal::gpio::gpioa::PA6;
use gd32vf103xx_hal::gpio::gpioa::PA7;
use gd32vf103xx_hal::gpio::gpiob::PB0;
use gd32vf103xx_hal::gpio::gpiob::PB1;
use gd32vf103xx_hal::gpio::gpiob::PB10;
use gd32vf103xx_hal::gpio::{Alternate, Floating, Input, Output, PullUp, PushPull};

#[macro_export]
macro_rules! epd_pins {
    ($gpioa:ident, $gpiob:ident) => {{
        $crate::epd27b::Pins {
            cs: $gpioa.pa4.into_push_pull_output(),
            sck: $gpioa.pa5.into_alternate_push_pull(),
            miso: $gpioa.pa6.into_floating_input(),
            mosi: $gpioa.pa7.into_alternate_push_pull(),
            dc: $gpiob.pb0.into_push_pull_output(),
            rst: $gpiob.pb1.into_push_pull_output(),
            busy: $gpiob.pb10.into_pull_up_input(),
        }
    }};
}
type CsPin = PA4<Output<PushPull>>;
type SckPin = PA5<Alternate<PushPull>>;
type MisoPin = PA6<Input<Floating>>;
type MosiPin = PA7<Alternate<PushPull>>;
type DcPin = PB0<Output<PushPull>>;
type RstPin = PB1<Output<PushPull>>;
type BusyPin = PB10<Input<PullUp>>;

/// Pins consumed by E-Paper driver
pub struct Pins {
    pub miso: MisoPin,
    pub mosi: MosiPin,
    pub sck: SckPin,
    pub cs: CsPin,
    pub dc: DcPin,
    pub rst: RstPin,
    pub busy: BusyPin,
}

const WIDTH: usize = 176;
const HEIGHT: usize = 264;
const NUM_DISPLAY_BITS: usize = WIDTH * HEIGHT / 8;
pub struct EPD27b<SPI, CS, DC, RST, BUSY, DELAY> {
    spi: SPI,
    cs: CS,
    busy: BUSY,
    dc: DC,
    rst: RST,
    delay: DELAY,
}

impl<SPI, CS, DC, RST, BUSY, DELAY> EPD27b<SPI, CS, DC, RST, BUSY, DELAY>
where
    SPI: Write<u8>,
    BUSY: InputPin,
    DC: OutputPin,
    CS: OutputPin,
    RST: OutputPin,
    DELAY: DelayMs<u32>,
{
    pub fn new(spi: SPI, cs: CS, dc: DC, rst: RST, busy: BUSY, delay: DELAY) -> Self {
        let mut this = Self {
            spi,
            cs,
            busy,
            dc,
            rst,
            delay,
        };
        this.init();
        this
    }
    //Hardware reset
    pub fn reset(&mut self) {
        self.rst.set_high().ok();
        self.delay.delay_ms(200);
        self.rst.set_low().ok();
        self.delay.delay_ms(2);
        self.rst.set_high().ok();
        self.delay.delay_ms(200);
    }

    pub fn send_command(&mut self, cmd: u8) {
        self.dc.set_low().ok();
        self.cs.set_low().ok();
        self.spi.write(&[cmd]).ok();
        self.cs.set_high().ok();
    }

    fn send_data(&mut self, data: u8) {
        self.dc.set_high().ok();
        self.cs.set_low().ok();
        self.spi.write(&[data]).ok();
        self.cs.set_high().ok();
    }

    pub fn send_cmd_and_data(&mut self, cmd: u8, data: &[u8]) {
        self.send_command(cmd);
        self.dc.set_high().ok();
        self.cs.set_low().ok();
        self.spi.write(data).ok();
        self.cs.set_high().ok();
    }

    pub fn wait_until_idle(&mut self) {
        while let Ok(true) = self.busy.is_high() {
            //忙状态输出引脚（高电平表示忙）
            self.delay.delay_ms(20);
        }
        self.delay.delay_ms(20);
    }

    pub fn sleep(&mut self) {
        self.send_command(0x10);
        self.send_data(0x01);
    }

    pub fn turn_on_display(&mut self) {
        self.send_command(0x20);
        self.wait_until_idle();
    }

    //# Setting the display window
    fn set_windows(&mut self, x: usize, y: usize, w: usize, h: usize) {
        self.send_command(0x44);
        self.send_data(((x >> 3) & 0xff) as u8);
        self.send_data(((w >> 3) & 0xff) as u8);

        self.send_command(0x45);
        self.send_data((y & 0xff) as u8);
        self.send_data(((y >> 8) & 0xff) as u8);
        self.send_data((h & 0xff) as u8);
        self.send_data(((h >> 8) & 0xff) as u8);
    }

    //# Set Cursor
    fn set_cursor(&mut self, x: usize, y: usize) {
        self.send_command(0x4E);
        self.send_data((x & 0xff) as u8);
        self.send_command(0x4F);
        self.send_data((y & 0xff) as u8);
        self.send_data(((y >> 8) & 0xff) as u8);
    }

    pub fn init(&mut self) {
        self.reset();
        self.wait_until_idle();
        self.send_command(0x12);
        self.wait_until_idle();

        self.send_command(0x00);
        self.send_data(0x27);
        self.send_data(0x01);
        self.send_data(0x00);

        self.send_command(0x11);
        self.send_data(0x03);
        self.set_windows(0, 0, WIDTH, HEIGHT);
        self.set_cursor(0, 0);
    }

    pub fn clear_white(&mut self) {
        self.send_command(0x24);
        (0..NUM_DISPLAY_BITS).for_each(|_| {
            self.send_data(0xff);
        });

        self.send_command(0x26);
        (0..NUM_DISPLAY_BITS).for_each(|_| {
            self.send_data(0x00);
        });
        self.turn_on_display()
    }

    pub fn clear_red(&mut self) {
        self.send_command(0x24);
        (0..NUM_DISPLAY_BITS).for_each(|_| {
            self.send_data(0xff);
        });
        self.send_command(0x26);
        (0..NUM_DISPLAY_BITS).for_each(|_| {
            self.send_data(0xff);
        });

        self.turn_on_display();
    }

    pub fn clear_black(&mut self) {
        self.send_command(0x24);
        (0..NUM_DISPLAY_BITS).for_each(|_| {
            self.send_data(0x00);
        });
        self.send_command(0x26);
        (0..NUM_DISPLAY_BITS).for_each(|_| {
            self.send_data(0x00);
        });
        self.turn_on_display();
    }

    pub fn display_all(&mut self, black: &[u8], red: &[u8]) {
        self.send_command(0x24);
        black.iter().for_each(|b| self.send_data(*b));
        self.send_command(0x26);
        red.iter().for_each(|b| self.send_data(*b));
        self.turn_on_display();
    }

    pub fn send_black(&mut self, black: &[u8]) {
        self.send_command(0x24);
        black.iter().for_each(|b| self.send_data(*b));
        // self.turn_on_display();
    }

    pub fn send_red(&mut self, red: &[u8]) {
        self.send_command(0x26);
        red.iter().for_each(|b| self.send_data(*b));
        // self.turn_on_display();
    }

    pub fn display(&mut self) {
        self.turn_on_display();
    }
}
