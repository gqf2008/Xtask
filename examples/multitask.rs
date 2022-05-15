#![no_std]
#![no_main]
extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::arch::asm;
use core::arch::global_asm;
use gd32vf103xx_hal as hal;
use hal::{
    backup_domain::BkpExt,
    eclic::{EclicExt, Level, LevelPriorityBits, Priority, TriggerType},
    gpio::GpioExt,
    pac,
    prelude::*,
    rcu::RcuExt,
    rtc::Rtc,
    signature,
    timer::{Event, Timer},
};
use xtask::bsp::longan_nano::led::BLUE;
use xtask::bsp::longan_nano::led::GREEN;
use xtask::bsp::longan_nano::led::RED;
use xtask::sync::broadcast::Broadcast;
use xtask::sync::queue::Queue;

use pac::interrupt::Nr;
use pac::{interrupt, Interrupt, CTIMER, ECLIC, RTC};
use panic_halt as _;
use riscv_rt as rt;
use xtask::bsp::longan_nano::led::{rgb, Led};
use xtask::prelude::*;
use xtask::sprintln;
use xtask::sync::notifier::Notifier;
use xtask::sync::semaphore::Semaphore;

fn init() {
    extern "C" {
        /// 堆内存开始地址，在riscv-rt link.x文件里定义
        static _sheap: u8;
        //static _heap_size: u8; //默认为2k
    }
    let start_addr = unsafe { &_sheap as *const u8 as usize };
    xtask::init_heap(start_addr, 32 * 1024);

    let dp = pac::Peripherals::take().unwrap();
    // 配置时钟
    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();

    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpioc = dp.GPIOC.split(&mut rcu);

    let (mut red, mut green, mut blue) = rgb(gpioc.pc13, gpioa.pa1, gpioa.pa2);
    red.off();
    green.off();
    blue.off();

    let mut afio = dp.AFIO.constrain(&mut rcu);

    xtask::chip::gd32vf103::stdout::configure(
        dp.USART0,
        gpioa.pa9,
        gpioa.pa10,
        57600.bps(), // 这块板子PCB设计上可能有瑕疵，uart速率只能到57600，再大收到的全是乱码
        &mut afio,
        &mut rcu,
    );
    sprintln!(
        "Starting [debug_id={:#08X}, flash_size: {}KB, sram_size={}KB]",
        dp.DBG.id.read().bits(),
        signature::flash_size_kb(),
        signature::sram_size_kb(),
    );
    //三个led
    example_led(red, green, blue);
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
        xtask::sleep_ms(10000);
    });
}

fn example_led(mut red: RED, mut green: GREEN, mut blue: BLUE) {
    xtask::spawn4("green", 256, 1, move || loop {
        green.on();
        xtask::sleep_ms(500);
        green.off();
        xtask::sleep_ms(500);
    });

    xtask::spawn4("red", 256, 1, move || loop {
        red.on();
        xtask::sleep_ms(500);
        red.off();
        xtask::sleep_ms(500);
    });

    xtask::spawn4("blue", 256, 1, move || loop {
        blue.on();
        xtask::sleep_ms(500);
        blue.off();
        xtask::sleep_ms(500);
    });
}
