//! 编译期设备清单：Zephyr 式"静态清单 + 线性扫描"的 Rust 对应物
//!
//! `drv`（运行期注册表）讲清了设备模型的形状；本模块回答 Zephyr 的另一个问题：
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
//! 与 `drv` 的分工：二者共用 `drv::DeviceApi` 枚举与 `LedDevice`/`UartDevice` 类 API，
//! 只换"表"的实现——`drv` 是可增删的运行期表（教学：模型透明、host 可测、可演示热注册），
//! 本模块是编译期定形的清单（真实内核心态：零运行期注册决策）。
//!
//! 设计取舍：清单存**实例槽指针**而非实例本身——Rust 嵌入生态的外设（GPIO 引脚、
//! Serial）没有 const 构造器，实例只能在 init 里运行期构造；槽把"编译期定形的名字/集合"
//! 与"运行期才存在的实例"焊接在一起，RAM 代价 = 每设备一个 `Option<DeviceApi>`。
//! （Zephyr 的 C 同样在 init 里编程硬件，只是它连 device 对象都 const——Rust 侧以
//! 槽代替 device_state，语义不变：**名字属于编译期，实例属于运行期**。）
//!
//! 纪律与 `drv` 同款：槽与表的访问全部在 `sync::free` 临界区内（单核关中断），
//! `unsafe impl Send/Sync` 的理由与 bus.rs/REGISTRY 同构。

use core::cell::UnsafeCell;

use crate::drv::{DeviceApi, LedDevice, UartDevice};
use crate::sync;

/// 设备实例槽：编译期清单里的"位置"，运行期由 init 填**一次**实例。
/// 未填 = 设备在清单里（存在）但未就绪——`find` 返回 `None`，与 Zephyr 的
/// `device_get_binding` 对未就绪设备返回 NULL 同语义。
pub struct DeviceSlot(UnsafeCell<Option<DeviceApi>>);

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
    pub fn fill(&self, api: DeviceApi) {
        sync::free(|_| {
            let slot = unsafe { &mut *self.0.get() };
            assert!(slot.is_none(), "设备槽重复填充：实例已存在，不允许覆盖");
            *slot = Some(api);
        });
    }

    /// 读槽——**调用方必须已处于 `sync::free` 临界区内**（仅 `DeviceTable::find`
    /// 等查找路径使用，与外层同一次临界区）：再包一层 `sync::free` 就是嵌套取锁，
    /// 而宿主端口 `free` 是进程互斥锁、不可重入（port.rs:78 明令"free 里再 free 会死锁"）；
    /// 芯片端口的 `interrupt::free` 虽可重入（深度计数），一次操作一次取锁才能让
    /// 宿主与真机行为一致。找不到这个纪律的地方 = bus.rs/REGISTRY 同款：整个
    /// 操作一块临界区，内部不许再 `free`。
    fn get_in_critical(&self) -> Option<DeviceApi> {
        unsafe { *self.0.get() }
    }
}

/// 设备描述符：编译期定形的"名字 → 实例槽"。
/// 实例本身在 run 期填槽，描述符只定三件事——**叫什么、属于哪类、槽在哪**，
/// 全部写入 ROM（const 数组），与 Zephyr 的 const `struct device` 同构。
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
        /// 由 `xtask::drv_static::find` 按名取用。
        pub static $name: &[$crate::drv_static::DeviceDef] = &[
            $( $crate::drv_static::DeviceDef::new($n, $s) ),+
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

    /// 按名查找设备能力（Zephyr `device_get_binding` 同款线性扫描）。
    /// 返回 `None` 的三种情形：清单未挂载 / 名字不在清单 / **在清单但槽未填**（未就绪）——
    /// 后两种与 Zephyr 的"找不到"与"未 ready 返回 NULL"一一对应。
    pub fn find(&self, name: &str) -> Option<DeviceApi> {
        sync::free(|_| unsafe {
            let list = (*self.0.get())?;
            list.iter()
                .find(|d| d.name == name)
                .and_then(|d| d.slot.get_in_critical())
        })
    }

    /// 按名查找 LED 设备；名字存在但不是 LED 类时返回 `None`（用 `find` 可区分）
    pub fn find_led(&self, name: &str) -> Option<&'static dyn LedDevice> {
        match self.find(name) {
            Some(DeviceApi::Led(dev)) => Some(dev),
            _ => None,
        }
    }

    /// 按名查找串口设备；名字存在但不是 UART 类时返回 `None`（用 `find` 可区分）
    pub fn find_uart(&self, name: &str) -> Option<&'static dyn UartDevice> {
        match self.find(name) {
            Some(DeviceApi::Uart(dev)) => Some(dev),
            _ => None,
        }
    }
}

/// 系统全局设备清单表（init 里 `attach` 后使用）
pub static TABLE: DeviceTable = DeviceTable::new();

/// 向全局清单表挂载编译期清单
pub fn attach(list: &'static [DeviceDef]) {
    TABLE.attach(list);
}

