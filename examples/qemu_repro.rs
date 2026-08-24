#![no_std]
#![no_main]

extern crate alloc;

// 调度器野跳回归:循环跑 12 项套件中崩溃窗口的负载(test1 优先级抢占 +
// test2 时间片),曾在 ~2-4 轮内必现整机"重启"(根因:RESTORE_CONTEXT
// 的 CSR 装载覆盖 t0,见 qemu_riscv/port.S;此例程用于防回归——若再入
// 探测器打印 RE-ENTERED 即红)。正常行为:无限打印轮次直到被杀。

use alloc::sync::Arc as XArc;

use xtask::arch::riscv::rt;
use xtask::chip::qemu_riscv::stdout::{qemu_exit_fail, write_str};
use xtask::prelude::*;
use xtask::sync::mutex::Mutex;

// 初值非零 → 落 .data,_start 重跑(bss 重清零)后幸存——再次进 main
// 即证明发生了野跳进启动代码(节拍无关:上电只写一次)
#[unsafe(no_mangle)]
static mut MAIN_MAGIC: u32 = 0xA5A5_0001;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

fn test_prio_preempt() {
    let done = XArc::new(Semaphore::new());
    let done_w = done.clone();
    TaskBuilder::new().name("t1.low").priority(10).spawn(move || {
        TaskBuilder::new().name("t1.high").priority(2).spawn(move || {});
        done_w.post();
    });
    done.wait();
}

fn test_time_slice() {
    static CNT: Mutex<(u64, u64)> = Mutex::new((0, 0));
    let done = XArc::new(Semaphore::new());
    let done_w = done.clone();
    TaskBuilder::new().name("t2.a").priority(8).stack_size(1024).spawn(move || {
        let t0 = xtask::tick();
        while xtask::tick() - t0 < 50 {
            CNT.lock().0 += 1;
        }
    });
    TaskBuilder::new().name("t2.b").priority(8).stack_size(1024).spawn(move || {
        let t0 = xtask::tick();
        while xtask::tick() - t0 < 50 {
            CNT.lock().1 += 1;
        }
        done_w.post();
    });
    done.wait();
}

#[rt::entry]
fn main() -> ! {
    unsafe {
        if core::ptr::addr_of!(MAIN_MAGIC).read_volatile() == 0xDEAD_BEEF {
            write_str("\r\n!!! RE-ENTERED — 调度器野跳回归!\r\n");
            qemu_exit_fail();
        }
        core::ptr::addr_of_mut!(MAIN_MAGIC).write_volatile(0xDEAD_BEEF);
    }
    extern "C" {
        static _sheap: u8;
    }
    let start_addr = unsafe { &_sheap as *const u8 as usize };
    xtask::init_heap(start_addr, 1 * 1024 * 1024);
    xtask::init_logger();

    TaskBuilder::new()
        .name("examiner")
        .priority(1)
        .stack_size(4096)
        .spawn(|| {
            let mut round: u32 = 0;
            loop {
                write_str("R\r\n");
                test_prio_preempt();
                write_str("r\r\n");
                test_time_slice();
                round = round.wrapping_add(1);
            }
        });

    xtask::start()
}
