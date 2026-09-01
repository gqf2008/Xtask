#![no_std]
#![no_main]

extern crate alloc;

// QEMU xlnx-zcu102 Cortex-R5F 执行级验证示例(经典乒乓——调度器/上下文
// 切换/时钟节拍全链路):
// - 任务 A/B:同优先级,各打印一行后 sleep_ms(10)——串口应严格交替
//   (sleep_ms 内部 yield_now → svc 指令 → SVC 入口压帧 → 调度切换);
// - 心跳任务:每秒打印 tick 数——验证 TTC 节拍与 sleep_ms 绝对时基;
// - 跑满 200 次乒乓后 semihosting SYS_EXIT 以 exit 0 退出
//   (CI 门禁无需 timeout 杀进程,退出码即测试结果)。
// 运行(修复版 QEMU,见 book/src/ch29-*.md / README):
//   qemu-system-aarch64 -M xlnx-zcu102 -smp 5 -m 1G -nographic \
//     -global xlnx-zynqmp.boot-cpu=rpu-cpu[0] -semihosting-config enable=on,target=native \
//     -device loader,file=target/armv7r-none-eabi/release/examples/qemu_arm_r52_pingpong

use xtask::chip::qemu_arm_r52::stdout::{putc, qemu_exit_pass, write_str};
use xtask::prelude::*;
use xtask::{sprint, sprintln};

// 覆盖 panic 处理:panic 位置打到串口并 FAIL 退出,而非静默挂死
// (lib 的 panic_probe 对 qemu_arm_r52 被排除——测试类示例各自定义)
// 注意:现场打印(sp/cpsr/lr)不依赖堆分配,先打——location/message 的
// format! 分配可能在"堆已被破坏"的 panic 里再失败(递归空转),原始
// 现场永远是最前一行
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use xtask::chip::qemu_arm_r52::stdout::{putc, qemu_exit_fail};
    let mut sp: usize = 0;
    let mut cpsr: u32 = 0;
    let mut lr: usize = 0;
    let mut r11: usize = 0;
    unsafe {
        core::arch::asm!("mov {0}, sp", out(reg) sp);
        core::arch::asm!("mrs {0}, cpsr", out(reg) cpsr);
        core::arch::asm!("mov {0}, lr", out(reg) lr);
        core::arch::asm!("mov {0}, r11", out(reg) r11);
    }
    const HEX: &[u8; 16] = b"0123456789abcdef";
    write_str("\r\n!!! PANIC sp=");
    for i in (0..8).rev() {
        putc(HEX[((sp as u32 >> (i * 4)) & 0xF) as usize]);
    }
    write_str(" cpsr=");
    for i in (0..8).rev() {
        putc(HEX[((cpsr >> (i * 4)) & 0xF) as usize]);
    }
    write_str(" lr=");
    for i in (0..8).rev() {
        putc(HEX[((lr as u32 >> (i * 4)) & 0xF) as usize]);
    }
    write_str(" r11=");
    for i in (0..8).rev() {
        putc(HEX[((r11 as u32 >> (i * 4)) & 0xF) as usize]);
    }
    write_str("\r\n");
    // 栈上 32 字:IRQ/SVC 调度栈里躺着调用链的返回地址(push {lr})
    write_str("stack:");
    let stack = sp as *const u32;
    for i in 0..32usize {
        if i % 4 == 0 {
            write_str("\r\n  ");
        }
        let v = unsafe { stack.add(i).read_volatile() };
        for j in (0..8).rev() {
            putc(HEX[((v >> (j * 4)) & 0xF) as usize]);
        }
        putc(b' ');
    }
    write_str("\r\n");
    write_str("at ");
    if let Some(l) = info.location() {
        let full = alloc::format!("{}:{}:{}", l.file(), l.line(), l.column());
        for b in full.bytes() {
            putc(b);
        }
    }
    write_str(" msg=");
    for b in alloc::format!("{}", info.message()).bytes() {
        putc(b);
    }
    write_str("\r\n");
    qemu_exit_fail()
}

