//! longan_nano UART0 设备：中断驱动接收 + 定长 SPSC 环形缓冲
//!
//! 读路径为什么自建环形缓冲而不是用内核 `Queue<u8>`：
//! - `Queue` 的 `VecDeque` 首次 push 会触发堆分配，而**分配绝不能出现在
//!   ISR 里**（堆不可重入：ISR 打断正在分配的任务会破坏堆）；
//! - `Queue::clear()` 曾只清列表不清信号量计数，预热后 clear 会产生幽灵出队
//!   （遗留问题 #7 已于 2026-08-22 修复：clear/trancate 现同步计数，
//!   见第 20 章踩坑记录 3）。
//! 因此这里用 64 字节定长环形缓冲：容量恒定、零分配，读写双方约定——
//! **ISR 是唯一生产者**（写 head），**读任务是唯一消费者**（写 tail）。
//!
//! 唤醒纪律（与内核信号量同款"检查+登记+挂起在同一临界区"）：
//! 任务在 `sync::free` 里先复查缓冲，仍为空才登记 waiter 并 `block()`，
//! ISR 不可能插进"发现空"与"挂起"之间——丢失唤醒从根上消除
//! （反面教材：`sync::notify` 的 CAS 在临界区外——遗留问题 #6，见第 20 章踩坑记录）。

use alloc::boxed::Box;
use core::cell::{RefCell, UnsafeCell};
use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

use gd32vf103xx_hal::afio::Afio;
use gd32vf103xx_hal::eclic::*;
use gd32vf103xx_hal::gpio::gpioa::{PA9, PA10};
use gd32vf103xx_hal::gpio::Active;
use gd32vf103xx_hal::pac::{ECLIC, Interrupt, USART0};
use gd32vf103xx_hal::prelude::*;
use gd32vf103xx_hal::rcu::Rcu;
use gd32vf103xx_hal::serial::{Config, Parity, Rx, Serial, StopBits, Tx};
use gd32vf103xx_hal::time::Bps;

use crate::drv::UartDevice;
use crate::port::{Portable, Porting};
use crate::sync;
use crate::task::executor::{xworker, Executor};
use crate::task::{yield_now, Task};

/// 环形缓冲长度（2 的幂，下标用 `& (BUF_LEN - 1)`）
const BUF_LEN: usize = 64;

/// ISR 转发锚点：`new()` 里设备 `Box::leak` 成 'static 后存入，供 `uart0_isr()` 定位
static UART0_DEV: AtomicPtr<Uart0> = AtomicPtr::new(core::ptr::null_mut());

/// UART0 设备：阻塞读挂进任务状态机（"等待即 Blocked"）
pub struct Uart0 {
    /// 发送侧（仅任务访问，RefCell 提供 `&self` 下的可变借用）
    tx: RefCell<Tx<USART0>>,
    /// 接收侧硬件句柄：**只允许 ISR 读**（SPSC 的"唯一生产者"）
    rx: UnsafeCell<Rx<USART0>>,
    /// 定长环形缓冲（零分配）
    buf: [UnsafeCell<u8>; BUF_LEN],
    /// 生产位置：仅 ISR 写（读任务只读）
    head: AtomicUsize,
    /// 消费位置：仅读任务写（ISR 只读）
    tail: AtomicUsize,
    /// 阻塞读者任务指针（0 = 无）：仅读任务写、ISR 取走
    waiter: AtomicUsize,
    /// 缓冲满被丢弃的字节数（诊断用；目前只累计不读出）
    #[allow(dead_code)]
    dropped: AtomicUsize,
}

