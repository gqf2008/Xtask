//! 编译期设备清单：Zephyr 式"静态清单 + 线性扫描"的 Rust 对应物
//!
//! `super`（运行期注册表）讲清了设备模型的形状；本模块回答 Zephyr 的另一个问题：
//! **为什么真实内核不在运行期做"注册"**——设备集合、名字、绑定在链接后全部定死，
//! 运行期只剩查找与分发。对应关系（Zephyr 源码核对）：
//!
//! | Zephyr | 本模块 |
//! |---|---|
//! | `struct device`（const，ROM 段 `.__device.*`） | `DeviceDef`（const 描述符：名字 + 槽指针） |
//! | `device_state`（RAM：`initialized`/`init_res`） | `DeviceSlot`（RAM：实例槽，init 填一次） |
//! | `DEVICE_DT_DEFINE`（宏，编译期生成设备） | `device_list!`（宏，const 数组） |
//! | `device_get_binding`（扫 ROM 段 + 名字匹配） | `find`（扫清单 + 名字匹配 + 槽就绪检查） |
//! | init 编程硬件、`device_is_ready` | `init` 里 `fill` 实例；未填 = 未就绪（`find` 返回 None） |
//!
//! 与运行期注册表的分工：二者共用同一种句柄 `&'static dyn Device` 与同一套
//! 能力 trait，只换"表"的实现——注册表是可增删的运行期表（教学：模型透明、
//! host 可测、可演示热注册），本模块是编译期定形的清单（真实内核心态：零运行期
//! 注册决策）。能力分发同样走 `as_*`：表对设备类透明，新增能力 trait 本模块
//! 只加一个 `find_xxx` 便捷查找，槽/描述符/宏一行不改。
//!
//! 设计取舍：清单存**实例槽指针**而非实例本身——Rust 嵌入生态的外设（GPIO 引脚、
//! Serial）没有 const 构造器，实例只能在 init 里运行期构造；槽把"编译期定形的名字/集合"
//! 与"运行期才存在的实例"焊接在一起，RAM 代价 = 每设备一个
//! `Option<&'static dyn Device>`。（Zephyr 的 C 同样在 init 里编程硬件，只是它连
//! device 对象都 const——Rust 侧以槽代替 device_state，语义不变：**名字属于编译期，
//! 实例属于运行期**。）
//!
//! 纪律与注册表同款：槽与表的访问全部在 `sync::free` 临界区内（单核关中断），
//! `unsafe impl Send/Sync` 的理由与 bus.rs/REGISTRY 同构。

use core::cell::UnsafeCell;

use crate::device::{BlockDevice, BusDevice, Control, Device, EventDevice, StreamDevice};
use crate::sync;

/// 设备实例槽：编译期清单里的"位置"，运行期由 init 填**一次**实例。
/// 未填 = 设备在清单里（存在）但未就绪——`find` 返回 `None`，与 Zephyr 的
/// `device_get_binding` 对未就绪设备返回 NULL 同语义。
pub struct DeviceSlot(UnsafeCell<Option<&'static dyn Device>>);

// SAFETY: 槽的读写全部发生在 sync::free 临界区内（单核关中断），ISR 与任务
// 不可能并发访问同一借用——与 bus.rs/REGISTRY 的 unsafe impl 同构。
unsafe impl Send for DeviceSlot {}
// SAFETY: 同上；单核模型下"共享引用"的可变访问由临界区串行化。
unsafe impl Sync for DeviceSlot {}

impl DeviceSlot {
    /// 空槽
    pub const fn new() -> Self {
        Self(UnsafeCell::new(None))
    }

    /// 填充实例（init 时调用；每槽恰好一次）。
    /// 重复填充直接 panic——"一台设备只构造一次"是 Zephyr 的隐式契约
    /// （它的 `device_init` 对已初始化设备返回 `-EALREADY`），教学内核宁可
    /// 早失败也不静默覆盖。
    pub fn fill(&self, dev: &'static dyn Device) {
        sync::free(|_| {
            let slot = unsafe { &mut *self.0.get() };
            assert!(slot.is_none(), "设备槽重复填充：实例已存在，不允许覆盖");
            *slot = Some(dev);
        });
    }

    /// 读槽——**调用方必须已处于 `sync::free` 临界区内**（仅 `DeviceTable::find`
    /// 等查找路径使用，与外层同一次临界区）：再包一层 `sync::free` 就是嵌套取锁，
    /// 而宿主端口 `free` 是进程互斥锁、不可重入（port.rs:78 明令"free 里再 free 会死锁"）；
    /// 芯片端口的 `interrupt::free` 虽可重入（深度计数），一次操作一次取锁才能让
    /// 宿主与真机行为一致。找不到这个纪律的地方 = bus.rs/REGISTRY 同款：整个
    /// 操作一块临界区，内部不许再 `free`。
    fn get_in_critical(&self) -> Option<&'static dyn Device> {
        unsafe { *self.0.get() }
    }
}

