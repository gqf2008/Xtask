# Xtask

可移植多任务调度内核，用于嵌入式环境，与应用程序一起编译打包，参考 FreeRTOS 实现。

[多任务调度原理分析][xtask]

### 说明

1. 软件还在开发中，API 也相当原始且不稳定
2. 关于工作原理，代码（包括汇编）中有详细说明，请先阅读

### 主要功能

- [x] 单物理线程任务优先级+时间片调度机制，高优先级抢占，同优先级公平调度
- [x] 堆内存分配器
- [x] 二值信号量、计数信号量、信号广播、互斥锁、任务通知
- [x] 多生产者多消费者队列
- [x] 临界段
- [x] 栈溢出检查
- [x] PubSub 模式消息总线
- [x] 软件定时器
- [x] 驱动抽象层：`Device` 顶层抽象 + 五能力 trait（流/块/控制/总线/事件），注册表与编译期清单只认 `&'static dyn Device`，"名字就是总线"
- [x] 基于 FatFS 文件系统
- [x] 基于 smoltcp 网络协议栈

### 验证体系

- [x] 宿主回归：`cargo test --lib` 默认 features 实测 **80 passed**;门禁全特征组合(见 `ci/gate.sh`,含 fs/net/usb/ble)实测 **159 passed**(2026-09-03,rustc 1.97;旧计数 148 已按实测修正,历史见 issue #11;计数随新增测试浮动,以 `ci/gate.sh` 实测为准)
- [x] QEMU 执行级：`ci/gate.sh` 在 virt 机真跑内核——`qemu_pingpong` 200 轮乒乓 + `qemu_kernel_tests` 24 项全内核机制自测(抢占/时间片/阻塞类 IPC/定时器/时基/堆/总线/任务回收/可重入锁/优先级继承/完整 PI 多锁/PCP 天花板阻塞/PI 交叉持锁死锁确认/TLSF 碎片共限/TLSF 分配确定性/tickless 错峰唤醒/远期期限单次到点/UART RX 外部中断冻眠唤醒/早醒弹墙钟拍账/噪声风暴停留 idle 不漂移),全绿自退出;另有 tlsf 全局后端门禁(24/24 不变 = 分配器换引擎对内核透明)与 `qemu_smp` 9 项多核调度门禁
- [ ] 真机验证：gd32vf103 已验；f4/f1 常数、h7 时序、cm32m4、rp2040、ch32 系、esp32c3 待上板

### 移植的芯片

- RISCV
  - [x] GD32VF103xx
  - [x] qemu_riscv: QEMU virt 机(标准 CLINT+NS16550;**执行级验证** 2026-08-24——15 项自测 ×10 连稳定,2026-08-26 扩至 23 项(新增 PCP 天花板阻塞、PI 交叉持锁死锁确认、TLSF 碎片共限、TLSF 分配确定性、tickless 错峰唤醒、远期期限单次到点、UART RX 外部中断冻眠唤醒 + 早醒弹墙钟拍账——ch29 章末练习 1 兑现,stdin 喂字节握手),2026-09-02 扩至 24 项(噪声风暴停留 idle 不漂移——tickless 踩坑 5 下半场专项守卫);**SMP 多核执行验证** 2026-08-26——`smp::enable()` 显式开启,从核参与调度,qemu_smp 9 项在 -smp 2/3/4/8 全绿;`TaskBuilder::affinity` 绑核确定性放置)
  - [x] CM32M4xxR(RISC-V/N308;构建级验证 2026-08-23,真机待验)
  - [x] ESP32C3: esp32c3(PAC 直依赖;构建级验证 2026-08-23,真机待验——启动需 direct boot/镜像头)
  - [x] CH32V3: ch32v307(QingKe V4F;构建级验证 2026-08-23,真机待验)
  - [x] CH32V2: ch32v203(QingKe V4B;构建级验证 2026-08-23,真机待验)
  - [x] CH32V1: ch32v103(QingKe V3A;构建级验证 2026-08-23,真机待验)
- CM7F
  - [x] STM32H7B0VBT6
- CM4F
  - [x] STM32F401CCU6
  - [x] STM32F427VIT6
  - [x] STM32F411CEU6
- CM3
  - [x] STM32F103C8T6
- CM0+
  - [x] RP2040(rp2040-hal 0.9;构建级验证 2026-08-23,真机待验)
- CR5F
  - [x] qemu_arm_r52: QEMU xlnx-zcu102 的 Cortex-R5F(armv7r-none-eabi;**执行级验证** 2026-08-30——200 轮乒乓 + tick 心跳 + VFP 帧 100 轮跨切换保持,补丁版 QEMU 跑通;**CI 执行级门禁** `.github/workflows/r52.yml`——补丁版 QEMU 构建 + 双例程断言全绿;无 PendSV 架构的「中断借道 + 调度循环」移植,方法论见书稿第 31/32 章;真机待验)

### 一键复现(读者零安装)

- **通道 A · 本机一条命令**(首次自动下载静态 QEMU 到 `.tools/`,约 90MB):
  ```bash
  bash ci/gate.sh        # host 回归 + 示例链接 + QEMU 执行级(24/24 等)
  ```
- **通道 B · GitHub Codespaces**:仓库根有 `.devcontainer.json`,云端打开即跑,本机什么都不装
- **通道 C · 只看结果**:每次 push 的 CI 门禁(含 QEMU 执行级)全绿记录:
  [![main-gate](https://github.com/gqf2008/Xtask/actions/workflows/main-gate.yml/badge.svg)](https://github.com/gqf2008/Xtask/actions/workflows/main-gate.yml)
- 只想跑 host 单测(不需要 QEMU):注意仓库 `.cargo/config.toml` 默认 `thumbv7em` target,
  请显式指定宿主或先 `rustup target add thumbv7em-none-eabihf`:
  ```bash
  cargo test --lib --target "$(rustc -vV | sed -n 's/^host: //p')"
  ```

### 快速开始

没有开发板也能跑:QEMU 执行级验证不需要任何硬件(安装 QEMU 后)

```bash
cargo build --example qemu_kernel_tests --features qemu_riscv,timer --target riscv32imac-unknown-none-elf --release
# 测试 22/23(冻眠唤醒/早醒拍账)需经 qemu stdin 喂字节——内核打
# 握手标记、主机读到才写(ci/feed_qemu.py,时序零假设):
python ci/feed_qemu.py 180 qemu-system-riscv32 \
  target/riscv32imac-unknown-none-elf/release/examples/qemu_kernel_tests
# 期望输出 24/24 passed,进程以退出码 0 自行结束

# 全局分配器换引擎(tlsf feature:整个内核的堆分配走手写迷你 TLSF):
cargo build --example qemu_kernel_tests --features qemu_riscv,timer,tlsf --target riscv32imac-unknown-none-elf --release
# 同上用 feed_qemu.py 跑,期望仍是 24/24 passed——分配器后端对内核透明(第 28 章)

# SMP 多核执行验证(应用 smp::enable() 后从核参与调度):
cargo build --example qemu_smp --features qemu_riscv,timer --target riscv32imac-unknown-none-elf --release
qemu-system-riscv32 -M virt -smp 2 -nographic -bios none -kernel \
  target/riscv32imac-unknown-none-elf/release/examples/qemu_smp
# 期望输出 smp PASS: 9/9(双核并行/跨核唤醒/锁与堆压力/节拍推进/绑核放置/定时器跨核/每核 runqueue 负载均衡)
# 同一 ELF 支持 -smp 2..8(核数梯度回归:-smp 2/3/4/8 全绿)
```

如果您有一块 longan-nano 或者 stm32f401ccu6 或者 stm32f103c8t6 最小系统板，那么[example](https://github.com/gqf2008/xtask/tree/master/examples)中的例子直接可以跑起来

![多任务调试1](debug/gd32vf103/debug1.png)![多任务调试2](debug/gd32vf103/debug2.png)

1. 打开一个终端

   - cd debug/gd32vf103
   - ./openocd.sh

2. 串口调试

   - 把调试器插到 usb 口
   - window 平台需要你找一款串口助手连接串口即可
   - linux 或者 mac 平台执行 screen /dev/$串口设备 57600

3. 打开另外一个终端

   - [x] gd32vf103: --target riscv32imac-unknown-none-elf
   - [x] stm32f4: --target thumbv7em-none-eabihf
   - [x] stm32f1: --target thumbv7m-none-eabi
   - [x] rp2040: --target thumbv6m-none-eabi(rp2040-hal 0.9 复活;构建级验证 2026-08-23,真机待验)
   - [x] qemu_riscv: QEMU virt 机(标准 CLINT+NS16550;**执行级验证**——见上文"验证体系")

   - 信号广播示例

     - cargo run --example broadcast --features=gd32vf103 --target riscv32imac-unknown-none-elf --release

   - LED 示例，三个任务分别控制三色 LED

     - cargo run --example led --features=gd32vf103 --target riscv32imac-unknown-none-elf --release

   - 多任务切换示例，5 个任务，4 个任务循环一段时间后退出

     - cargo run --example multitask --features=gd32vf103 --target riscv32imac-unknown-none-elf --release

   - 通知示例，一个通知另外一个

     - cargo run --example notify --features=gd32vf103 --target riscv32imac-unknown-none-elf --release

   - 队列示例，两个发，三个收消息

     - cargo run --example queue --features=gd32vf103 --target riscv32imac-unknown-none-elf --release

   - 信号量示例，两个发，三个收

     - cargo run --example semaphore --features=gd32vf103 --target riscv32imac-unknown-none-elf --release

   - 消息服务总线示例，

     - cargo run --example evbus --features=gd32vf103 --target riscv32imac-unknown-none-elf --release

   - 软件定时器
     - cargo run --example timer --features=gd32vf103,timer --target riscv32imac-unknown-none-elf --release

4. 如果您能在终端看到任务工作时的日志输出，恭喜您已经成功了

### 移植层接口

```rust

/// 移植层接口定义
pub trait Portable {
    /// 完全内存屏障
    /// 保证在屏障之前的任何存储操作先于屏障之后的代码执行。
    fn barrier();
    /// 临界区保护函数
    fn free<F, R>(f: F) -> R
    where
        F: FnOnce(&CriticalSection) -> R;
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

    // ---- SMP 扩展面(第 25 章改造路线②③,全部带单核默认实现)----
    /// 当前核(hart)ID——SMP 口按 mhartid;单核口恒 0(默认)
    fn hart_id() -> u16 {
        0
    }
    /// 参与调度的物理核数——单核恒 1(默认)
    fn core_count() -> u16 {
        1
    }
    /// 向指定核发软中断(IPI)——默认退化为本核 irq();SMP 口按目标核寻址
    fn irq_to(hart: u16) {
        let _ = hart;
        Self::irq();
    }
    /// 启动从核参与调度——单核默认空操作;多核口唤醒停泊的从核
    fn start_secondary_cores() {}
}

```

[xtask]: Xtask.md

## 版权与授权

- 本仓库**代码**遵循 MIT 协议（见 LICENSE），可自由使用、修改、商用，请保留版权声明。
- 作者正在撰写基于本仓库的专著《用 Rust 手写 RTOS 内核》（书稿文字独立于 MIT 授权，未经许可不得复制/出版）。
- 若你计划基于本仓库编写书籍、教程、课程或深度文章：欢迎先通过 GitHub Issue / Discussions 联系作者，可提供内容审阅与联合署名，避免与在写专著内容冲突。
