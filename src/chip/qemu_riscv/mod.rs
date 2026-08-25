//! QEMU RISC-V `virt` 机移植——**标准 CLINT,零厂商层**(参考实现级)。
//!
//! 本口是第一个**可执行**移植(不只是链接):`qemu-system-riscv32 -M virt
//! -nographic -bios none -kernel <elf>` 直接跑,串口(NS16550A)输出进
//! 终端,SiFive test 设备支持测试自退出——RTOS 心脏(上下文切换/时钟
//! 节拍/调度器/IPC)第一次有了**执行级**验证(此前十口全部停在链接档)。
//!
//! 硬件面(全部标准,QEMU 11.x virt 机):
//! - **CLINT @0x0200_0000**:MSIP@+0x0(软件中断,mcause=3——yield 用,
//!   写 1 触发/写 0 清除,电平源)、MTIMECMP@+0x4000(定时中断,mcause=7,
//!   重装=MTIME+TICKS,gd32 同款 hi/lo 写序)、MTIME@+0xBFF8(64 位单调);
//! - **NS16550A @0x1000_0000**:THR@+0、LSR@+5(bit5=THRE)——轮询写,
//!   `-nographic` 下串口=stdout;
//! - **SiFive test @0x0010_0000**:写 0x5555=PASS 退出(exit 0),
//!   0x3333/0x7777=FAIL——CI 门禁无需 timeout 杀进程;
//! - mtvec=Direct 单入口分发(36 字帧与 gd32/ch32/esp 同一约定)。
//!
//! env:CPU/SYSTICK = 10MHz(virt 机 timebase 默认——mtime 10MHz 计数)。
//!
//! **panic handler 契约**:lib.rs 把本口排除出了默认 `panic_halt`
//! (`#[cfg(all(not(test), not(target_arch = "arm"), not(feature = "qemu_riscv")))]`),
//! 因此**每个 qemu_riscv 例程/应用必须自带 `#[panic_handler]`**
//! (惯例:位置打到串口 + `qemu_exit_fail()` 确定性退出,见
//! examples/qemu_kernel_tests.rs),否则得到的是链接错误
//! "`#[panic_handler]` function required"。
mod port;
pub mod stdout;
use super::{CPU_CLOCK_HZ, SYSTICK_CLOCK_HZ, TICK_CLOCK_HZ};
use crate::port::Portable;
use crate::prelude::CriticalSection;
use crate::task::Task;
use core::arch::asm;

/// CLINT 基址
pub(crate) const CLINT_BASE: usize = 0x0200_0000;
const CLINT_MSIP: usize = 0x0;
const CLINT_MTIMECMP: usize = 0x4000;
const CLINT_MTIME: usize = 0xBFF8;

/// SiFive test 设备(测试自退出)
pub(crate) const SIFIVE_TEST: usize = 0x0010_0000;

/// 重装 MTIMECMP(= MTIME + TICKS;先写 hi 再写 lo 防中途匹配——gd32 同序)
#[inline]
pub(crate) fn reset_systick() {
    const TICKS: u64 = (SYSTICK_CLOCK_HZ / TICK_CLOCK_HZ) as u64;
    let mtime = QemuRiscvPorting::systick();
    let v = mtime + TICKS;
    let cmp = (CLINT_BASE + CLINT_MTIMECMP) as *mut u32;
    unsafe {
        cmp.add(1).write_volatile(((v >> 32) as u32) & 0xffff_ffff); // hi 先
        cmp.write_volatile(v as u32); // lo 后
    }
}

/// 使能机器定时/软件中断(mstatus.MIE 之外,mie CSR 的位 7/3)
#[inline]
pub(crate) fn setup_intrrupt() {
    unsafe {
        // mie:bit7=MTIE(定时)、bit3=MSIE(软件)
        let bits: u32 = (1 << 7) | (1 << 3);
        core::arch::asm!("csrs mie, {0}", in(reg) bits);
    }
}

/// QEMU virt 机移植层实现
pub struct QemuRiscvPorting;

// port.S 依赖的 Task 布局偏移(蹦床 `lw t1, 8(t0)` 取 entry、trap 入口
// `sw sp, 0(t0)` 存 sp)——失配在编译期炸,而非静默错跳
const _: () = assert!(core::mem::offset_of!(Task, sp) == 0);
const _: () = assert!(core::mem::offset_of!(Task, entry) == 8);

