//! 宿主 e2e:MockUsbBus + 脚本化主机模型。
//!
//! 「协议(枚举/描述符/请求)全在 usb-device 内部」意味着:我们的
//! UsbBus 实现遵守契约没有?——把真实 crate 状态机在宿主上跑起来,
//! 给 mock 总线喂"主机侧"的脚本,断言设备侧吐出的每一包,就是裁判。
//! 与 ch22 的 cross-wire e2e 同哲学,但形态不同(书稿"验证"节对照表):
//!
//! - net:两个**自家栈**对等耦合(协议正确性在 smoltcp,我们测接口);
//! - usb:设备侧是 usb-device 标准状态机,对端是**脚本化主机模型**
//!   (协议正确性在 crate 内部,我们测**契约遵守**——描述符字节才是
//!   规范一致性的最终落点);
//! - 线模型:net 是重编码字节流;usb 的"线"是**包级容器**
//!   (带 EP/方向/SETUP 标记),因为 UsbBus 契约消费的就是包。
//!
//! 本模块只在 `#[cfg(test)]` 编译;mock 语义刻意**模拟寄存器行为**
//! (latch 保持/单发),不模拟 OTG 内部时序——那是真机的活。

#![cfg(test)]

use std::collections::VecDeque;
use std::sync::Mutex;

use crate::usb::otg::{allocate_slot, EpRequest, EpSlotState};
use usb_device::bus::{PollResult, UsbBus};
use usb_device::endpoint::EndpointAddress;
use usb_device::{UsbDirection, UsbError};

/// 故障注入(阳性对照):验证测试套件**真能**抓住这类总线违约
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Faults {
    /// poll 后不清 in_complete_latch(违反"每完成只报一次")
    pub forget_in_complete: bool,
    /// ep_setup 只报一次即视为已读(违反"持续上报直到 read")
    pub setup_once_only: bool,
}

/// 主机侧摆入的包(带目标 EP 号)
enum HostPacket {
    Setup { bytes: [u8; 8] },
    Out { ep: u8, data: Vec<u8> },
}

struct MockState {
    slots: EpSlotState,
    /// alloc_ep 结果表(调用记录)
    caps: Vec<(EndpointAddress, u8, u16)>,
    addr: u8,
    enabled: bool,
    suspended: bool,
    stall: [bool; 16],
    /// 模拟 DIEPnINTF.TF:write() 置位,poll() 读清(W1C 单发)
    in_complete_latch: u16,
    /// 未读的 SETUP/OUT(保持语义:读到才清)
    ep_setup_latch: u16,
    ep_out_latch: u16,
    pending_in_reset: bool,
    call_log: Vec<String>,
    /// 设备经 write() 写出的 IN 包(等主机断言)
    host_rx: Vec<(EndpointAddress, Vec<u8>)>,
    /// 主机摆出的 SETUP/OUT(等设备 read 取走)
    host_tx: VecDeque<HostPacket>,
}

impl MockState {
    fn new() -> Self {
        MockState {
            slots: EpSlotState::default(),
            caps: Vec::new(),
            addr: 0,
            enabled: false,
            suspended: false,
            stall: [false; 16],
            in_complete_latch: 0,
            ep_setup_latch: 0,
            ep_out_latch: 0,
            pending_in_reset: false,
            call_log: Vec::new(),
            host_rx: Vec::new(),
            host_tx: VecDeque::new(),
        }
    }

    fn log(&mut self, line: impl Into<String>) {
        self.call_log.push(line.into());
    }
}

/// 模拟总线(实现 UsbBus,语义对应真实寄存器行为)
pub struct MockUsbBus {
    inner: Mutex<MockState>,
    faults: Faults,
}

impl MockUsbBus {
    pub fn new(faults: Faults) -> Self {
        MockUsbBus {
            inner: Mutex::new(MockState::new()),
            faults,
        }
    }

