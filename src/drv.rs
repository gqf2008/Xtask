//! 驱动抽象层：类无关的按名设备表（仿 Zephyr 设备模型）
//!
//! 模型（对照 Zephyr）：`struct device` 只挂一个**不透明 api 指针** + 名字；类 API
//! （`uart.h`/`gpio.h` 的函数）分层在设备之上，绑定/查找（`device_get_binding`）本身
//! 是类无关的。本模块的对应物：
//! - **设备对象** = `DeviceApi` 枚举——类型安全的 api 指针（枚举白名单，不做裸指针强转）；
//! - **总线** = `DriverRegistry`：一张定长表 `名字 → DeviceApi`，`register/find/unregister`
//!   三个通用操作，名字全局唯一（Zephyr binding 语义：跨类同名算 Duplicate）；
//! - **类 API** = `LedDevice`/`UartDevice` trait + `register_led/find_led` 等薄封装。
//!   新增设备类 = 新 trait + 新枚举变体，注册表本体一行不改。
//!
//! 与 embedded-hal 的分工：引脚/串口的寄存器级操作直接复用 embedded-hal 0.2
//! （`OutputPin`/`serial::Write` 等）——**片内外设的寄存器级驱动属于 HAL**；本层只补
//! embedded-hal 表达不了的两件事——**按名字找设备**（解耦"生产者与消费者互相不认识
//! 对方的结构"，这是 OS 驱动模型的事）与**阻塞 I/O 挂进任务状态机**（读没有字节时
//! 任务进入 `Blocked`，而不是空转轮询）。
//!
//! 纪律（与 `bus.rs` 同构）：
//! - 注册表内部状态全部包在 `RefCell` 里，所有访问都必须在 `sync::free` 临界区内，
//!   单核 + 关中断模型下不可能有数据竞争，故 `unsafe impl Send/Sync` 是 sound 的；
//! - 设备句柄是 `&'static dyn Trait`：设备实例由创建方 `Box::leak` 或存放在静态
//!   存储里，生命周期与系统等长——注册表只登记"谁是谁"，不拥有设备；
//! - trait 方法全是 `&self`：注册表分发的是共享引用，设备内部用 `RefCell`/原子
//!   自持可变状态（与内核 `Queue`/`Notifier` 同款）。

use core::cell::RefCell;
use crate::sync;

/// LED 类设备能力
pub trait LedDevice {
    /// 点亮
    fn on(&self);
    /// 熄灭
    fn off(&self);
    /// 翻转
    fn toggle(&self);
}

/// 串口类设备能力（阻塞式，挂进任务状态机）
pub trait UartDevice {
    /// 阻塞写出全部字节。写是"发送寄存器等下就空"的微秒级操作，
    /// 不值得进状态机，用轮询等待（不放弃 CPU）。
    fn write_all(&self, buf: &[u8]);
    /// 接收缓冲中待读字节数
    fn rx_len(&self) -> usize;
    /// 阻塞读一个字节：缓冲为空时任务进入 `Blocked` 挂起，
    /// 数据到达由中断唤醒——"等待即 Blocked"，比空转轮询省电省 CPU。
    /// 单读者契约：同一设备同一时刻只允许一个任务读。
    fn read_byte(&self) -> u8;
}

/// 块设备逻辑扇区大小（字节）。SD 卡协议约定扇区 512B，
/// 寻址单位 = 扇区号（SPI 模式下命令参数 = 字节地址 = 扇区号 << 9）。
pub const SECTOR_SIZE: u64 = 512;

/// 块设备错误。
/// 前两个变体来自硬件（超时/传输错误）；后两个专为 fatfs::io::IoError 的
/// 语义构造器服务（适配器在卷尾截断/零写入时构造它们）——见 fs/block.rs。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BdError {
    /// 命令/数据超时（R1 无应答、忙等超限）
    Timeout,
    /// 传输 IO 错误（坏扇区等硬件级失败）
    Io,
    /// 参数非法（扇区越界、seek 越界等）
    InvalidInput,
    /// 读到卷尾（`IoError::new_unexpected_eof_error` 的对应物）
    UnexpectedEof,
    /// 写入零字节（`IoError::new_write_zero_error` 的对应物）
    WriteZero,
}

