//! smoltcp 设备适配:字节流(SLIP) → IP 帧。
//!
//! 分层哲学与 ch21 的 `FatAdapter` 同款——smoltcp 的 `phy::Device` 是"设备是字节的
//! 搬运工,协议栈是字节的法官"这第一句话的落点:我们只写**借用与翻译**,
//! 位级编码/解码全部在 [`super::slip`] 里、已被宿主回归钉死。
//!
//! 设计要点(书稿代码精读 3):
//! - **非阻塞排空纪律**:`receive` 里只允许经 `PhyIo::rx_len` 确认有数据才
//!   `read_byte`——SLIP 占用的是 UART,阻塞读会让整条协议链睡死在空缓冲上
//!   (踩坑 a;`PhyIo` 就是"非阻塞读"这个契约的名字);
//! - **字段级拆借**:`receive` 同时返回两枚 token,各借 `SlipDevice` 的**不同字段**
//!   (rx 帧缓冲 / tx 编码缓冲)——都想借 `&mut self` 是 borrowck 必拒的错误
//!   (踩坑 b),纯 safe 写法,无裸指针;
//! - **共享引用 + 内部可变 + `Sync` 上界**:与 `BdDevice` 同款论证——physical 层
//!   经 `&` 被协议栈与任务共享,`&dyn PhyIo` 要 Send 必须先 `PhyIo: Sync`
//!   (auto-trait 链,见 ch21 踩坑 2)。

use crate::drv::UartDevice;
use crate::net::slip::{SlipDecoder, SlipEncoder, SLIP_MAX_FRAME, SLIP_MTU};
use smoltcp::phy::{Device, DeviceCapabilities, Medium, RxToken, TxToken};
use smoltcp::Result;
use smoltcp::time::Instant;

/// 物理字节流小接口——"非阻塞读 + 阻塞写"的契约。
///
/// 契约:
/// - `read_byte` **仅在 `rx_len() > 0` 时调用**(调用方保证,本 trait 不检查);
/// - `write_all` 在任务上下文调用,允许轮询阻塞(57600 下 ~174µs/字节);
/// - `Sync` 上界:方法与 `UartDevice` 一致全 `&self`,内部自持可变状态,
///   经 `&dyn PhyIo` 共享(见模块文档的 auto-trait 论证)。
pub trait PhyIo: Sync {
    /// 接收缓冲中待读字节数(非阻塞查询)
    fn rx_len(&self) -> usize;
    /// 读一个字节。**仅当 rx_len() > 0**,等价于"非阻塞读"
    fn read_byte(&self) -> u8;
    /// 写一块字节(可阻塞轮询)
    fn write_all(&self, buf: &[u8]);
}

/// 一切 `UartDevice` 都是 `PhyIo`(方法直接转发——UART 的 SPSC 环形缓冲
/// + waiter 槽已是现成的字节流;trait 名不同只为了把"非阻塞读"契约钉进类型)。
impl<T: UartDevice + ?Sized + Sync> PhyIo for T {
    #[inline]
    fn rx_len(&self) -> usize {
        UartDevice::rx_len(self)
    }
    #[inline]
    fn read_byte(&self) -> u8 {
        UartDevice::read_byte(self)
    }
    #[inline]
    fn write_all(&self, buf: &[u8]) {
        UartDevice::write_all(self, buf)
    }
}

/// SLIP 设备:把 UART 字节流翻译成 smoltcp 的 IP 帧。
///
/// 字段布局直接决定两枚 token 的借用(书稿教学点):
/// `rx_buf` 归 RxToken、`tx_enc` 归 TxToken——两个独立字段,
/// `receive` 才能一次给出两枚互不冲突的 `&mut`。
pub struct SlipDevice<'a> {
    phy: &'a dyn PhyIo,
    decoder: SlipDecoder,
    /// 帧装配 + 交付共用缓冲(decoder 产出即 smoltcp 要消费的帧)
    rx_buf: [u8; SLIP_MTU],
    /// 编码输出缓冲(完整 SLIP 帧:END + 转义后数据 + END)
    tx_enc: [u8; SLIP_MAX_FRAME],
    /// 当前可交付帧长;0 = 无完整帧
    rx_len: usize,
}

/// RxToken:交付的是已解好的 IP 帧(帧缓冲就躺在设备字段里,token 只是一个切片借用)。
/// 关联类型必须 pub(出现在 `Device` 的公开接口里),外部只经 `consume` 使用。
pub struct SlipRxToken<'a> {
    buf: &'a mut [u8],
}

