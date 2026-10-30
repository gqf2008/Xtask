//! longan_nano UART0 设备：中断驱动接收，缓冲与唤醒都交给 `device::EventChannel`
//!
//! 为什么用 `EventChannel<u8, 64>` 而不是自建环形缓冲（v3 之前的写法）：
//! - 定长、零分配（`[MaybeUninit; N]` 内联在通道里）——`Queue` 的 `VecDeque`
//!   首次 push 会堆分配，而**分配绝不能出现在 ISR 里**（堆不可重入）；
//! - 通道自带单等待者槽 + 唤醒（`post_isr` 内部即完成唤醒 + 软中断），驱动
//!   不必再自己维护 `waiter: AtomicUsize` 与 `Porting::irq()`；
//! - 同一台机器只留一份实现：满/关闭/溢出计数/`recv_timeout` 都是通道的既有
//!   语义，`qemu_kernel_tests` 25–27 与 `event.rs` 单测在跑它们。
//!
//! 纪律：**ISR 是唯一生产者**（`post_isr`），**读任务是唯一消费者**
//! （`StreamDevice::read` 经 `try_recv` 排空）。`EventDevice` 只是把裸等待者
//! 登记转发给通道，供内核通用适配器 `device::read_blocking`（`&dyn` 路径）使用——
//! “复查 + 登记 + 挂起”在同一临界区，丢失唤醒从根上消除。
//!
//! 能力组合（`device` 层）：`StreamDevice`（非阻塞读/轮询写）+ `EventDevice`
//! （裸等待者转发）；控制面已删除（v3：能力要么真能，要么不存在）。

use alloc::boxed::Box;
use core::cell::{RefCell, UnsafeCell};
use core::sync::atomic::{AtomicBool, AtomicPtr, Ordering};

use gd32vf103xx_hal::afio::Afio;
use gd32vf103xx_hal::eclic::*;
use gd32vf103xx_hal::gpio::gpioa::{PA10, PA9};
use gd32vf103xx_hal::gpio::Active;
use gd32vf103xx_hal::pac::{Interrupt, ECLIC, USART0};
use gd32vf103xx_hal::prelude::*;
use gd32vf103xx_hal::rcu::Rcu;
use gd32vf103xx_hal::serial::{Config, Parity, Rx, Serial, StopBits, Tx};
use gd32vf103xx_hal::time::Bps;

use crate::device::event::EventChannel;
use crate::device::{Device, DeviceBase, DeviceError, DeviceKind, EventDevice, StreamDevice};
use crate::sync;

/// 接收事件通道容量。
///
/// 定为 64 而不是 §8.3 的默认 8：USART0 是 16 字节 RX FIFO + 本板 57600 波特率，
/// 一个调度节拍（1ms）内最坏到达 ≈ 6 字节；64 ≈ 16B FIFO × 4 轮突发，给
/// `read_blocking` 的任务唤醒延迟留余量。满时丢弃并计 `overflow_count`（不覆盖）。
const BUF_LEN: usize = 64;

/// ISR 转发锚点：`new()` 里设备 `Box::leak` 成 'static 后存入，供 `uart0_isr()` 定位
static UART0_DEV: AtomicPtr<Uart0> = AtomicPtr::new(core::ptr::null_mut());

/// UART0 设备：阻塞读由内核 `read_blocking` 适配器挂进任务状态机（"等待即 Blocked"）
pub struct Uart0 {
    /// 发送侧（仅任务访问，RefCell 提供 `&self` 下的可变借用）
    tx: RefCell<Tx<USART0>>,
    /// 接收侧硬件句柄：**只允许 ISR 读**（SPSC 的"唯一生产者"）
    rx: UnsafeCell<Rx<USART0>>,
    /// 接收事件通道（定长、零分配）：ISR `post_isr` 投递，读任务 `try_recv` 排空；
    /// 满/关闭/溢出计数/唤醒都是它的既有语义
    events: EventChannel<u8, BUF_LEN>,
    /// 单订阅：`events()` 只在首次返回通道，第二次 `Busy`（§5.8）
    events_taken: AtomicBool,
}

// SAFETY:
// - `rx` 的访问只发生在 ISR（唯一读方），任务从不碰硬件句柄；
// - `events` 自带原子游标与临界区（`EventChannel` 自身即 `Sync`）：ISR 侧
//   `post_isr`、任务侧 `try_recv`、桥接的裸等待者登记都走它的临界区；
// - `tx` 的任务侧访问全部在 `sync::free`（MIE=0）临界区内，与 ISR 串行化。
unsafe impl Sync for Uart0 {}

