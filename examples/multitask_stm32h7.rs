#![no_std]
#![no_main]
extern crate alloc;

use cortex_m_rt::{entry, exception, ExceptionFrame};
use stm32h7xx_hal::{hal, pac, prelude::*};
use xtask::arch::cortex_m::rt;
use xtask::prelude::*;

#[exception]
unsafe fn DefaultHandler(irqn: i16) {
    log::info!("irqn: {:?}", irqn);
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    log::info!("{:?}", ef);
    loop {}
}

fn init() {
    if let Some(dp) = pac::Peripherals::take() {
        log::info!("Setup PWR...                  ");
        let pwr = dp.PWR.constrain();
        let pwrcfg = pwr.freeze();

        // Constrain and Freeze clock
        log::info!("Setup RCC...                  ");
        let rcc = dp.RCC.constrain();

        let ccdr = rcc
            .use_hse(25.MHz())
            .sys_ck(48.MHz())
            .sysclk(24.MHz())
            .freeze(pwrcfg, &dp.SYSCFG);
        let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
        let led = Led::new(gpioc.pc1);
        log::info!("clocks {}", ccdr.clocks.sysclk());
        //使用AXISRAM这块连续块作为堆内存
        let start_addr = 0x24000000;
        log::info!("Start Address 0x{:02x}", start_addr);
        //4k留给主栈
        xtask::init_heap(start_addr, 512 * 1024);
        log::info!("Blackpill initialize ok");
        example_led(led);
    }
}

#[rt::entry]
fn main() -> ! {
    xtask::init_logger();
    init();

    //启动多任务
    example_task();
    //启动调度器
    xtask::start()
}

fn example_task() {
    xtask::spawn(|| {
        for i in 0..10 {
            log::info!("{} 循环测试任务0", i + 1);
            xtask::sleep_ms(1000);
        }
    });
    xtask::spawn(|| {
        for i in 0..50 {
            log::info!("{} 循环测试任务1", i + 1);
            xtask::sleep_ms(1000);
        }
    });

    xtask::spawn(|| {
        for i in 0..100 {
            log::info!("{} 循环测试任务2", i + 1);
            xtask::sleep_ms(1000);
        }
    });

    xtask::spawn(|| {
        for i in 0..500 {
            log::info!("{} 循环测试任务4", i + 1);
            xtask::sleep_ms(1000);
        }
    });

    xtask::spawn(|| loop {
        log::info!("死循环测试任务 {}", tick());
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

use stm32h7xx_hal::gpio::{Output, Pin, PushPull};

pub struct Led {
    port: Pin<'C', 1, Output<PushPull>>,
}

impl Led {
    pub fn new(port: Pin<'C', 1>) -> Self {
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

    pub fn toggle(&mut self) {
        self.port.toggle();
    }
}
