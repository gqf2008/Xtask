//! GD32VF103 USBFS(OTG_FS 内芯)的 usb-device 总线实现。
//!
//! 分层:本文件只做**寄存器 ↔ 包**的搬运(UsbBus trait 的 12 个方法),
//! 协议(枚举/描述符/请求)全在 usb-device crate 内部——与 ch22 的
//! `SlipDevice` 同地位,但寄存器面是 OTG_FS 而非 PMA:
//!
//! - 包缓冲 = 内建 FIFO(字深度),无 BTABLE/块表/64 字节对齐;
//! - 收:公共 RX FIFO(`GRFLEN.RXFD`),包描述在 `GRSTATR_DEVICE`,
//!   数据从 FIFO0 数据口读;出队用 `GRSTATP_DEVICE` 弹状态槽;
//! - 发:每 IN 端点独立 TX FIFO(`DIEPnTFLEN`,被 `DIEPnCTL.TXFNUM` 选定),
//!   数据直接写对应 FIFO 数据口(字小端);
//! - 事件:`GINTF` 全局位图(无中断方案下 1ms 轮询读出——不绑
//!   `USBFS`/`USBFS_WKUP` 向量)。
//!
//! ## 无中断设计的三个前提(真机留档核对点,按风险排序)
//! 1. **GINTF 事件位不依赖 GINTEN 置位**(掩码只控 IRQ 路由)——若相反,
//!    退化方案:开 GINTEN 但 ECLIC 不解禁 USBFS 向量;
//! 2. **RPCKST 编码**(`crate::usb::otg` 常量)与手册一致——枚举时 read
//!    落 0/WouldBlock 即此嫌疑;
//! 3. **FIFO 数据口地址**(本文件 `FIFO_BASE`/`TX_FIFO_STRIDE` 常量)与
//!    手册一致——写错地址枚举不出来(第一嫌疑人)。

use crate::usb::fifo::{plan_for, FifoPlan};
use crate::usb::otg::{decode_gintf, decode_grstatr, RxKind};
use gd32vf103xx_hal::pac::{RCU, USBFS_DEVICE, USBFS_GLOBAL, USBFS_PWRCLK};
use usb_device::bus::{PollResult, UsbBus};
use usb_device::endpoint::{EndpointAddress, EndpointType};
use usb_device::{UsbDirection, UsbError};

/// 寄存器基址(固定外设地址,PAC 结构体自带 const ptr,零资源自定位)
fn global() -> &'static gd32vf103xx_hal::pac::usbfs_global::RegisterBlock {
    unsafe { &*USBFS_GLOBAL::ptr() }
}
// SAFETY: USBFS 是固定内存映射外设,ptr() 恒定;单核单任务轮询访问,
// 无并发写(无中断方案),与寄存器读写的并发模型一致
fn device() -> &'static gd32vf103xx_hal::pac::usbfs_device::RegisterBlock {
    unsafe { &*USBFS_DEVICE::ptr() }
}
// SAFETY: 同上
fn pwrclk() -> &'static gd32vf103xx_hal::pac::usbfs_pwrclk::RegisterBlock {
    unsafe { &*USBFS_PWRCLK::ptr() }
}

/// RX FIFO 数据口基址(OTG 布局:FIFO0=收)。
/// ⚠️ 本地未实测,以 GD32VF103 手册"FIFO 访问"段为准——真机留着第一核对点。
const FIFO0_RX_BASE: usize = 0x5000_1000;
/// TX FIFO 数据口步进(每 FIFO 一段)。
const TX_FIFO_STRIDE: usize = 0x1000;
/// 各 IN 端点 TX FIFO 数据口(TXFNUM=EP 号,EP0→FIFO1)。
fn tx_fifo_ptr(ep: u8) -> *mut u32 {
    (FIFO0_RX_BASE + TX_FIFO_STRIDE * (ep as usize + 1)) as *mut u32
}