    fn st<F: FnOnce(&mut MockState) -> R, R>(&self, f: F) -> R {
        f(&mut self.inner.lock().unwrap())
    }

    // ---- 主机侧操作(测试脚本用;经 `dev.bus()` 取 &MockUsbBus) ----

    /// 主机置入一枚 SETUP 包(目标 EP0)
    pub fn host_setup(&self, bytes: [u8; 8]) {
        self.st(|s| {
            s.host_tx.push_back(HostPacket::Setup { bytes });
            s.ep_setup_latch |= 1; // "持续上报"语义:读到才清
            s.log(format!("host: SETUP {bytes:?}"));
        });
    }

    /// 主机置入 OUT 数据(模拟主线:按该端点的 MPS 切成多个包,
    /// 最后一个短包;满 MPS 结尾不补 ZLP——那是设备 IN 侧的纪律)
    pub fn host_out(&self, ep: u8, data: &[u8]) {
        let mps = self.st(|s| {
            s.caps
                .iter()
                .find(|(a, _, _)| a.index() == ep as usize && !a.is_in())
                .map(|(_, _, m)| *m as usize)
                .unwrap_or(64)
        });
        let mut rest = data;
        loop {
            if rest.is_empty() {
                break;
            }
            let n = rest.len().min(mps);
            let chunk = rest[..n].to_vec();
            self.st(|s| {
                s.host_tx.push_back(HostPacket::Out { ep, data: chunk.clone() });
                s.ep_out_latch |= 1 << ep;
                s.log(format!("host: OUT ep{ep} {}B", chunk.len()));
            });
            rest = &rest[n..];
        }
    }

    /// 主机执行总线复位
    pub fn host_reset(&self) {
        self.st(|s| {
            s.pending_in_reset = true;
            s.log("host: RESET");
        });
    }

    // ---- 断言助手 ----

    /// 当前总线地址(模拟 DCFG.DAR)
    pub fn addr(&self) -> u8 {
        self.st(|s| s.addr)
    }

    /// 取走该 EP 途中所有 IN 包并拼接(断言用;跨包自动拼接)
    pub fn drain_in(&self, ep: EndpointAddress) -> Vec<u8> {
        self.st(|s| {
            let mut out = Vec::new();
            let mut keep = Vec::new();
            for (a, d) in s.host_rx.drain(..) {
                if a == ep {
                    out.extend(d);
                } else {
                    keep.push((a, d));
                }
            }
            s.host_rx = keep;
            out
        })
    }

    /// 全部 alloc 调用记录[(地址, 类型, mps)]
    pub fn caps(&self) -> Vec<(EndpointAddress, u8, u16)> {
        self.st(|s| s.caps.clone())
    }
}

impl UsbBus for MockUsbBus {
    fn alloc_ep(
        &mut self,
        ep_dir: UsbDirection,
        ep_addr: Option<EndpointAddress>,
        ep_type: usb_device::endpoint::EndpointType,
        max_packet_size: u16,
        _interval: u8,
    ) -> Result<EndpointAddress, UsbError> {
        let req = EpRequest::from_bus(ep_dir, ep_addr, ep_type, max_packet_size);
        let num = self.st(|s| {
            let n = allocate_slot(&mut s.slots, &req)?; // EpAllocError -> UsbError 经 From
            s.caps.push((EndpointAddress::from_parts(n as usize, ep_dir), req.ep_type, max_packet_size));
            s.log(format!(
                "alloc_ep {:?} type={} mps={}",
                EndpointAddress::from_parts(n as usize, ep_dir),
                req.ep_type,
                max_packet_size
            ));
            Ok::<_, UsbError>(n)
        })?;
        Ok(EndpointAddress::from_parts(num as usize, ep_dir))
    }

    fn enable(&mut self) {
        self.st(|s| {
            s.enabled = true;
            s.log("enable");
        });
    }

    fn reset(&self) {
        self.st(|s| {
            s.addr = 0;
            s.log("bus reset");
        });
    }

