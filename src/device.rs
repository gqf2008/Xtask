//! 设备驱动抽象层：`Device` 顶层抽象 + 开放能力 trait 集 + 按名注册表
//!
//! 模型（依据 `docs/设备驱动抽象层设计.md` 定稿，对照 Unix char/block/ioctl
//! 三分法与 RT-Thread 的 control 基座；查找机制沿用 Zephyr binding）：
//! - **`Device` 顶层 trait**：`kind()` 分类标签 + 五个 `as_*` 能力查询（默认
//!   返回 None）。注册表/编译期清单只持有 `&'static dyn Device`，不感知具体
//!   设备类——新增能力 = 新 trait + 新 `as_*` 默认方法，表本体一行不改；
//! - **五个能力 trait**：`StreamDevice`（字节流：UART/USB-CDC/SLIP…）、
//!   `BlockDevice`（块：SD/Flash/内存盘…）、`Control`（ioctl 控制面：LED/参数
//!   配置/厂商命令…）、`BusDevice`（同步字节事务：SPI/I2C…）、`EventDevice`
//!   （事件/唤醒：阻塞 I/O 的 ISR 唤醒通道）；
//! - **阻塞读由内核适配器承接**：`read_blocking` 把 `StreamDevice + EventDevice`
//!   组合出"等待即 Blocked"（复查+登记+挂起在同一临界区），trait 本体保持
//!   非阻塞形态（对齐 embedded-io / Zephyr `uart_poll_in` 的真机实践）；
//! - **两张表**：运行期 [`DriverRegistry`]（教学：模型透明、可演示热注册）与
//!   编译期 [`table::DeviceTable`]（真实内核心态：零运行期注册决策），共用
//!   同一种句柄，只换"表"的实现。
//!
//! 与 embedded-hal 的分工（不变）：引脚/串口的寄存器级操作属于芯片 HAL；
//! 本层补的是**按名找设备**（生产者与消费者解耦）与**统一的设备级契约**
//! （对象安全、统一 `DeviceError`、`&self` + 内部可变、`Sync` 上界）。
//!
//! 纪律（与 `bus.rs` 同构）：
//! - 表内部状态全部包在 `RefCell`/`UnsafeCell` 里，所有访问都在 `sync::free`
//!   临界区内（单核关中断模型下不可能有数据竞争），`unsafe impl Send/Sync`
//!   是 sound 的；
//! - 设备句柄是 `&'static dyn Device`：实例由创建方 `Box::leak` 或静态存储
//!   保证与系统等长——表只登记"谁是谁"，不拥有设备；
//! - trait 方法全 `&self`：表分发的是共享引用，设备内部用 `RefCell`/原子
//!   自持可变状态（与内核 `Queue`/`Notifier` 同款）。

use core::cell::RefCell;

use crate::sync;
use crate::task::executor::{xworker, Executor};
use crate::task::{yield_now, Task};

pub mod table;

// ---------------------------------------------------------------------------
// 分类标签与统一错误
// ---------------------------------------------------------------------------

/// 设备分类标签——只回答"它是什么"，不携带任何能力。
/// 消费方：诊断日志 / 设备清单 / 板级匹配；**不参与能力分发**（能力走 `as_*`）。
/// 复合设备以"主类"报告（如 UART 报 Stream，尽管它同时有 Control/Event 能力）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceKind {
    /// 字节流设备：UART / USB CDC / BLE 数据通道……
    Stream,
    /// 块设备：SD 卡 / Flash / 内存盘……
    Block,
    /// 纯控制设备：LED / PWM / GPIO 控制面……
    Control,
    /// 总线设备：SPI / I2C / I3C 同步事务……
    Bus,
}

/// 统一设备错误（全局一份：注册错误与 I/O 错误同枚举，仿 Zephyr errno 全局
/// 命名空间——上层 fs/net/应用得以一致、可穷尽地处理错误）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceError {
    /// 注册时名字重复（名字全局唯一，跨类同名也算）
    Duplicate,
    /// 注册表已满（定长表，容量见 [`DEV_CAP`]）
    Full,
    /// 参数非法（扇区越界 / seek 越界 / 未知控制命令）
    InvalidInput,
    /// 控制命令 / 能力不被支持（`as_*` 返回 None 的设备、未实现的 op）
    Unsupported,
    /// 底层 IO 错误（坏扇区等硬件级失败）
    Io,
    /// 操作超时
    Timeout,
    /// 读到卷尾（`fatfs::io::IoError::new_unexpected_eof_error` 的对应物）
    UnexpectedEof,
    /// `write_all` 进度停滞（`fatfs::io::IoError::new_write_zero_error` 的对应物）
    WriteZero,
}

// ---------------------------------------------------------------------------
// 顶层抽象与五个能力 trait
// ---------------------------------------------------------------------------

