//! CH583 移植模块实现(QingKe V4A(第三代以上 PFIC 内核,SysTick 独立外设) 内核,
//! PFIC + WCH 自有 SysTick 模型)。以 `ch32v103`(V3A)为模板——两者架构同源:
//! 都是 36 字纯软件全保存帧、都是 Direct mtvec(模板用 `STK_CTLR.SWIE` 请求切换,
//! 本口改成官方 RTOS 移植的 `SWI_IRQn=14`,见下)。
//!
//! 与 `ch32v103` 的关键差异:
//! - **代码基址 = `0x00000000`**(不是 `0x08000000`!),见 `memory.x`;
//!   官方 EVT 只给 448K,尾部 64K 留给 BootLoader/ISP/配置区。
//! - **私有 CSR `0xbc0`**:`_setup_interrupts` 多一条 `csrw 0xbc0, 0x1f`
//!   (流水线控制 + 动态分支预测控制位,官方 `startup_CH583.S` 固定写法)。
//! - **SysTick 是 64 位**(CH572 是 32 位),但寄存器字偏移与 V3A 一致:
//!   CTLR@+0 SR@+4 CNT@+8(64 位,字 2/3) CMP@+16(64 位,字 4/5)。
//!
//! 本口按**标准 RV32 机器模式**设计(与现有 RISC-V 口一致):
//!
//! - **HPE 关闭**:`_setup_interrupts` 显式写 `intsyscr(0x804)=0`(CH583 的
//!   HWSTKEN 在 bit0),上下文切换回到**纯软件全保存**模型(36 字帧)。
//! - **mtvec = Direct 模式**:单入口 `_start_trap` 读 `mcause` 分发——
//!   中断号 12 = SysTick;其余走 `DefaultHandler`。
//! - **软中断(yield/抢占请求)走 QingKe 专用 `SWI_IRQn=14`**:
//!   `irq()` 置 `PFIC->IPSR[0]` 的 bit14(pending),trap 侧按 mcause=14 直接进
//!   切换/恢复路径;`disable_irq()` 用 `IPRR[0]` 清 pending。
//!   这是**官方**路径:官方 CH583 的 FreeRTOS / RT-Thread / HarmonyOS 三套移植
//!   一处都没用 `STK_CTLR.SWIE`(SDK 里只定义、零使用),yield 一律
//!   `PFIC_SetPendingIRQ(SWI_IRQn)` + `SW_Handler`。改走它之后,"SWIE 在真机上
//!   到底灵不灵"这个只能上板才能排除的赌注就没了——本口初始化与运行期都不碰 SWIE。
//!   (ch32v103/ch32v203/ch32v307 三口仍沿模板的 SWIE 路线,见各口注。)
//! - **SysTick@0xE000F000**(WCH 自有,非 CLINT mtime);`SR.CNTIF` 写 0 清零。
//! - **无 A 扩展(如 CH572)用 `riscv32imc-unknown-none-elf`**:该 target 实测只有
//!   `target_has_atomic_load_store`、没有 `target_has_atomic`,内核 `Arc`/信号量的
//!   `fetch_add`/`swap` 由 `atomic-polyfill` 兜到 `critical-section`——RISC-V 侧那份
//!   实现在 `src/arch/riscv/critical.rs`(`mstatus.MIE` 存取恢复,单核语义);
//!   2026-09-19 起 `ci/gate.sh` 用同一示例构建该 target(产物 0 条 AMO/LR/SC)。
//!   CH583(V4A)有 A 扩展,本口不受影响。
//!
//! ⚠️ 真机核对点(构建级验证,板上行为待验):
//! ①`0x00000000` 是否可写 / 是否有 `0x08000000` 别名;
//! ②默认主频 = 60MHz(官方 `CH58x_common.h` 的 `FREQ_SYS` 默认值;env 常数按此配,
//!   PLL 配好后须同步改);
//! ③`csrw 0xbc0, 0x1f`(官方 `startup_CH583.S` 的 reset 序列同款,已按 SDK
//!   源码核对;上板只需确认不触发 Illegal Instruction);
//! ④`0x804=0` 后异常/中断入口是否确实不再硬件压栈(决定 36 字帧成立);
//! ⑤置 `SWI_IRQn=14` pending 后是否即刻进 trap、并沿 mcause=14 走切换
//!   (官方三套移植的 yield 路径;`SWIE` 已不使用);
//! ⑥`delay_us` 的实测精度(现用 SysTick 计数,不再依赖 `mcycle`)。
//! BootLoader 默认开启:本口已在 flash `0x0` 复刻官方启动头(`.bootvec` =
//! 入口跳转 + 向量表前 5 字,第 5 字为 boot option `0xF3F9BDA9`;见 `port.S`
//! 与 `memory.x` 的链接期 ASSERT)。若上板仍停在 ISP,则改用关闭 BootLoader 的
//! 配置字。
//!
//! **零 PAC 依赖**:CH58x 无现成 `ch32-rs` PAC,本口全量直址访问
//! (与 `qemu_riscv` 口的思路一致),`Cargo.toml` 只挂 `riscv-rt`。

