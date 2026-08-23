#![no_std]
#![no_main]

extern crate alloc;

// QEMU 执行级验证示例(经典点:"两个任务轮流打印 + 串口输出稳定 = 
// 调度器/上下文切换/时钟节拍都对了"):
// - 任务 A/B:同优先级,各打印一行后 sleep_ms(10)——串口应严格交替;
// - 心跳任务:每秒打印 tick 数——验证 sleep_ms 的绝对时基;
// - 跑满 200 次乒乓后写 SiFive test 设备 0x5555,QEMU 以 exit 0 退出
//   (CI 门禁无需 timeout 杀进程,退出码即测试结果)。
// 运行:qemu-system-riscv32 -M virt -nographic -bios none -kernel \
//       target/riscv32imac-unknown-none-elf/release/examples/qemu_pingpong

use xtask::arch::riscv::rt;
use xtask::chip::qemu_riscv::stdout::{qemu_exit_pass, write_str};
use xtask::{sprint, sprintln};
use xtask::prelude::*;

/// 目标乒乓次数(双方各 200 次)
const ROUNDS: usize = 200;

#[rt::entry]
fn main() -> ! {
    extern "C" {
        static _sheap: u8;
    }
    let start_addr = unsafe { &_sheap as *const u8 as usize };
    xtask::init_heap(start_addr, 1 * 1024 * 1024); // virt 机 128M DRAM,取 1M 堆
    xtask::init_logger();
    write_str("qemu_pingpong: scheduler starting\r\n");

    // 乒乓 A/B(同优先级 8,sleep 交替——时间片与 sleep_ms 双验证)
    TaskBuilder::new().name("A").priority(8).spawn(|| {
        for _ in 0..ROUNDS {
            sprintln!("A");
            xtask::sleep_ms(10);
        }
        // 任一方跑满即宣布通过(A 后启动、先到的大概率是 A)
        sprintln!("PASS: {} rounds ping-pong", ROUNDS);
        qemu_exit_pass();
    });
    TaskBuilder::new().name("B").priority(8).spawn(|| {
        for _ in 0..ROUNDS {
            sprintln!("B");
            xtask::sleep_ms(10);
        }
    });
    // 心跳:验证 sleep_ms 的绝对时基(200 轮 × 10ms ≈ 2s,应见 ~2 次/秒心跳)
    TaskBuilder::new().name("tick").priority(9).spawn(|| loop {
        sprintln!("tick {}", xtask::tick());
        xtask::sleep_ms(1000);
    });

    xtask::start()
}
