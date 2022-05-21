#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};
use cortex_m_semihosting::*;
use embedded_hal::digital::v2::{OutputPin, StatefulOutputPin, ToggleableOutputPin};
use xtask::arch::cortex_m::rt;
use xtask::bsp::bluepill;
use xtask::bsp::bluepill::hal::pac;
use xtask::bsp::bluepill::hal::prelude::*;
use xtask::bsp::bluepill::hal::serial;
use xtask::bsp::bluepill::led::Led;
use xtask::bsp::bluepill::stdout;
use xtask::prelude::*;

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    heprintln!("{:?}", info);
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

fn init() {
    let start_addr = rt::heap_start() as usize;
    xtask::init_heap(start_addr, 20 * 1024);
    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = dp.AFIO.constrain();
    let mut gpioc = dp.GPIOC.split();
    let mut gpioa = dp.GPIOA.split();

    let led_pin = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let led = Led::new(led_pin);
    let tx_pin = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx_pin = gpioa.pa10;

    let (tx, _rx) = serial::Serial::usart1(
        dp.USART1,
        (tx_pin, rx_pin),
        &mut afio.mapr,
        serial::Config::default().baudrate(115200.bps()),
        clocks,
    )
    .split();
    stdout::use_tx1(tx);
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

    xtask::spawn(|| loop {
        sprintln!("死循环测试任务 {}", tick());
        xtask::sleep_ms(1000);
    });
}

fn example_led(mut blue: Led) {
    TaskBuilder::new()
        .name("blue")
        .priority(1)
        .spawn(move || loop {
            blue.on();
            xtask::sleep_ms(500);
            blue.off();
            xtask::sleep_ms(500);
        });
}