mod port;

use super::{SYSTICK_CLOCK_HZ, TICK_CLOCK_HZ};
use crate::port::Portable;
use crate::prelude::CriticalSection;
use crate::task::Task;
use core::arch::asm;

/// SysTick 寄存器基址(WCH 自有,非 CLINT;CH583 为 64 位计数器)
pub(crate) const STK_BASE: usize = 0xE000_F000;

/// PFIC 寄存器基址(CH583 与 CH572/ch32v103/ch32v307 同址)
pub(crate) const PFIC_BASE: usize = 0xE000_E000;

/// 配置 SysTick 与 PFIC(在 start_scheduler 里、restore_ctx 之前调用)。
/// CH583 的 SysTick 与 V3A 同布局,只是计数器/比较值 64 位;
/// PFIC 无 PAC,直接访问 `IENR[0]`(offset 0x100)使能 SysTick。
#[inline]
pub(crate) fn setup_intrrupt() {
    let stk = STK_BASE as *mut u32;
    unsafe {
        // 1. 停计数(CTLR=0)、清 SR.CNTIF(写 0)、CNT/CMP 清零装初值(高字先写)
        stk.write_volatile(0);
        stk.add(1).write_volatile(0); // SR:CNTIF 写 0 清
        stk.add(2).write_volatile(0); // CNTL
        stk.add(3).write_volatile(0); // CNTH
        const TICKS: u32 = (SYSTICK_CLOCK_HZ / TICK_CLOCK_HZ) as u32 - 1;
        stk.add(5).write_volatile(0); // CMPHR
        stk.add(4).write_volatile(TICKS); // CMPLR
        // 2. CTLR = INIT(5)|STRE(3)|STCLK(HCLK,2)|STIE(1)|STE(0) = 0x2F
        //    ——逐位对齐官方 `EVT/EXAM/SRC/RVMSIS/core_riscv.h` 的
        //    `SysTick_Config()`:`SysTick->CMP = ticks - 1;` 然后
        //    `CTLR = INIT|STRE|STCLK|STIE|STE`。
        //    **不含 SWIE(bit31)**:本口的 yield 走 `SWI_IRQn=14`(见 `irq()`),
        //    初始化与运行期都不碰 SWIE。顺带免掉一个隐含依赖:若初始化就置 SWIE,
        //    第一个任务 `mret`(MIE←MPIE=1)后会立刻多进一次 trap,而该 trap 早于
        //    restore_ctx.S 写 mscratch → 依赖"进入 setup_intrrupt 时 mstatus.MIE=0"。
        stk.write_volatile(0x0000_002F);
    }
    // 3. PFIC:使能 SysTick(IENR[0] @0xE000E100,bit12 = 核中断号 12)
    let ienr0 = (PFIC_BASE + 0x100) as *mut u32;
    unsafe {
        ienr0.write_volatile(1 << 12);
    }
}

/// 清 SysTick 的 CNTIF(写 0)——真 tick 路径调用
#[inline]
pub(crate) fn reset_systick() {
    let stk = STK_BASE as *mut u32;
    unsafe {
        stk.add(1).write_volatile(0); // SR @ +4
    }
}

/// CH583 芯片移植层实现
pub struct Ch583Porting;

// port.S 蹦床 `_task_entry_trampoline` 依赖的 Task 布局偏移(失配编译期炸)
const _: () = assert!(core::mem::offset_of!(Task, sp) == 0);
const _: () = assert!(core::mem::offset_of!(Task, entry) == 8);

impl Portable for Ch583Porting {
    /// 完全内存屏障
    #[inline]
    fn barrier() {
        unsafe {
            // CH583 无 MMU/虚存,通用 fence(iorw) 即可
            core::arch::asm!("fence iorw, iorw");
        }
    }
    /// 临界区保护(本核 mstatus.MIE——单核语义与 gd32 同)
    #[inline]
    fn free<F, R>(f: F) -> R
    where
        F: FnOnce(&CriticalSection) -> R,
    {
        riscv::interrupt::free(f)
    }

