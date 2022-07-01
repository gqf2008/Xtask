#![no_std]
#![no_main]

extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use xtask::arch::riscv::rt;
use xtask::prelude::*;

#[rt::entry]
fn main() -> ! {
    extern "C" {
        /// 堆内存开始地址，在riscv-rt link.x文件里定义
        static _sheap: u8;
    }
    let start_addr = unsafe { &_sheap as *const u8 as usize };
    xtask::init(start_addr, 128 * 1024);

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
        log::info!("发送通知信号 {}", xtask::tick());
        notifier.notify();
        xtask::sleep_ms(1000);
    });
    TaskBuilder::new().name("waiter").spawn(move || loop {
        waiter.wait();
        log::info!("收到通知信号 {}", xtask::tick());
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
        log::info!("发送广播信号 {}", xtask::tick());
        caster.notify();
        xtask::sleep_ms(1000);
    });
    TaskBuilder::new().name("listener1").spawn(move || loop {
        waiter1.wait();
        log::info!("1收到广播信号 {}", xtask::tick());
    });
    TaskBuilder::new().name("listener1").spawn(move || loop {
        waiter2.wait();
        log::info!("2收到广播信号 {}", xtask::tick());
    });
    TaskBuilder::new().name("listener1").spawn(move || loop {
        waiter3.wait();
        log::info!("3收到广播信号 {}", xtask::tick());
    });
    TaskBuilder::new().name("listener1").spawn(move || loop {
        waiter4.wait();
        log::info!("4收到广播信号 {}", xtask::tick());
    });
    TaskBuilder::new().name("listener1").spawn(move || loop {
        waiter5.wait();
        log::info!("5收到广播信号 {}", xtask::tick());
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
            log::info!("收到消息1 {:?}", msg);
        }
    });
    TaskBuilder::new().name("queue.recv2").spawn(move || loop {
        if let Some(msg) = qrecv2.pop_front() {
            log::info!("收到消息2 {:?}", msg);
        }
    });
    TaskBuilder::new().name("queue.recv3").spawn(move || loop {
        if let Some(msg) = qrecv3.pop_front() {
            log::info!("收到消息3 {:?}", msg);
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
            //log::info!("发送计数信号");
            sender.post();
            xtask::sleep_ms(100);
        });
    TaskBuilder::new()
        .name("semaphore.poster2")
        .spawn(move || loop {
            //log::info!("发送计数信号");
            sender2.post();
            xtask::sleep_ms(100);
        });

    TaskBuilder::new()
        .name("semaphore.waiter1")
        .spawn(move || loop {
            recver.wait();
            log::info!("收到计数信号");
        });
    TaskBuilder::new()
        .name("semaphore.waiter2")
        .spawn(move || loop {
            recver2.wait();
            log::info!("收到计数信号");
        });
    TaskBuilder::new()
        .name("semaphore.waiter3")
        .spawn(move || loop {
            recver3.wait();
            log::info!("收到计数信号");
        });
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
        xtask::sleep_ms(10000);
    });
}
