#![no_std]
#![no_main]

extern crate alloc;

use gd32vf103xx_hal as hal;
use hal::{gpio::GpioExt, pac, prelude::*, rcu::RcuExt};

use xtask::arch::riscv::rt;
use xtask::bsp::longan_nano::drv_uart::{uart0_isr, Uart0};
use xtask::drv::register_uart;
use xtask::net::iface::SocketHandle;
use xtask::net::socket::TcpSocket;
use xtask::net::stack::SlipStack;
use xtask::prelude::*;
use xtask::sync::notify::Notifier;

// 网络示例(第 22 章):把板子变成网络节点——物理层是 UART SLIP(RFC 1055)。
// 链路:Uart0(USART0 57600,ISR 环形缓冲,ch20)→ SlipDevice(非阻塞排空 +
// 编码写回,net::device)→ smoltcp Interface → socket。
// 任务:
// - net-pump(prio 2):协议栈唯一"时间"的持有者——轮询 poll + 分发
//   (TCP 可读 → Notifier 信号);**锁必须 drop 后再 sleep**(踩坑 c)。
// - net-echo(prio 3):Notifier::wait 挂起等数据,唤醒后 can_recv 复查 →
//   recv → send(socket 与任务阻塞对接的现场)。
// 网络:10.0.0.9/24;对端 PC 10.0.0.8/24(同一子网直连,无网关;pointopoint
// 是 PC 侧 ifconfig 的事)。日志走默认 RTT,不占串口。
// 真机实验(PC 侧):slattach -s 57600 -p slip /dev/ttyUSB0 →
// ifconfig sl0 10.0.0.8 pointopoint 10.0.0.9 → ping 10.0.0.9 →
// nc 10.0.0.9 1234 输入回显。

/// 协议栈是全任务共享的临界资源(Interface 的 socket 操作要 `&mut`),
/// `Mutex<Option<..>>` 串行——与 ch21 的 FS 锁同款
static STACK: Mutex<Option<SlipStack>> = Mutex::new(None);
/// TCP 监听 socket 句柄(init 时拿到,只读;SocketHandle 是 Copy 的 usize)
static TCP_H: Mutex<Option<SocketHandle>> = Mutex::new(None);

fn init() {
    extern "C" {
        /// 堆内存开始地址,在 riscv-rt link.x 文件里定义
        static _sheap: u8;
    }
    let start_addr = unsafe { &_sheap as *const u8 as usize };
    xtask::init_heap(start_addr, 64 * 1024);

    let dp = pac::Peripherals::take().unwrap();
    // 配置时钟
    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();

    let gpioa = dp.GPIOA.split(&mut rcu);

    let mut afio = dp.AFIO.constrain(&mut rcu);
    // USART0 给 SLIP 用;日志走默认 RTT,不需要 stdout 配置(driver 示例先例)
    let uart0 = Uart0::new(dp.USART0, gpioa.pa9, gpioa.pa10, 57600.bps(), &mut afio, &mut rcu);
    register_uart("uart0", uart0).expect("register uart0");

    // 协议栈 + socket 都在调度器启动前挂载(与 ch21 挂载同款纪律)
    // 具体的 Uart0(unsize 成 trait 对象)——UartDevice 的 blanket impl 使其也是 PhyIo
    let phy: &'static dyn xtask::net::device::PhyIo = uart0;
    let mut stack = SlipStack::build(phy, [10, 0, 0, 9], 24);
    let tcp_h = stack.add_tcp(2048, 2048, Some(1234)); // TCP 监听 1234
    *STACK.lock() = Some(stack);
    *TCP_H.lock() = Some(tcp_h);

    log::info!("net_echo: SLIP@57600 (USART0), iface 10.0.0.9/24, echo :1234");
}

/// USART0 中断向量:port.S 里的 `.weak USART0` 被这里的强定义绑定
/// (ch20 司机层弱符号机制;thin LTO 是它成立的前提)。
/// SLIP 的 RX 全靠它往环形缓冲里填字节。
#[no_mangle]
extern "C" fn USART0() {
    uart0_isr();
}

#[rt::entry]
fn main() -> ! {
    init();

    // 泵/echo 共享的"有数据了"信号(ch9 的 Notifier,clone 两边各一份)
    let ready = Notifier::new();
    let ready_pump = ready.clone();
    let ready_echo = ready.clone();

    // 协议栈泵
    TaskBuilder::new()
        .name("net-pump")
        .priority(2)
        .stack_size(1024)
        .spawn(move || {
            let tcp_h = TCP_H.lock().expect("tcp handle mounted");
            let mut tick = 0u64;
            loop {
                let poll_delay = {
                    let mut guard = STACK.lock();
                    let st = guard.as_mut().expect("stack mounted");
                    // 驱动一次协议栈(时间戳 = 内核 tick,毫秒)
                    let _ = st.poll();
                    // 分发:TCP 可读 → 喊 echo(notify_isr:信号已满则丢弃——
                    // 无害,echo 醒来还会 can_recv 复查;若用 notify() 满信号
                    // 会挂起泵自身,数据不进不出)
                    let tcp = st.iface().get_socket::<TcpSocket>(tcp_h);
                    if tcp.can_recv() {
                        let _ = ready_pump.notify_isr();
                    }
                    // ICMP echo 由 smoltcp iface 自动应答,无需应用侧处理
                    let d = st.poll_delay_ms();
                    drop(guard); // 锁必须释放后再睡(踩坑 c)
                    d
                };
                xtask::sleep_ms(poll_delay.max(10) as usize);
                tick += 1;
                if tick % 2000 == 0 {
                    log::info!("net-pump: heartbeat, {tick} pumps");
                }
            }
        });

    // TCP echo 任务:数据到达前挂在 Notifier 上(等待即 Blocked)
    TaskBuilder::new()
        .name("net-echo")
        .priority(3)
        .stack_size(1024)
        .spawn(move || {
            let tcp_h = TCP_H.lock().expect("tcp handle mounted");
            let mut buf = [0u8; 512];
            loop {
                ready_echo.wait();
                let mut guard = STACK.lock();
                let st = guard.as_mut().expect("stack mounted");
                let tcp = st.iface().get_socket::<TcpSocket>(tcp_h);
                if tcp.can_recv() {
                    match tcp.recv_slice(&mut buf) {
                        Ok(0) => {}
                        Ok(n) => {
                            log::info!("echo: {:?} ({n}B)", &buf[..n]);
                            let _ = tcp.send_slice(&buf[..n]);
                        }
                        Err(_) => {}
                    }
                }
                drop(guard);
            }
        });

    xtask::start()
}