/// EP0 的 MPL 2 位编码(OTG 惯例:00=64B,01=32B,10=16B,11=8B)。
/// ⚠️ 本地未实测,以手册 DIEP0CTL 字段表为准(枚举不响应 SETUP 即此嫌疑)。
fn mpl_code(mps: u16) -> u8 {
    match mps {
        64 => 0b00,
        32 => 0b01,
        16 => 0b10,
        8 => 0b11,
        _ => 0b11, // 8B 兜底(控制端点最小)
    }
}

/// FIFO 预算(保守值;总深度未核实,Gd32UsbBus::new 内联常量见下)
fn fifo_plan() -> FifoPlan {
    // EP0 8B 控制 / EP1 8B 中断(通知)/ EP2 64B 批量(数据)——usbd-serial 布局
    plan_for(&[8, 8, 64, 8])
    // 注意:plan_for 的每个 IN EP 预算 ≥ ceil(mps/4) 字 → 单包必放得下,
    // `write()` 才无需等待 FIFO(见 usb::fifo 模块文档)
}

/// GD32VF103 的 UsbBus 实现。
///
/// 唯一内部状态是端点分配计数器(`alloc_ep` 只发生在 build 阶段、
/// 调度器启动前、单任务上下文),其余状态全在寄存器里。
pub struct Gd32UsbBus {
    slots: core::cell::Cell<crate::usb::otg::EpSlotState>,
}

impl Gd32UsbBus {
    /// 初始化 USBFS 并进入设备模式。
    ///
    /// 前置:系统时钟经 HAL `freeze()` 配置(本示例=`sysclk(96Mhz())`,
    /// `usbfspsc=/2 → 48MHz`;108MHz 分不出 48M)。
    pub fn new() -> Self {
        // 1. 时钟使能 + 外设复位(RCU;HAL 的 Enable/Reset trait 是
        //    pub(crate) 不可外用,直接写 PAC)
        unsafe {
            RCU::ptr().as_ref().unwrap().ahben.modify(|_r, w| w.usbfsen().set_bit());
            RCU::ptr().as_ref().unwrap().ahbrst.modify(|_r, w| w.usbfsrst().set_bit());
            RCU::ptr().as_ref().unwrap().ahbrst.modify(|_r, w| w.usbfsrst().clear_bit());
        }
        // 2. PHY 时钟门控位保持复位值 0(SUCLK/SHCLK 任一为 1 就停时钟)
        //    PWRCLKCTL 复位值即 0,这里不写(注释钉住"必须保持 0")
        // 3. 强制设备模式
        global().gusbcs.modify(|_r, w| w.fdm().set_bit());
        // 4. 核心软复位(等待自清)
        global().grstctl.modify(|_r, w| w.csrst().set_bit());
        while global().grstctl.read().csrst().bit() {}
        // 5. 收发器上电(备注:VBUSIG 不置——供电/检测依赖按板卡接线,
        //    自供电或外部检测场景可置位 `vbusig`)
        global().gccfg.modify(|_r, w| w.pwron().set_bit());
        // 6. FIFO 预算(RX 公共 + 各 IN EP)
        let plan = fifo_plan();
        global().grflen.modify(|_r, w| unsafe { w.rxfd().bits(plan.rx_words) });
        global().diep0tflen().modify(|_r, w| unsafe { w.iep0txfd().bits(plan.tx_words[0].into()) });
        global().diep1tflen.modify(|_r, w| unsafe {
            w.ieptxfd().bits(plan.tx_words[1].into()).ieptxrsar().bits(0)
        });
        global().diep2tflen.modify(|_r, w| unsafe {
            w.ieptxfd().bits(plan.tx_words[2].into()).ieptxrsar().bits(0)
        });
        // 7. 设备模式基础:全速 + 地址 0(复位态)
        device().dcfg.modify(|_r, w| unsafe { unsafe { w.ds().bits(0) }.dar().bits(0) });
        Gd32UsbBus { slots: core::cell::Cell::new(crate::usb::otg::EpSlotState::new()) }
    }

