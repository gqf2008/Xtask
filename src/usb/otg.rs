//! OTG 寄存器位域的纯解码层——"会算错"的部分全在这里,宿主回归钉死。
//!
//! GD32VF103 的 USBFS 是 Synopsys OTG_FS 内芯(STM32F105/F107 同源),
//! 与老式 STM32F103 的 PMA 型控制器(EPnR/BTABLE)完全不同:
//! 无"端点 RAM 索引",数据走内建 FIFO(字深度),端点行为由
//! DIEPnCTL/DOEPnCTL 描述。本模块只做位域数学与分配策略,
//! 不碰任何寄存器地址(那是 chip 层的事)。

use usb_device::endpoint::{EndpointAddress, EndpointType};
use usb_device::UsbDirection;
use usb_device::UsbError;

/// RX 收包状态(RPCKST,GRSTATR_DEVICE bit17:20)的 OTG 惯例编码。
/// 集中一处:本地未实测,以 GD32VF103 手册"接收包状态"表为准;
/// 真机留档时此处是**第一核对点**(枚举时 read 落 0 或 WouldBlock 即编码错)。
pub const RPCKST_OUT_DATA: u8 = 0x1;
pub const RPCKST_SETUP: u8 = 0x2;
pub const RPCKST_SETUP_COMPLETE: u8 = 0x3;

/// 一个已收包(从 GRSTATR_DEVICE/GRSTATP_DEVICE 读出,peek 不弹)。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RxPacket {
    pub epnum: u8,
    /// 包字节数(不含 4B 字对齐填充)
    pub bcount: u16,
    pub dpid: u8,
    pub kind: RxKind,
}

/// 收包类型:RPCKST 编码 → 语义
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxKind {
    OutData,
    Setup,
    SetupComplete,
    /// 未知编码 / 其它状态(丢弃该条目)
    Other(u8),
}

/// 解析收包状态字(GRSTATR_DEVICE / GRSTATP_DEVICE 同布局)。
pub fn decode_grstatr(bits: u32) -> RxPacket {
    let rpcst = ((bits >> 17) & 0xF) as u8;
    let kind = match rpcst {
        RPCKST_OUT_DATA => RxKind::OutData,
        RPCKST_SETUP => RxKind::Setup,
        RPCKST_SETUP_COMPLETE => RxKind::SetupComplete,
        other => RxKind::Other(other),
    };
    RxPacket {
        epnum: (bits & 0xF) as u8,
        bcount: ((bits >> 4) & 0x7FF) as u16,
        dpid: ((bits >> 15) & 0x3) as u8,
        kind,
    }
}

/// 全局中断(GINTF)解码——无中断方案下,事件从这一位图轮询读出。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Gint {
    pub reset: bool,
    pub enum_done: bool,
    pub suspended: bool,
    pub resume: bool,
    pub in_ep: bool,
    pub out_ep: bool,
    pub rx_fifo_ne: bool,
}

/// GINTF 位图 → 事件(位号以 PAC 核实为准:4=RXFNEIF,11=SP,12=RST,
/// 13=ENUMF,18=IEPIF,19=OEPIF,31=WKUPIF)。
pub fn decode_gintf(bits: u32) -> Gint {
    Gint {
        reset: bits & (1 << 12) != 0,
        enum_done: bits & (1 << 13) != 0,
        suspended: bits & (1 << 11) != 0,
        resume: bits & (1 << 31) != 0,
        in_ep: bits & (1 << 18) != 0,
        out_ep: bits & (1 << 19) != 0,
        rx_fifo_ne: bits & (1 << 4) != 0,
    }
}

/// EP 控制/中断/长度寄存器的设备组内偏移(与 PAC 地址 0x100/0x300 起
/// 每 EP 0x20 间隔的 4 元组核对过):
/// IN:  CTL=+0x00 INTF=+0x08 LEN=+0x10 TFSTAT=+0x18
/// OUT: CTL=+0x00 INTF=+0x08 LEN=+0x10
pub fn diep_ctl_off(ep: u8) -> usize {
    0x100 + 0x20 * ep as usize
}
pub fn diep_intf_off(ep: u8) -> usize {
    0x108 + 0x20 * ep as usize
}
pub fn diep_len_off(ep: u8) -> usize {
    0x110 + 0x20 * ep as usize
}
pub fn doep_ctl_off(ep: u8) -> usize {
    0x300 + 0x20 * ep as usize
}
pub fn doep_intf_off(ep: u8) -> usize {
    0x308 + 0x20 * ep as usize
}
pub fn doep_len_off(ep: u8) -> usize {
    0x310 + 0x20 * ep as usize
}

