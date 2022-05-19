#![no_std]
#![no_main]
extern crate alloc;

use panic_halt as _;

use xtask::arch::cortex_m::rt;
use xtask::bsp::greenpill;
use xtask::bsp::greenpill::hal::prelude::*;
use xtask::bsp::greenpill::led::Led;
use xtask::chip::stm32f4;
use xtask::prelude::*;

fn init() {
    let start_addr = rt::heap_start() as usize;
    xtask::init_heap(start_addr, 32 * 1024);

    if let Some((_cp, dp)) = greenpill::take() {
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.use_hse(25.MHz()).sysclk(84.MHz()).freeze();
        let gpioa = dp.GPIOA.split();
        let gpioc = dp.GPIOC.split();
        let led = Led::new(gpioc.pc13);
        let tx = dp
            .USART1
            .tx(gpioa.pa9.into_alternate(), 115200.bps(), &clocks)
            .unwrap();
        stm32f4::stdout::use_tx1(tx);
        sprintln!("Greenpill initialize ok");
        example_led(led);
    }
}

#[rt::entry]
fn main() -> ! {
    init();

    //启动多任务
    // example_task();
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

fn example_led(mut blue: Led) {
    TaskBuilder::new()
        .name("blue")
        .priority(1)
        .spawn(move || loop {
            sprintln!("example_led");
            blue.on();
            xtask::sleep_ms(500);
            blue.off();
            xtask::sleep_ms(500);
        });
}