/// 所有设备的公共入口。
///
/// `Sync` 是硬约束（沿用旧 `BdDevice: Sync` 的论证）：设备句柄以
/// `&'static dyn Device` 在任务间共享（文件系统链把它塞进静态
/// `Mutex<Option<...>>`），`&dyn Device` 要 `Send` 必须先 `Device: Sync`。
/// 实现方用 `RefCell`/原子自持可变状态；`RefCell` 设备需 `unsafe impl Sync`
/// 并给出 SAFETY 论证（所有访问在 `sync::free` 临界区内，见 BSP 各驱动）。
pub trait Device: Sync {
    /// 设备类别（主类标签；消费方是诊断/清单，不参与能力分发）
    fn kind(&self) -> DeviceKind;

    /// 取字节流能力，不支持返回 None
    fn as_stream(&self) -> Option<&dyn StreamDevice> {
        None
    }

    /// 取块设备能力，不支持返回 None
    fn as_block(&self) -> Option<&dyn BlockDevice> {
        None
    }

    /// 取控制面能力，不支持返回 None
    fn as_control(&self) -> Option<&dyn Control> {
        None
    }

    /// 取总线能力，不支持返回 None
    fn as_bus(&self) -> Option<&dyn BusDevice> {
        None
    }

    /// 取事件/唤醒能力，不支持返回 None
    fn as_event(&self) -> Option<&dyn EventDevice> {
        None
    }
}

/// 字节流设备（数据面 = 连续字节流，边界由上层协议管理）。
///
/// 对齐 `embedded_io::Read/Write` 的形状；因注册表返回的是共享 `&'static dyn`，
/// 本内核用 `&self` + 内部可变性而不是 `&mut self`。
pub trait StreamDevice: Device {
    /// 接收缓冲中当前可读字节数（非阻塞探测）。
    /// **lock-free 硬约束**：实现必须纯原子操作、不得自取临界区——内核阻塞
    /// 适配器 `read_blocking` 会在一段 `sync::free` 内调用它（宿主端 `free`
    /// 是不可重入进程互斥锁，方法内再取锁即死锁）。
    fn available(&self) -> usize;

    /// 非阻塞读：最多读 `buf.len()`，返回实际字节数；无数据返回 `Ok(0)`。
    /// 阻塞读（"等待即 Blocked"）不放进 trait——由 [`read_blocking`] 适配器
    /// 组合 `EventDevice` 实现。
    fn read(&self, buf: &mut [u8]) -> Result<usize, DeviceError>;

    /// 写：可短暂阻塞等待流控（等 TXE / FIFO 空位，微秒级，不值得进任务
    /// 状态机），返回实际写入数；非空缓冲通常写满。
    fn write(&self, buf: &[u8]) -> Result<usize, DeviceError>;

    /// 写满整个缓冲区。空缓冲直接 `Ok(())`；循环 `write` 期间某次返回 0
    /// 且尚未写完 → `Err(WriteZero)`（防御性停滞检查）。
    fn write_all(&self, buf: &[u8]) -> Result<(), DeviceError> {
        if buf.is_empty() {
            return Ok(());
        }
        let mut done = 0;
        while done < buf.len() {
            let n = self.write(&buf[done..])?;
            if n == 0 {
                return Err(DeviceError::WriteZero);
            }
            done += n;
        }
        Ok(())
    }
}

/// 块设备（数据面 = 随机访问的定长块）——文件系统的地基只有读/写扇区两个原语。
///
/// **单使用者契约**（沿用旧 `BdDevice`）：块设备事务（一条命令 + 一个扇区的
/// 数据）是协议原子单元，同一时刻只允许一个任务操作——第 21 章的用法是所有
/// 访问经同一把文件系统互斥锁串行化；违反契约由设备内部的 `RefCell` 双重
/// 借用 panic 探测。
pub trait BlockDevice: Device {
    /// 逻辑扇区大小（字节）。SD 卡为 512；`fs::FatAdapter` 当前断言只支持 512。
    fn sector_size(&self) -> u64;

    /// 设备总扇区数；0 = 无介质/未就绪
    fn sector_count(&self) -> u64;

    /// 读一个扇区到 `buf`；`buf.len()` 必须等于扇区大小
    fn read_sector(&self, no: u64, buf: &mut [u8]) -> Result<(), DeviceError>;

    /// 写一个扇区；`buf.len()` 必须等于扇区大小
    fn write_sector(&self, no: u64, buf: &[u8]) -> Result<(), DeviceError>;
}

