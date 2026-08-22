//! SLIP 协议栈组装 helper:Interface + 时间戳 + 常用 socket,示例与宿主 e2e 共用。
//!
//! 这一层把三件事钉在一处(书稿代码精读 4 的入口):
//! - **时间戳 = 内核 tick**:`now_instant()` 用 `xtask::time::tick_ms()`(TICK_CLOCK_HZ
//!   = 1000,即毫秒)——不碰 `Porting::systick()`(gd32vf103 下是 27MHz 的 MTIME 计数,
//!   单位陷阱会咬人);
//! - **接口 IP**:`Medium::Ip`(裸 IP 媒体)无 ARP、无邻居表,`InterfaceBuilder` 的
//!   `finalize` 断言硬件地址未设置——所以这里**没有** `hardware_addr`/`neighbor_cache`
//!   (对 Ip 媒体调用会 panic,书稿踩坑 d);
//! - **socket 存储**:0.8 的 socket 内置在 Interface 里(`InterfaceBuilder::new(device,
//!   sockets)`,sockets 传 `Vec::new()` 即按需分配),无需外部 SocketSet。

use crate::net::device::{PhyIo, SlipDevice};
use crate::time;
use alloc::vec;
use alloc::vec::Vec;
use smoltcp::iface::{Interface, InterfaceBuilder, SocketHandle};
use smoltcp::socket::{IcmpEndpoint, IcmpSocket, TcpSocket};
use smoltcp::storage::{PacketBuffer, PacketMetadata};
use smoltcp::time::Instant;
use smoltcp::wire::{IpAddress, IpCidr};

/// 一条 SLIP 链路 + 一台 smoltcp 协议栈。
pub struct SlipStack {
    iface: Interface<'static, SlipDevice<'static>>,
}

impl SlipStack {
    /// 组装协议栈:设备按值进 Interface,phy 必须 `'static`
    /// (示例里是 `find_uart("uart0")` 返回的 `&'static dyn UartDevice`)。
    pub fn build(phy: &'static dyn PhyIo, ip: [u8; 4], prefix: u8) -> SlipStack {
        let iface = InterfaceBuilder::new(SlipDevice::new(phy), Vec::new())
            .ip_addrs([IpCidr::new(IpAddress::v4(ip[0], ip[1], ip[2], ip[3]), prefix)])
            .finalize();
        SlipStack { iface }
    }

    /// 驱动一次协议栈轮询(时间戳取当前内核 tick)
    pub fn poll(&mut self) -> smoltcp::Result<bool> {
        self.poll_at(now_instant())
    }

    /// 指定时刻驱动一次轮询——宿主 e2e 用虚拟时钟驱动,真机走 `poll()`
    pub fn poll_at(&mut self, ts: Instant) -> smoltcp::Result<bool> {
        self.iface.poll(ts)
    }

    /// 距下一次必须轮询的毫秒数;无待办事件回退到 50ms(心跳节奏)
    pub fn poll_delay_ms(&mut self) -> u64 {
        match self.iface.poll_delay(now_instant()) {
            Some(d) => d.millis(),
            None => 50,
        }
    }

    /// 加一个 TCP socket。`listen_port = Some(p)` 则为监听(服务端),`None` 为
    /// 连接方(客户端,连接前经 `iface().get_socket_and_context` 调 `connect`)。
    pub fn add_tcp(&mut self, rx_cap: usize, tx_cap: usize, listen_port: Option<u16>) -> SocketHandle {
        // RingBuffer 只接受 ManagedSlice(从 Vec 转),不接受裸 Vec——经其构造
        let mk = |n: usize| smoltcp::storage::RingBuffer::new(vec![0u8; n]);
        let mut sock = TcpSocket::new(mk(rx_cap), mk(tx_cap));
        if let Some(p) = listen_port {
            sock.listen(p).expect("TCP 监听端口");
        }
        self.iface.add_socket(sock)
    }

    /// 加一个 ICMP socket(绑定 `Ident`)。ping 应答**不**由 smoltcp 自动回,
    /// 应用层 `recv_slice` 后 `send_slice` 回源(书稿代码精读 4 末尾)。
    pub fn add_icmp(&mut self, ident: u16, cap: usize, max_packets: usize) -> SocketHandle {
        let mk = || {
            PacketBuffer::new(vec![PacketMetadata::EMPTY; max_packets], vec![0; cap])
        };
        let mut sock = IcmpSocket::new(mk(), mk());
        sock.bind(IcmpEndpoint::Ident(ident)).expect("ICMP ident");
        self.iface.add_socket(sock)
    }

    /// 直接访问 Interface(应用做 socket 操作走这里;注意返回 `&mut`,
    /// 协议栈是任务间的临界资源,示例用 `Mutex<Option<SlipStack>>` 串行)
    pub fn iface(&mut self) -> &mut Interface<'static, SlipDevice<'static>> {
        &mut self.iface
    }
}

/// 内核 tick → smoltcp 虚拟时间。tick 周期=1ms(TICK_CLOCK_HZ=1000),
/// 与 smoltcp `Instant` 的毫秒计数器直接对齐。
pub fn now_instant() -> Instant {
    Instant::from_millis(time::tick_ms() as i64)
}
