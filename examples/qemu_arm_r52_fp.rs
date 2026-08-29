#![no_std]
#![no_main]

extern crate alloc;

// QEMU xlnx-zcu102 Cortex-R5F VFP 帧验证例程:
// 两个任务各持独有 FPU 状态(任务 A:s3=3.0;任务 B:s3=9.0),经
// sleep 频繁切换——若 VFP 帧保存/恢复正确,各自醒来后 s3 保持不变;
// 否则互串(任务读到对方的值)或清零,立即 FAIL 退出。
// 这是 ch32 §32.5「FPU 帧」的实现验证:49 字帧 = 现场 16 字 +
// VFP 33 字(D0-D15 + FPSCR),FPEXC.EN 在 _start 置位。

use xtask::chip::qemu_arm_r52::stdout::{qemu_exit_fail, qemu_exit_pass, write_str};
use xtask::prelude::*;
use xtask::sprintln;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use xtask::chip::qemu_arm_r52::stdout::{putc, write_str};
    write_str("\r\n!!! PANIC ");
    write_str(alloc::format!("{}", info.message()).as_str());
    write_str("\r\n");
    qemu_exit_fail()
}

#[no_mangle]
extern "C" fn main() -> ! {
    extern "C" {
        static _sheap: u8;
        static __heap_end: u8;
    }
    let start_addr = unsafe { &_sheap as *const u8 as usize };
    let end_addr = unsafe { &__heap_end as *const u8 as usize };
    xtask::init_heap(start_addr, end_addr - start_addr);
    xtask::init_logger();
    write_str("qemu_arm_r52_fp: VFP frame test starting\r\n");

    const A_EXPECT: u32 = 0x4040_0000; // 3.0f32
    const B_EXPECT: u32 = 0x4110_0000; // 9.0f32

    TaskBuilder::new().name("fpA").priority(8).spawn(move || {
        unsafe {
            core::arch::asm!(
                "vmov.f32 s0, #1.0",
                "vmov.f32 s1, #2.0",
                "vadd.f32 s3, s0, s1", // s3 = 3.0
                options(nostack)
            );
        }
        for round in 0..100 {
            xtask::sleep_ms(3);
            let v: u32;
            unsafe {
                core::arch::asm!("vmov {0}, s3", out(reg) v, options(nostack));
            }
            if v != A_EXPECT {
                sprintln!("fpA round {} BAD s3={:#010x} expect {:#010x}", round, v, A_EXPECT);
                qemu_exit_fail();
            }
        }
        sprintln!("fpA: 100 rounds s3 kept = {:#010x}", A_EXPECT);
        qemu_exit_pass();
    });

    TaskBuilder::new().name("fpB").priority(8).spawn(move || {
        unsafe {
            core::arch::asm!(
                "vmov.f32 s0, #7.0",
                "vmov.f32 s1, #2.0",
                "vadd.f32 s3, s0, s1", // s3 = 9.0
                options(nostack)
            );
        }
        loop {
            xtask::sleep_ms(3);
            let v: u32;
            unsafe {
                core::arch::asm!("vmov {0}, s3", out(reg) v, options(nostack));
            }
            if v != B_EXPECT {
                sprintln!("fpB BAD s3={:#010x} expect {:#010x}", v, B_EXPECT);
                qemu_exit_fail();
            }
        }
    });

    xtask::start()
}