/// 块设备类设备能力——**整个文件系统的地基只有两个函数**：读扇区、写扇区。
/// FAT 层不需要知道设备是 SD 卡还是内存盘；第 21 章的适配器把这两个函数
/// 翻译成 fatfs 要的"字节流 + 游标"。
///
/// 方法与 `LedDevice`/`UartDevice` 同款 `&self`（内部自持可变状态），为了
/// 能挂进 `&'static dyn` 注册表。**单使用者契约**：块设备的事务（一条命令 +
/// 一个扇区的数据）是协议原子单元，同一时刻只允许一个任务操作——第 21 章
/// 的用法是所有访问经同一把文件系统互斥锁串行化；违反契约由设备内部的
/// `RefCell` 双重借用 panic 探测（与 UART 的每字节短借用形成对照，见 ch21）。
pub trait BdDevice {
    /// 设备容量（扇区数）；0 = 无介质/未就绪
    fn sector_count(&self) -> u64;
    /// 读一个完整扇区（`buf.len() == SECTOR_SIZE`）。
    fn read_sector(&self, no: u64, buf: &mut [u8]) -> Result<(), BdError>;
    /// 写一个完整扇区（`data.len() == SECTOR_SIZE`）。
    fn write_sector(&self, no: u64, data: &[u8]) -> Result<(), BdError>;
}

// 引用转发：`&'static dyn BdDevice` 也是 `BdDevice`，适配器可以统一处理
// 具体设备（bsp 直用）与注册表出来的 trait 对象（示例用法）两种形态。
impl<T: BdDevice + ?Sized> BdDevice for &T {
    fn sector_count(&self) -> u64 {
        (**self).sector_count()
    }
    fn read_sector(&self, no: u64, buf: &mut [u8]) -> Result<(), BdError> {
        (**self).read_sector(no, buf)
    }
    fn write_sector(&self, no: u64, data: &[u8]) -> Result<(), BdError> {
        (**self).write_sector(no, data)
    }
}