/// 目标乒乓次数(双方各 200 次)
const ROUNDS: usize = 200;

// R5 无 runtime crate:启动由 chip 口自备(port.S _start),例程只提供
// main(向量表/启动栈/IRQ 栈全在汇编与链接脚本里)
#[no_mangle]
extern "C" fn main() -> ! {
    extern "C" {
        static _sheap: u8;
        static __heap_end: u8; // link.x:堆终点 = IRQ 栈底余量下沿
    }
    let start_addr = unsafe { &_sheap as *const u8 as usize };
    let end_addr = unsafe { &__heap_end as *const u8 as usize };
    sprintln!("_sheap = 0x{:08x} heap_end = 0x{:08x}", start_addr, end_addr);
    // 堆取"栈区以下全部"(自动适配链接布局,绝不分进中断栈区)
    xtask::init_heap(start_addr, end_addr - start_addr);
    sprintln!("heap: used={}KiB free={}KiB", xtask::used_memory() / 1024, xtask::free_memory() / 1024);
    let probe_vec: alloc::vec::Vec<u8> = alloc::vec::Vec::with_capacity(1024);
    sprintln!("alloc probe: Vec(1024) ok");
    drop(probe_vec);
    let probe2: alloc::vec::Vec<usize> = alloc::vec::Vec::with_capacity(257);
    sprintln!("alloc probe: Vec(257usize) ok");
    drop(probe2);
    let s = alloc::string::String::from("A");
    sprintln!("alloc probe: String ok");
    drop(s);
    // 手动复刻 Task::new 的分配步骤,定位 capacity overflow
    let stack: alloc::vec::Vec<usize> = alloc::vec::Vec::with_capacity(257);
    sprintln!("step1: task stack vec ok");
    drop(stack);
    let tname: alloc::string::String = alloc::string::String::from("A");
    sprintln!("step2: task name ok");
    drop(tname);
    let f: alloc::boxed::Box<alloc::boxed::Box<dyn FnOnce() + Send + 'static>> =
        alloc::boxed::Box::new(alloc::boxed::Box::new(|| {}));
    sprintln!("step3: closure box ok");
    drop(f);
    let q: alloc::collections::VecDeque<usize> = alloc::collections::VecDeque::new();
    let mut q = q;
    q.push_back(1);
    sprintln!("step4: queue push ok");
    drop(q);
    xtask::init_logger();
    write_str("qemu_arm_r52_pingpong: scheduler starting\r\n");

    // 乒乓 A/B(同优先级 8,sleep 交替——时间片与 sleep_ms 双验证)
    TaskBuilder::new().name("A").priority(8).spawn(|| {
        unsafe { putc(b'X'); } // DEBUG: 任务体进入探针(不经 println 宏)
        for _ in 0..ROUNDS {
            unsafe { putc(b'a'); } // DEBUG: 每轮裸字符(不经 fmt)
            xtask::sleep_ms(1);
        }
        // 任一方跑满即宣布通过(A 后启动、先到的大概率是 A)
        write_str("\r\nPASS: ping-pong done\r\n");
        qemu_exit_pass();
    });
    TaskBuilder::new().name("B").priority(8).spawn(|| {
        unsafe { putc(b'Y'); } // DEBUG: 任务体进入探针
        for _ in 0..ROUNDS {
            unsafe { putc(b'b'); } // DEBUG: 每轮裸字符
            xtask::sleep_ms(1);
        }
    });
    // 心跳:验证 sleep_ms 的绝对时基(200 轮 × 10ms ≈ 2s,应见 ~2 次/秒心跳)
    TaskBuilder::new().name("tick").priority(9).spawn(|| loop {
        sprintln!("tick {}", xtask::tick());
        xtask::sleep_ms(1000);
    });

    xtask::start()
}