/// 控制面（ioctl 风格）：一切非数据路径的操作——参数配置、状态查询、开关、
/// 厂商私有命令——都走这一个出口。
///
/// **`arg` 的指针/标量双义约定**（Linux `ioctl` 第三参 `unsigned long` 的
/// Rust 直译，双义由每个 op 的 ABI 契约定义）：
/// - 标量一律 ≤u32 直接传值（本内核全是 32 位目标，`usize` 装不下 u64——
///   SD 扇区号 / Flash 擦除地址等 64 位参数必须走指针路径）；
/// - 指针路径：调用侧 `&mut buf as *mut _ as usize` 是 safe 的；设备侧
///   `unsafe` 重建引用，**unsafe 全关在设备实现内**，每个 `unsafe` 块必须带
///   `// SAFETY:` 注释写明该 op 的契约（类型/长度/方向），误用即 UB；
/// - `arg` 指针仅在 `control` 调用期间有效，**不得留存**（不支持把缓冲
///   交给 DMA 事后使用）；
/// - op 编码建议：简化版 Linux `_IOC`——高 16 位族 magic + 低 16 位命令
///   （可加方向/尺寸位），让"指针还是标量"在命令码上可判读。
///
/// 返回值：查询型 op 返回设备写回的字节数/数值；设置型返回 `Ok(0)`；
/// 未知 op 返回 `Err(InvalidInput)`；设备不支持控制面则不实现本 trait
/// （`as_control` 默认 None）。
pub trait Control: Device {
    /// 执行控制指令；`op`/`arg` 语义由设备族自定（见 trait 级约定）
    fn control(&self, op: u32, arg: usize) -> Result<usize, DeviceError>;
}

/// 总线传输能力（同步字节事务：SPI / I2C / I3C）。
///
/// - SPI：片选 / 时钟相位 / 极性由具体驱动内部持有；
/// - I2C/I3C：**每个 (总线, 从机地址) 建模为一个 `BusDevice` 实例**，地址在
///   构造期绑定（不可变状态），消除"先设地址再传输"的跨任务竞态；寄存器
///   地址走 tx 首字节负载（读寄存器 R = `transfer(&[R], &mut data)`，两段
///   时序由总线驱动内部拼接）。
pub trait BusDevice: Device {
    /// 一次传输：发送 `tx`，同时把读入字节写入 `rx`。
    /// `rx` 为空表示只写；`tx` 为空表示只读（具体时序由总线驱动决定）。
    fn transfer(&self, tx: &[u8], rx: &mut [u8]) -> Result<(), DeviceError>;
}

/// 事件/唤醒设备：为阻塞 I/O 提供 ISR 唤醒通道。
///
/// 契约（每一条都有现 `drv_uart` 的踩坑记录背书）：
/// - **单等待者**：重复 `register_waiter` 覆盖旧等待者（与 Uart0 的原子
///   `store` 一致）；
/// - **lock-free 硬约束**：`register_waiter` 必须纯原子操作、不得自取临界区——
///   内核阻塞适配器 [`read_blocking`] 会在同一段 `sync::free` 内连续调用
///   `available()` 与它（宿主端 `free` 不可重入，再取锁即死锁）；
/// - `register_waiter` 由内核阻塞适配器使用，应用不直接调用；`wake` 由
///   设备侧（ISR / 状态变化）调用。
pub trait EventDevice: Device {
    /// 登记一个等待者（任务指针的地址值）；数据/状态就绪时由设备负责唤醒
    fn register_waiter(&self, waiter: usize) -> Result<(), DeviceError>;
    /// 唤醒当前等待者（无等待者时为空操作）
    fn wake(&self);
}

// ---------------------------------------------------------------------------
// 通用阻塞适配器："等待即 Blocked"
// ---------------------------------------------------------------------------

/// 阻塞读：把 `StreamDevice + EventDevice` 组合出"等待即 Blocked"——
/// 无数据时任务进入 `Blocked` 挂起，数据到达由设备 `wake()` 唤醒，
/// 不空转轮询、不烧 CPU。按名消费者一次 `find(name)` 即可使用。
///
/// 丢失唤醒免疫（沿用 `drv_uart` 的踩坑纪律）：**"复查 available + 登记
/// waiter + 挂起"在同一 `sync::free` 临界区内**——ISR 不可能插进"发现空"
/// 与"挂起"之间；先入队的字节必然看到 waiter 已登记。
pub fn read_blocking(dev: &dyn Device, buf: &mut [u8]) -> Result<usize, DeviceError> {
    let stream = dev.as_stream().ok_or(DeviceError::Unsupported)?;
    let event = dev.as_event().ok_or(DeviceError::Unsupported)?;
    if buf.is_empty() {
        return Ok(0); // 空读立即返回：否则"无数据"分支会把任务永远挂起
    }
    loop {
        // 快速路径：非阻塞读（无临界区）
        let n = stream.read(buf)?;
        if n > 0 {
            return Ok(n);
        }
        // 慢路径：复查 + 登记 + 挂起在同一临界区
        let ready = sync::free(|_| -> Result<bool, DeviceError> {
            if stream.available() > 0 {
                return Ok(true); // 临界区内复查到有数据：出临界区回快速路径
            }
            let t = xworker.current();
            let addr = (t as *mut Task).addr();
            event.register_waiter(addr)?;
            t.block();
            Ok(false)
        })?;
        if ready {
            continue;
        }
        // 让出 CPU：ISR wake() 唤醒后调度器把自己排回来
        yield_now();
    }
}