    fn set_device_address(&self, addr: u8) {
        self.st(|s| {
            s.addr = addr;
            s.log(format!("set_device_address {addr}"));
        });
    }

    fn write(&self, ep_addr: EndpointAddress, buf: &[u8]) -> Result<usize, UsbError> {
        self.st(|s| {
            s.host_rx.push((ep_addr, buf.to_vec()));
            // 模拟"发送完成":TF 位置位(单发由 poll 清)
            s.in_complete_latch |= 1 << ep_addr.index();
            s.log(format!("write {:?} {}B", ep_addr, buf.len()));
        });
        Ok(buf.len())
    }

    fn read(&self, ep_addr: EndpointAddress, buf: &mut [u8]) -> Result<usize, UsbError> {
        // 取走**第一个**匹配该 EP 的包;找不到 → WouldBlock。
        // 保持语义:ep_setup/ep_out 的 latch 只在"读到"时清。
        let mut n_read = 0usize;
        self.st(|s| {
            let mut rest = VecDeque::new();
            while let Some(p) = s.host_tx.pop_front() {
                let is_match = match &p {
                    HostPacket::Setup { .. } => ep_addr.index() == 0,
                    HostPacket::Out { ep, .. } => *ep == (ep_addr.index() as u8),
                };
                if is_match && n_read == 0 {
                    match p {
                        HostPacket::Setup { bytes } => {
                            s.ep_setup_latch &= !1u16; // 读到才清(保持语义)
                            buf[..8].copy_from_slice(&bytes);
                            n_read = 8;
                        }
                        HostPacket::Out { data, .. } => {
                            let n = data.len().min(buf.len());
                            buf[..n].copy_from_slice(&data[..n]);
                            // 多余字节丢弃(与真机"弹完一个包"语义一致,防失步)
                            s.ep_out_latch &= !(1u16 << ep_addr.index());
                            n_read = n;
                        }
                    }
                } else {
                    rest.push_back(p);
                }
            }
            s.host_tx = rest;
        });
        if n_read == 0 {
            Err(UsbError::WouldBlock)
        } else {
            Ok(n_read)
        }
    }

    fn set_stalled(&self, ep_addr: EndpointAddress, stalled: bool) {
        self.st(|s| {
            s.stall[ep_addr.index()] = stalled;
            s.log(format!("stall {:?} = {stalled}", ep_addr));
        });
    }

    fn is_stalled(&self, ep_addr: EndpointAddress) -> bool {
        self.st(|s| s.stall[ep_addr.index()])
    }

    fn suspend(&self) {
        self.st(|s| {
            s.suspended = true;
            s.log("suspend");
        });
    }

    fn resume(&self) {
        self.st(|s| {
            s.suspended = false;
            s.log("resume");
        });
    }

    fn poll(&self) -> PollResult {
        let mut r = PollResult::None;
        self.st(|s| {
            if s.pending_in_reset {
                s.pending_in_reset = false;
                r = PollResult::Reset;
                return;
            }
            if s.suspended {
                s.suspended = false; // 事件一次(挂起态本身由 device 记住)
                r = PollResult::Suspend;
                return;
            }
            let in_complete = s.in_complete_latch;
            if !self.faults.forget_in_complete {
                s.in_complete_latch = 0; // 单发:报一次即清(模拟 W1C)
            }
            let mut ep_setup = s.ep_setup_latch;
            if self.faults.setup_once_only && ep_setup != 0 {
                s.ep_setup_latch = 0; // 违约:报一次即"忘"——不等到 read
            }
            let ep_out = s.ep_out_latch;
            if ep_setup != 0 || ep_out != 0 || in_complete != 0 {
                r = PollResult::Data { ep_out, ep_in_complete: in_complete, ep_setup };
            }
        });
        r
    }
}

// ======================= e2e 脚本与断言 =======================