/// 设备能力枚举——设备对象的"api 指针"（类型安全版）。
/// 新增设备类 = 加一个变体；注册表本体（register/find/unregister）不感知类。
#[derive(Clone, Copy)]
pub enum DeviceApi {
    /// LED 类设备
    Led(&'static dyn LedDevice),
    /// 串口类设备
    Uart(&'static dyn UartDevice),
    /// 块设备（读/写扇区；第 21 章文件系统的地基）
    Bd(&'static dyn BdDevice),
}

/// 注册表容量（定长数组，注册表不依赖堆；全部设备共用这一张表）
pub const DEV_CAP: usize = 8;

/// 注册错误
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrvError {
    /// 名字已被占用
    Duplicate,
    /// 注册表已满
    Full,
}

/// 驱动注册表：一张全局表，按名登记/查找设备能力
pub struct DriverRegistry {
    devices: RefCell<[Option<(&'static str, DeviceApi)>; DEV_CAP]>,
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

    /// 登记设备（通用）。重名返回 Err(Duplicate)，满返回 Err(Full)。
    /// 名字全局唯一：跨设备类同名同样算 Duplicate（Zephyr binding 语义）。
    pub fn register(&self, name: &'static str, api: DeviceApi) -> Result<(), DrvError> {
        sync::free(|_| {
            let mut devices = self.devices.borrow_mut();
            if devices
                .iter()
                .any(|e| matches!(e, Some((n, _)) if *n == name))
            {
                return Err(DrvError::Duplicate);
            }
            for slot in devices.iter_mut() {
                if slot.is_none() {
                    *slot = Some((name, api));
                    return Ok(());
                }
            }
            Err(DrvError::Full)
        })
    }

    /// 按名查找设备能力（通用）。拿到后按类匹配（`DeviceApi` 枚举），
    /// 也可用下面的 find_led/find_uart 直接取类句柄。
    pub fn find(&self, name: &str) -> Option<DeviceApi> {
        sync::free(|_| {
            self.devices.borrow().iter().find_map(|e| match e {
                Some((n, api)) if *n == name => Some(*api),
                _ => None,
            })
        })
    }

    /// 注销设备（通用，幂等）：名字不在时返回 false
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

    // ---- 类级便捷 API（薄包装；仿 Zephyr：绑定通用，类 API 分层其上）----

    /// 登记一个 LED 设备（等价 `register(name, DeviceApi::Led(dev))`）。
    /// 重名返回 Err(Duplicate)，满返回 Err(Full)。
    pub fn register_led(
        &self,
        name: &'static str,
        dev: &'static dyn LedDevice,
    ) -> Result<(), DrvError> {
        self.register(name, DeviceApi::Led(dev))
    }

    /// 按名查找 LED 设备；名字存在但不是 LED 类时返回 None（用 `find` 可区分"没有"与"类不符"）
    pub fn find_led(&self, name: &str) -> Option<&'static dyn LedDevice> {
        match self.find(name) {
            Some(DeviceApi::Led(dev)) => Some(dev),
            _ => None,
        }
    }

    /// 注销 LED 设备（幂等）：仅当该名字确实是 LED 类时删除
    pub fn unregister_led(&self, name: &str) -> bool {
        sync::free(|_| {
            let mut devices = self.devices.borrow_mut();
            for slot in devices.iter_mut() {
                if let Some((n, DeviceApi::Led(_))) = slot {
                    if *n == name {
                        *slot = None;
                        return true;
                    }
                }
            }
            false
        })
    }

    /// 登记一个串口设备（等价 `register(name, DeviceApi::Uart(dev))`）。
    /// 重名返回 Err(Duplicate)，满返回 Err(Full)。
    pub fn register_uart(
        &self,
        name: &'static str,
        dev: &'static dyn UartDevice,
    ) -> Result<(), DrvError> {
        self.register(name, DeviceApi::Uart(dev))
    }

    /// 按名查找串口设备；名字存在但不是 UART 类时返回 None（用 `find` 可区分"没有"与"类不符"）
    pub fn find_uart(&self, name: &str) -> Option<&'static dyn UartDevice> {
        match self.find(name) {
            Some(DeviceApi::Uart(dev)) => Some(dev),
            _ => None,
        }
    }

    /// 注销串口设备（幂等）：仅当该名字确实是 UART 类时删除
    pub fn unregister_uart(&self, name: &str) -> bool {
        sync::free(|_| {
            let mut devices = self.devices.borrow_mut();
            for slot in devices.iter_mut() {
                if let Some((n, DeviceApi::Uart(_))) = slot {
                    if *n == name {
                        *slot = None;
                        return true;
                    }
                }
            }
            false
        })
    }

    /// 登记一个块设备（等价 `register(name, DeviceApi::Bd(dev))`）。
    /// 重名返回 Err(Duplicate)，满返回 Err(Full)。
    pub fn register_bd(
        &self,
        name: &'static str,
        dev: &'static dyn BdDevice,
    ) -> Result<(), DrvError> {
        self.register(name, DeviceApi::Bd(dev))
    }

    /// 按名查找块设备；名字存在但不是块设备类时返回 None（用 `find` 可区分"没有"与"类不符"）
    pub fn find_bd(&self, name: &str) -> Option<&'static dyn BdDevice> {
        match self.find(name) {
            Some(DeviceApi::Bd(dev)) => Some(dev),
            _ => None,
        }
    }

    /// 注销块设备（幂等）：仅当该名字确实是块设备类时删除
    pub fn unregister_bd(&self, name: &str) -> bool {
        sync::free(|_| {
            let mut devices = self.devices.borrow_mut();
            for slot in devices.iter_mut() {
                if let Some((n, DeviceApi::Bd(_))) = slot {
                    if *n == name {
                        *slot = None;
                        return true;
                    }
                }
            }
            false
        })
    }
}

/// 系统全局注册表（应用初始化时向它登记设备）
pub static REGISTRY: DriverRegistry = DriverRegistry::new();

/// 向全局注册表登记设备（通用）
pub fn register(name: &'static str, api: DeviceApi) -> Result<(), DrvError> {
    REGISTRY.register(name, api)
}

/// 从全局注册表查找设备能力（通用）
pub fn find(name: &str) -> Option<DeviceApi> {
    REGISTRY.find(name)
}

/// 从全局注册表注销设备（通用，幂等）
pub fn unregister(name: &str) -> bool {
    REGISTRY.unregister(name)
}

/// 向全局注册表登记 LED 设备
pub fn register_led(name: &'static str, dev: &'static dyn LedDevice) -> Result<(), DrvError> {
    REGISTRY.register_led(name, dev)
}

/// 从全局注册表查找 LED 设备
pub fn find_led(name: &str) -> Option<&'static dyn LedDevice> {
    REGISTRY.find_led(name)
}

/// 从全局注册表注销 LED 设备
pub fn unregister_led(name: &str) -> bool {
    REGISTRY.unregister_led(name)
}

/// 向全局注册表登记串口设备
pub fn register_uart(name: &'static str, dev: &'static dyn UartDevice) -> Result<(), DrvError> {
    REGISTRY.register_uart(name, dev)
}

/// 从全局注册表查找串口设备
pub fn find_uart(name: &str) -> Option<&'static dyn UartDevice> {
    REGISTRY.find_uart(name)
}