    /// 按常量表逐一 arm 端点(enable 与 reset 共用;幂等)。
    /// 端点 0/1/2:0=控制,1=中断 IN + 批量 OUT,2=批量 IN(见 otg 常量表)。
    fn arm_endpoints(&self) {
        // EP0:控制(IN+OUT),MPS=8
        // EP0 的 EPTYPE 是只读字段(恒控制)——不写
        device().diep0ctl.modify(|_r, w| unsafe {
            w.mpl().bits(mpl_code(8))
                .txfnum().bits(0)
                .cnak().set_bit()
                .epen().set_bit()
        });
        // DOEP0CTL.MPL 是只读字段(W 侧无 mpl 方法)——EP0 OUT 的 MPS 由硬件决定
        // (🚩真机核对点:若 EP0 收包异常,先查该字段的复位值与 DIEP0CTL 侧一致性)
        device().doep0ctl.modify(|_r, w| w.cnak().set_bit().epen().set_bit());
        // EP1 IN:中断(通知),MPS=8 —— TXFIFO=1
        device().diep1ctl.modify(|_r, w| unsafe {
            w.mpl().bits(8).eptype().bits(3).txfnum().bits(1).cnak().set_bit().epen().set_bit()
        });
        // EP1 OUT:批量(数据读),MPS=64
        device().doep1ctl.modify(|_r, w| unsafe {
            w.mpl().bits(64).eptype().bits(2).cnak().set_bit().epen().set_bit()
        });
        // EP2 IN:批量(数据写),MPS=64 —— TXFIFO=2
        device().diep2ctl.modify(|_r, w| unsafe {
            w.mpl().bits(64).eptype().bits(2).txfnum().bits(2).cnak().set_bit().epen().set_bit()
        });
        // 清各 EP 中断标志(W1C 全 1)与 GINTF 的 RST/ENUMF
        device().diep0intf.modify(|_r, w| unsafe { w.bits(0xFFFF) });
        device().diep1intf.modify(|_r, w| unsafe { w.bits(0xFFFF) });
        device().diep2intf.modify(|_r, w| unsafe { w.bits(0xFFFF) });
        device().doep0intf.modify(|_r, w| unsafe { w.bits(0xFFFF) });
        device().doep1intf.modify(|_r, w| unsafe { w.bits(0xFFFF) });
        global().gintf.modify(|_r, w| unsafe { w.bits((1 << 12) | (1 << 13)) });
    }
}

impl UsbBus for Gd32UsbBus {
    fn alloc_ep(
        &mut self,
        ep_dir: UsbDirection,
        ep_addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        _interval: u8,
    ) -> Result<EndpointAddress, UsbError> {
        // 分配计数与 MockUsbBus/宿主 e2e 完全相同一语义(同一纯函数),
        // 寄存器配置统一在 enable/reset 的 arm_endpoints 里按常量表写
        let req = crate::usb::otg::EpRequest::from_bus(ep_dir, ep_addr, ep_type, max_packet_size);
        let mut st = self.slots.get();
        let num = crate::usb::otg::allocate_slot(&mut st, &req)?;
        self.slots.set(st);
        Ok(EndpointAddress::from_parts(num as usize, ep_dir))
    }

    fn enable(&mut self) {
        self.arm_endpoints();
        // 连接:撤下软断连(上拉电阻接入总线)
        device().dctl.modify(|_r, w| w.sd().clear_bit());
    }

    fn reset(&self) {
        self.arm_endpoints();
        device().dcfg.modify(|_r, w| unsafe { w.dar().bits(0) });
    }

    fn set_device_address(&self, addr: u8) {
        device().dcfg.modify(|_r, w| unsafe { w.dar().bits(addr) });
    }

