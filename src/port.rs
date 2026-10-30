//! 移植层定义&配置

// ---- 芯片移植实现的**单一来源清单** ----------------------------------------
// 每个芯片在这里登记一次(feature 名 => 实现类型),下面由宏一次性生成:
//   ① 正选别名 `pub use <实现类型> as Porting`;
//   ② 兜底桩的 feature 名单(`not(any(...))` → `DefaultPorting`);
//   ③ 编译期守卫:`RealPorting` 标记实现 + "选了芯片就不许落兜底桩"断言。
//
// 为什么收成一处:①与②原本是两份手写名单,新增芯片时只改一份,`Porting`
// 就静默落到 `DefaultPorting`(各方法 `unimplemented!()`),构建与门禁全绿、
// 直到运行期 `xtask::start()` 才 panic——2026-09-18 的 ch583 骨架 PR 实测
// 踩过此坑(见 PR #15 自审节 / issue #14)。
macro_rules! chip_portings {
    ($($feat:literal => $ty:path),* $(,)?) => {
        $(
            #[cfg(all(feature = $feat, not(test)))]
            pub use $ty as Porting;
        )*

        #[cfg(all(not(test), not(any($(feature = $feat),*))))]
        pub use DefaultPorting as Porting;

        /// 真实芯片移植实现的标记 trait:兜底桩(`DefaultPorting`)与 host
        /// mock(`HostPorting`)故意不实现——配合下面的编译期断言,把"选了
        /// 芯片 feature 却落到兜底桩"钉成编译错误(否则要等运行期
        /// `unimplemented!()` 才暴露)。
        pub trait RealPorting {}

        $(
            #[cfg(all(feature = $feat, not(test)))]
            impl RealPorting for $ty {}
        )*

        // 编译期守卫:选了任一芯片 feature 时,`Porting` 必须是真实实现
        #[cfg(all(not(test), any($(feature = $feat),*)))]
        const _: fn() = {
            fn assert_real_porting<T: RealPorting>() {}
            assert_real_porting::<Porting>
        };
    };
}

chip_portings! {
    "gd32vf103" => crate::chip::gd32vf103::Gd32vf103Porting,
    "stm32f4" => crate::chip::stm32f4::STM32F4Porting,
    "stm32f1" => crate::chip::stm32f1::STM32F1Porting,
    "rp2040" => crate::chip::rp2040::RP2040Porting,
    "stm32h7" => crate::chip::stm32h7::STM32H7Porting,
    "cm32m4" => crate::chip::cm32m4::CM32M4Porting,
    "ch32v307" => crate::chip::ch32v307::Ch32v307Porting,
    "ch32v203" => crate::chip::ch32v203::Ch32v203Porting,
    "ch32v103" => crate::chip::ch32v103::Ch32v103Porting,
    "ch583" => crate::chip::ch583::Ch583Porting,
    "ch572" => crate::chip::ch572::Ch572Porting,
    "esp32c3" => crate::chip::esp32c3::Esp32c3Porting,
    "qemu_riscv" => crate::chip::qemu_riscv::QemuRiscvPorting,
    "qemu_arm_r52" => crate::chip::qemu_arm_r52::QemuArmR52Porting,
}

// host 测试环境：提供一个可运行的 Porting mock，让纯逻辑（信号量、队列、总线、延时队列）
// 能在 `cargo test` 下被驱动。单线程语义下临界区只是一个标记，无需真实关中断。
#[cfg(test)]
pub use HostPorting as Porting;

use crate::task::Task;
use bare_metal::CriticalSection;

/// 内核数组定界用的核数上限(实际参与调度的核数由 `Porting::core_count()`
/// 运行期决定,恒 ≤ 本值)。**定义在 `chip/env.rs` 里按芯片取值**(单核 WCH 口
/// 为 1,省下 `READYQ` 的 4K 静态;多核口 16),这里 re-export 保持
/// `crate::port::MAX_HARTS` 这个既有引用路径不变(ch25 改造路线②)
pub(crate) use crate::chip::MAX_HARTS;