impl Uart0 {
    /// 初始化 UART0：配串口 → 挂接收中断 → 登记锚点 → **最后才 unmask**
    ///
    /// 顺序是硬约束：锚点就绪之前绝不能打开中断，否则 ISR 先于 `new()` 返回
    /// 被触发时 `uart0_isr()` 会解引用 null 指针。
    pub fn new<X, Y>(
        uart: USART0,
        tx: PA9<X>,
        rx: PA10<Y>,
        baud_rate: Bps,
        afio: &mut Afio,
        rcu: &mut Rcu,
    ) -> &'static Uart0
    where
        X: Active,
        Y: Active,
    {
        // 与 chip/gd32vf103/stdout.rs 同款接线：PA9-TX 推挽复用、PA10-RX 浮空输入
        let tx = tx.into_alternate_push_pull();
        let rx = rx.into_floating_input();
        let config = Config {
            baudrate: baud_rate,
            parity: Parity::ParityNone,
            stopbits: StopBits::STOP1,
        };
        let serial = Serial::new(uart, (tx, rx), config, afio, rcu);
        let (tx, mut rx) = serial.split();
        // 只挂接收中断（RBNE）；发送走轮询（微秒级，不值得进状态机）
        rx.listen();

        let dev = Box::leak(Box::new(Self {
            tx: RefCell::new(tx),
            rx: UnsafeCell::new(rx),
            events: EventChannel::new(),
            events_taken: AtomicBool::new(false),
        }));
        UART0_DEV.store(dev, Ordering::SeqCst);

        // 电平触发：RBNE 置位期间中断输出持续有效，ISR 排空后自动回落。
        // 与内核定时器/软中断同级（L0/P0）：互不嵌套、不打断内核调度。
        // SAFETY: 上述锚点已就绪，此时打开中断才安全。
        unsafe {
            ECLIC::setup(
                Interrupt::USART0,
                TriggerType::Level,
                Level::L0,
                Priority::P0,
            );
            ECLIC::unmask(Interrupt::USART0);
        }
        dev
    }

    /// 直接拿接收通道（typed 快速路径）：`recv` / `recv_timeout` / `len` / `close`
    /// 都在通道上，不经 `&dyn`、不碰能力面。
    ///
    /// **单订阅**（§5.8）：首次调用返回通道，之后一律 `Err(Busy)`——通道是单消费者
    /// （SPSC），与 `read_blocking` / `StreamDevice::read` 二选一。订阅在设备生命周期
    /// 内不释放；要结束消费请 `close()` 通道，而不是重复 `events()`。
    /// 注意：该标志只约束 `events()` 的调用者——`read_blocking` / `StreamDevice::read`
    /// 不经过它，「通道消费与 read_blocking 二选一」仍是调用方纪律（不作运行时检查）。
    pub fn events(&self) -> Result<&EventChannel<u8, BUF_LEN>, DeviceError> {
        if self
            .events_taken
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return Err(DeviceError::Busy);
        }
        Ok(&self.events)
    }

    /// 因通道满（已存 `BUF_LEN` 字节）而丢弃的字节数。诊断用，语义等同旧实现的
    /// `dropped` 计数器；溢出时旧事件**不**被覆盖。
    pub fn dropped(&self) -> usize {
        self.events.overflow_count()
    }

    /// 中断服务例程（由应用层的 `#[no_mangle] extern "C" fn USART0()` 转发调用）：
    /// 排空接收寄存器，逐字节投递给 `EventChannel`——**唤醒与软中断都由通道内部完成**
    /// （`post_isr` → 唤醒单等待者 + `Porting::irq()`），驱动不再自己 `wakeup`。
    /// ISR 纪律：不允许阻塞/格式化/分配；`post_isr` 内部只取一段**可重入**临界区
    /// （MIE=0），在 ISR 里调用安全。
    /// 成本记账：57600bps 满速 ≈ 576 B/s，每字节一次 `post_isr`（一段临界区进出）
    /// → 约 576 次/秒；108MHz 下占比可忽略，将来上 1 Mbaud 时需回看这条。
    #[inline]
    pub fn isr(&self) {
        // SAFETY: `rx` 是"ISR 独占"：读任务从不碰硬件句柄，只消费事件通道
        let rx = unsafe { &mut *self.rx.get() };
        loop {
            match rx.read() {
                // 满 → `Err(Full)`：通道自己累计 `overflow_count()`，旧事件不被覆盖，
                // 本字节丢弃（与旧实现 `dropped` 计数同语义）。不 break：电平触发下
                // 必须把硬件 FIFO 排空，否则中断源一直挂着。
                Ok(b) => {
                    let _ = self.events.post_isr(b);
                }
                Err(nb::Error::WouldBlock) => break,
                // HAL 的错误返回已经把错误标志读清（stat+data 各读一次），继续排空
                Err(nb::Error::Other(_)) => continue,
            }
        }
    }
}