    fn write(&self, ep_addr: EndpointAddress, buf: &[u8]) -> Result<usize, UsbError> {
        let idx = ep_addr.index();
        // FIFO 预算保证单包放得下(usb::fifo 模块文档)——不需要等待
        // 逐字写入 TX FIFO 数据口(字内小端)
        for chunk in buf.chunks(4) {
            let mut w = [0u8; 4];
            w[..chunk.len()].copy_from_slice(chunk);
            let word = u32::from_le_bytes(w);
            unsafe { core::ptr::write_volatile(tx_fifo_ptr(idx as u8), word) };
        }
        // 配一次传输:长度 + 包数(1 包;>MPS 由 usbd-serial 的短包纪律保证)
        match idx {
            0 => device().diep0len.modify(|_r, w| unsafe { w.tlen().bits(buf.len() as u8).pcnt().bits(0) }),
            1 => device().diep1len.modify(|_r, w| unsafe { w.tlen().bits(buf.len() as u32).pcnt().bits(1) }),
            2 => device().diep2len.modify(|_r, w| unsafe { w.tlen().bits(buf.len() as u32).pcnt().bits(1) }),
            _ => return Err(UsbError::InvalidEndpoint),
        }
        Ok(buf.len())
    }

    fn read(&self, ep_addr: EndpointAddress, buf: &mut [u8]) -> Result<usize, UsbError> {
        let idx = ep_addr.index();
        if idx > 1 {
            return Err(UsbError::InvalidEndpoint);
        }
        // RX FIFO 非空才有条目可读
        if !global().gintf.read().rxfneif().bit() {
            return Err(UsbError::WouldBlock);
        }
        let stat = decode_grstatr(global().grstatr_device().read().bits());
        if stat.epnum != idx as u8 {
            // 队列头是别的端点的包——等 poll 先报它(契约:只有被 poll
            // 报的 EP 才会被 read)
            return Err(UsbError::WouldBlock);
        }
        match stat.kind {
            RxKind::Setup => {
                // SETUP 8 字节 = 2 字,走 RX FIFO
                for k in 0..2 {
                    let word = unsafe { core::ptr::read_volatile(FIFO0_RX_BASE as *const u32) };
                    let b = word.to_le_bytes();
                    if k * 4 + 4 <= buf.len() {
                        buf[k * 4..k * 4 + 4].copy_from_slice(&b);
                    }
                }
                let _ = global().grstatp_device().read(); // 弹状态槽(RO:读即弹)
                Ok(8.min(buf.len()))
            }
            RxKind::OutData => {
                let bcount = stat.bcount as usize;
                let words = bcount.div_ceil(4);
                let mut n = 0usize;
                for _ in 0..words {
                    let word = unsafe { core::ptr::read_volatile(FIFO0_RX_BASE as *const u32) };
                    let b = word.to_le_bytes();
                    for j in 0..4 {
                        if n < bcount {
                            if n < buf.len() {
                                buf[n] = b[j];
                            }
                            n += 1;
                        }
                    }
                }
                let _ = global().grstatp_device().read(); // 弹状态槽(RO:读即弹)
                Ok(n.min(buf.len()))
            }
            RxKind::SetupComplete | RxKind::Other(_) => {
                // 丢弃该类条目(弹状态槽,不取数据)
                let _ = global().grstatp_device().read(); // 弹状态槽(RO:读即弹)
                Err(UsbError::WouldBlock)
            }
        }
    }

    fn set_stalled(&self, ep_addr: EndpointAddress, stalled: bool) {
        let idx = ep_addr.index();
        // 按方向写对应 CTL 的 STALL 位(EP0 控制端点两侧都要)
        match (idx, ep_addr.is_in()) {
            (_, true) => match idx {
                0 => device().diep0ctl.modify(|_r, w| {
                    if stalled { w.stall().set_bit() } else { w.stall().clear_bit() }
                }),
                1 => device().diep1ctl.modify(|_r, w| {
                    if stalled { w.stall().set_bit() } else { w.stall().clear_bit() }
                }),
                2 => device().diep2ctl.modify(|_r, w| {
                    if stalled { w.stall().set_bit() } else { w.stall().clear_bit() }
                }),
                _ => {}
            },
            (_, false) => match idx {
                0 => device().doep0ctl.modify(|_r, w| {
                    if stalled { w.stall().set_bit() } else { w.stall().clear_bit() }
                }),
                1 => device().doep1ctl.modify(|_r, w| {
                    if stalled { w.stall().set_bit() } else { w.stall().clear_bit() }
                }),
                _ => {}
            },
        }
    }

