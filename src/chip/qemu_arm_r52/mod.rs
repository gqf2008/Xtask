//! QEMU xlnx-zcu102 Cortex-R5F 移植——**第二个可执行口**(qemu_riscv 之后)。
//!
//! R5 无 PendSV/无 PSP 双栈:上下文切换走 **SVC 异常 + IRQ 入口尾部切换**。
//! 所有调度切换收口到 port.S 的 `switch_and_restore` 公共路径:入口(IRQ/
//! SVC)各存现场帧 → Rust handler → `switch_context` → 从新任务帧恢复。
//! 帧恒建在"入口当前 SP"所在栈(IRQ 入口压专用 IRQ 栈、SVC 入口压任务
//! 栈),切换只认帧指针——两种来源天然兼容,见 port.S 头注。
//!
//! 硬件面(QEMU 源码级确认,见 `/tmp/r5probe` 探针实测):
//! - **R5 复位@0xFFFF0000**(SCTLR.V=1 高向量,OCM bank3)——镜像布局:
//!   向量表 32B 在 OCM,其余全在 DDR@0x00000000;
//! - **rpu_gic**(GICv2 单核):dist@0xF9000000、CPU 接口@**0xF9001000**
//!   (QEMU 上游 bug!`hw/arm/xlnx-zynqmp.c` 的 `GIC_BASE_ADDR + i*0x1000`
//!   使 CPU 接口与 dist 同址重叠、GICC_CTLR 恒 0 → IRQ 永不投递。
//!   **必须用打过补丁的 QEMU**(`GIC_BASE_ADDR + (i+1)*0x1000`),见
//!   书稿第 30/31 章 与 README);
//! - **TTC0 @0xFF110000** 133MHz:CLK_CTRL=0x03(预分频÷4)+ INTERVAL=33249
//!   → 1kHz 节拍,SPI#36 → intid 68;
//! - **Cadence UART0 @0xFF000000** 轮询 TX;
//! - semihosting(ARM 态 `svc #0x123456`)SYS_EXIT:例程自退出门禁。
//!
//! **panic handler 契约**:lib.rs 把本口排除出了默认 `panic_probe`
//! (`#[cfg(all(not(test), target_arch = "arm", not(feature = "qemu_arm_r52")))]`),
//! 因此**每个 qemu_arm_r52 例程必须自带 `#[panic_handler]`**
//! (惯例:位置打到串口 + semihosting FAIL 退出),否则链接错误。

mod port;
pub mod stdout;

use crate::port::Portable;
use crate::prelude::CriticalSection;
use crate::task::Task;
use core::arch::asm;

/// 系统时钟(HZ)
pub(crate) const CPU_CLOCK_HZ: u32 = 133_000_000;
/// 节拍定时器频率(HZ)——TTC0 预分频/4
pub(crate) const SYSTICK_CLOCK_HZ: u32 = CPU_CLOCK_HZ / 4;
/// 节拍频率(HZ)
pub(crate) const TICK_CLOCK_HZ: u32 = 1000;

// ---- 外设地址(QEMU 源码级确认) ----
/// Cadence UART0
pub(crate) const UART0: usize = 0xFF00_0000;
/// rpu_gic distributor
pub(crate) const GICD: usize = 0xF900_0000;
/// rpu_gic CPU 接口(补丁后地址 = dist + 0x1000;上游 bug 使其与 dist 重叠)
pub(crate) const GICC: usize = 0xF900_1000;
/// TTC0(timer0)
pub(crate) const TTC: usize = 0xFF11_0000;
/// TTC0 SPI 中断号 → GIC intid(SPI#36 + 32)
pub(crate) const TTC_SPI_INTID: u32 = 36 + 32;

/// QEMU zcu102(R5)移植层实现
pub struct QemuArmR52Porting;

/// TTC0 + rpu_gic 初始化(探针 /tmp/r5probe 实测序列):
/// TTC0 timer0 → 1ms tick;SPI 68(电平)经 GIC 投递。
/// start_scheduler 调用,例程零配置
pub(crate) fn setup_irqs() {
    unsafe {
        let ttc = TTC as *mut u32;
        let gicd = GICD as *mut u32;
        let gicc = GICC as *mut u32;
        // TTC0:CLK_CTRL 预分频÷4 → 33.25MHz;INTERVAL=33249 → 1ms;
        // INT_EN.IV 开 interval 中断;CNT_CTRL 区间模式(最后写,触发运行)
        ttc.add(0x00 / 4).write_volatile(0x03);
        ttc.add(0x24 / 4).write_volatile(33249);
        ttc.add(0x60 / 4).write_volatile(0x01);
        ttc.add(0x0C / 4).write_volatile(0x02);
        // GIC dist:G0E 使能 + SPI 68 使能/优先级/目标
        gicd.add(0x00 / 4).write_volatile(1);
        gicd.add(0x108 / 4).write_volatile(1 << 4);
        let p = gicd.add(0x444 / 4);
        p.write_volatile((p.read_volatile() & !0xFF) | 0x80);
        let t = gicd.add(0x844 / 4);
        t.write_volatile((t.read_volatile() & !0xFF) | 0x01);
        // GIC CPU 接口:PMR=0xFF(全开)+ CTLR=1(组 0 使能)
        gicc.add(0x004 / 4).write_volatile(0xFF);
        gicc.add(0x000 / 4).write_volatile(1);
    }
}

