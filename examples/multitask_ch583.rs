#![no_std]
#![no_main]

extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

// CH583(QingKe V4A,RISC-V RV32IMAC softfp)多任务示例 —— 以 ch32v103 为模板。
// ⚠️ 真机核对点(构建级验证已过,板上行为待验):①HPE 关闭路径
// (intsyscr 0x804 写 0 + 私有 CSR 0xbc0=0x1f);②mtvec Direct 分发与
// mcause=12(SysTick);③SysTick CTLR=0x2F(INIT|STRE|STCLK|STIE|STE,
// **不含 SWIE**——与官方 SysTick_Config 同值) + SR.CNTIF 写 0 清零;
// ④yield 走 **`SWI_IRQn=14`**(置 PFIC `IPSR` pending,官方 FreeRTOS/
// RT-Thread/HarmonyOS 三套移植同款;`CTLR.SWIE` 已不使用)后,是否即刻进 trap
// 并沿 mcause=14 完成切换;
// ⑤flash 基址 0x00000000(尾部 64K 留给 BootLoader);⑥默认主频 60MHz
// (env 按此配);⑦`mcycle` 是否实现(决定 `delay_us`,未实现则 `delay_us`
// 会死循环——见 chip/ch583/mod.rs 注);⑧栈账:12 任务+timer+idle 实测
// ≈20.9K 堆(见 main 注释,别再往上加任务)。
//
// TODO(CH583 外设,待上板核对后补):
//   - GPIO:R32_PA_DIR/R32_PA_OUT @0x400010A0..(与 CH572 窗口大小不同),
//           PA 引脚复用 R32_PA_PIN/R32_PA_OUT 一带。
//   - UART:UART1 @0x40003400(与 ch32v103 的 USART1 不同址,4 个 UART 可选)。
//   - 日志后端:RTT/stdout 均未接(当前纯任务演示,log 走内核默认后端)。
// 本示例目标是「构建级通过」,外设定义留待 BSP 层。

use xtask::arch::riscv::rt;
use xtask::prelude::*;

#[rt::entry]
fn main() -> ! {
    extern "C" {
        /// 堆内存开始地址，在riscv-rt link.x文件里定义
        static _sheap: u8;
    }
    let start_addr = unsafe { &_sheap as *const u8 as usize };
    // 32K SRAM:静态(data 4.1K + bss 0.3K)后到 _stack_start 还有 ~27.6K。
    // 任务栈全部从堆上分配,实测(QEMU 探针:同一 32 位分配器/同一 Task 布局,
    // 方法见 PR #15 评论——WCH 侧无法仿真,只能这样取数):
    //   默认任务(256 字栈 + Task 112B + 名字)≈ 1.19K
    //   软件定时器任务(1024 字,timer 特征)≈ 4.4K
    //   idle 任务(512 字)≈ 2.2K
    // 12 个示例任务 + timer + idle ≈ 20.9K,故取 24K(堆顶 0x20007188,
    // 距中断栈基 0x20007E00 还有 3.2K,首次上板余量约 3.6K)。
    // ⚠️ 不要再往本示例加任务:15 个任务时实测需 24.4K,24K 堆会在 start()
    // 里 `panic!("memory out")`——会被误判成移植层的寄存器问题。
    xtask::init_heap(start_addr, 24 * 1024);

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
    // 只保留 2 个(原 ch32v103 示例是 5 个):32K SRAM 上 12 个示例任务 +
    // timer + idle 实测 20.9K,再加任务会在 24K 堆上 OOM(见 main 里的实测注)
    xtask::spawn(|| {
        for i in 0..10 {
            log::info!("{} 循环测试任务0", i + 1);
            xtask::sleep_ms(1000);
        }
    });
    xtask::spawn(|| loop {
        log::info!("死循环测试任务 {}", tick());
        xtask::sleep_ms(10000);
    });
}