    fn is_stalled(&self, ep_addr: EndpointAddress) -> bool {
        let idx = ep_addr.index();
        if ep_addr.is_in() {
            match idx {
                0 => device().diep0ctl.read().stall().bit(),
                1 => device().diep1ctl.read().stall().bit(),
                2 => device().diep2ctl.read().stall().bit(),
                _ => false,
            }
        } else {
            match idx {
                0 => device().doep0ctl.read().stall().bit(),
                1 => device().doep1ctl.read().stall().bit(),
                _ => false,
            }
        }
    }

    fn suspend(&self) {
        // 总线挂起:设备自动进低功耗;这里无额外动作(书稿改造练习:
        // 可切时钟、置 EVT 标志)
    }

    fn resume(&self) {
        // 远程唤醒
        device().dctl.modify(|_r, w| w.rwkup().set_bit());
    }

    fn poll(&self) -> PollResult {
        let g = decode_gintf(global().gintf.read().bits());
        if g.reset {
            global().gintf.modify(|_r, w| unsafe { w.bits(1 << 12) }); // W1C 清 RST
            return PollResult::Reset;
        }
        if g.suspended {
            global().gintf.modify(|_r, w| unsafe { w.bits(1 << 11) }); // W1C 清 SP
            return PollResult::Suspend;
        }
        if g.resume {
            global().gintf.modify(|_r, w| unsafe { w.bits(1 << 31) }); // W1C 清 WKUPIF
            return PollResult::Resume;
        }
        let mut ep_out = 0u16;
        let mut ep_setup = 0u16;
        if g.rx_fifo_ne {
            // peek 队头条目合成事件;条目在 read() 弹走前一直在队列里 →
            // "持续上报直到 read" 由硬件队列保证
            let stat = decode_grstatr(global().grstatr_device().read().bits());
            match stat.kind {
                RxKind::Setup => ep_setup |= 1 << stat.epnum,
                RxKind::OutData => ep_out |= 1 << stat.epnum,
                _ => {}
            }
        }
        let mut ep_in_complete = 0u16;
        // IN 完成边沿:TF 位(W1C 清 → "每完成只报一次")——逐个 EP 读+清
        {
            let intf = &device().diep0intf;
            if intf.read().bits() & 0x01 != 0 {
                ep_in_complete |= 1 << 0;
                intf.modify(|_r, w| unsafe { w.bits(0x01) }); // W1C 清 TF
            }
        }
        {
            let intf = &device().diep1intf;
            if intf.read().bits() & 0x01 != 0 {
                ep_in_complete |= 1 << 1;
                intf.modify(|_r, w| unsafe { w.bits(0x01) });
            }
        }
        {
            let intf = &device().diep2intf;
            if intf.read().bits() & 0x01 != 0 {
                ep_in_complete |= 1 << 2;
                intf.modify(|_r, w| unsafe { w.bits(0x01) });
            }
        }
        if ep_out != 0 || ep_setup != 0 || ep_in_complete != 0 {
            PollResult::Data { ep_out, ep_in_complete, ep_setup }
        } else {
            PollResult::None
        }
    }
}

// SAFETY: ①寄存器状态:单核单任务 + 无中断轮询,不存在并发访问;
// ②唯一内部状态 `slots`(Cell)只在 alloc_ep(&mut self) 里读写,
//   而 alloc_ep 只发生在 build 阶段——调度器启动前、单任务上下文,
//   与"&self 侧只碰寄存器"的模型一致(论证写进书稿代码精读)
unsafe impl Sync for Gd32UsbBus {}