/// 设备描述符：编译期定形的"名字 → 实例槽"。
/// 实例本身在 run 期填槽，描述符只定两件事——**叫什么、槽在哪**，全部写入 ROM
/// （const 数组），与 Zephyr 的 const `struct device` 同构。设备类别由槽内实例的
/// `kind()`/`as_*` 回答，描述符不携带（能力开放扩展，描述符跟着定死就退化成
/// 封闭枚举了）。
pub struct DeviceDef {
    /// 设备名（`find` 的查找键；全局唯一，Zephyr binding 语义）
    pub name: &'static str,
    /// 实例槽（RAM；init 里 `fill`）
    slot: &'static DeviceSlot,
}

impl DeviceDef {
    /// 构造描述符。`slot` 引用必须是与设备等长的静态槽。
    pub const fn new(name: &'static str, slot: &'static DeviceSlot) -> Self {
        Self { name, slot }
    }
}

/// 编译期设备清单：const 数组（ROM），由 `device_list!` 宏声明。
/// 与 Zephyr 的差异只在聚合方式——我们用宏生成 const 数组，
/// Zephyr 用链接器段聚合 `DEVICE_DT_DEFINE`（各驱动各自注册）；Rust 侧
/// 无链接段魔法，数组由应用/BSP 一处声明（教学：清单一眼看全）。
#[macro_export]
macro_rules! device_list {
    ($name:ident { $($n:expr => $s:expr),+ $(,)? }) => {
        /// 编译期设备清单（ROM）：名字 → 实例槽；实例在 init 里 `fill` 后
        /// 由 `xtask::device::table::find` 按名取用。
        pub static $name: &[$crate::device::table::DeviceDef] = &[
            $( $crate::device::table::DeviceDef::new($n, $s) ),+
        ];
    };
}

/// 清单表：持有"应用在 init 时 attach 的清单切片"。
/// 只存一个切片指针——清单本体在 ROM，RAM 零拷贝（Zephyr 的 `_device_list_start/end`
/// 也是"指针按数组遍历"，这里等价物是一个 fat 指针）。
pub struct DeviceTable(UnsafeCell<Option<&'static [DeviceDef]>>);

// SAFETY: 表的读写全部发生在 sync::free 临界区内（单核关中断），ISR 与任务
// 不可能并发访问同一借用——与 bus.rs/REGISTRY 的 unsafe impl 同构。
unsafe impl Send for DeviceTable {}
// SAFETY: 同上；单核模型下"共享引用"的可变访问由临界区串行化。
unsafe impl Sync for DeviceTable {}

impl DeviceTable {
    /// 空表
    pub const fn new() -> Self {
        Self(UnsafeCell::new(None))
    }

    /// 挂载编译期清单（应用 init 里调用一次；清单必须是 `device_list!` 声明的
    /// `&'static [DeviceDef]` 或等价的静态切片）。
    pub fn attach(&self, list: &'static [DeviceDef]) {
        sync::free(|_| unsafe {
            *self.0.get() = Some(list);
        });
    }

    /// 按名查找设备（Zephyr `device_get_binding` 同款线性扫描；通用入口，
    /// 拿到后用 `as_*` 取能力，或用下面的 `find_stream`/`find_block`/… 直接取）。
    /// 返回 `None` 的三种情形：清单未挂载 / 名字不在清单 / **在清单但槽未填**（未就绪）——
    /// 后两种与 Zephyr 的"找不到"与"未 ready 返回 NULL"一一对应。
    pub fn find(&self, name: &str) -> Option<&'static dyn Device> {
        sync::free(|_| unsafe {
            let list = (*self.0.get())?;
            list.iter()
                .find(|d| d.name == name)
                .and_then(|d| d.slot.get_in_critical())
        })
    }

    // ---- 能力级便捷查找（薄包装：find + as_*）----

    /// 按名取字节流能力；名字不存在/槽未填/设备无此能力时返回 None
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

/// 系统全局设备清单表（init 里 `attach` 后使用）
pub static TABLE: DeviceTable = DeviceTable::new();

/// 向全局清单表挂载编译期清单
pub fn attach(list: &'static [DeviceDef]) {
    TABLE.attach(list);
}

/// 从全局清单表按名查找设备（通用入口）
pub fn find(name: &str) -> Option<&'static dyn Device> {
    TABLE.find(name)
}

/// 从全局清单表按名取字节流能力
pub fn find_stream(name: &str) -> Option<&'static dyn StreamDevice> {
    TABLE.find_stream(name)
}

/// 从全局清单表按名取块设备能力
pub fn find_block(name: &str) -> Option<&'static dyn BlockDevice> {
    TABLE.find_block(name)
}