/// TxToken:持有物理层(共享引用)与编码缓冲(独立字段借用)。
pub struct SlipTxToken<'a> {
    phy: &'a dyn PhyIo,
    enc: &'a mut [u8],
}

impl<'a> SlipDevice<'a> {
    pub const fn new(phy: &'a dyn PhyIo) -> Self {
        SlipDevice {
            phy,
            decoder: SlipDecoder::new(),
            rx_buf: [0; SLIP_MTU],
            tx_enc: [0; SLIP_MAX_FRAME],
            rx_len: 0,
        }
    }

    /// 非阻塞排空:把 UART 里现有的字节全部喂给解码器。
    /// 出现完整帧即返回 `true`(此后**必须停止喂入**——decoder 下一帧会从头
    /// 写 `rx_buf`,把还没交付的帧踩掉)。
    pub fn filter(&mut self) -> bool {
        while self.phy.rx_len() > 0 {
            let b = self.phy.read_byte();
            if let Some(n) = self.decoder.feed(b, &mut self.rx_buf) {
                self.rx_len = n;
                return true;
            }
        }
        false
    }
}

impl<'d, 'a> Device<'d> for SlipDevice<'a> {
    type RxToken = SlipRxToken<'d>;
    type TxToken = SlipTxToken<'d>;

    fn receive(&'d mut self) -> Option<(Self::RxToken, Self::TxToken)> {
        // 帧还在等待交付时,绝不能继续喂(见 filter 的契约)
        if self.rx_len == 0 {
            self.filter();
        }
        if self.rx_len == 0 {
            return None;
        }
        // 字段级拆借:标量与共享引用先拷走,再把两个不同字段各自借出——
        // 两枚 token 的 &mut 互不重叠,borrowck 直接放行。
        //
        // 关键:这里**立即**把 rx_len 清零——帧内容已交给 token(rx_buf 的借用
        // 由 token 持有,下一次 receive 的 filter 要等 token 被 consume/丢弃后才
        // 会运行,不会踩掉未消费的帧)。若不清零,同一帧会被无限次重投,
        // smoltcp 的 poll 内循环永远有"帧"可处理 → 卡死(宿主 e2e 抓到的真 bug)。
        let len = self.rx_len;
        self.rx_len = 0;
        let phy = self.phy;
        let rx = &mut self.rx_buf[..len];
        let enc = &mut self.tx_enc[..];
        Some((SlipRxToken { buf: rx }, SlipTxToken { phy, enc }))
    }

    fn transmit(&'d mut self) -> Option<Self::TxToken> {
        let phy = self.phy;
        Some(SlipTxToken { phy, enc: &mut self.tx_enc[..] })
    }

    fn capabilities(&self) -> DeviceCapabilities {
        // non_exhaustive 结构体在外部 crate 不能用结构体字面量(含 .. 语法也不行),
        // 用 Default + 公开字段赋值
        let mut c = DeviceCapabilities::default();
        c.medium = Medium::Ip;
        c.max_transmission_unit = SLIP_MTU;
        c.max_burst_size = None;
        // checksum 保持默认:收发双向都算校验和(Checksum::Both)——SLIP 无校验和
        // 卸载,协议栈全权计算;宿主 e2e 全链路真实计算,钉死默认值
        c
    }
}

impl<'a> RxToken for SlipRxToken<'a> {
    fn consume<R, F>(self, _timestamp: Instant, f: F) -> Result<R>
    where
        F: FnOnce(&mut [u8]) -> Result<R>,
    {
        f(self.buf)
    }
}