/// 移植层接口定义
pub trait Portable {
    /// 完全内存屏障
    /// 保证在屏障之前的任何存储操作先于屏障之后的代码执行。
    fn barrier();
    /// 临界区保护函数
    fn free<F, R>(f: F) -> R
    where
        F: FnOnce(&CriticalSection) -> R;

    /// 当前上下文是否在中断服务例程内（R8 的 ISR 半边）。
    ///
    /// 语义：只反映**本核/本线程当前上下文**（不是全局标志），与
    /// `in_critical_section()` 正交——关中断的临界区内、带 PRIMASK 的
    /// 线程上下文都返回 false。默认 `false` 表示端口未提供判据，此时
    /// C2 约束只能靠代码审查。
    ///
    /// 本轮端口覆盖（14 个芯片口）：
    /// - Cortex-M（stm32f1/f4/h7/rp2040）：ICSR.VECTACTIVE（=IPSR[8:0]）
    ///   非 0 即 Handler 模式；只读。`SCB::vect_active()` 内部按
    ///   `icsr as u8` 取号：IRQn≥256 且低 8 位为 0 会误判 Thread；低 8 位
    ///   <16 的非 0 保留号会下溢（debug panic）。本仓四口最大 IRQ 号 <256。
    /// - Cortex-R5（qemu_arm_r52）：CPSR.M==IRQ(0x12)；任务与调度循环跑
    ///   SVC(0x13)，SVC 入口（任务主动 yield）按任务上下文计；FIQ 未使用。
    /// - RISC-V Direct 单入口（qemu_riscv/ch32v103/ch32v203/ch32v307/
    ///   ch583/ch572/esp32c3）：由 port.S 在 `_start_trap` 入口/出口维护
    ///   本核嵌套深度 `IN_ISR_DEPTH`；>0 即 ISR。异常与中断共用入口，
    ///   异常也计入（本仓异常即致命）。
    /// - RISC-V Nuclei ECLIC（gd32vf103/cm32m4）：同一计数器，挂在
    ///   `_irq_handler`（mtvt2 公共入口）；`_start_trap` 异常路径不计。
    ///   前提：被中断的 trap 帧不会在任务运行期间挂起（唯一切换点在
    ///   `_irq_handler` 出口；INT_SFT 与 INT_TMR 同 Level，SWI 不能抢占
    ///   处理中的 handler）。
    /// - HostPorting(cfg(test)) / DefaultPorting：保持 false。
    ///
    /// 维护约束：计数器只允许在既有 trap 入口/出口增减，ENTER/LEAVE 必须
    /// 配平（不配平会回绕/下溢，使 in_isr 永久为真）；不得改动上下文
    /// 保存/恢复与嵌套语义。
    ///
    /// 残留边界：
    /// 1. host mock 不模拟中断上下文，默认 false 是诚实边界；目标端判据只能
    ///    靠构建覆盖 + QEMU/真机执行验证。
    /// 2. `sync::assert_c2` 仅 debug/test 求值，release 门禁不验证。
    /// 3. Cortex-M 见上述 `icsr as u8` 截断与 `irqn - 16` 下溢；新增 M 核口
    ///    须复核最大 IRQ 号 <256。
    /// 4. gd32/cm32m4 的 `_start_trap` 异常路径不计数；0x7ED 真机行为未核实；
    ///    "挂起帧不变量"是维护红线。
    /// 5. Cortex-R5 的 FIQ 未纳入判据；SVC yield 按任务上下文计。
    /// 6. RISC-V 计数器宏按 riscv32 槽宽（4 字节），riscv64 口须改槽宽；
    ///    计数器失衡会回绕/下溢成永久 ISR（响亮失败）。
    fn in_isr() -> bool {
        false
    }
    /// 开全局中断
    fn enable_interrupt();
    /// 关全局中断
    fn disable_interrupt();
    /// 启动调度器
    fn start_scheduler() -> !;
    /// 软中断
    fn irq();
    /// 关闭软中断
    fn disable_irq();
    /// 获取systick
    fn systick() -> u64;
    /// 硬件延时，单位us
    fn delay_us(us: u64);
    /// 保存任务环境到任务栈
    fn save_context(task: &mut Task);