// ---------------------------------------------------------------------------
// 引用转发 blanket impl
// ---------------------------------------------------------------------------

/// `&T` 也是 `Device`（逐方法转发）——泛型适配器可以统一处理具体设备与
/// 表取出的 trait 对象两种形态（`FatAdapter<&'static dyn BlockDevice>` 依赖）。
impl<T: Device + ?Sized> Device for &T {
    fn kind(&self) -> DeviceKind {
        (**self).kind()
    }
    fn as_stream(&self) -> Option<&dyn StreamDevice> {
        (**self).as_stream()
    }
    fn as_block(&self) -> Option<&dyn BlockDevice> {
        (**self).as_block()
    }
    fn as_control(&self) -> Option<&dyn Control> {
        (**self).as_control()
    }
    fn as_bus(&self) -> Option<&dyn BusDevice> {
        (**self).as_bus()
    }
    fn as_event(&self) -> Option<&dyn EventDevice> {
        (**self).as_event()
    }
}

/// `&T` 也是 `BlockDevice`（沿用旧 `impl BdDevice for &T` 的转发模式）。
impl<T: BlockDevice + ?Sized> BlockDevice for &T {
    fn sector_size(&self) -> u64 {
        (**self).sector_size()
    }
    fn sector_count(&self) -> u64 {
        (**self).sector_count()
    }
    fn read_sector(&self, no: u64, buf: &mut [u8]) -> Result<(), DeviceError> {
        (**self).read_sector(no, buf)
    }
    fn write_sector(&self, no: u64, buf: &[u8]) -> Result<(), DeviceError> {
        (**self).write_sector(no, buf)
    }
}

/// 块设备逻辑扇区大小（字节）的现行基准值。SD 卡协议约定扇区 512B，
/// `FatAdapter` 断言只支持此值（栈上定长暂存依赖编译期常量）；
/// `sector_size()` 查询是前向兼容位。
pub const SECTOR_SIZE: u64 = 512;

// ---------------------------------------------------------------------------
// 运行期注册表：按名登记/查找设备（Zephyr binding 语义）
// ---------------------------------------------------------------------------

/// 注册表容量（定长数组，注册表不依赖堆；全部设备共用这一张表）
pub const DEV_CAP: usize = 8;

/// 驱动注册表：一张全局表，按名登记/查找设备。
/// 只认识 `&'static dyn Device`——能力分发走 `as_*`，表对设备类透明。
pub struct DriverRegistry {
    devices: RefCell<[Option<(&'static str, &'static dyn Device)>; DEV_CAP]>,
}

// SAFETY: 所有状态在 RefCell 里且访问全部发生在 sync::free 临界区内（单核关中断），
// ISR 与任务不可能并发访问同一借用——与 bus.rs 的 unsafe impl 同理。
unsafe impl Send for DriverRegistry {}
// SAFETY: 同上；单核模型下"共享引用"的可变访问由临界区串行化。
unsafe impl Sync for DriverRegistry {}

impl DriverRegistry {
    /// 空注册表
    pub const fn new() -> Self {
        Self {
            devices: RefCell::new([None; DEV_CAP]),
        }
    }

    /// 登记设备。重名返回 `Err(Duplicate)`，满返回 `Err(Full)`。
    /// 名字全局唯一：跨设备类同名同样算 Duplicate（Zephyr binding 语义）。
    pub fn register(
        &self,
        name: &'static str,
        dev: &'static dyn Device,
    ) -> Result<(), DeviceError> {
        sync::free(|_| {
            let mut devices = self.devices.borrow_mut();
            if devices
                .iter()
                .any(|e| matches!(e, Some((n, _)) if *n == name))
            {
                return Err(DeviceError::Duplicate);
            }
            for slot in devices.iter_mut() {
                if slot.is_none() {
                    *slot = Some((name, dev));
                    return Ok(());
                }
            }
            Err(DeviceError::Full)
        })
    }