impl Portable for QemuRiscvPorting {
    #[inline]
    fn barrier() {
        unsafe {
            core::arch::asm!("fence iorw, iorw");
        }
    }
    /// 临界区保护(本核 mstatus.MIE)
    #[inline]
    fn free<F, R>(f: F) -> R
    where
        F: FnOnce(&CriticalSection) -> R,
    {
        riscv::interrupt::free(f)
    }

    #[inline]
    fn enable_interrupt() {
        unsafe {
            riscv::interrupt::enable();
        }
    }
    #[inline]
    fn disable_interrupt() {
        unsafe {
            riscv::interrupt::disable();
        }
    }

    /// 启动调度器:使能 mie 位 → 恢复第一个任务(汇编,不返回)
    fn start_scheduler() -> ! {
        reset_systick();
        setup_intrrupt();
        log::info!("Start scheduler");
        unsafe { asm!(include_str!("restore_ctx.S"), options(noreturn, raw)) };
    }

    /// 软中断(yield/调度请求):CLINT MSIP 写 1(电平源,ISR 写 0 清)
    /// 按本核 hartid 寻址——单核时即 MSIP 基址,与旧实现等价
    #[inline]
    fn irq() {
        Self::irq_to(Self::hart_id());
    }
    /// 清软中断(MSIP 写 0)
    #[inline]
    fn disable_irq() {
        let msip = (CLINT_BASE + CLINT_MSIP + 4 * Self::hart_id() as usize) as *mut u32;
        unsafe {
            msip.write_volatile(0);
        }
    }

    /// 当前核 ID:读 mhartid CSR(`-smp N` 启动时为 0..N-1)
    #[inline]
    fn hart_id() -> u16 {
        riscv::register::mhartid::read() as u16
    }
    /// 向指定核发软中断(IPI):CLINT MSIP 是 per-hart 寄存器(基址+4*hart),
    /// 按目标核寻址即天然 IPI(ch25 失效清单里的"写死单核 MSIP"就此解开)
    #[inline]
    fn irq_to(hart: u16) {
        let msip = (CLINT_BASE + CLINT_MSIP + 4 * hart as usize) as *mut u32;
        unsafe {
            msip.write_volatile(1);
        }
    }

    /// 读 CLINT MTIME(64 位 @10MHz;高:低:高重读防翻转)
    #[inline]
    fn systick() -> u64 {
        let mtime = (CLINT_BASE + CLINT_MTIME) as *mut u32;
        loop {
            unsafe {
                let hi = mtime.add(1).read_volatile();
                let lo = mtime.read_volatile();
                if hi == mtime.add(1).read_volatile() {
                    return ((hi as u64) << 32) | lo as u64;
                }
            }
        }
    }

    /// 硬件延时(mcycle @CPU 时钟——virt 机核频默认与 timebase 同源量级,
    /// env 按 10MHz 配;QEMU TCG 下时间是虚拟的,断言不依赖绝对时长)
    #[inline]
    fn delay_us(us: u64) {
        let t0 = riscv::register::mcycle::read64();
        let clock = (us * (CPU_CLOCK_HZ as u64)) / 1_000_000;
        while riscv::register::mcycle::read64().wrapping_sub(t0) <= clock {}
    }

    /// 任务现场(36 字帧,[35]=mcause 取定时中断形态)
    ///
    /// 初始帧必须把除定值外的全部槽清零、mepc 指向 port.S 的首调蹦床:
    /// 任务首次调度经 mret 直入入口,而编译器(LTO)会把入口序言/尾声
    /// outlined 成只按 jalr 调用约定成立的共享 stub(实测会读到帧里
    /// 未初始化的陈旧堆数据并野跳,见 port.S 蹦床与 RESTORE_CONTEXT 注)
    #[inline]
    fn save_context(task: &mut Task) {
        unsafe {
            let sp = task.stack.add(task.stack_size - 1);
            sp.offset(-1).write_volatile(0x8000_0007); // mcause:中断|7(定时)
            sp.offset(-2).write_volatile(0); // 保留槽
            unsafe extern "C" {
                fn _task_entry_trampoline();
            }
            sp.offset(-3)
                .write_volatile((_task_entry_trampoline as *const ()).addr()); // mepc
            sp.offset(-4).write_volatile(0x0000_1880); // mstatus:MPP=M, MPIE=1
            for i in 0..32usize {
                if i != 1 && i != 10 {
                    sp.offset(i as isize - 36).write_volatile(0);
                }
            }
            sp.offset(-26).write_volatile(task.args.addr()); // a0
            sp.offset(-35)
                .write_volatile((port::task_exit as *const ()).addr()); // ra
            task.sp = sp.offset(-36).addr();
        }
    }
}
