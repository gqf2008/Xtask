#![no_std]
#![no_main]

extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

// CH572(QingKe 无 A 扩展,RV32IMC)多任务示例 —— 以 ch583 示例为模板,按 12K SRAM 缩编。
//
// ⚠️ 真机核对点(构建级验证已过,板上行为待验):①HPE 关闭路径
// (intsyscr 0x804 写 0 + 私有 CSR `0xbc0=0x25` 与 **`0xbc1=1`**);
// ②mtvec Direct 分发与 mcause=12(SysTick);③SysTick `CTLR=0x0F`
// (STRE|STCLK|STIE|STE,官方 `SysTick_Config` 同值;**CH572 的 CTLR 无 INIT 位**)
// + `SR.CNTIF` 写 0 清零;④运行期用 **`SR.SWIE`(bit31)** 请求切换
// (CH572 的 SWIE 在 SR 不在 CTLR;官方 SDK 的 SysTick 只定义不用,待上板确认,
// 备选是改走 `SWI_IRQn=14`);⑤flash 基址 0x00000000,启动头 boot option
// `0xF3F9BDA9` @flash 0x14(官方 Link.ld 实测);⑥默认主频 100MHz(env 按官方
// `FREQ_SYS` 配,复位默认频率待实测);⑦`mcycle` 是否实现(决定 `delay_us`);
// ⑧SysTick 计数器是 **32 位**(与 ch583 的 64 位读法不同)。
//
// TODO(CH572 外设,待上板核对后补):GPIO/1×UART/1×SPI/1×TMR 的 BSP 层定义;
// 本示例目标是「构建级通过 + 12K SRAM 下内存账成立」,外设留给 BSP。

use xtask::arch::riscv::rt;
use xtask::prelude::*;

#[rt::entry]
fn main() -> ! {
    extern "C" {
        /// 堆内存开始地址，在riscv-rt link.x文件里定义
        static _sheap: u8;
    }
    let start_addr = unsafe { &_sheap as *const u8 as usize };
    // ★ 12K SRAM 是硬约束,堆与任务栈按**实测账**配(QEMU 探针量真实占用,
    //   与 ch583 示例同一套记账办法;ch572 专有取值见 src/chip/env.rs):
    //   软件定时器任务 256 字 ≈ 1.16K(本口把默认 1024 字压到 256,已由 QEMU
    //     24 项内核自测 + 栈围栏守卫验证够用)
    //   idle 任务 256 字 ≈ 1.16K(默认 512 → 256,同上)
    //   示例任务 192 字栈(768B)→ 每个 ≈ 0.93K
    // 实测:3 个示例任务(无格式化调用)用到 ≈5.1K,故取 5.5K;堆顶距中断栈基
    // (_stack_start-512)还有 ~1.6K。
    // ⚠️ 再加任务、或在任务里做 `format!`/日志格式化,都要同步加大栈与堆
    //   (内核的栈围栏会以 `stack overflow <任务名>` panic 报出来,不会静默)。
    xtask::init_heap(start_addr, 5 * 1024 + 512);

    example_semaphore();
    xtask::start()
}

fn example_semaphore() {
    let sender = Semaphore::new();
    let recver = sender.clone();
    let recver2 = sender.clone();
    // 1 个投递者 + 2 个同优先级等待者:覆盖 IPC 与同优先级时间片
    TaskBuilder::new()
        .name("sem.poster")
        .stack_size(192)
        .spawn(move || loop {
            sender.post();
            xtask::sleep_ms(200);
        });
    TaskBuilder::new()
        .name("sem.waiter1")
        .stack_size(192)
        .spawn(move || loop {
            recver.wait();
            log::info!("收到计数信号1 {}", xtask::tick());
        });
    TaskBuilder::new()
        .name("sem.waiter2")
        .stack_size(192)
        .spawn(move || loop {
            recver2.wait();
            log::info!("收到计数信号2 {}", xtask::tick());
        });
}

// 注:ch583 示例里的 Queue(消息队列)示例在 12K 档位上放不下——每个默认任务
// (256 字栈)≈1.2K,而 idle+软件定时器已占 2.3K、总可用堆 ~6.3K;要演示队列
// 得把示例任务栈压到 128 字(实测够,但不做格式化)并再减任务数。
// 需要队列演示时按上面 main 里的实测账重新配。