impl Portable for QemuArmR52Porting {
    /// 完全内存屏障(DSB + ISB——ARM 侧 barrier 语义)
    #[inline]
    fn barrier() {
        unsafe {
            asm!("dsb ish", "isb");
        }
    }
    /// 临界区保护(本核 CPSR.I——经典 ARM 关中断模型)
    #[inline]
    fn free<F, R>(f: F) -> R
    where
        F: FnOnce(&CriticalSection) -> R,
    {
        // 关中断、执行、恢复(嵌套深度由 critical.rs 配平)
        unsafe {
            let mut cpsr: u32;
            asm!("mrs {0}, cpsr", "cpsid i", out(reg) cpsr, options(nostack));
            let r = f(unsafe { &CriticalSection::new() });
            if cpsr & (1 << 7) == 0 {
                // 原 I=0(中断开着)才恢复
                unsafe { asm!("cpsie i") };
            }
            r
        }
    }

    #[inline]
    fn enable_interrupt() {
        unsafe {
            asm!("cpsie i");
        }
    }
    #[inline]
    fn disable_interrupt() {
        unsafe {
            asm!("cpsid i");
        }
    }

    /// 启动调度器:TTC/GIC 就绪 → 恢复第一个任务(idle 初始帧),不返回。
    /// GIC 使能后必须立即关中断直到首任务被恢复——此窗口内 CPU 仍在
    /// "启动上下文"(main/start_scheduler 的调用栈),而 CURRENT 已被
    /// start_idle_task 指向未首跑的 idle;若此刻 tick 中断插入,
    /// irq_entry 会把启动栈现场补成帧存进 idle.sp(lr=Task::new 内部
    /// 地址,实测),僵尸 Task::new 从中间恢复执行,后续帧全污染。
    /// 首任务的中断开启由帧内 spsr(0x13)在 movs pc 时一次性完成
    fn start_scheduler() -> ! {
        // 先关中断再使能外设:TTC 的 CNT_CTRL 一写入就开始计数,1ms 后
        // 中断即来,必须保证那时 CPU 已在"只可能打断真实任务"的状态
        unsafe { asm!("cpsid i") };
        setup_irqs();
        log::info!("Start scheduler");
        // 调度循环恢复首任务前置位门控(此刻 I=1,无被打断窗口):
        // 之后 current!=NULL ⟺ 线程真实在跑,irq_entry 补帧才合法
        unsafe {
            asm!("ldr r0, =SCHED_STARTED", "mov r1, #1", "strb r1, [r0]");
        }
        unsafe { asm!(include_str!("restore_ctx.S"), options(noreturn, raw)) };
    }

    /// 软中断(yield/调度请求)——任务侧 yield_now 触发 SVC 异常,
    /// 无独立软中断源(R5 无 PendSV/无软件中断寄存器)
    #[inline]
    fn irq() {
        unsafe {
            asm!("svc #0x5", options(nomem, nostack));
        }
    }
    /// IPI(抢占请求)必须无操作!——R5 的抢占已由 IRQ 入口架构保证:
    /// 每次中断退出必经 `switch_and_restore → do_schedule`,无需再发
    /// 软中断请求。若用默认退化(irq_to → irq → svc),会在 tick ISR
    /// 处理中途(do_systick → submit_task → request_preempt)嵌套触发
    /// 第二次 SVC 入口——SAVE_CTX 把 64B 帧压进 IRQ 栈、覆盖 CURRENT
    /// 任务的 sp 字段、`ldr sp,=__stack_irq` 丢弃外层帧——零重入设计
    /// 被打破,调度器错乱(实测 panic `put_task, illegal task 0x0`)
    #[inline]
    fn irq_to(_hart: u16) {}
    #[inline]
    fn disable_irq() {}

    /// 运行时时钟(微秒):TTC 计数器 33.25MHz——1 个计数 ≈ 0.0301us,
    /// 用整数近似 cnt×4/133。QEMU 虚拟时钟下仅作相对延时/日志,
    /// 精度不参与断言
    #[inline]
    fn systick() -> u64 {
        let c = unsafe { ((TTC + 0x18) as *mut u32).read_volatile() & 0xFFFF };
        (c as u64) * 4 / 133
    }
    /// 硬件延时(忙等 TTC 计数;QEMU 虚拟时钟,断言不依赖绝对时长)
    #[inline]
    fn delay_us(us: u64) {
        // TTC 计数器 @33.25MHz 每 1ms 回绕:逐次采样做增量累计,
        // 跨窗口用 (1000 - last) + now 补足——任意时长都正确
        let mut last = Self::systick();
        let mut acc = 0u64;
        while acc < us {
            let now = Self::systick();
            let d = if now >= last {
                now - last
            } else {
                1000 - last + now
            };
            last = now;
            acc += d;
        }
    }

    /// 任务现场(16 字帧,见 port.rs save_context)
    #[inline]
    fn save_context(task: &mut Task) {
        port::save_context(task);
    }
}
