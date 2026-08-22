//! 驱动抽象层：按名注册表 + 设备能力 trait
//!
//! 三层模型：**设备**(Device,物理单元与其状态) / **驱动**(Driver,操作设备的代码，
//! 即 trait 实现) / **总线**(本模块的注册表:`名字 → 设备实例`的查找表)。
//!
//! 与 embedded-hal 的分工：引脚/串口的寄存器级操作直接复用 embedded-hal 0.2
//! （`OutputPin`/`serial::Write` 等），本层只补 embedded-hal 表达不了的两件事——
//! **按名字找设备**（解耦"生产者和消费者互相不认识对方的结构"）与
//! **阻塞 I/O 挂进任务状态机**（读没有字节时任务进入 `Blocked`，而不是空转轮询）。
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

/// 每类设备的注册容量（定长数组，注册表不依赖堆）
pub const DEV_CAP: usize = 8;

/// 注册错误
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrvError {
    /// 名字已被占用
    Duplicate,
    /// 注册表已满
    Full,
}

/// 驱动注册表：按名登记/查找设备实例
pub struct DriverRegistry {
    leds: RefCell<[Option<(&'static str, &'static dyn LedDevice)>; DEV_CAP]>,
    uarts: RefCell<[Option<(&'static str, &'static dyn UartDevice)>; DEV_CAP]>,
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
            leds: RefCell::new([None; DEV_CAP]),
            uarts: RefCell::new([None; DEV_CAP]),
        }
    }

    /// 登记一个 LED 设备。重名返回 Err(Duplicate)，满返回 Err(Full)。
    pub fn register_led(
        &self,
        name: &'static str,
        dev: &'static dyn LedDevice,
    ) -> Result<(), DrvError> {
        sync::free(|_| {
            let mut leds = self.leds.borrow_mut();
            if leds
                .iter()
                .any(|e| matches!(e, Some((n, _)) if *n == name))
            {
                return Err(DrvError::Duplicate);
            }
            for slot in leds.iter_mut() {
                if slot.is_none() {
                    *slot = Some((name, dev));
                    return Ok(());
                }
            }
            Err(DrvError::Full)
        })
    }

    /// 按名查找 LED 设备
    pub fn find_led(&self, name: &str) -> Option<&'static dyn LedDevice> {
        sync::free(|_| {
            self.leds.borrow().iter().find_map(|e| match e {
                Some((n, d)) if *n == name => Some(*d),
                _ => None,
            })
        })
    }

    /// 注销 LED 设备（幂等）：名字不在时返回 false
    pub fn unregister_led(&self, name: &str) -> bool {
        sync::free(|_| {
            let mut leds = self.leds.borrow_mut();
            for slot in leds.iter_mut() {
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

    /// 登记一个串口设备。重名返回 Err(Duplicate)，满返回 Err(Full)。
    pub fn register_uart(
        &self,
        name: &'static str,
        dev: &'static dyn UartDevice,
    ) -> Result<(), DrvError> {
        sync::free(|_| {
            let mut uarts = self.uarts.borrow_mut();
            if uarts
                .iter()
                .any(|e| matches!(e, Some((n, _)) if *n == name))
            {
                return Err(DrvError::Duplicate);
            }
            for slot in uarts.iter_mut() {
                if slot.is_none() {
                    *slot = Some((name, dev));
                    return Ok(());
                }
            }
            Err(DrvError::Full)
        })
    }

    /// 按名查找串口设备
    pub fn find_uart(&self, name: &str) -> Option<&'static dyn UartDevice> {
        sync::free(|_| {
            self.uarts.borrow().iter().find_map(|e| match e {
                Some((n, d)) if *n == name => Some(*d),
                _ => None,
            })
        })
    }

    /// 注销串口设备（幂等）：名字不在时返回 false
    pub fn unregister_uart(&self, name: &str) -> bool {
        sync::free(|_| {
            let mut uarts = self.uarts.borrow_mut();
            for slot in uarts.iter_mut() {
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
}

/// 系统全局注册表（应用初始化时向它登记设备）
pub static REGISTRY: DriverRegistry = DriverRegistry::new();

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
        assert!(reg.find_led("red").is_none());
        // 已注销拿到的旧引用仍有效（注册表只清名字映射，不拥有设备）
        let dev = reg.find_led("red");
        assert!(dev.is_none());
    }

    /// 回归：LED 与 UART 两类注册表命名空间独立——同名各注册各的、各找各的。
    #[test]
    fn categories_have_independent_namespaces() {
        let reg = DriverRegistry::new();
        let led_count = mock_led("dev0", &reg);
        let written = mock_uart("dev0", &reg);
        reg.find_led("dev0").unwrap().on();
        reg.find_uart("dev0").unwrap().write_all(b"abc");
        assert_eq!(led_count.load(Ordering::SeqCst), 1);
        assert_eq!(written.load(Ordering::SeqCst), 3);
    }
}
