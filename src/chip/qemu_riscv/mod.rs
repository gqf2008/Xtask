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

// ---- SMP bring-up(ch25 改造路线②)----
// 从核在 riscv-rt 的 `_mp_hook` 里停泊;应用 `smp::enable()` 后,
// 调度器 start() 经 `start_secondary_cores` 写 SMP_GO 放行。

/// 已登记的核数(从核进 _mp_hook 时按 hartid+1 抬升;主核出厂即 1)。
/// core_count() 据此返回真实核数——同一 ELF 跑 -smp 1 时恒 1
static HARTS_ONLINE: core::sync::atomic::AtomicU16 = core::sync::atomic::AtomicU16::new(1);
/// 放行魔数闸:主核调度数据(就绪队列/每核 idle)就绪后写入,
/// 从核轮询到此值才进调度。魔数而非 bool:bss 清零(0)与未初始化
/// 内存(任意值)都必须判"未放行"
static SMP_GO: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
const SMP_GO_MAGIC: u32 = 0xC0FF_EE11;

/// riscv-rt 启动闸(覆盖其 PROVIDE 默认):hart0 返回 true 走正常启动;
/// 从核直接进 secondary_main——登记、装 trap、等放行、进调度,皆不返回
#[no_mangle]
extern "Rust" fn _mp_hook(hartid: usize) -> bool {
    if hartid == 0 {
        return true;
    }
    unsafe { secondary_main(hartid) }
}

/// 从核入口(mp_hook 上下文,M 态,本核启动栈上运行)
///
/// # Safety
/// 只应由 `_mp_hook` 以本核 hartid 调用一次。不返回。
unsafe fn secondary_main(hartid: usize) -> ! {
    use core::sync::atomic::Ordering;
    extern "C" {
        fn _setup_interrupts();
    }
    // 登记本核存在(主核 core_count() 依此放行对应数量的 idle/IPI)
    HARTS_ONLINE.fetch_max((hartid + 1) as u16, Ordering::Relaxed);
    // mtvec = _start_trap(hart0 由 riscv-rt 代调;从核不返回 mp_hook,自调)
    _setup_interrupts();
    // 只开 MSIE(IPI 唤醒)——不开 MTIE:tick 主核独占(ch25 ⑤),
    // 本核 MTIMECMP 永不装载(reset=0 会立触发,绝不能开)
    core::arch::asm!("csrs mie, {0}", in(reg) 1u32 << 3);
    // 等主核放行:自旋轮询(一次性启动会合,毫秒级——不用 wfi:
    // wfi 需要放行方再补一脚 IPI 才能醒,且"踢早于 GO 可见"会永久
    // 睡死;启动会合用自旋最简单且无竞争窗口)
    while SMP_GO.load(Ordering::Acquire) != SMP_GO_MAGIC {
        core::hint::spin_loop();
    }
    // 首调度:挑一个就绪任务(或本核 idle)装进 CURRENT[本核]
    crate::task::scheduler::schedule();
    // 恢复现场进任务(mret,不返回)——与主核共用 restore_ctx.S:
    // mscratch 按 hartid 分址、CURRENT 按 hartid 索引,天然每核正确
    core::arch::asm!(include_str!("restore_ctx.S"), options(noreturn, raw));
}

/// 重装 MTIMECMP(= MTIME + TICKS;先写 hi 再写 lo 防中途匹配——gd32 同序)。
/// MTIMECMP 是 per-hart 寄存器(+8*hartid);tick 主核独占,故实际只写 hart0 的
#[inline]
pub(crate) fn reset_systick() {
    const TICKS: u64 = (SYSTICK_CLOCK_HZ / TICK_CLOCK_HZ) as u64;
    let mtime = QemuRiscvPorting::systick();
    let v = mtime + TICKS;
    let cmp = (CLINT_BASE + CLINT_MTIMECMP + 8 * QemuRiscvPorting::hart_id() as usize) as *mut u32;
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

// ---- tickless 动态节拍(ch29)----

/// 一次性武装时刻(= 0 未武装/恒定节拍模式)。单核独占(tickless 门控在
/// 单核语义):volatile 而非原子——RV32 无 64 位原子指令,与 TICKS 同款
/// 处理(写在武装临界区、读在 ISR,单核无并发)
static mut TICKLESS_ARMED: vcell::VolatileCell<u64> = vcell::VolatileCell::new(0);

/// tick 中断进入次数——测试/调试差分计数器:tickless 下 ≈ 到点数,
/// 恒定节拍下 ≈ 墙钟拍数(examples/qemu_kernel_tests.rs 第 20/21 项
/// 以两者之比做阳性对照)。取差分使用,无需清零
static TICK_ISR_COUNT: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);

