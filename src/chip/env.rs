/// 移植环境配置参数

/// 定时控制器基地址
#[cfg(feature = "gd32vf103")]
pub const TIMER_CTRL_ADDR: usize = 0xD100_0000;
/// 中断控制器基地址
#[cfg(feature = "gd32vf103")]
pub const ECLIC_CTRL_ADDR: usize = 0xD200_0000;
/// CPU时钟频率
#[cfg(feature = "gd32vf103")]
pub const CPU_CLOCK_HZ: usize = 108_000_000;
/// SYSTICK时钟频率
#[cfg(feature = "gd32vf103")]
pub const SYSTICK_CLOCK_HZ: usize = 108_000_000 / 4;

/// 定时控制器基地址
#[cfg(feature = "cm32m4")]
pub const TIMER_CTRL_ADDR: usize = 0xE002_0000;
/// 中断控制器基地址
#[cfg(feature = "cm32m4")]
pub const ECLIC_CTRL_ADDR: usize = 0xE001_0000;
/// CH32V307(QingKe V4F):复位默认 HSI 8MHz——本口无 BSP/时钟初始化,
/// 诚实按默认配;PLL(最高 144M)配置后须同步改这里与 STCLK 语义
#[cfg(feature = "ch32v307")]
pub const CPU_CLOCK_HZ: usize = 8_000_000;
#[cfg(feature = "ch32v307")]
pub const SYSTICK_CLOCK_HZ: usize = 8_000_000; // STCLK=1 → HCLK(=8M)
// V203/V103 同款:复位默认 HSI 8MHz(PLL 最高 144M,配置后同步改)
#[cfg(feature = "ch32v203")]
pub const CPU_CLOCK_HZ: usize = 8_000_000;
#[cfg(feature = "ch32v203")]
pub const SYSTICK_CLOCK_HZ: usize = 8_000_000;
#[cfg(feature = "ch32v103")]
pub const CPU_CLOCK_HZ: usize = 8_000_000;
#[cfg(feature = "ch32v103")]
pub const SYSTICK_CLOCK_HZ: usize = 8_000_000;
/// CH583(QingKe V4A):默认主频取官方 `CH58x_common.h` 的 `FREQ_SYS` 默认值
/// 60MHz(⚠️ 真机核对点——本口无 BSP/时钟初始化,按官方默认配;
/// PLL 配置后须同步改这里与 STCLK 语义,见 memory.x/mod.rs 注)
#[cfg(feature = "ch583")]
pub const CPU_CLOCK_HZ: usize = 60_000_000;
#[cfg(feature = "ch583")]
pub const SYSTICK_CLOCK_HZ: usize = 60_000_000; // STCLK=1 → HCLK(=60M)
/// CH572(QingKe 无 A 扩展):默认主频取官方 `CH57x_common.h` 的 `FREQ_SYS`
/// 默认值 100MHz(⚠️ 真机核对点——本口无 BSP/时钟初始化,SAM 解锁与 PLL 未做,
/// 复位默认频率须实测校正;PLL 配置后同样同步改这里)
#[cfg(feature = "ch572")]
pub const CPU_CLOCK_HZ: usize = 100_000_000;
#[cfg(feature = "ch572")]
pub const SYSTICK_CLOCK_HZ: usize = 100_000_000; // STCLK=1 → HCLK(=100M)
// ESP32-C3:复位默认 CPU 80MHz(TRM;PLL 160M 配好后同步改);
// SYSTICK 是独立 16MHz 时基(与 CPU 时钟无关—— 调研已核)
#[cfg(feature = "esp32c3")]
pub const CPU_CLOCK_HZ: usize = 80_000_000;
#[cfg(feature = "esp32c3")]
pub const SYSTICK_CLOCK_HZ: usize = 16_000_000;
// QEMU RISC-V virt 机:timebase 10MHz(mtime/mtimecmp 时基;核频 TCG 虚拟)
#[cfg(feature = "qemu_riscv")]
pub const CPU_CLOCK_HZ: usize = 10_000_000;
#[cfg(feature = "qemu_riscv")]
pub const SYSTICK_CLOCK_HZ: usize = 10_000_000;