    /// 按名查找设备（通用入口；拿到后用 `as_*` 取能力，或用下面的
    /// `find_stream`/`find_block`/… 直接取能力句柄）。
    pub fn find(&self, name: &str) -> Option<&'static dyn Device> {
        sync::free(|_| {
            self.devices.borrow().iter().find_map(|e| match e {
                Some((n, dev)) if *n == name => Some(*dev),
                _ => None,
            })
        })
    }

    /// 注销设备（幂等）：名字不在时返回 false
    pub fn unregister(&self, name: &str) -> bool {
        sync::free(|_| {
            let mut devices = self.devices.borrow_mut();
            for slot in devices.iter_mut() {
                if let Some((n, _)) = slot {
                    if *n == name {
                        *slot = None;
                        return true;
                    }
                }
            }
            false
        })
    }

    // ---- 能力级便捷查找（薄包装：find + as_*）----

    /// 按名取字节流能力；名字不存在或设备无此能力时返回 None
    /// （用 `find` 可区分"没有"与"能力不符"）
    pub fn find_stream(&self, name: &str) -> Option<&'static dyn StreamDevice> {
        self.find(name).and_then(|d| d.as_stream())
    }

    /// 按名取块设备能力
    pub fn find_block(&self, name: &str) -> Option<&'static dyn BlockDevice> {
        self.find(name).and_then(|d| d.as_block())
    }

    /// 按名取控制面能力
    pub fn find_control(&self, name: &str) -> Option<&'static dyn Control> {
        self.find(name).and_then(|d| d.as_control())
    }

    /// 按名取总线能力
    pub fn find_bus(&self, name: &str) -> Option<&'static dyn BusDevice> {
        self.find(name).and_then(|d| d.as_bus())
    }

    /// 按名取事件/唤醒能力
    pub fn find_event(&self, name: &str) -> Option<&'static dyn EventDevice> {
        self.find(name).and_then(|d| d.as_event())
    }
}

/// 系统全局注册表（应用初始化时向它登记设备）
pub static REGISTRY: DriverRegistry = DriverRegistry::new();

/// 向全局注册表登记设备
pub fn register(name: &'static str, dev: &'static dyn Device) -> Result<(), DeviceError> {
    REGISTRY.register(name, dev)
}

/// 从全局注册表按名查找设备（通用入口）
pub fn find(name: &str) -> Option<&'static dyn Device> {
    REGISTRY.find(name)
}

/// 从全局注册表注销设备（幂等）
pub fn unregister(name: &str) -> bool {
    REGISTRY.unregister(name)
}

/// 从全局注册表按名取字节流能力
pub fn find_stream(name: &str) -> Option<&'static dyn StreamDevice> {
    REGISTRY.find_stream(name)
}

/// 从全局注册表按名取块设备能力
pub fn find_block(name: &str) -> Option<&'static dyn BlockDevice> {
    REGISTRY.find_block(name)
}

/// 从全局注册表按名取控制面能力
pub fn find_control(name: &str) -> Option<&'static dyn Control> {
    REGISTRY.find_control(name)
}

/// 从全局注册表按名取总线能力
pub fn find_bus(name: &str) -> Option<&'static dyn BusDevice> {
    REGISTRY.find_bus(name)
}