/// 从全局清单表按名查找设备能力
pub fn find(name: &str) -> Option<DeviceApi> {
    TABLE.find(name)
}

/// 从全局清单表按名查找 LED 设备
pub fn find_led(name: &str) -> Option<&'static dyn LedDevice> {
    TABLE.find_led(name)
}

/// 从全局清单表按名查找串口设备
pub fn find_uart(name: &str) -> Option<&'static dyn UartDevice> {
    TABLE.find_uart(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::boxed::Box;
    use alloc::sync::Arc;
    use core::sync::atomic::{AtomicUsize, Ordering};

    /// mock LED：on 计数 +1、toggle 计数 +10（与 drv.rs 的 mock 同款）
    struct MockLed {
        count: Arc<AtomicUsize>,
    }
    impl LedDevice for MockLed {
        fn on(&self) {
            self.count.fetch_add(1, Ordering::SeqCst);
        }
        fn off(&self) {}
        fn toggle(&self) {
            self.count.fetch_add(10, Ordering::SeqCst);
        }
    }

    /// mock UART：write_all 累加字节数
    struct MockUart {
        written: Arc<AtomicUsize>,
    }
    impl UartDevice for MockUart {
        fn write_all(&self, buf: &[u8]) {
            self.written.fetch_add(buf.len(), Ordering::SeqCst);
        }
        fn rx_len(&self) -> usize {
            0
        }
        fn read_byte(&self) -> u8 {
            0
        }
    }

    /// 清单未挂载 → find 一律 None（attach 前的安全失败）。
    #[test]
    fn find_before_attach_is_none() {
        let table = DeviceTable::new();
        assert!(table.find("led0").is_none());
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

    /// 回归：fill 后 find 命中；查到的确实是同一个实例（计数透传验证）。
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

        let count = Arc::new(AtomicUsize::new(0));
        let dev: &'static dyn LedDevice = Box::leak(Box::new(MockLed { count: count.clone() }));
        LED_SLOT.fill(DeviceApi::Led(dev));

        let led = table.find_led("led0").expect("填槽后应能找到");
        led.on();
        led.toggle();
        assert_eq!(count.load(Ordering::SeqCst), 11, "on(+1) 与 toggle(+10) 都应生效");

        let written = Arc::new(AtomicUsize::new(0));
        let uart: &'static dyn UartDevice = Box::leak(Box::new(MockUart { written: written.clone() }));
        UART_SLOT.fill(DeviceApi::Uart(uart));
        table.find_uart("uart0").expect("填槽后应能找到").write_all(b"hi");
        assert_eq!(written.load(Ordering::SeqCst), 2);
    }

    /// 回归：名字不在清单 → None；类不符 → find_led/ find_uart 各得 None。
    #[test]
    fn unknown_name_and_wrong_class_are_none() {
        static UART_SLOT: DeviceSlot = DeviceSlot::new();
        const LIST: &[DeviceDef] = &[DeviceDef::new("uart0", &UART_SLOT)];
        let table = DeviceTable::new();
        table.attach(LIST);
        let uart: &'static dyn UartDevice = Box::leak(Box::new(MockUart {
            written: Arc::new(AtomicUsize::new(0)),
        }));
        UART_SLOT.fill(DeviceApi::Uart(uart));

        assert!(table.find("ghost").is_none(), "名字不在清单");
        assert!(table.find_led("uart0").is_none(), "名字是 UART 类，按 LED 找应 None");
        match table.find("uart0") {
            Some(DeviceApi::Uart(_)) => {}
            other => panic!("find 应返回 Uart 变体，实际 {:?}", other.map(|_| "非 Uart")),
        }
    }

    /// 阳性对照：重复填充必须 panic（设备只构造一次的契约）。
    #[test]
    #[should_panic(expected = "重复填充")]
    fn double_fill_panics() {
        static SLOT: DeviceSlot = DeviceSlot::new();
        let dev: &'static dyn LedDevice = Box::leak(Box::new(MockLed {
            count: Arc::new(AtomicUsize::new(0)),
        }));
        SLOT.fill(DeviceApi::Led(dev));
        let dev2: &'static dyn LedDevice = Box::leak(Box::new(MockLed {
            count: Arc::new(AtomicUsize::new(0)),
        }));
        SLOT.fill(DeviceApi::Led(dev2)); // 第二次：panic
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
        let count = Arc::new(AtomicUsize::new(0));
        let dev: &'static dyn LedDevice = Box::leak(Box::new(MockLed { count: count.clone() }));
        S1.fill(DeviceApi::Led(dev));

        let table = DeviceTable::new();
        table.attach(TEST_BOARD);
        table.find_led("red").expect("宏清单应可查").on();
        assert_eq!(count.load(Ordering::SeqCst), 1);
        assert!(table.find_led("uart0").is_none(), "S2 未填，应未就绪");
    }
}