/// 从全局注册表注销串口设备
pub fn unregister_uart(name: &str) -> bool {
    REGISTRY.unregister_uart(name)
}

/// 向全局注册表登记块设备
pub fn register_bd(name: &'static str, dev: &'static dyn BdDevice) -> Result<(), DrvError> {
    REGISTRY.register_bd(name, dev)
}

/// 从全局注册表查找块设备
pub fn find_bd(name: &str) -> Option<&'static dyn BdDevice> {
    REGISTRY.find_bd(name)
}

/// 从全局注册表注销块设备
pub fn unregister_bd(name: &str) -> bool {
    REGISTRY.unregister_bd(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::boxed::Box;
    use alloc::sync::Arc;
    use core::sync::atomic::{AtomicUsize, Ordering};

    /// mock LED：on 计数 +1、toggle 计数 +10，用于区分调用
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

    /// mock UART：write_all 把写入字节数累加；读路径恒空（host 上无真实任务切换）
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

    /// mock 块设备：容量 2 扇区；每次读/写把扇区号累进计数（透传验证）
    struct MockBd {
        ops: Arc<AtomicUsize>,
    }
    impl BdDevice for MockBd {
        fn sector_count(&self) -> u64 {
            2
        }
        fn read_sector(&self, _no: u64, _buf: &mut [u8]) -> Result<(), BdError> {
            self.ops.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
        fn write_sector(&self, _no: u64, _data: &[u8]) -> Result<(), BdError> {
            self.ops.fetch_add(10, Ordering::SeqCst);
            Ok(())
        }
    }

    fn mock_led(name: &'static str, reg: &DriverRegistry) -> Arc<AtomicUsize> {
        let count = Arc::new(AtomicUsize::new(0));
        let dev: &'static dyn LedDevice =
            Box::leak(Box::new(MockLed { count: count.clone() }));
        reg.register_led(name, dev).unwrap();
        count
    }

    fn mock_uart(name: &'static str, reg: &DriverRegistry) -> Arc<AtomicUsize> {
        let written = Arc::new(AtomicUsize::new(0));
        let dev: &'static dyn UartDevice =
            Box::leak(Box::new(MockUart { written: written.clone() }));
        reg.register_uart(name, dev).unwrap();
        written
    }

    /// 回归：注册→查找→使用 全流程（LED 类）。
    /// mock 设备计数验证"查到的确实是同一个实例"（引用未换手）。
    #[test]
    fn led_register_find_use_roundtrip() {
        let reg = DriverRegistry::new();
        let count = mock_led("red", &reg);
        let dev = reg.find_led("red").expect("注册后应能找到");
        dev.on();
        dev.toggle();
        assert_eq!(count.load(Ordering::SeqCst), 11, "on(+1) 与 toggle(+10) 都应生效");
    }

    /// 回归：注册→查找→使用 全流程（UART 类）。
    /// write_all 的字节数透传到 mock 计数器，钉死"分发到同一实例"。
    #[test]
    fn uart_register_find_use_roundtrip() {
        let reg = DriverRegistry::new();
        let written = mock_uart("uart0", &reg);
        let dev = reg.find_uart("uart0").expect("注册后应能找到");
        dev.write_all(b"hi");
        assert_eq!(written.load(Ordering::SeqCst), 2, "写入字节数应透传");
        assert_eq!(dev.rx_len(), 0);
    }

    /// 回归：重名必须拒绝——同名二次注册 Err(Duplicate)，第一个设备不受影响。
    /// 阳性对照：若实现漏掉重名检查（直接覆盖），注册重名返回 Ok 即测试红。
    #[test]
    fn duplicate_name_rejected() {
        let reg = DriverRegistry::new();
        let count = mock_led("red", &reg);
        let count2 = Arc::new(AtomicUsize::new(0));
        let dev2: &'static dyn LedDevice =
            Box::leak(Box::new(MockLed { count: count2.clone() }));
        assert_eq!(reg.register_led("red", dev2), Err(DrvError::Duplicate));
        // 名字仍指向第一个设备（未被子覆盖）
        reg.find_led("red").unwrap().on();
        assert_eq!(count.load(Ordering::SeqCst), 1);
        assert_eq!(count2.load(Ordering::SeqCst), 0);
    }

    /// 回归：名字全局唯一（Zephyr binding 语义）——LED 已占的名字,
    /// UART 同名的二次注册必须拒绝，而不是各挂名的。
    #[test]
    fn names_are_global_across_classes() {
        let reg = DriverRegistry::new();
        let count = mock_led("dev0", &reg);
        let dev: &'static dyn UartDevice =
            Box::leak(Box::new(MockUart { written: Arc::new(AtomicUsize::new(0)) }));
        assert_eq!(
            reg.register_uart("dev0", dev),
            Err(DrvError::Duplicate),
            "跨类同名必须拒绝（名字全局唯一）"
        );
        // 原设备不受影响
        reg.find_led("dev0").unwrap().on();
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }

    /// 回归：类不符的查找——名字存在但是另一类设备，find_led 应返回 None；
    /// 通用 find 能查到并区分（DeviceApi::Uart）。
    #[test]
    fn wrong_class_lookup_is_none() {
        let reg = DriverRegistry::new();
        mock_uart("port0", &reg);
        assert!(reg.find_led("port0").is_none(), "名字是 UART 类，按 LED 找应得 None");
        match reg.find("port0") {
            Some(DeviceApi::Uart(_)) => {}
            other => panic!("find 应返回 Uart 变体，实际 {:?}", other.map(|_| "非 Uart")),
        }
    }

    /// 回归：容量满必须报错（阳性对照——定长数组漏检查时第 9 个静默丢 => 红）。
    #[test]
    fn capacity_full_error() {
        let reg = DriverRegistry::new();
        for i in 0..DEV_CAP {
            let count = Arc::new(AtomicUsize::new(0));
            // mock 名字各不相同，写进常量池（'static）
            let name = Box::leak(format!("led{i}").into_boxed_str()) as &'static str;
            let dev: &'static dyn LedDevice = Box::leak(Box::new(MockLed { count }));
            reg.register_led(name, dev).unwrap();
        }
        let dev: &'static dyn LedDevice =
            Box::leak(Box::new(MockLed { count: Arc::new(AtomicUsize::new(0)) }));
        assert_eq!(
            reg.register_led("overflow", dev),
            Err(DrvError::Full),
            "第 {} 个应报满",DEV_CAP + 1
        );
    }

    /// 回归：未注册的名字返回 None（不 panic、不误配）。
    #[test]
    fn unknown_name_is_none() {
        let reg = DriverRegistry::new();
        assert!(reg.find("ghost").is_none());
        assert!(reg.find_led("ghost").is_none());
        assert!(reg.find_uart("ghost").is_none());
    }

    /// 回归：注销幂等——首次 true、二次 false，其余槽位不受影响。
    #[test]
    fn unregister_is_idempotent() {
        let reg = DriverRegistry::new();
        let count = mock_led("red", &reg);
        drop(count);
        assert!(reg.unregister_led("red"), "在册的设备注销应返回 true");
        assert!(!reg.unregister_led("red"), "二次注销应返回 false（幂等）");
        // 注销后按名找不到；已拿到的旧引用仍有效（注册表只清名字映射，不拥有设备）
        assert!(reg.find_led("red").is_none());
    }

    /// 回归：跨类注销互不干扰——UART 名字不会被 unregister_led 误删。
    #[test]
    fn unregister_does_not_touch_other_class() {
        let reg = DriverRegistry::new();
        mock_led("led0", &reg);
        let written = mock_uart("u0", &reg);
        assert!(!reg.unregister_led("u0"), "UART 的名字不该被 LED 注销碰掉");
        reg.find_uart("u0").unwrap().write_all(b"abc");
        assert_eq!(written.load(Ordering::SeqCst), 3);
    }

    /// 回归：Bd 类注册→查找→使用 全流程；容量与读/写计数透传验证"同一实例"。
    #[test]
    fn bd_register_find_use_roundtrip() {
        let reg = DriverRegistry::new();
        let ops = Arc::new(AtomicUsize::new(0));
        let dev: &'static dyn BdDevice = Box::leak(Box::new(MockBd { ops: ops.clone() }));
        reg.register_bd("sd0", dev).unwrap();
        let bd = reg.find_bd("sd0").expect("注册后应能找到");
        assert_eq!(bd.sector_count(), 2);
        let mut buf = [0u8; SECTOR_SIZE as usize];
        bd.read_sector(0, &mut buf).unwrap();
        bd.write_sector(1, &buf).unwrap();
        assert_eq!(ops.load(Ordering::SeqCst), 11, "读(+1) 与写(+10) 都应透传");
        // 类不符：按 UART 找应 None；通用 find 能区分
        assert!(reg.find_uart("sd0").is_none(), "块设备名字按 UART 找应得 None");
        match reg.find("sd0") {
            Some(DeviceApi::Bd(_)) => {}
            other => panic!("find 应返回 Bd 变体，实际 {:?}", other.map(|_| "非 Bd")),
        }
    }

    /// 回归：Bd 同名二次注册拒绝（名字全局唯一不因类而异）。
    #[test]
    fn bd_duplicate_name_rejected() {
        let reg = DriverRegistry::new();
        let dev: &'static dyn BdDevice =
            Box::leak(Box::new(MockBd { ops: Arc::new(AtomicUsize::new(0)) }));
        reg.register_bd("sd0", dev).unwrap();
        let dev2: &'static dyn BdDevice =
            Box::leak(Box::new(MockBd { ops: Arc::new(AtomicUsize::new(0)) }));
        assert_eq!(
            reg.register_bd("sd0", dev2),
            Err(DrvError::Duplicate),
            "Bd 重名必须拒绝"
        );
    }
}