/// 端点类型 → OTG 的 EPTYPE 编码(0=控制,1=等时,2=批量,3=中断)
pub fn eptype_code(t: EndpointType) -> u8 {
    match t {
        EndpointType::Control => 0,
        EndpointType::Isochronous { .. } => 1,
        EndpointType::Bulk => 2,
        EndpointType::Interrupt => 3,
    }
}

/// 端点分配请求(UsbBus::alloc_ep 的入参,纯数据)。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EpRequest {
    pub dir_in: bool,
    /// 显式地址(EP0 由 usb-device crate 指定 0x00/0x80);None = 由我们分配
    pub addr: Option<u8>,
    pub ep_type: u8,
    pub mps: u16,
}

impl EpRequest {
    pub fn from_bus(dir: UsbDirection, addr: Option<EndpointAddress>, t: EndpointType, mps: u16) -> EpRequest {
        EpRequest {
            dir_in: dir == UsbDirection::In,
            addr: addr.map(|a| a.into()),
            ep_type: eptype_code(t),
            mps,
        }
    }
}

/// 槽位分配错误 → usb_device::UsbError 的映射
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EpAllocError {
    /// 端点号越界(本芯片只有 EP0-3)
    Overflow,
    /// 重复分配(同方向同号)
    Duplicate,
}

impl From<EpAllocError> for UsbError {
    fn from(e: EpAllocError) -> Self {
        match e {
            EpAllocError::Overflow => UsbError::EndpointOverflow,
            EpAllocError::Duplicate => UsbError::InvalidEndpoint,
        }
    }
}

/// 端点分配状态:每方向独立发号(IN 从 0x81 起,OUT 从 0x01 起),
/// EP0(号 0)允许 IN/OUT 各分配一次(crate 会先 0x00 后 0x80)。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EpSlotState {
    in_next: u8,
    out_next: u8,
    in_used: u8,
    out_used: u8,
}

/// 分配一个端点号(纯函数,宿主测过)。
/// 显式地址(EP0)校验方向一致性后返回同号;隐式地址按方向计数器发号。
pub fn allocate_slot(st: &mut EpSlotState, req: &EpRequest) -> Result<u8, EpAllocError> {
    let (num, used) = match req.addr {
        Some(a) => {
            let num = a & 0x0F;
            let a_dir_in = a & 0x80 != 0;
            if a_dir_in != req.dir_in {
                return Err(EpAllocError::Duplicate);
            }
            (num, if req.dir_in { st.in_used } else { st.out_used })
        }
        None => {
            if req.dir_in {
                if st.in_next > 3 {
                    return Err(EpAllocError::Overflow);
                }
                (st.in_next, st.in_used)
            } else {
                if st.out_next > 3 {
                    return Err(EpAllocError::Overflow);
                }
                (st.out_next, st.out_used)
            }
        }
    };
    let m = 1u8 << num;
    if used & m != 0 {
        return Err(EpAllocError::Duplicate);
    }
    if req.dir_in {
        st.in_used |= m;
        if st.in_next <= num {
            st.in_next = num + 1;
        }
    } else {
        st.out_used |= m;
        if st.out_next <= num {
            st.out_next = num + 1;
        }
    }
    Ok(num)
}