/// 读 tick 中断计数(测试/调试用)
pub fn debug_tick_isr_count() -> u32 {
    TICK_ISR_COUNT.load(core::sync::atomic::Ordering::Relaxed)
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
    /// 参与调度的核数:应用 `smp::enable()` 后 = 实际在线核数(-smp 2 → 2);
    /// 未开启恒 1——同一 ELF 跑 -smp 1、或未开启 SMP 的 -smp 2(如
    /// qemu_kernel_tests),都保持单核语义逐字不变
    #[inline]
    fn core_count() -> u16 {
        if crate::smp::enabled() {
            HARTS_ONLINE.load(core::sync::atomic::Ordering::Acquire)
        } else {
            1
        }
    }
    /// 放行从核:写魔数闸(从核正在 _mp_hook 里 wfi 轮询)。
    /// 仅在应用开启 SMP 时放行;否则从核永远停泊(单核语义)
    #[inline]
    fn start_secondary_cores() {
        if crate::smp::enabled() {
            SMP_GO.store(SMP_GO_MAGIC, core::sync::atomic::Ordering::Release);
        }
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

    // ---- tickless 动态节拍(ch29,见 book/src/ch29-tickless.md)----

    #[inline]
    fn tickless_supported() -> bool {
        true
    }
    /// 一次性武装:cmp = 当前 mtime + delta×PERIOD;TICKLESS_ARMED 记武装
    /// 时刻,到点 ISR 实测 el = 距离(mtime-armed)/PERIOD 跳账。
    /// 整段在临界区内(单核 ISR 不可插入,武装窗口无竞态);先记 flag
    /// 再写 cmp——任何时刻进入的 ISR 看到的状态都自洽
    #[inline]
    fn tickless_arm_delta(delta_ticks: u64) {
        Self::free(|_| unsafe {
            // 局部 const:PERIOD = 每拍 mtime 计数 = 10MHz/1000 = 10000
            const PERIOD: u64 = (SYSTICK_CLOCK_HZ / TICK_CLOCK_HZ) as u64;
            let now = QemuRiscvPorting::systick();
            TICKLESS_ARMED.set(now);
            let v = now + delta_ticks * PERIOD;
            let cmp = (CLINT_BASE + CLINT_MTIMECMP + 8 * Self::hart_id() as usize) as *mut u32;
            cmp.add(1).write_volatile(((v >> 32) as u32) & 0xffff_ffff); // hi 先
            cmp.write_volatile(v as u32); // lo 后(防中途匹配)
            // 停表可能已清 MTIE,补开
            core::arch::asm!("csrs mie, {0}", in(reg) 1u32 << 7);
        });
    }
    /// 停表:清 MTIE + cmp 推到 64 位上限——CLINT 无显式 enable/disable,
    /// 双保险(清使能位防新触发,推 cmp 防陈旧电平残留导致 wfi 立返)
    #[inline]
    fn tickless_stop_timer() {
        unsafe {
            core::arch::asm!("csrc mie, {0}", in(reg) 1u32 << 7);
            let cmp = (CLINT_BASE + CLINT_MTIMECMP + 8 * Self::hart_id() as usize) as *mut u32;
            cmp.add(1).write_volatile(u32::MAX);
            cmp.write_volatile(u32::MAX);
        }
    }
    /// 睡眠等待中断(wfi:任意已使能中断 pending 即返回)
    #[inline]
    fn tickless_wait() {
        unsafe {
            core::arch::asm!("wfi");
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
