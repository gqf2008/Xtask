#![no_std]
#![no_main]
extern crate alloc;

use gd32vf103xx_hal as hal;
use hal::{gpio::GpioExt, pac, prelude::*, rcu::RcuExt, signature};
use xtask::bsp::longan_nano::led::BLUE;
use xtask::bsp::longan_nano::led::GREEN;
use xtask::bsp::longan_nano::led::RED;

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
    xtask::init(start_addr, 32 * 1024);

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

    xtask::bsp::longan_nano::stdout::configure(
        dp.USART0,
        gpioa.pa9,
        gpioa.pa10,
        57600.bps(), // 这块板子PCB设计上可能有瑕疵，uart速率只能到57600，再大收到的全是乱码
        &mut afio,
        &mut rcu,
    );
    log::info!(
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
    //初始化外设&内存
    init();
    //bus
    example_bus();
    //启动调度器
    xtask::start()
}

fn example_bus() {
    static BUS: Bus<Event> = Bus::new();
    #[derive(Debug, Clone, Copy)]
    enum Event {
        Key(u8),
        Mouse(isize, isize),
    }
    let kqueue = Queue::new();
    let krecv = kqueue.clone();
    let mqueue = Queue::new();
    let mrecv = mqueue.clone();
    BUS.subscribe("ev.key", move |topic, ev| match ev {
        Event::Key(code) => {
            //log::info!("{} {:?}", topic, ev);
            //kqueue.push_back_isr(ev);
            kqueue.push_back(ev);
        }
        _ => {}
    });
    BUS.subscribe("ev.mouse", move |topic, ev| match ev {
        Event::Mouse(x, y) => {
            //log::info!("{} {:?}", topic, ev);
            //mqueue.push_back_isr(ev);
            mqueue.push_back(ev);
        }
        _ => {}
    });

    /// 模拟两个中断服务程序
    TaskBuilder::new().name("isr1").spawn(move || loop {
        BUS.publish("ev.key", Event::Key(8));
        xtask::sleep_ms(1000);
    });
    TaskBuilder::new().name("isr2").spawn(move || loop {
        BUS.publish("ev.mouse", Event::Mouse(8378, 10036));
        xtask::sleep_ms(100);
    });

    /// 两个从中断服务接收数据的服务
    TaskBuilder::new().name("key.service").spawn(move || loop {
        if let Some(msg) = krecv.pop_front() {
            log::info!("收到消息key {:?}", msg);
        }
    });
    TaskBuilder::new()
        .name("mouse.service")
        .spawn(move || loop {
            if let Some(msg) = mrecv.pop_front() {
                log::info!("收到消息mouse {:?}", msg);
            }
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
