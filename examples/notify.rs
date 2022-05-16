#![no_std]
#![no_main]
extern crate alloc;

use gd32vf103xx_hal as hal;
use hal::{gpio::GpioExt, pac, prelude::*, rcu::RcuExt, signature};
use xtask::bsp::longan_nano::led::BLUE;
use xtask::bsp::longan_nano::led::GREEN;
use xtask::bsp::longan_nano::led::RED;

use panic_halt as _;
use xtask::arch::riscv::rt;
use xtask::bsp::longan_nano::led::{rgb, Led};
use xtask::prelude::*;

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

    //启动通知任务
    example_notify();
    //启动调度器
    xtask::start()
}

fn example_notify() {
    let notifier = Notifier::new();
    let waiter = notifier.clone();
    TaskBuilder::new().name("notifier").spawn(move || loop {
        sprintln!("发送通知信号 {}", xtask::tick());
        notifier.notify();
        xtask::sleep_ms(1000);
    });
    TaskBuilder::new().name("waiter").spawn(move || loop {
        waiter.wait();
        sprintln!("收到通知信号 {}", xtask::tick());
    });
}

fn example_led(mut red: RED, mut green: GREEN, mut blue: BLUE) {
    TaskBuilder::new()
        .name("green")
        .priority(1)
        .spawn(move || loop {
            green.on();
            xtask::sleep_ms(500);
            green.off();
            xtask::sleep_ms(500);
        });

    TaskBuilder::new()
        .name("red")
        .priority(1)
        .spawn(move || loop {
            red.on();
            xtask::sleep_ms(500);
            red.off();
            xtask::sleep_ms(500);
        });

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