use usb_device::bus::UsbBusAllocator;
use usb_device::device::{UsbDevice, UsbDeviceBuilder, UsbDeviceState, UsbVidPid};
use usb_device::LangID;
use usbd_serial::SerialPort;

/// 主机脚本步骤
enum Step {
    /// 总线复位
    Reset,
    /// 置入一枚 SETUP 包(原始 8 字节,解析是 crate 的事)
    Setup([u8; 8]),
    /// 置入一枚 OUT 数据包(ep 为端点号,如 0x01;空数据 = ZLP)
    Out { ep: u8, data: Vec<u8> },
    /// 泵设备 n 次(poll + 应用级回显,与示例 usb-pump 同逻辑)
    Pump(u32),
    /// 断言总线地址 == n
    ExpectBusAddr(u8),
    /// 断言设备状态 == Configured
    ExpectConfigured,
    /// 断言该 EP 自上次以来全部 IN 包拼接 == bytes(跨包自动拼接)
    ExpectIn { ep: u8, bytes: &'static [u8] },
}

/// 拼一枚标准请求的原始 8 字节(字节序=小端)
fn setup(bmrt: u8, req: u8, val: u16, idx: u16, len: u16) -> [u8; 8] {
    let mut b = [0u8; 8];
    b[0] = bmrt;
    b[1] = req;
    b[2..4].copy_from_slice(&val.to_le_bytes());
    b[4..6].copy_from_slice(&idx.to_le_bytes());
    b[6..8].copy_from_slice(&len.to_le_bytes());
    b
}

const GET_DESCRIPTOR: u8 = 0x06;
const DEVICE: u16 = 0x0100;
const CONFIG: u16 = 0x0200;
const STRING: u16 = 0x0300;

/// 组装"设备侧"(真实 usb-device + usbd-serial)
fn build_device(
    faults: Faults,
) -> (
    &'static UsbBusAllocator<MockUsbBus>,
    UsbDevice<'static, MockUsbBus>,
    SerialPort<'static, MockUsbBus>,
) {
    // 测试专用:泄漏 allocator 换 'static 生命周期(等价"全局单例设备")
    let alloc: &'static UsbBusAllocator<MockUsbBus> =
        Box::leak(Box::new(UsbBusAllocator::new(MockUsbBus::new(faults))));
    let serial = SerialPort::new(alloc);
    let strings = [
        usb_device::device::StringDescriptors::new(LangID::EN_US)
            .manufacturer("gqf")
            .product("Xtask CDC"),
    ];
    let dev = UsbDeviceBuilder::new(alloc, UsbVidPid(0x0512, 0xCECE))
        .strings(&strings)
        .expect("strings")
        .device_class(usbd_serial::USB_CLASS_CDC)
        .build();
    (alloc, dev, serial)
}

fn pump(dev: &mut UsbDevice<MockUsbBus>, serial: &mut SerialPort<MockUsbBus>, n: u32) {
    let mut buf = [0u8; 64];
    for _ in 0..n {
        let _ = dev.poll(&mut [&mut *serial]);
        // 应用级回显:与 examples/usb_cdc.rs 的 usb-pump 同一段逻辑
        if let Ok(k) = serial.read(&mut buf) {
            if k > 0 {
                let _ = serial.write(&buf[..k]);
            }
        }
    }
}

/// 跑脚本;断言失败直接 panic(EndPointAddr 来自 usb_device::endpoint)
fn run_script(faults: Faults, steps: &[Step]) -> (Vec<String>, bool) {
    let (_alloc, mut dev, mut serial) = build_device(faults);
    for step in steps {
        match step {
            Step::Reset => dev.bus().host_reset(),
            Step::Setup(bytes) => dev.bus().host_setup(*bytes),
            Step::Out { ep, data } => dev.bus().host_out(*ep, data),
            Step::Pump(n) => pump(&mut dev, &mut serial, *n),
            Step::ExpectBusAddr(a) => {
                assert_eq!(dev.bus().addr(), *a, "总线地址不符");
            }
            Step::ExpectConfigured => {
                assert_eq!(dev.state(), UsbDeviceState::Configured, "未达 Configured");
            }
            Step::ExpectIn { ep, bytes } => {
                let got = dev.bus().drain_in(EndpointAddress::from(*ep));
                assert_eq!(&got, bytes, "EP {ep:#04x} 数据不符");
            }
        }
    }
    let log = dev.bus().caps().iter().map(|e| format!("{e:?}")).collect();
    (log, dev.state() == UsbDeviceState::Configured)
}

/// **主 e2e**:枚举七步 + CDC 初始化 + 单包数据往返
#[test]
fn enumeration_and_cdc_roundtrip() {
    let (log, configured) = run_script(
        Faults::default(),
        &[
            Step::Reset,
            // GET_DESCRIPTOR(Device, 18B)——设备描述符逐字节
            Step::Setup(setup(0x80, GET_DESCRIPTOR, DEVICE, 0x0000, 18)),
            Step::Pump(20),
            Step::ExpectIn {
                ep: 0x80,
                bytes: &[
                    0x12, 0x01, 0x10, 0x02, 0x02, 0x00, 0x00, 0x08, // bLength…bMaxPacketSize0
                    0x12, 0x05, 0xCE, 0xCE, 0x10, 0x00, 0x01, 0x02, // VID PID bcdDevice=0x0010
                    0x00, 0x01, // iSerial=0(未设), bNumConfigurations=1
                ],
            },
            // GET_DESCRIPTOR(Config)——断言 9 字节头
            Step::Setup(setup(0x80, GET_DESCRIPTOR, CONFIG, 0x0000, 255)),
            Step::Pump(20),
            Step::ExpectIn {
                ep: 0x80,
                // 完整配置描述符 67B(bTotalLength=0x43):接口 0 = CDC/ACM + 功能符
                // + 0x81 中断 IN;接口 1 = 数据类(0x0A)+ 0x82 批量 IN + 0x01 批量 OUT
                bytes: &[
                    0x09, 0x02, 0x43, 0x00, 0x02, 0x01, 0x00, 0x80, 0x32, // 配置头
                    0x09, 0x04, 0x00, 0x00, 0x01, 0x02, 0x02, 0x00, 0x00, // interface 0 (CDC/ACM)
                    0x05, 0x24, 0x00, 0x10, 0x01, // Header (bcdCDC 1.10)
                    0x04, 0x24, 0x02, 0x00, // ACM functional
                    0x05, 0x24, 0x06, 0x00, 0x01, // CallManagement
                    0x05, 0x24, 0x01, 0x00, 0x01, // Union
                    0x07, 0x05, 0x81, 0x03, 0x08, 0x00, 0xFF, // 0x81 INTR IN 8B
                    0x09, 0x04, 0x01, 0x00, 0x02, 0x0A, 0x00, 0x00, 0x00, // interface 1 (数据类)
                    0x07, 0x05, 0x82, 0x02, 0x40, 0x00, 0x00, // 0x82 BULK IN 64
                    0x07, 0x05, 0x01, 0x02, 0x40, 0x00, 0x00, // 0x01 BULK OUT 64
                ],
            },
            // SET_ADDRESS(1):完成 status 后才写总线地址
            Step::Setup(setup(0x00, 0x05, 0x0001, 0x0000, 0)),
            Step::Pump(20),
            Step::ExpectBusAddr(1),
            // 地址 1 下再取一次 config(DCFG.DAR 保持不回 0)
            Step::Setup(setup(0x80, GET_DESCRIPTOR, CONFIG, 0x0000, 255)),
            Step::Pump(20),
            // SET_CONFIGURATION(1)
            Step::Setup(setup(0x00, 0x09, 0x0001, 0x0000, 0)),
            Step::Pump(20),
            Step::ExpectConfigured,
            // CDC:SET_LINE_CODING(9600 8N1)+ 7B 数据段
            Step::Setup(setup(0x21, 0x20, 0x0000, 0x0000, 7)),
            Step::Out { ep: 0x00, data: vec![0x80, 0x25, 0x00, 0x00, 0x00, 0x08, 0x00] },
            Step::Pump(20),
            // 数据面:单包往返(0x01 收 / 0x82 发)
            Step::Out { ep: 0x01, data: b"hello xtask".to_vec() },
            Step::Pump(50),
            Step::ExpectIn { ep: 0x82, bytes: b"hello xtask" },
        ],
    );
    assert!(configured, "枚举未达 Configured; alloc={log:?}");
}

/// 600B 跨包往返(独立脚本;含 10 满包强制短包纪律)
#[test]
fn bulk_600_bytes_roundtrip() {
    let (_alloc, mut dev, mut serial) = build_device(Faults::default());
    dev.bus().host_reset();
    pump(&mut dev, &mut serial, 2);
    let _ = dev.bus().drain_in(EndpointAddress::from(0x80u8)); // 清掉复位期杂讯
    dev.bus().host_out(0x01, &vec![0x5A; 600]);
    pump(&mut dev, &mut serial, 600);
    let got = dev.bus().drain_in(EndpointAddress::from(0x82u8));
    assert_eq!(got.len(), 600, "600B 应全量回显,实际 {}", got.len());
    assert!(got.iter().all(|&b| b == 0x5A));
}

/// PollResult 白盒契约(不经过 usb-device,直接驱动 mock 总线)
#[test]
fn poll_contract_whitebox() {
    let bus = MockUsbBus::new(Faults::default());
    // ① write → 一次 Data{in_complete} → 再 poll 恒 None(单发)
    bus.write(EndpointAddress::from(0x82u8), b"ping").unwrap();
    match bus.poll() {
        PollResult::Data { ep_in_complete, .. } => assert_eq!(ep_in_complete & (1 << 2), 1 << 2),
        _ => panic!("期望 Data(in_complete)"),
    }
    assert!(matches!(bus.poll(), PollResult::None));
    // ② SETUP 保持语义:未 read 前连续 poll 都报 ep_setup
    bus.host_setup([0x80, 0x06, 0x00, 0x01, 0x00, 0x00, 0x12, 0x00]);
    for _ in 0..3 {
        match bus.poll() {
            PollResult::Data { ep_setup, .. } => assert_eq!(ep_setup, 1),
            _ => panic!("SETUP 应持续上报"),
        }
    }
    // ③ read 后消失 + 空读 WouldBlock
    let mut buf = [0u8; 8];
    assert_eq!(bus.read(EndpointAddress::from(0x00u8), &mut buf).unwrap(), 8);
    assert_eq!(bus.read(EndpointAddress::from(0x00u8), &mut buf), Err(UsbError::WouldBlock));
    assert!(matches!(bus.poll(), PollResult::None));
}

/// 阳性对照①:in_complete 忘记清(违反"每完成只报一次")→ 状态机无法推进
#[test]
fn faulty_forget_in_complete_blocks_enumeration() {
    let (_alloc, mut dev, mut serial) = build_device(Faults {
        forget_in_complete: true,
        ..Default::default()
    });
    dev.bus().host_reset();
    pump(&mut dev, &mut serial, 100);
    assert_ne!(dev.state(), UsbDeviceState::Configured, "违约总线不应完成枚举");
}

/// 阳性对照②:ep_setup 报一次即忘(违反"持续上报直到 read")→ SETUP 断链
#[test]
fn faulty_setup_once_only_blocks_enumeration() {
    let (_alloc, mut dev, mut serial) = build_device(Faults {
        setup_once_only: true,
        ..Default::default()
    });
    dev.bus().host_reset();
    pump(&mut dev, &mut serial, 100);
    assert_ne!(dev.state(), UsbDeviceState::Configured, "违约总线不应完成枚举");
}