// SAFETY: 单硬件线程 + SPSC 纪律（与内核信号量同款的并发放置方式）：
// - `head` 仅 ISR 写、`tail` 仅任务写，两侧互不相交，原子读改写即可；
// - `rx` 的访问只发生在 ISR（唯一读方），任务只消费环形缓冲；
// - `tx`/`waiter`/缓冲槽的任务侧访问全部在 `sync::free`（MIE=0）临界区内，
//   与 ISR 串行化——临界区同时保证"检查+登记+挂起"三步不可拆分。
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
            buf: [const { UnsafeCell::new(0) }; BUF_LEN],
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            waiter: AtomicUsize::new(0),
            dropped: AtomicUsize::new(0),
        }));
        UART0_DEV.store(dev, Ordering::SeqCst);

        // 电平触发：RBNE 置位期间中断输出持续有效，ISR 排空后自动回落。
        // 与内核定时器/软中断同级（L0/P0）：互不嵌套、不打断内核调度。
        // SAFETY: 上述锚点已就绪，此时打开中断才安全。
        unsafe {
            ECLIC::setup(Interrupt::USART0, TriggerType::Level, Level::L0, Priority::P0);
            ECLIC::unmask(Interrupt::USART0);
        }
        dev
    }

    /// 中断服务例程（由应用层的 `#[no_mangle] extern "C" fn USART0()` 转发调用）：
    /// 排空接收寄存器进环形缓冲，随后**必须自己触发软中断**——port.S 的 ISR
    /// 返回路径直接恢复被中断任务、没有 schedule()（先例：定时器中断 port.rs）。
    /// ISR 纪律：不允许临界区/阻塞/格式化/分配——本函数只用原子操作与硬件读。
    #[inline]
    pub fn isr(&self) {
        let mut got = false;
        // SAFETY: `rx` 是"ISR 独占"：读任务从不碰硬件句柄，只消费环形缓冲
        let rx = unsafe { &mut *self.rx.get() };
        loop {
            match rx.read() {
                Ok(b) => {
                    got = true;
                    if !self.try_enqueue(b) {
                        self.dropped.fetch_add(1, Ordering::Relaxed);
                    }
                }
                Err(nb::Error::WouldBlock) => break,
                // HAL 的错误返回已经把错误标志读清（stat+data 各读一次），继续排空
                Err(nb::Error::Other(_)) => continue,
            }
        }
        if got {
            // 唤醒阻塞读者：waiter 是"检查+登记+挂起"同一临界区写下的，
            // 取到非零即对应一次真实的阻塞——不会唤醒到已不在等的人
            let w = self.waiter.swap(0, Ordering::SeqCst);
            if w != 0 {
                // SAFETY: waiter 存的是 xworker.current() 的有效任务指针，
                // 任务未 Terminated 前不会被释放；wakeup 只把状态改回就绪
                unsafe { &mut *(w as *mut Task) }.wakeup();
            }
            // 必做：软中断唤醒内核调度器
            Porting::irq();
        }
    }

    /// 生产一个字节（仅 ISR 侧调用）：满返回 false 并丢弃
    #[inline]
    fn try_enqueue(&self, b: u8) -> bool {
        let head = self.head.load(Ordering::SeqCst);
        let tail = self.tail.load(Ordering::SeqCst);
        if head.wrapping_sub(tail) >= BUF_LEN {
            return false;
        }
        // SAFETY: SPSC——head 槽只有唯一生产者（ISR）写，且槽位空闲
        unsafe {
            *self.buf[head & (BUF_LEN - 1)].get() = b;
        }
        self.head.store(head.wrapping_add(1), Ordering::SeqCst);
        true
    }

    /// 消费一个字节（仅读任务侧调用）：空返回 None
    #[inline]
    fn try_pop(&self) -> Option<u8> {
        let head = self.head.load(Ordering::SeqCst);
        let tail = self.tail.load(Ordering::SeqCst);
        if head == tail {
            return None;
        }
        // SAFETY: SPSC——tail 槽只有唯一消费者（读任务）读
        let b = unsafe { *self.buf[tail & (BUF_LEN - 1)].get() };
        self.tail.store(tail.wrapping_add(1), Ordering::SeqCst);
        Some(b)
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

impl UartDevice for Uart0 {
    /// 阻塞写出全部字节。发送是"等 TXE"的微秒级操作，不值得进任务状态机，
    /// 用轮询等待（不放弃 CPU）。每字节一段临界区：RefCell 借用不能跨越
    /// 任务抢占（借用不关中断，跨切换会让被抢占任务 double-borrow panic）；
    /// 57600 波特率下每字节最长约 174 µs，远短于一个调度节拍，不会饿死节拍中断。
    fn write_all(&self, buf: &[u8]) {
        for &b in buf {
            sync::free(|_| {
                let mut tx = self.tx.borrow_mut();
                // Write 的 Error 是 Infallible，实际只会在空转等 TXE
                let _ = nb::block!(tx.write(b));
            });
        }
    }

    /// 缓冲中待读字节数
    fn rx_len(&self) -> usize {
        self.head
            .load(Ordering::SeqCst)
            .wrapping_sub(self.tail.load(Ordering::SeqCst))
    }

    /// 阻塞读一个字节（**单读者契约**：同一时刻只允许一个任务读）。
    /// 缓冲为空时挂起自身（Blocked），数据到达由 ISR 唤醒——"等待即 Blocked"，
    /// 不空转轮询、不烧 CPU。
    fn read_byte(&self) -> u8 {
        loop {
            if let Some(b) = self.try_pop() {
                return b;
            }
            // 检查与登记在同一临界区：ISR 不可能插在两者之间，
            // 先入队的字节必然看到 waiter 已登记——丢失唤醒从根上消除
            let got = sync::free(|_| {
                if let Some(b) = self.try_pop() {
                    return Some(b);
                }
                let t = xworker.current();
                let addr = (t as *mut Task).addr();
                self.waiter.store(addr, Ordering::SeqCst);
                t.block();
                None
            });
            if let Some(b) = got {
                return b;
            }
            // 让出 CPU：ISR 唤醒后调度器把自己排回来
            yield_now();
        }
    }
}