impl<'a> TxToken for SlipTxToken<'a> {
    fn consume<R, F>(self, _timestamp: Instant, len: usize, f: F) -> Result<R>
    where
        F: FnOnce(&mut [u8]) -> Result<R>,
    {
        // 原始 IP 帧暂存走栈(consume 在泵任务上下文,296B 栈上有余量)
        let mut raw = [0u8; SLIP_MTU];
        let r = f(&mut raw[..len])?;
        let n = SlipEncoder::new().encode(&raw[..len], self.enc);
        self.phy.write_all(&self.enc[..n]);
        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::net::slip::{SLIP_END, SLIP_ESC, SLIP_ESC_END, SLIP_ESC_ESC};
    use std::cell::RefCell;
    use std::collections::VecDeque;

    /// 测试用字节流:tx 按"帧"整块捕获,rx 是待读字节队列。
    /// `unsafe impl Sync`:持有 RefCell 非 Sync,但测试单线程且访问严格串行——
    /// 与 drv.rs 的测试 mock 同款论证。
    struct MockLink {
        rx: RefCell<VecDeque<u8>>,
        tx: RefCell<Vec<Vec<u8>>>,
    }
    // SAFETY: 仅测试用;单线程测试内借用严格串行,不存在并发访问
    unsafe impl Sync for MockLink {}

    impl MockLink {
        fn new() -> Self {
            MockLink { rx: RefCell::new(VecDeque::new()), tx: RefCell::new(Vec::new()) }
        }
        /// "电线"传输:把捕获到的 SLIP 编码帧**原样**压入对端 rx——
        /// 线上跑的就是编码后的字节流(含 END/转义),对端设备的 decoder
        /// 自己再解一次。若这里先解码,对端就永远等不到帧尾 END(初版踩坑)。
        fn wire_to(&self, peer: &MockLink) {
            let frames: Vec<Vec<u8>> = self.tx.borrow_mut().drain(..).collect();
            for f in frames {
                peer.rx.borrow_mut().extend(f);
            }
        }
    }

    impl UartDevice for MockLink {
        fn write_all(&self, buf: &[u8]) {
            self.tx.borrow_mut().push(buf.to_vec());
        }
        fn rx_len(&self) -> usize {
            self.rx.borrow().len()
        }
        fn read_byte(&self) -> u8 {
            self.rx.borrow_mut().pop_front().unwrap()
        }
    }

    // ---- 设备级 ----

    #[test]
    fn tx_encodes_to_wire() {
        let link = MockLink::new();
        let mut dev = SlipDevice::new(&link);
        let tk = dev.transmit().unwrap();
        tk.consume(Instant::from_millis(0), 3, |buf| {
            buf[..3].copy_from_slice(b"abc");
            Ok(())
        })
        .unwrap();
        let frames = link.tx.borrow();
        assert_eq!(
            frames[0],
            vec![SLIP_END, b'a', b'b', b'c', SLIP_END],
            "wire 上应是 END+payload+END 成品帧"
        );
    }

    #[test]
    fn tx_escapes_control_bytes() {
        let link = MockLink::new();
        let mut dev = SlipDevice::new(&link);
        dev.transmit().unwrap().consume(Instant::from_millis(0), 2, |buf| {
            buf[..2].copy_from_slice(&[SLIP_END, SLIP_ESC]);
            Ok(())
        }).unwrap();
        let frames = link.tx.borrow();
        assert_eq!(frames[0], vec![SLIP_END, SLIP_ESC, SLIP_ESC_END, SLIP_ESC, SLIP_ESC_ESC, SLIP_END]);
    }

    #[test]
    fn rx_decodes_from_wire() {
        // 预置一帧编码字节流 → receive 应产出原始帧
        let link = MockLink::new();
        link.rx.borrow_mut().extend(vec![SLIP_END, b'h', b'i', SLIP_END]);
        let mut dev = SlipDevice::new(&link);
        let (rx, _tx) = dev.receive().expect("应有帧");
        let got = rx.consume(Instant::from_millis(0), |buf| {
            let v = buf.to_vec();
            Ok(v)
        });
        assert_eq!(got.unwrap(), b"hi".to_vec());
    }

    #[test]
    fn receive_none_when_no_frame() {
        let link = MockLink::new();
        let mut dev = SlipDevice::new(&link);
        assert!(dev.receive().is_none(), "无字节时不应产出帧");
        // 半帧(缺尾部 END)也不算
        link.rx.borrow_mut().extend(vec![SLIP_END, b'h']);
        let mut dev2 = SlipDevice::new(&link);
        assert!(dev2.receive().is_none());
    }

    #[test]
    fn capabilities_are_ip_medium() {
        let link = MockLink::new();
        let dev = SlipDevice::new(&link);
        let c = dev.capabilities();
        assert!(matches!(c.medium, Medium::Ip));
        assert_eq!(c.max_transmission_unit, SLIP_MTU);
    }

    #[test]
    fn rx_overflow_no_panic() {
        // 超长字节流(超过 MTU)不应 panic:decoder 丢弃整帧至下一 END
        let link = MockLink::new();
        let mut rx = Vec::new();
        rx.push(SLIP_END);
        // 避开 0xC0/0xDB 的普通字节序列(10..=97)
        for i in 0..(SLIP_MTU + 16) {
            rx.push((i % 88) as u8 + 10);
        }
        rx.push(SLIP_END);
        link.rx.borrow_mut().extend(rx);
        let mut dev = SlipDevice::new(&link);
        // 第一帧超长被丢;随后正常帧仍可解
        assert!(dev.receive().is_none());
    }

    // ---- 全栈 e2e:两个 smoltcp 栈穿过自家 codec 握手 TCP + 互 ping ----

    use crate::net::stack::SlipStack;
    use crate::net::slip::SlipDecoder;
    use smoltcp::socket::{TcpSocket, TcpState};
    use smoltcp::wire::IpEndpoint;

    fn ep(ip: [u8; 4], port: u16) -> IpEndpoint {
        smoltcp::wire::IpEndpoint {
            addr: smoltcp::wire::IpAddress::v4(ip[0], ip[1], ip[2], ip[3]),
            port,
        }
    }

    /// 构造一枚合法 ICMP EchoRequest 报文(类型/校验和由 smoltcp wire 层算好)。
    /// `IcmpSocket::send_slice` 要求的是**完整 ICMP 报文**,裸 b"ping" 不是
    /// (dispatch 里 parse 直接判 Unrecognized——e2e 抓出的真坑)。
    fn icmp_echo_packet(ident: u16, seq_no: u16, data: &[u8]) -> Vec<u8> {
        use smoltcp::phy::ChecksumCapabilities;
        use smoltcp::wire::{Icmpv4Packet, Icmpv4Repr};
        let mut buf = vec![0u8; 4 + 4 + data.len()];
        let mut pkt = Icmpv4Packet::new_unchecked(&mut buf);
        Icmpv4Repr::EchoRequest { ident, seq_no, data }.emit(&mut pkt, &ChecksumCapabilities::default());
        buf
    }

    #[test]
    fn icmp_echo_over_cross_wired_slip() {
        // 最小链路实验:仅 ICMP——smoltcp 的 iface 层对 EchoRequest 自动应答,
        // 应用侧 IcmpSocket 只是"收发副本"(观察者/发起者)
        let a: &'static MockLink = Box::leak(Box::new(MockLink::new()));
        let b: &'static MockLink = Box::leak(Box::new(MockLink::new()));
        let mut sa = SlipStack::build(a, [10, 0, 0, 9], 24);
        let mut sb = SlipStack::build(b, [10, 0, 0, 8], 24);
        let _ha_i = sa.add_icmp(0x1234, 128, 2);
        let hb_i = sb.add_icmp(0x1234, 128, 2);

        let mut t: u64 = 0;
        let mut sent = false;
        let mut replied = false;
        let mut tmp_i = [0u8; 128];
        while t < 10_000 {
            t += 1;
            let ts = smoltcp::time::Instant::from_millis(t as i64);
            // B 发 ping(完整 ICMP 报文)
            {
                let icmp = sb.iface().get_socket::<smoltcp::socket::IcmpSocket>(hb_i);
                if !sent && icmp.can_send() {
                    sent = icmp
                        .send_slice(
                            &icmp_echo_packet(0x1234, 1, b"ping"),
                            smoltcp::wire::IpAddress::v4(10, 0, 0, 9),
                        )
                        .is_ok();
                }
                if let Ok((n, _)) = icmp.recv_slice(&mut tmp_i) {
                    use smoltcp::phy::ChecksumCapabilities;
                    use smoltcp::wire::{Icmpv4Packet, Icmpv4Repr};
                    if let Ok(repr) = Icmpv4Repr::parse(&Icmpv4Packet::new_checked(&tmp_i[..n]).unwrap(), &ChecksumCapabilities::default())
                    {
                        if let Icmpv4Repr::EchoReply { data, .. } = repr {
                            replied = data == b"ping";
                        }
                    }
                }
            }
            sb.poll_at(ts).expect("B poll");
            b.wire_to(a);
            sa.poll_at(ts).expect("A poll");
            a.wire_to(b);
            if replied {
                break;
            }
        }
        assert!(sent, "B 未能发出 ping");
        assert!(replied, "Ping 未收到应答");
    }

    #[test]
    fn tcp_echo_and_icmp_over_cross_wired_slip() {
        // A = 服务器 10.0.0.9/24:TCP 监听 1234(ICMP 由 iface 层自动应答,
        //     IcmpSocket 只作观察者,不加应用侧应答——0.8.2 的行为)
        // B = 客户端 10.0.0.8/24:TCP 连 A + ICMP ping
        let a: &'static MockLink = Box::leak(Box::new(MockLink::new()));
        let b: &'static MockLink = Box::leak(Box::new(MockLink::new()));
        let mut sa = SlipStack::build(a, [10, 0, 0, 9], 24);
        let mut sb = SlipStack::build(b, [10, 0, 0, 8], 24);
        let ha = sa.add_tcp(512, 512, Some(1234));
        let hb = sb.add_tcp(512, 512, None);
        let ha_i = sa.add_icmp(0x1234, 128, 2);
        let hb_i = sb.add_icmp(0x1234, 128, 2);
        // B 发起连接(connect 需要 Interface 的 Context)
        {
            let (tcp, cx) = sb.iface().get_socket_and_context::<TcpSocket>(hb);
            tcp.connect(cx, ep([10, 0, 0, 9], 1234), ep([10, 0, 0, 8], 40000))
                .expect("connect");
        }

        let mut t: u64 = 0;
        let mut b_sent = false;
        let mut a_echoed = false;
        let mut icmp_sent = false;
        let mut icmp_replied = false;
        let mut tmp_a = [0u8; 64];
        let mut tmp_i = [0u8; 128];

        while t < 100_000 {
            t += 1;
            let ts = smoltcp::time::Instant::from_millis(t as i64);
            // 1) 泵 A(服务器)
            sa.poll_at(ts).expect("A poll");
            // 2) A 应用层:TCP 收到即回(TCP echo 是应用职责;ICMP 应答是
            //    smoltcp iface 自动做的,不需应用代码)
            if let Ok(n) = sa.iface().get_socket::<TcpSocket>(ha).recv_slice(&mut tmp_a) {
                if n > 0 {
                    sa.iface().get_socket::<TcpSocket>(ha).send_slice(&tmp_a[..n]).unwrap();
                    if &tmp_a[..n] == b"ping" {
                        a_echoed = true;
                    }
                }
            }
            let _ = sa.iface().get_socket::<smoltcp::socket::IcmpSocket>(ha_i).recv_slice(&mut tmp_i);
            // 3) 接线 A→B
            a.wire_to(b);
            // 4) 泵 B(客户端)
            sb.poll_at(ts).expect("B poll");
            // 5) B 应用层:连接建立后发一次;ICMP 发一枚合法 EchoRequest
            {
                let tcp = sb.iface().get_socket::<TcpSocket>(hb);
                if !b_sent && tcp.state() == TcpState::Established {
                    b_sent = tcp.send_slice(b"ping").map(|n| n == 4).unwrap_or(false);
                }
                let _ = tcp.recv_slice(&mut tmp_a); // 回显(如有)由 A 侧判 a_echoed,这里收副本即可
            }
            {
                let icmp = sb.iface().get_socket::<smoltcp::socket::IcmpSocket>(hb_i);
                if !icmp_sent && icmp.can_send() {
                    icmp_sent = icmp
                        .send_slice(
                            &icmp_echo_packet(0x1234, 1, b"ping"),
                            smoltcp::wire::IpAddress::v4(10, 0, 0, 9),
                        )
                        .is_ok();
                }
                if let Ok((n, _)) = icmp.recv_slice(&mut tmp_i) {
                    use smoltcp::phy::ChecksumCapabilities;
        use smoltcp::wire::{Icmpv4Packet, Icmpv4Repr};
                    if let Ok(repr) = Icmpv4Repr::parse(
                        &Icmpv4Packet::new_checked(&tmp_i[..n]).unwrap(),
                        &ChecksumCapabilities::default(),
                    ) {
                        if let Icmpv4Repr::EchoReply { data, .. } = repr {
                            icmp_replied = data == b"ping";
                        }
                    }
                }
            }
            // 6) 接线 B→A
            b.wire_to(a);

            if b_sent && a_echoed && icmp_replied {
                break;
            }
        }

        assert!(b_sent, "B 未能在超时前连上并发数据");
        assert!(a_echoed, "A 未收到并回显数据");
        assert!(icmp_replied, "ICMP echo 未收到应答");
    }
}