/// USART0 向量入口：示例里定义
/// `#[no_mangle] extern "C" fn USART0() { ...drv_uart::uart0_isr(); }`
/// port.S 的 `vectors` 表对 USART0 声明了 `.weak`，应用层实现同名符号即被链接器绑定。
pub fn uart0_isr() {
    let ptr = UART0_DEV.load(Ordering::SeqCst);
    // SAFETY: `new()` 里"先存锚点、后 unmask"的顺序保证 ISR 首次触发时锚点必非空；
    // 设备 Box::leak 后为 'static，引用全程有效
    let dev = unsafe { &*ptr };
    dev.isr();
}

impl DeviceBase for Uart0 {
    fn kind(&self) -> DeviceKind {
        DeviceKind::Stream
    }
}

impl Device for Uart0 {
    fn as_stream(&self) -> Option<&dyn StreamDevice> {
        Some(self)
    }

    fn as_event(&self) -> Option<&dyn EventDevice> {
        Some(self)
    }
}

impl StreamDevice for Uart0 {
    /// 通道中待读字节数。通道内部取一次可重入临界区：内核阻塞适配器本就在
    /// `sync::free` 内调它，临界区可重入，不会自锁。
    fn available(&self) -> usize {
        self.events.len()
    }

    /// 非阻塞读：把通道里现有字节尽量搬进 `buf`，返回实际字节数；无数据 Ok(0)。
    /// **单读者契约**：同一时刻只允许一个消费者（SPSC 的唯一消费者）。
    /// 阻塞读由内核 `read_blocking` 适配器实现（组合本方法与 EventDevice）。
    fn read(&self, buf: &mut [u8]) -> Result<usize, DeviceError> {
        let mut n = 0;
        while n < buf.len() {
            match self.events.try_recv() {
                Some(b) => {
                    buf[n] = b;
                    n += 1;
                }
                None => break,
            }
        }
        Ok(n)
    }

    /// 写出全部字节。发送是"等 TXE"的微秒级操作，不值得进任务状态机，
    /// 用轮询等待（不放弃 CPU）。每字节一段临界区：RefCell 借用不能跨越
    /// 任务抢占（借用不关中断，跨切换会让被抢占任务 double-borrow panic）；
    /// 57600 波特率下每字节最长约 174 µs，远短于一个调度节拍，不会饿死节拍中断。
    fn write(&self, buf: &[u8]) -> Result<usize, DeviceError> {
        for &b in buf {
            sync::free(|_| {
                let mut tx = self.tx.borrow_mut();
                // Write 的 Error 是 Infallible，实际只会在空转等 TXE
                let _ = nb::block!(tx.write(b));
            });
        }
        Ok(buf.len())
    }
}

impl EventDevice for Uart0 {
    /// 转发给通道的单等待者槽。**lock-free**（CAS）：内核阻塞适配器把它放在
    /// "复查 available + 挂起"的同一段 `sync::free` 里，ISR 不可能插进"发现空"
    /// 与"挂起"之间。已有人在等（含并发的 `recv`）→ `Busy`，绝不覆盖。
    fn register_waiter(&self, waiter: usize) -> Result<(), DeviceError> {
        self.events.register_raw_waiter(waiter)
    }

    /// 唤醒阻塞读者（ISR 侧调用）：走通道同一条唤醒路径（含软中断）。
    /// 正常数据路径不需要它——`post_isr` 已经唤醒；保留它是为了 `EventDevice`
    /// 契约完整（"状态就绪但无数据"类设备没有事件可投递）。
    fn wake(&self) {
        self.events.wake_raw_waiter()
    }
}