    // ---- SMP 扩展面(第 25 章改造路线②③)----
    // 三个方法全部带单核默认实现:现有各口零改动、行为逐字不变;
    // 多核口按需覆盖。见书稿第 25 章。

    /// 当前核(hart)ID——SMP 口按 mhartid;单核口恒 0(默认)
    #[inline]
    fn hart_id() -> u16 {
        0
    }
    /// 参与调度的物理核数——单核恒 1(默认)。
    /// 注意:这是"内核调度会使用的核数",不是硅片上的核数;
    /// hart0-only 双核起跑阶段仍返回 1
    #[inline]
    fn core_count() -> u16 {
        1
    }
    /// 向指定核发软中断(IPI)——默认退化为本核 `irq()`;
    /// SMP 口按目标核寻址(CLINT MSIP 本就是 per-hart 寄存器)
    #[inline]
    fn irq_to(hart: u16) {
        let _ = hart;
        Self::irq();
    }
    /// 启动从核参与调度——单核默认空操作;多核口唤醒停泊的从核
    /// (在 `start_scheduler` 之前由调度器调用,此刻就绪队列/每核 idle 已就绪)
    #[inline]
    fn start_secondary_cores() {}

    // ---- tickless 动态节拍扩展面(第 29 章)----
    // 六个方法全部带"恒定节拍"默认实现:现有各口零改动、行为逐字不变
    // (idle 会自旋等中断,与旧 `loop {}` 等价);要省电的口按需覆盖。
    // 见书稿第 29 章。

    /// 本口是否支持 tickless 动态节拍(一次性节拍定时器 + wfi 等待)
    #[inline]
    fn tickless_supported() -> bool {
        false
    }
    /// 把节拍定时器重装为"delta 拍后触发一次"的一次性模式:
    /// 到时产生一次节拍中断(与恒定节拍的重装相对),由中断路径实测
    /// 时长跳账(`scheduler::systick_jump`)。默认实现按恒定节拍语义
    /// 忽略——节拍恒在,无"武装"概念
    #[inline]
    fn tickless_arm_delta(_delta_ticks: u64) {}
    /// 停掉节拍定时器(无期限可睡时):冻结 tick 计数——tick() 是运行时
    /// 时钟,冻结是正确的语义(墙钟仍走 `systick()`);被外部中断唤醒
    /// 后由 idle 重新决策并再次武装
    #[inline]
    fn tickless_stop_timer() {}
    /// 睡眠等待中断(实现 wfi/wfe):被任意已使能中断唤醒即返回。
    /// 默认实现空操作——调用方(恒定节拍口)不会走到这里
    #[inline]
    fn tickless_wait() {}
    /// 即将离开本核 idle 时的回调(调度器在会切出 idle 前调用):
    /// tickless 口在此把刚睡过的一段的账按实测补上(`systick_jump(el)`,
    /// 只补整拍,子拍不记),并把节拍定时器拨回恒定节拍——否则"睡眠中
    /// 被外部中断早醒 → 有任务运行 → idle 重新武装"会把新武装锚在
    /// 冻结的 TICKS 上,墙钟期限被每个清醒片段整体拖后;任务运行期也
    /// 失去逐拍时间片。默认实现空操作(恒定节拍口无此状态)
    #[inline]
    fn tickless_leave_idle() {}
    /// 恢复恒定节拍兜底自旋前的"节拍恢复":tickless 引擎停表(睡眠)
    /// 期间若应用/ISR 把开关关掉,自旋等待的正是被 mask 掉的节拍中断
    /// ——不恢复就整机饿死。实现应自检幂等(未停表时零成本返回)。
    /// 默认实现空操作(恒定节拍口表从未停过)
    #[inline]
    fn tickless_resume_periodic() {}
}