/// 从全局清单表按名取控制面能力
pub fn find_control(name: &str) -> Option<&'static dyn Control> {
    TABLE.find_control(name)
}

/// 从全局清单表按名取总线能力
pub fn find_bus(name: &str) -> Option<&'static dyn BusDevice> {
    TABLE.find_bus(name)
}

/// 从全局清单表按名取事件/唤醒能力
pub fn find_event(name: &str) -> Option<&'static dyn EventDevice> {
    TABLE.find_event(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::{DeviceError, DeviceKind, SECTOR_SIZE};
    use alloc::boxed::Box;
    use alloc::sync::Arc;
    use core::sync::atomic::{AtomicUsize, Ordering};

    /// mock LED：op1 计数 +1、op3 计数 +10（与 device.rs 的 mock 同款）
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

    /// mock 流设备：write 累加字节数；同时实现事件能力（验证复合设备按名取多能力）
    struct MockStream {
        written: Arc<AtomicUsize>,
        registered: Arc<AtomicUsize>,
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
            0
        }
        fn read(&self, _buf: &mut [u8]) -> Result<usize, DeviceError> {
            Ok(0)
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
        fn wake(&self) {}
    }

    /// mock 块设备：容量 4 扇区，读计数透传
    struct MockBlock {
        reads: Arc<AtomicUsize>,
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
            4
        }
        fn read_sector(&self, _no: u64, _buf: &mut [u8]) -> Result<(), DeviceError> {
            self.reads.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
        fn write_sector(&self, _no: u64, _buf: &[u8]) -> Result<(), DeviceError> {
            Ok(())
        }
    }

    /// mock 总线设备：transfer 累计 tx 字节数
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
        fn transfer(&self, tx: &[u8], _rx: &mut [u8]) -> Result<(), DeviceError> {
            self.xfer.fetch_add(tx.len(), Ordering::SeqCst);
            Ok(())
        }
    }

    fn mock_led() -> (&'static dyn Device, Arc<AtomicUsize>) {
        let count = Arc::new(AtomicUsize::new(0));
        let dev: &'static dyn Device = Box::leak(Box::new(MockLed {
            count: count.clone(),
        }));
        (dev, count)
    }

    fn mock_stream() -> (&'static dyn Device, Arc<AtomicUsize>, Arc<AtomicUsize>) {
        let written = Arc::new(AtomicUsize::new(0));
        let registered = Arc::new(AtomicUsize::new(0));
        let dev: &'static dyn Device = Box::leak(Box::new(MockStream {
            written: written.clone(),
            registered: registered.clone(),
        }));
        (dev, written, registered)
    }

    /// 清单未挂载 → find 一律 None（attach 前的安全失败）。
    #[test]
    fn find_before_attach_is_none() {
        let table = DeviceTable::new();
        assert!(table.find("led0").is_none());
        assert!(table.find_control("led0").is_none());
        assert!(table.find("ghost").is_none());
    }

    /// 挂载但槽未填 → find None（"在清单里但未就绪"，
    /// 对应 Zephyr `device_get_binding` 对未 ready 设备返回 NULL）。
    ///
    /// 槽/清单一律**每测试私有**（函数内 static/const）：libtest 并行跑测试线程，
    /// 共享静态会被其他测试并发 fill 踩踏（"重复填充"panic 或读到半成品）。
    #[test]
    fn find_unfilled_slot_is_none() {
        static SLOT: DeviceSlot = DeviceSlot::new();
        const LIST: &[DeviceDef] = &[DeviceDef::new("led0", &SLOT)];
        let table = DeviceTable::new();
        table.attach(LIST);
        assert!(
            table.find("led0").is_none(),
            "清单里有 led0 但槽未填，应视为未就绪"
        );
    }

    /// 回归：fill 后 find 命中；查到的确实是同一个实例（计数透传验证）；
    /// 复合能力设备同一注册项上 `find_stream` 与 `find_event` 都可用。
    #[test]
    fn attach_fill_find_use_roundtrip() {
        static LED_SLOT: DeviceSlot = DeviceSlot::new();
        static UART_SLOT: DeviceSlot = DeviceSlot::new();
        const LIST: &[DeviceDef] = &[
            DeviceDef::new("led0", &LED_SLOT),
            DeviceDef::new("uart0", &UART_SLOT),
        ];
        let table = DeviceTable::new();
        table.attach(LIST);

        let (led, count) = mock_led();
        LED_SLOT.fill(led);
        let c = table.find_control("led0").expect("填槽后应能找到控制面");
        c.control(1, 0).unwrap();
        c.control(3, 0).unwrap();
        assert_eq!(
            count.load(Ordering::SeqCst),
            11,
            "op1(+1) 与 op3(+10) 都应生效"
        );

        let (uart, written, registered) = mock_stream();
        UART_SLOT.fill(uart);
        table
            .find_stream("uart0")
            .expect("填槽后应能找到流能力")
            .write_all(b"hi")
            .unwrap();
        assert_eq!(written.load(Ordering::SeqCst), 2);
        // 同一槽：事件能力也可取（read_blocking 的组合原料）
        table
            .find_event("uart0")
            .expect("同一设备应能取到事件能力")
            .register_waiter(0x55)
            .unwrap();
        assert_eq!(registered.load(Ordering::SeqCst), 0x55);
    }

    /// 回归：清单侧 `find_block` / `find_bus` 全路径。五条便捷查找同为
    /// find+as_* 薄包装，但按设计稿 §10 纪律——每能力的 host 单测必须覆盖
    /// register/fill→find_xxx→透传 全路径，防 `as_*` 漏 override 静默 None——
    /// 清单侧也要 5/5（本测试补块与总线两条）。
    #[test]
    fn find_block_and_bus_roundtrip() {
        static SD_SLOT: DeviceSlot = DeviceSlot::new();
        static SPI_SLOT: DeviceSlot = DeviceSlot::new();
        const LIST: &[DeviceDef] = &[
            DeviceDef::new("sd0", &SD_SLOT),
            DeviceDef::new("spi1", &SPI_SLOT),
        ];
        let table = DeviceTable::new();
        table.attach(LIST);

        let reads = Arc::new(AtomicUsize::new(0));
        let sd: &'static dyn Device = Box::leak(Box::new(MockBlock {
            reads: reads.clone(),
        }));
        SD_SLOT.fill(sd);
        let b = table.find_block("sd0").expect("填槽后应能找到块能力");
        assert_eq!(b.sector_size(), SECTOR_SIZE);
        assert_eq!(b.sector_count(), 4);
        let mut buf = [0u8; SECTOR_SIZE as usize];
        b.read_sector(0, &mut buf).unwrap();
        assert_eq!(reads.load(Ordering::SeqCst), 1, "读计数应透传到同一实例");
        assert!(table.find_bus("sd0").is_none(), "块设备按总线找应 None");

        let xfer = Arc::new(AtomicUsize::new(0));
        let spi: &'static dyn Device = Box::leak(Box::new(MockBus { xfer: xfer.clone() }));
        SPI_SLOT.fill(spi);
        let bus = table.find_bus("spi1").expect("填槽后应能找到总线能力");
        bus.transfer(&[1, 2], &mut []).unwrap();
        assert_eq!(xfer.load(Ordering::SeqCst), 2, "tx 字节数应透传到同一实例");
        assert!(table.find_block("spi1").is_none(), "总线设备按块找应 None");
    }

    /// 回归：名字不在清单 → None；能力不符 → find_control/find_stream 各得 None。
    #[test]
    fn unknown_name_and_wrong_class_are_none() {
        static UART_SLOT: DeviceSlot = DeviceSlot::new();
        const LIST: &[DeviceDef] = &[DeviceDef::new("uart0", &UART_SLOT)];
        let table = DeviceTable::new();
        table.attach(LIST);
        let (uart, _, _) = mock_stream();
        UART_SLOT.fill(uart);

        assert!(table.find("ghost").is_none(), "名字不在清单");
        assert!(
            table.find_control("uart0").is_none(),
            "流设备未 override as_control，按控制面找应 None"
        );
        assert_eq!(
            table.find("uart0").unwrap().kind(),
            DeviceKind::Stream,
            "通用 find 应命中并给出主类标签"
        );
    }

    /// 阳性对照：重复填充必须 panic（设备只构造一次的契约）。
    #[test]
    #[should_panic(expected = "重复填充")]
    fn double_fill_panics() {
        static SLOT: DeviceSlot = DeviceSlot::new();
        let (dev, _) = mock_led();
        SLOT.fill(dev);
        let (dev2, _) = mock_led();
        SLOT.fill(dev2); // 第二次：panic
    }

    /// 回归：`device_list!` 宏声明 → attach → find 全流程。
    static S1: DeviceSlot = DeviceSlot::new();
    static S2: DeviceSlot = DeviceSlot::new();
    crate::device_list! { TEST_BOARD {
        "red" => &S1,
        "uart0" => &S2,
    } }

    #[test]
    fn device_list_macro_roundtrip() {
        let (dev, count) = mock_led();
        S1.fill(dev);

        let table = DeviceTable::new();
        table.attach(TEST_BOARD);
        table
            .find_control("red")
            .expect("宏清单应可查")
            .control(1, 0)
            .unwrap();
        assert_eq!(count.load(Ordering::SeqCst), 1);
        assert!(table.find_stream("uart0").is_none(), "S2 未填，应未就绪");
    }
}