/// CPU时钟频率
#[cfg(feature = "cm32m4")]
pub const CPU_CLOCK_HZ: usize = 144000000;
/// SYSTICK时钟频率
#[cfg(feature = "cm32m4")]
pub const SYSTICK_CLOCK_HZ: usize = 144000000 / 4;

/// CPU时钟频率
#[cfg(feature = "stm32f4")]
pub const CPU_CLOCK_HZ: usize = 84_000_000;
#[cfg(feature = "stm32f4")]
pub const SYSTICK_CLOCK_HZ: usize = 84_000_000;

#[cfg(feature = "stm32f1")]
pub const CPU_CLOCK_HZ: usize = 8_000_000;
#[cfg(feature = "stm32f1")]
pub const SYSTICK_CLOCK_HZ: usize = 8_000_000;

#[cfg(feature = "stm32h7")]
pub const CPU_CLOCK_HZ: usize = 280_000_000;
#[cfg(feature = "stm32h7")]
pub const SYSTICK_CLOCK_HZ: usize = 24_000_000;

#[cfg(feature = "rp2040")]
pub const CPU_CLOCK_HZ: usize = 125_000_000;
#[cfg(feature = "rp2040")]
pub const SYSTICK_CLOCK_HZ: usize = 125_000_000; // SysTick clock_source=Core;原 1M 是错的(时间快 125 倍)

/// ⚠️ 时钟契约:SYSTICK_CLOCK_HZ 必须 == SysTick 的实际输入时钟
/// (ARM 口 clock_source(Core) 即 HCLK;示例 freeze() 的实际配置决定它——
/// 常数与示例时钟不一致 = tick/delay 整体漂移,f4/f1 口都栽过:原 f4 写
/// 180M/24M(f427 遗留)而示例默认 HSI 16M;原 f1 写 74M(72M 笔误)而
/// 示例默认 HSI 8M。现在:f4=84M(示例显式 sysclk(84M));f1=8M(默认 HSI)。
/// 改示例时钟时必须同步改这里。

/// 每秒产生多少次中断，没一次中断间隔就是任务能获得的时间片
pub const TICK_CLOCK_HZ: usize = 1000;

/// 软件定时器任务栈大小（单位：字长）。
///
/// 默认 1024 字(4K)——定时器任务要走 `do_tick` → 软定时器堆 → 唤醒链,栈较深。
/// **CH572(12K SRAM)按 256 字(1K)**:它还要与 idle 任务、应用任务共挤 12K
/// (见 `examples/multitask_ch572.rs` 的实测账);256 字是否够由 QEMU 口的
/// 24 项内核自测(其中多项专测软定时器/延时)与栈围栏守卫兜住。
#[cfg(not(feature = "ch572"))]
pub const TIMER_STACK_SIZE_WORD: usize = 1024;
#[cfg(feature = "ch572")]
pub const TIMER_STACK_SIZE_WORD: usize = 256;

/// idle 任务栈大小（单位：字长）。
///
/// 默认 512 字(2K)——idle 走 tickless 决策/让出链。**CH572(12K SRAM)按 256 字**,
/// 理由同上(12K 上 idle 独占 2K 太奢侈)。
#[cfg(not(feature = "ch572"))]
pub const IDLE_STACK_SIZE_WORD: usize = 512;
#[cfg(feature = "ch572")]
pub const IDLE_STACK_SIZE_WORD: usize = 256;

/// 内核数组定界用的**核数上限**——实际参与调度的核数由 `Porting::core_count()`
/// 运行期决定,恒 ≤ 本值;`CURRENT_TASK`/`IDLE_TASKS`/`READYQ`/`READY_BITS`
/// 等每核数组一律按它定界。
///
/// **按芯片取值**:WCH 各口(ch32v103/203/307、ch583、ch572)都是单核,`READYQ`
/// 的"每核 × 16 优先级桶"里那个"每核"维度留 1 份就够——按默认 16 算,它白占
/// `16 × 16 × 16B = 4K`(实测符号大小 `READYQ = 0x1000`),在 12K RAM 的
/// **CH572 上等于三分之一的 RAM**。多核口(qemu_riscv 的 SMP)保持 16。
#[cfg(any(feature = "ch572", feature = "ch583"))]
pub const MAX_HARTS: usize = 1;
#[cfg(not(any(feature = "ch572", feature = "ch583")))]
pub const MAX_HARTS: usize = 16;