    /// 开全局中断
    #[inline]
    fn enable_interrupt() {
        unsafe {
            riscv::interrupt::enable();
        }
    }
    /// 关全局中断
    #[inline]
    fn disable_interrupt() {
        unsafe {
            riscv::interrupt::disable();
        }
    }

    /// 启动调度器:配置 SysTick/PFIC → 恢复第一个任务(汇编,不返回)
    fn start_scheduler() -> ! {
        setup_intrrupt();
        log::info!("Start scheduler");
        unsafe { asm!(include_str!("restore_ctx.S"), options(noreturn, raw)) };
    }

    /// 软中断(调度请求):置 PFIC 的 `SWI_IRQn=14` pending
    /// (`IPSR[0] @0xE000E200` bit14)——官方三套 RTOS 移植的 yield 机制
    /// (`PFIC_SetPendingIRQ(SWI_IRQn)`),见模块头注。
    #[inline]
    fn irq() {
        let ipsr0 = (PFIC_BASE + 0x200) as *mut u32;
        unsafe {
            ipsr0.write_volatile(1 << 14);
        }
    }
    /// 关闭软中断:清 `SWI_IRQn=14` 的 pending(`IPRR[0] @0xE000E280` bit14)。
    /// 取中断时 PFIC 一般已自动清 pending(官方 `SW_Handler` 不显式清),这里
    /// 防御性再清一次——残留 pending 会让 trap 出口立刻重入。
    #[inline]
    fn disable_irq() {
        let iprr0 = (PFIC_BASE + 0x280) as *mut u32;
        unsafe {
            iprr0.write_volatile(1 << 14);
        }
    }

    /// 读 SysTick 64 位计数器(高:低:高重读防翻转;CH583 是 64 位)
    #[inline]
    fn systick() -> u64 {
        let stk = STK_BASE as *mut u32;
        loop {
            unsafe {
                let hi = stk.add(3).read_volatile();
                let lo = stk.add(2).read_volatile();
                if hi == stk.add(3).read_volatile() {
                    return ((hi as u64) << 32) | lo as u64;
                }
            }
        }
    }

    /// 硬件延时,单位 us —— **用 SysTick 计数器**实现。
    ///
    /// 为什么不再用 `mcycle`(ch32v103 模板的做法):"CH583 是否实现 Zicntr"没有
    /// 一手证据,而未实现时 `mcycle` 读出恒为常量 → 本函数死循环(上板才能发现)。
    /// SysTick 则是内核点拍赖以工作的外设,必然实现,故改用它的自由计数:
    /// 计数器按 `CMP` 自动重装,等待按"到下次重装还剩多少"分块(跨重装差值见
    /// `crate::chip::delay`,有 host 单测钉边界)。
    #[inline]
    fn delay_us(us: u64) {
        let period = (SYSTICK_CLOCK_HZ / TICK_CLOCK_HZ) as u64; // = TICKS + 1
        let mut remaining = us * (SYSTICK_CLOCK_HZ as u64) / 1_000_000;
        while remaining > 0 {
            let start = Self::systick();
            let chunk = crate::chip::delay::next_chunk(remaining, start, period);
            while crate::chip::delay::elapsed(start, Self::systick(), period) < chunk {}
            remaining -= chunk;
        }
    }

    /// 任务创建时为 CPU 准备任务现场(36 字帧,与 port.S 的保存宏互为镜像):
    /// [35]=mcause(0x8000_000C)[34]=0(mcause 保留槽)
    /// [33]=mepc=_task_entry_trampoline [32]=mstatus=0x1880(MPP=M|MPIE=1)
    /// [10]=a0=args [1]=ra=task_exit,任务块 sp 指向帧底
    #[inline]
    fn save_context(task: &mut Task) {
        unsafe {
            let sp = task.stack.add(task.stack_size - 1);
            sp.offset(-1).write_volatile(0x8000_000C); // mcause:中断|12(SysTick)
            sp.offset(-2).write_volatile(0); // 保留槽(无 msubm)
            // mepc = 首调蹦床:经标准 jalr 进入 task.entry(mret 直入会被
            // 编译器 outlined 的入口 stub 坑到野跳,见 port.S 蹦床注)
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
