#![no_std]
#![no_main]

extern crate alloc;

use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin, ToggleableOutputPin};
use xtask::arch::cortex_m::rt;
use xtask::bsp::rp_pico;
use xtask::bsp::rp_pico::hal;
use xtask::bsp::rp_pico::hal::pac;
use xtask::bsp::rp_pico::hal::prelude::*;
use xtask::bsp::rp_pico::led::Led;
use xtask::bsp::rp_pico::stdout;
use xtask::isr_sprintln;
use xtask::prelude::*;

pub fn stack_start() -> usize {
    extern "C" {
        static mut _stack_start: u32;
    }

    unsafe { &mut _stack_start as *mut u32 as usize }
}
fn init() {
    let start_addr = rt::heap_start() as usize;
    //2k留给主栈
    xtask::init_heap(start_addr, 128 * 1024);
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    let sio = hal::Sio::new(pac.SIO);
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let led_pin = pins.led.into_push_pull_output();
    let led = Led::new(led_pin);

    let uart_pins = (
        pins.gpio0.into_mode::<hal::gpio::FunctionUart>(),
        pins.gpio1.into_mode::<hal::gpio::FunctionUart>(),
    );
    let uart = hal::uart::UartPeripheral::new(pac.UART0, uart_pins, &mut pac.RESETS)
        .enable(
            hal::uart::common_configs::_9600_8_N_1,
            clocks.peripheral_clock.into(),
        )
        .unwrap();
    stdout::use_uart0(uart);
    let msp = cortex_m::register::msp::read();
    let psp = cortex_m::register::psp::read();
    sprintln!(
        "stack_start {:#08x} msp {:#08x} psp {:#08x}",
        stack_start(),
        msp,
        psp
    );
    example_led(led);
}

#[rt::entry]
fn main() -> ! {
    init();
    //启动多任务
    example_task();
    //启动调度器
    xtask::start()
}

fn example_task() {
    xtask::spawn(|| {
        for i in 0..10 {
            sprintln!("{} 循环测试任务0", i + 1);
            xtask::sleep_ms(1000);
        }
    });
    xtask::spawn(|| {
        for i in 0..50 {
            sprintln!("{} 循环测试任务1", i + 1);
            xtask::sleep_ms(1000);
        }
    });

    xtask::spawn(|| {
        for i in 0..100 {
            sprintln!("{} 循环测试任务2", i + 1);
            xtask::sleep_ms(1000);
        }
    });

    xtask::spawn(|| {
        for i in 0..500 {
            sprintln!("{} 循环测试任务4", i + 1);
            xtask::sleep_ms(1000);
        }
    });

    xtask::spawn(|| {
        for i in 0..u64::MAX {
            sprintln!("死循环测试任务 {} {}", tick(), i);
            xtask::sleep_ms(1000);
        }
    });
}

fn example_led(mut blue: Led) {
    xtask::spawn(move || loop {
        blue.on();
        xtask::sleep_ms(1000);
        blue.off();
        xtask::sleep_ms(1000);
    });
}
