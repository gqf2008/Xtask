#![no_std]
#![no_main]
extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use gd32vf103xx_hal as hal;
use hal::{gpio::GpioExt, pac, prelude::*, rcu::RcuExt, signature};
use xtask::bsp::longan_nano::led::BLUE;
use xtask::bsp::longan_nano::led::GREEN;
use xtask::bsp::longan_nano::led::RED;

use xtask::arch::riscv::rt;
use xtask::bsp::longan_nano::led::{rgb, Led};
use xtask::prelude::*;

#[rt::entry]
fn main() -> ! {
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

    sprintln!(
        "Starting [debug_id={:#08X}, flash_size: {}KB, sram_size={}KB]",
        dp.DBG.id.read().bits(),
        signature::flash_size_kb(),
        signature::sram_size_kb(),
    );

    /// 所有任务跑起来大概还是剩下2k内存，如果发现运行失败多少是内存不够了，可以调整下内存栈大小
    example_led(red, green, blue);
    //example_notify();
    //example_broadcast();
    example_semaphore();
    example_queue();
    example_task();
    xtask::start()
}

fn example_notify() {
    let notifier = Notifier::new();
    let waiter = notifier.clone();
    TaskBuilder::new().name("notifier").spawn(move || loop {
        //  sprintln!("发送通知信号 {}", xtask::tick());
        notifier.notify();
        xtask::sleep_ms(1000);
    });
    TaskBuilder::new().name("waiter").spawn(move || loop {
        waiter.wait();
        // sprintln!("收到通知信号 {}", xtask::tick());
    });
}

fn example_broadcast() {
    let caster = Broadcast::new();
    let waiter1 = caster.clone();
    let waiter2 = caster.clone();
    let waiter3 = caster.clone();
    let waiter4 = caster.clone();
    let waiter5 = caster.clone();

    TaskBuilder::new().name("caster").spawn(move || loop {
        // sprintln!("发送广播信号 {}", xtask::tick());
        caster.notify();
        xtask::sleep_ms(1000);
    });
    TaskBuilder::new().name("listener1").spawn(move || loop {
        waiter1.wait();
        // sprintln!("1收到广播信号 {}", xtask::tick());
    });
    TaskBuilder::new().name("listener1").spawn(move || loop {
        waiter2.wait();
        //  sprintln!("2收到广播信号 {}", xtask::tick());
    });
    TaskBuilder::new().name("listener1").spawn(move || loop {
        waiter3.wait();
        // sprintln!("3收到广播信号 {}", xtask::tick());
    });
    TaskBuilder::new().name("listener1").spawn(move || loop {
        waiter4.wait();
        // sprintln!("4收到广播信号 {}", xtask::tick());
    });
    TaskBuilder::new().name("listener1").spawn(move || loop {
        waiter5.wait();
        // sprintln!("5收到广播信号 {}", xtask::tick());
    });
}
fn example_queue() {
    #[derive(Debug, Clone)]
    struct Message {
        id: u64,
        msg: String,
        data: Vec<u8>,
    }
    let qsender = Queue::new();
    let qsender2 = qsender.clone();
    let qrecv = qsender.clone();
    let qrecv2 = qsender.clone();
    let qrecv3 = qsender.clone();
    TaskBuilder::new().name("queue.sender1").spawn(move || {
        let mut id = 0;
        loop {
            id += 1;
            let msg = Message {
                id,
                msg: format!("这是一条消息1 {}", xtask::tick()),
                data: Vec::new(),
            };
            qsender.push_back(msg);
            xtask::sleep_ms(100);
        }
    });
    TaskBuilder::new().name("queue.sender2").spawn(move || {
        let mut id = 0;
        loop {
            id += 1;
            let msg = Message {
                id,
                msg: format!("这是一条消息2 {}", xtask::tick()),
                data: Vec::new(),
            };
            qsender2.push_back(msg);
            xtask::sleep_ms(100);
        }
    });
    TaskBuilder::new().name("queue.recv1").spawn(move || loop {
        if let Some(msg) = qrecv.pop_front() {
            // sprintln!("收到消息1 {:?}", msg);
        }
    });
    TaskBuilder::new().name("queue.recv2").spawn(move || loop {
        if let Some(msg) = qrecv2.pop_front() {
            // sprintln!("收到消息2 {:?}", msg);
        }
    });
    TaskBuilder::new().name("queue.recv3").spawn(move || loop {
        if let Some(msg) = qrecv3.pop_front() {
            // sprintln!("收到消息3 {:?}", msg);
        }
    });
}

fn example_semaphore() {
    let sender = Semaphore::new();
    let sender2 = sender.clone();
    let recver = sender.clone();
    let recver2 = sender.clone();
    let recver3 = sender.clone();
    TaskBuilder::new()
        .name("semaphore.poster1")
        .spawn(move || loop {
            //  sprintln!("发送计数信号");
            sender.post();
            xtask::sleep_ms(100);
        });
    TaskBuilder::new()
        .name("semaphore.poster2")
        .spawn(move || loop {
            //sprintln!("发送计数信号");
            sender2.post();
            xtask::sleep_ms(100);
        });

    TaskBuilder::new()
        .name("semaphore.waiter1")
        .spawn(move || loop {
            recver.wait();
            //sprintln!("收到计数信号");
        });
    TaskBuilder::new()
        .name("semaphore.waiter2")
        .spawn(move || loop {
            recver2.wait();
            //sprintln!("收到计数信号");
        });
    TaskBuilder::new()
        .name("semaphore.waiter3")
        .spawn(move || loop {
            recver3.wait();
            //sprintln!("收到计数信号");
        });
}

fn example_task() {
    xtask::spawn(|| {
        for i in 0..10 {
            //  sprintln!("{} 循环测试任务0", i + 1);
            xtask::sleep_ms(1000);
        }
    });
    xtask::spawn(|| {
        for i in 0..50 {
            //  sprintln!("{} 循环测试任务1", i + 1);
            xtask::sleep_ms(1000);
        }
    });

    xtask::spawn(|| {
        for i in 0..100 {
            // sprintln!("{} 循环测试任务2", i + 1);
            xtask::sleep_ms(1000);
        }
    });

    xtask::spawn(|| {
        for i in 0..500 {
            //   sprintln!("{} 循环测试任务4", i + 1);
            xtask::sleep_ms(1000);
        }
    });

    xtask::spawn(|| loop {
        // sprintln!("死循环测试任务 {}", tick());
        xtask::sleep_ms(10000);
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