/// 从全局注册表按名取事件/唤醒能力
pub fn find_event(name: &str) -> Option<&'static dyn EventDevice> {
    REGISTRY.find_event(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::boxed::Box;
    use alloc::collections::VecDeque;
    use alloc::sync::Arc;
    use core::cell::RefCell;
    use core::sync::atomic::{AtomicUsize, Ordering};

    /// mock 字节流 + 事件设备：rx 队列 + write 计数 + waiter 登记/唤醒计数
    struct MockStream {
        rx: RefCell<VecDeque<u8>>,
        written: Arc<AtomicUsize>,
        registered: Arc<AtomicUsize>,
        woken: Arc<AtomicUsize>,
    }

    // SAFETY: 仅宿主测试内使用；借用严格串行于单个测试线程，无并发访问
    unsafe impl Sync for MockStream {}

    impl MockStream {
        fn new() -> (Self, Arc<AtomicUsize>, Arc<AtomicUsize>, Arc<AtomicUsize>) {
            let written = Arc::new(AtomicUsize::new(0));
            let registered = Arc::new(AtomicUsize::new(0));
            let woken = Arc::new(AtomicUsize::new(0));
            (
                Self {
                    rx: RefCell::new(VecDeque::new()),
                    written: written.clone(),
                    registered: registered.clone(),
                    woken: woken.clone(),
                },
                written,
                registered,
                woken,
            )
        }
    }

    impl Device for MockStream {
        fn kind(&self) -> DeviceKind {
            DeviceKind::Stream
        }
        fn as_stream(&self) -> Option<&dyn StreamDevice> {
            Some(self)
        }
        fn as_event(&self) -> Option<&dyn EventDevice> {
            Some(self)
        }
    }

    impl StreamDevice for MockStream {
        fn available(&self) -> usize {
            self.rx.borrow().len()
        }
        fn read(&self, buf: &mut [u8]) -> Result<usize, DeviceError> {
            let mut rx = self.rx.borrow_mut();
            let mut n = 0;
            while n < buf.len() {
                match rx.pop_front() {
                    Some(b) => {
                        buf[n] = b;
                        n += 1;
                    }
                    None => break,
                }
            }
            Ok(n)
        }
        fn write(&self, buf: &[u8]) -> Result<usize, DeviceError> {
            self.written.fetch_add(buf.len(), Ordering::SeqCst);
            Ok(buf.len())
        }
    }

    impl EventDevice for MockStream {
        fn register_waiter(&self, waiter: usize) -> Result<(), DeviceError> {
            self.registered.store(waiter, Ordering::SeqCst);
            Ok(())
        }
        fn wake(&self) {
            self.woken.fetch_add(1, Ordering::SeqCst);
        }
    }

    /// mock 纯控制设备（LED 类）：op1 计数 +1、op3 计数 +10，未知 op 报错
    struct MockLed {
        count: Arc<AtomicUsize>,
    }

    impl Device for MockLed {
        fn kind(&self) -> DeviceKind {
            DeviceKind::Control
        }
        fn as_control(&self) -> Option<&dyn Control> {
            Some(self)
        }
    }

    impl Control for MockLed {
        fn control(&self, op: u32, _arg: usize) -> Result<usize, DeviceError> {
            match op {
                1 => {
                    self.count.fetch_add(1, Ordering::SeqCst);
                    Ok(0)
                }
                3 => {
                    self.count.fetch_add(10, Ordering::SeqCst);
                    Ok(0)
                }
                _ => Err(DeviceError::InvalidInput),
            }
        }
    }

    /// mock 块设备：容量 2 扇区；读 +1、写 +10 透传计数
    struct MockBlock {
        ops: Arc<AtomicUsize>,
    }

    impl Device for MockBlock {
        fn kind(&self) -> DeviceKind {
            DeviceKind::Block
        }
        fn as_block(&self) -> Option<&dyn BlockDevice> {
            Some(self)
        }
    }

    impl BlockDevice for MockBlock {
        fn sector_size(&self) -> u64 {
            SECTOR_SIZE
        }
        fn sector_count(&self) -> u64 {
            2
        }
        fn read_sector(&self, _no: u64, _buf: &mut [u8]) -> Result<(), DeviceError> {
            self.ops.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
        fn write_sector(&self, _no: u64, _buf: &[u8]) -> Result<(), DeviceError> {
            self.ops.fetch_add(10, Ordering::SeqCst);
            Ok(())
        }
    }

    /// mock 总线设备：transfer 把 tx 字节数累计、rx 填 0xA5
    struct MockBus {
        xfer: Arc<AtomicUsize>,
    }

    impl Device for MockBus {
        fn kind(&self) -> DeviceKind {
            DeviceKind::Bus
        }
        fn as_bus(&self) -> Option<&dyn BusDevice> {
            Some(self)
        }
    }

    impl BusDevice for MockBus {
        fn transfer(&self, tx: &[u8], rx: &mut [u8]) -> Result<(), DeviceError> {
            self.xfer.fetch_add(tx.len(), Ordering::SeqCst);
            rx.fill(0xA5);
            Ok(())
        }
    }

    fn mock_led(name: &'static str, reg: &DriverRegistry) -> Arc<AtomicUsize> {
        let count = Arc::new(AtomicUsize::new(0));
        let dev: &'static dyn Device = Box::leak(Box::new(MockLed {
            count: count.clone(),
        }));
        reg.register(name, dev).unwrap();
        count
    }

    /// 回归：流设备注册→查找→使用 全流程；同一注册项上 `find_stream` 与
    /// `find_event` 都能取到能力（复合能力设备的核心用例）。
    #[test]
    fn stream_register_find_use_roundtrip() {
        let reg = DriverRegistry::new();
        let (dev, written, registered, _woken) = MockStream::new();
        dev.rx.borrow_mut().extend([1u8, 2, 3]);
        let dev: &'static dyn Device = Box::leak(Box::new(dev));
        reg.register("uart0", dev).unwrap();

        let s = reg.find_stream("uart0").expect("注册后应能找到流能力");
        assert_eq!(s.available(), 3);
        let mut buf = [0u8; 2];
        assert_eq!(s.read(&mut buf).unwrap(), 2);
        assert_eq!(buf, [1, 2]);
        assert_eq!(s.available(), 1);
        s.write_all(b"hi").unwrap();
        assert_eq!(written.load(Ordering::SeqCst), 2);

        // 同一注册项：事件能力也可取（read_blocking 的组合原料）
        let e = reg.find_event("uart0").expect("同一设备应能取到事件能力");
        e.register_waiter(0x1234).unwrap();
        assert_eq!(registered.load(Ordering::SeqCst), 0x1234);

        // 能力不符：未 override as_control 的设备按控制面找应得 None
        assert!(reg.find_control("uart0").is_none());
        // kind 标签（诊断用途）
        assert_eq!(reg.find("uart0").unwrap().kind(), DeviceKind::Stream);
    }

    /// 回归：纯控制设备（LED 类）注册→查找→命令分派；未知 op 报 InvalidInput。
    #[test]
    fn control_register_find_use_roundtrip() {
        let reg = DriverRegistry::new();
        let count = mock_led("red", &reg);
        let c = reg.find_control("red").expect("注册后应能找到控制面");
        c.control(1, 0).unwrap();
        c.control(3, 0).unwrap();
        assert_eq!(
            count.load(Ordering::SeqCst),
            11,
            "op1(+1) 与 op3(+10) 都应生效"
        );
        assert_eq!(c.control(0xFF, 0), Err(DeviceError::InvalidInput));
        // 能力不符：纯控制设备按流找应得 None；通用 find 能区分
        assert!(reg.find_stream("red").is_none());
        assert_eq!(reg.find("red").unwrap().kind(), DeviceKind::Control);
    }

    /// 回归：块设备注册→查找→使用；读/写计数透传验证"同一实例"。
    #[test]
    fn block_register_find_use_roundtrip() {
        let reg = DriverRegistry::new();
        let ops = Arc::new(AtomicUsize::new(0));
        let dev: &'static dyn Device = Box::leak(Box::new(MockBlock { ops: ops.clone() }));
        reg.register("sd0", dev).unwrap();
        let b = reg.find_block("sd0").expect("注册后应能找到块能力");
        assert_eq!(b.sector_size(), SECTOR_SIZE);
        assert_eq!(b.sector_count(), 2);
        let mut buf = [0u8; SECTOR_SIZE as usize];
        b.read_sector(0, &mut buf).unwrap();
        b.write_sector(1, &buf).unwrap();
        assert_eq!(ops.load(Ordering::SeqCst), 11, "读(+1) 与写(+10) 都应透传");
        assert!(reg.find_stream("sd0").is_none());
    }

    /// 回归：总线设备注册→查找→transfer 透传。
    #[test]
    fn bus_register_find_use_roundtrip() {
        let reg = DriverRegistry::new();
        let xfer = Arc::new(AtomicUsize::new(0));
        let dev: &'static dyn Device = Box::leak(Box::new(MockBus { xfer: xfer.clone() }));
        reg.register("spi1", dev).unwrap();
        let b = reg.find_bus("spi1").expect("注册后应能找到总线能力");
        let mut rx = [0u8; 4];
        b.transfer(&[1, 2, 3], &mut rx).unwrap();
        assert_eq!(xfer.load(Ordering::SeqCst), 3);
        assert_eq!(rx, [0xA5; 4]);
        assert_eq!(reg.find("spi1").unwrap().kind(), DeviceKind::Bus);
    }

    /// 回归：重名必须拒绝——同名二次注册 Err(Duplicate)，第一个设备不受影响。
    /// 阳性对照：若实现漏掉重名检查（直接覆盖），注册重名返回 Ok 即测试红。
    #[test]
    fn duplicate_name_rejected() {
        let reg = DriverRegistry::new();
        let count = mock_led("red", &reg);
        let count2 = Arc::new(AtomicUsize::new(0));
        let dev2: &'static dyn Device = Box::leak(Box::new(MockLed { count: count2 }));
        assert_eq!(reg.register("red", dev2), Err(DeviceError::Duplicate));
        reg.find_control("red").unwrap().control(1, 0).unwrap();
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }

    /// 回归：名字全局唯一（Zephyr binding 语义）——LED 已占的名字，
    /// 流设备同名注册必须拒绝。
    #[test]
    fn names_are_global_across_classes() {
        let reg = DriverRegistry::new();
        mock_led("dev0", &reg);
        let (dev, _, _, _) = MockStream::new();
        let dev: &'static dyn Device = Box::leak(Box::new(dev));
        assert_eq!(
            reg.register("dev0", dev),
            Err(DeviceError::Duplicate),
            "跨类同名必须拒绝（名字全局唯一）"
        );
    }

    /// 回归：容量满必须报错（阳性对照——定长数组漏检查时第 9 个静默丢 => 红）。
    #[test]
    fn capacity_full_error() {
        let reg = DriverRegistry::new();
        for i in 0..DEV_CAP {
            let count = Arc::new(AtomicUsize::new(0));
            let name = Box::leak(format!("led{i}").into_boxed_str()) as &'static str;
            let dev: &'static dyn Device = Box::leak(Box::new(MockLed { count }));
            reg.register(name, dev).unwrap();
        }
        let dev: &'static dyn Device = Box::leak(Box::new(MockLed {
            count: Arc::new(AtomicUsize::new(0)),
        }));
        assert_eq!(
            reg.register("overflow", dev),
            Err(DeviceError::Full),
            "第 {} 个应报满",
            DEV_CAP + 1
        );
    }

    /// 回归：未注册的名字所有查找都返回 None（不 panic、不误配）。
    #[test]
    fn unknown_name_is_none() {
        let reg = DriverRegistry::new();
        assert!(reg.find("ghost").is_none());
        assert!(reg.find_stream("ghost").is_none());
        assert!(reg.find_block("ghost").is_none());
        assert!(reg.find_control("ghost").is_none());
        assert!(reg.find_bus("ghost").is_none());
        assert!(reg.find_event("ghost").is_none());
    }

    /// 回归：注销幂等——首次 true、二次 false；注销后按名找不到。
    #[test]
    fn unregister_is_idempotent() {
        let reg = DriverRegistry::new();
        mock_led("red", &reg);
        assert!(reg.unregister("red"), "在册的设备注销应返回 true");
        assert!(!reg.unregister("red"), "二次注销应返回 false（幂等）");
        assert!(reg.find("red").is_none());
    }

    /// 回归：`write_all` 默认实现——空缓冲直接 Ok；正常全写透传；
    /// 零进度必须报 WriteZero（阳性对照：把 0 当成功则测试红）。
    #[test]
    fn write_all_contract() {
        let (dev, written, _, _) = MockStream::new();
        // 空缓冲：Ok 且不触发写
        dev.write_all(&[]).unwrap();
        assert_eq!(written.load(Ordering::SeqCst), 0);
        // 正常写满
        dev.write_all(b"abc").unwrap();
        assert_eq!(written.load(Ordering::SeqCst), 3);

        /// 零进度设备：write 恒返回 Ok(0)
        struct Stuck;
        impl Device for Stuck {
            fn kind(&self) -> DeviceKind {
                DeviceKind::Stream
            }
            fn as_stream(&self) -> Option<&dyn StreamDevice> {
                Some(self)
            }
        }
        impl StreamDevice for Stuck {
            fn available(&self) -> usize {
                0
            }
            fn read(&self, _buf: &mut [u8]) -> Result<usize, DeviceError> {
                Ok(0)
            }
            fn write(&self, _buf: &[u8]) -> Result<usize, DeviceError> {
                Ok(0)
            }
        }
        assert_eq!(Stuck.write_all(b"x"), Err(DeviceError::WriteZero));
    }

    /// 回归：`read_blocking` 的 available>0 直通路径——有数据时立即返回，
    /// 绝不登记 waiter（阻塞路径涉及真实任务切换，宿主不可测，由真机验证）。
    #[test]
    fn read_blocking_fast_path_no_waiter() {
        let (dev, _, registered, _) = MockStream::new();
        dev.rx.borrow_mut().extend([0xAA, 0xBB]);
        let dev: &'static dyn Device = Box::leak(Box::new(dev));
        let mut buf = [0u8; 4];
        let n = read_blocking(dev, &mut buf).unwrap();
        assert_eq!(n, 2);
        assert_eq!(buf[..2], [0xAA, 0xBB]);
        assert_eq!(
            registered.load(Ordering::SeqCst),
            0,
            "有数据时不得登记 waiter"
        );
        // 空读：立即 Ok(0)，不得走等待分支（否则任务会永远挂起）
        assert_eq!(read_blocking(dev, &mut []).unwrap(), 0);
        assert_eq!(registered.load(Ordering::SeqCst), 0, "空读不得登记 waiter");
        // 设备无事件能力：报 Unsupported 而不是 panic
        let blk: &'static dyn Device = Box::leak(Box::new(MockBlock {
            ops: Arc::new(AtomicUsize::new(0)),
        }));
        let mut b2 = [0u8; 1];
        assert_eq!(read_blocking(blk, &mut b2), Err(DeviceError::Unsupported));
    }

    /// 回归：`&T` 引用转发——泛型适配器统一吃具体设备与 trait 对象两种形态
    /// （`FatAdapter<B: BlockDevice>` 的依赖路径）。
    #[test]
    fn reference_forwarding_blanket() {
        fn via_generic<B: BlockDevice>(b: B) -> u64 {
            assert_eq!(b.kind(), DeviceKind::Block); // Device for &T 也要转发
            b.sector_count()
        }
        let mock = MockBlock {
            ops: Arc::new(AtomicUsize::new(0)),
        };
        assert_eq!(via_generic(&mock), 2);
        let dev: &'static dyn BlockDevice = Box::leak(Box::new(MockBlock {
            ops: Arc::new(AtomicUsize::new(0)),
        }));
        assert_eq!(via_generic(dev), 2);
    }
}