/// host 测试用移植层 mock。
/// 仅在 `cfg(test)` 下编译。单线程测试语义下：
/// - `irq`/`save_context` 等为空调用，因为 host 不做真实任务切换；
/// - `systick`/`delay_us` 给确定值，避免测试依赖真实时钟。
#[cfg(test)]
pub struct HostPorting;

/// host 测试临界区锁。`cargo test` 默认多线程并行跑测试，仅靠"测试不并发访问全局状态"
/// 的口头约定太脆：一旦将来某个测试驱动了全局队列/TICKS，就会在测试线程间静默数据竞争。
/// 用进程内互斥锁给临界区提供真实互斥。**不可重入的是这个原语本身**
/// （`Porting::free`），不是 `sync::free`：后者按线程/每核深度计数、可嵌套
/// （见 `sync/critical.rs` 的 `nested_free_does_not_deadlock`）。
#[cfg(test)]
static HOST_TEST_CS_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(test)]
impl Portable for HostPorting {
    fn barrier() {}
    fn free<F, R>(f: F) -> R
    where
        F: FnOnce(&CriticalSection) -> R,
    {
        // SAFETY: host 上无真实中断可屏蔽，CriticalSection 仅为 API 形状匹配的标记；
        // 互斥由上面的进程内锁提供。锁被 panic 毒化后继续取用，避免连锁 panic 掩盖首个失败。
        let _guard = HOST_TEST_CS_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        f(unsafe { &CriticalSection::new() })
    }
    fn enable_interrupt() {}
    fn disable_interrupt() {}
    fn start_scheduler() -> ! {
        unimplemented!("host 测试不启动调度器")
    }
    fn irq() {}
    fn disable_irq() {}
    fn systick() -> u64 {
        0
    }
    fn delay_us(_us: u64) {}
    fn save_context(_task: &mut Task) {}
}

/// 移植层默认实现
pub type DefaultPorting = ();

/// 默认实现
impl Portable for DefaultPorting {
    /// 完全内存屏障
    /// 保证在屏障之前的任何存储操作先于屏障之后的代码执行。
    fn barrier() {
        unimplemented!()
    }
    fn free<F, R>(_f: F) -> R
    where
        F: FnOnce(&CriticalSection) -> R,
    {
        unimplemented!()
    }

    /// 开全局中断
    fn enable_interrupt() {
        unimplemented!()
    }
    /// 关全局中断
    fn disable_interrupt() {
        unimplemented!()
    }
    /// 启动调度器
    fn start_scheduler() -> ! {
        unimplemented!()
    }
    /// 开启软中断
    fn irq() {
        unimplemented!()
    }
    /// 关闭软中断
    fn disable_irq() {
        unimplemented!()
    }
    /// 获取rtc tick
    fn systick() -> u64 {
        unimplemented!()
    }
    /// 硬件延时，单位us
    fn delay_us(_us: u64) {
        unimplemented!()
    }
    /// 保存任务环境到任务栈
    fn save_context(_task: &mut Task) {
        unimplemented!()
    }
}

#[cfg(test)]
mod in_isr_tests {
    use super::{DefaultPorting, HostPorting, Portable};

    /// host mock 不模拟中断上下文：默认 false 是 R8 的诚实边界。
    #[test]
    fn host_porting_reports_thread_context() {
        assert!(!<HostPorting as Portable>::in_isr());
    }

    /// 兜底桩同样保持默认 false——不得为"看起来覆盖"给一个猜的值。
    #[test]
    fn default_porting_reports_thread_context() {
        assert!(!<DefaultPorting as Portable>::in_isr());
    }
}