/// **CDC 示例的端点布局**(与 usbd-serial 0.2.2 的真实 alloc 序列逐项对齐,
/// 源码核实:interface + interrupt(8,255) + interface + bulk(64)×2):
/// crate 先分配 EP0(0x00 控制 OUT / 0x80 控制 IN)→ 0x81 中断 IN(通知)→
/// 0x01 批量 OUT(数据读)→ 0x82 批量 IN(数据写)——
/// 隐式地址按方向计数器发号(OUT from 1,IN from 0x81)。
/// 地址由**总线侧**分配策略决定(alloc_ep 的 addr=None 分支)——这就是
/// `allocate_slot` 的计数器语义;本函数是它的"预期调用序列"参照。
pub fn serial_ep_requests() -> [EpRequest; 5] {
    [
        EpRequest { dir_in: false, addr: Some(0x00), ep_type: 0, mps: 8 },
        EpRequest { dir_in: true, addr: Some(0x80), ep_type: 0, mps: 8 },
        EpRequest { dir_in: true, addr: None, ep_type: 3, mps: 8 },
        EpRequest { dir_in: false, addr: None, ep_type: 2, mps: 64 },
        EpRequest { dir_in: true, addr: None, ep_type: 2, mps: 64 },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_grstatr_separates_fields() {
        // 字段各自落位:EPNUM=2 BCOUNT=300 DPID=1 RPCKST=OUT_DATA
        let bits = (0x1u32 << 17) | (1 << 15) | (300 << 4) | 2;
        let p = decode_grstatr(bits);
        assert_eq!(p.epnum, 2);
        assert_eq!(p.bcount, 300);
        assert_eq!(p.dpid, 1);
        assert_eq!(p.kind, RxKind::OutData);
    }

    #[test]
    fn decode_grstatr_maps_rpcst_kinds() {
        let mk = |r: u8| decode_grstatr((r as u32) << 17).kind;
        assert_eq!(mk(RPCKST_OUT_DATA), RxKind::OutData);
        assert_eq!(mk(RPCKST_SETUP), RxKind::Setup);
        assert_eq!(mk(RPCKST_SETUP_COMPLETE), RxKind::SetupComplete);
        assert_eq!(mk(0x5), RxKind::Other(0x5));
    }

    #[test]
    fn gint_bits_map_to_events() {
        // 全位对照(与 PAC 核实过的位号)
        let g = decode_gintf((1u32 << 4) | (1 << 11) | (1 << 12) | (1 << 13) | (1 << 18) | (1 << 19) | (1 << 31));
        assert!(g.rx_fifo_ne);
        assert!(g.suspended);
        assert!(g.reset);
        assert!(g.enum_done);
        assert!(g.in_ep);
        assert!(g.out_ep);
        assert!(g.resume);
        // 无关位不误报
        let g2 = decode_gintf(1 << 0);
        assert_eq!(g2, Gint::default());
    }

    #[test]
    fn ep_offset_math_matches_pac() {
        // 与 PAC 地址核对:IN 组 0x100/0x108/0x110/0x118 起每 EP 0x20;
        // OUT 组 0x300/0x308/0x310。
        assert_eq!(diep_ctl_off(0), 0x100);
        assert_eq!(diep_intf_off(0), 0x108);
        assert_eq!(diep_len_off(0), 0x110);
        assert_eq!(diep_ctl_off(3), 0x160);
        assert_eq!(diep_intf_off(3), 0x168);
        assert_eq!(diep_len_off(3), 0x170);
        assert_eq!(doep_ctl_off(0), 0x300);
        assert_eq!(doep_intf_off(0), 0x308);
        assert_eq!(doep_len_off(0), 0x310);
        assert_eq!(doep_ctl_off(3), 0x360);
        assert_eq!(doep_len_off(3), 0x370);
    }

    #[test]
    fn eptype_codes_are_otg_encoding() {
        assert_eq!(eptype_code(EndpointType::Control), 0);
        assert_eq!(
            eptype_code(EndpointType::Isochronous {
                synchronization: usb_device::endpoint::IsochronousSynchronizationType::Asynchronous,
                usage: usb_device::endpoint::IsochronousUsageType::Data,
            }),
            1
        );
        assert_eq!(eptype_code(EndpointType::Bulk), 2);
        assert_eq!(eptype_code(EndpointType::Interrupt), 3);
    }

    #[test]
    fn serial_sequence_allocates_expected_addresses() {
        // usbd-serial 的五连发 → 0x00/0x80(显式)+ 0x81/0x02/0x82(隐式计数)
        let mut st = EpSlotState::default();
        let reqs = serial_ep_requests();
        let mut outs = Vec::new();
        for r in &reqs {
            let n = allocate_slot(&mut st, r).expect("alloc");
            outs.push(if r.dir_in { n | 0x80 } else { n });
        }
        assert_eq!(outs, vec![0x00, 0x80, 0x81, 0x01, 0x82]);
    }

    #[test]
    fn allocate_slot_duplicate_addr_rejected() {
        // 阳性对照:同一 (号,方向) 二次分配必须红
        let mut st = EpSlotState::default();
        let ep0_out = EpRequest { dir_in: false, addr: Some(0x00), ep_type: 0, mps: 8 };
        allocate_slot(&mut st, &ep0_out).expect("first");
        assert_eq!(allocate_slot(&mut st, &ep0_out), Err(EpAllocError::Duplicate));
    }

    #[test]
    fn allocate_slot_direction_mismatch_rejected() {
        // 显式地址方向位与请求方向矛盾 → 拒绝
        let mut st = EpSlotState::default();
        let bad = EpRequest { dir_in: false, addr: Some(0x81), ep_type: 2, mps: 64 };
        assert_eq!(allocate_slot(&mut st, &bad), Err(EpAllocError::Duplicate));
    }

    #[test]
    fn allocate_slot_overflow_after_four_each_direction() {
        let mut st = EpSlotState::default();
        for i in 0..4u8 {
            let r = EpRequest { dir_in: true, addr: None, ep_type: 2, mps: 64 };
            allocate_slot(&mut st, &r).expect("in");
            let r2 = EpRequest { dir_in: false, addr: None, ep_type: 2, mps: 64 };
            allocate_slot(&mut st, &r2).expect("out");
            let _ = i;
        }
        let extra = EpRequest { dir_in: true, addr: None, ep_type: 2, mps: 64 };
        assert_eq!(allocate_slot(&mut st, &extra), Err(EpAllocError::Overflow));
    }
}
