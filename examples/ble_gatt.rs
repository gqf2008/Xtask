#![no_std]
#![no_main]

extern crate alloc;

use gd32vf103xx_hal as hal;
use hal::{gpio::GpioExt, pac, prelude::*, rcu::RcuExt};

use xtask::arch::riscv::rt;
use xtask::ble::at::{AtSession, Cmd, RespLine};
use xtask::ble::gatt::Service;
use xtask::bsp::longan_nano::drv_uart::{uart0_isr, Uart0};
use xtask::bsp::longan_nano::led::{rgb, GREEN, Led};
use xtask::device::register;
use xtask::prelude::*;

// BLE 示例(第 24 章):E104-BT5032A 模组 + GATT 服务建模——手机 nRF Connect
// 看到板子的自定义 GATT 服务。链路:Uart0(USART0@57600,ch20 零新驱动)→
// AT 命令层(ble::at)→ 模组固件内的 BLE 协议栈(nRF52832)→ 手机。
//
// ⚠️ 一次性配置(书稿实验 0):模组出厂 115200,而本板 USART0 因 PCB 瑕疵
// 只能跑 57600——用 USB-TTL@115200 接模组发一条 `AT+BAUD=8`(立即生效并
// 保存,永不重配),之后模组永久 57600。本示例开头 `AT` 握手失败会打日志
// 指路,不 panic。
//
// 接线:模组 TX→PA10(RX)、RX→PA9(TX)、VCC/GND 3.3V(5V 会烧)。
// 真机:nRF Connect 扫描(默认名 E104-BT5032A)→ Connect → Discover →
//   Service 0x1102: 0x1103 notify(板→手机,每秒心跳)/ 0x1104 write
//   (手机→板,写了会被回显)/ 0xFFF3(空中配置:开 notify 后写
//   `at+auth=12345` 认证,手机变遥控器)。
//
// 任务:ble-pump(prio 2)五阶段——握手(容错)→ 配置 7 步 → 等
// STA:wakeup → 开广播+打印 GATT 树 → 稳态(事件日志/FFF2 回显/心跳);
// led-blink(prio 1)绿 500ms。日志走 RTT。
// ⚠️ 红线:握手/配置全部在任务里(xtask::start() 之前 read_byte 会踩
// xworker.current() 空指针——书稿踩坑 5)。

/// 自定义 GATT 树:服务 0x1102,板→手机 0x1103,手机→板 0x1104
const SVR: u16 = 0x1102;
const CH_FROM: u16 = 0x1103;
const CH_TO: u16 = 0x1104;

fn init() -> (&'static Uart0, GREEN) {
    extern "C" {
        /// 堆内存开始地址,在 riscv-rt link.x 文件里定义
        static _sheap: u8;
    }
    let start_addr = unsafe { &_sheap as *const u8 as usize };
    xtask::init_heap(start_addr, 32 * 1024);

    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();

    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpioc = dp.GPIOC.split(&mut rcu);
    let (mut red, mut green, _blue) = rgb(gpioc.pc13, gpioa.pa1, gpioa.pa2);
    red.off();
    green.off();

    let mut afio = dp.AFIO.constrain(&mut rcu);
    // USART0 给 BLE 模组(57600=PCB 上限;模组先按书稿实验 0 一次性配好)。
    // Uart0::new 返回 &'static Uart0(具体类型)——注册表收 &dyn Device,
    // 会话收 &dyn BleIo:两个 unsize 方向都从具体类型出发才成立
    // (trait 对象之间不能互转)
    let uart0: &'static Uart0 =
        Uart0::new(dp.USART0, gpioa.pa9, gpioa.pa10, 57600.bps(), &mut afio, &mut rcu);
    register("ble0", uart0).expect("register ble0");

    log::info!("ble_gatt: uart0@57600 -> E104-BT5032A");
    (uart0, green)
}

/// USART0 中断向量:port.S `.weak USART0` 的强定义绑定(ch20 机制)。
/// 模组应答/事件/透传数据全靠它进环形缓冲。
#[no_mangle]
extern "C" fn USART0() {
    uart0_isr();
}

/// STA 事件字样是否表示"已连接"(事件词汇未核实——真机核对点,
/// 只改这一处)
fn is_connect(t: &[u8]) -> bool {
    t.starts_with(b"connect") && !t.starts_with(b"disconnect")
}

/// 发命令并在 ms 毫秒内等第一条应答行(容错版 send:超时返回 None 不 panic)
fn wait_line(sess: &mut AtSession<'_>, cmd: &Cmd, ms: usize) -> Option<RespLine> {
    sess.request(cmd);
    for _ in 0..ms {
        if let Some(line) = sess.poll() {
            return Some(line);
        }
        xtask::sleep_ms(1);
    }
    None
}

#[rt::entry]
fn main() -> ! {
    let (uart0, mut green) = init();

    // BLE 泵:握手/配置/稳态全在此任务内(红线:init 里绝不 read)
    TaskBuilder::new()
        .name("ble-pump")
        .priority(2)
        .stack_size(1024)
        .spawn(move || {
            let mut sess = AtSession::new(uart0);

            // ---- 阶段 A:握手(三种失败各有精确指路,绝不 panic)----
            match wait_line(&mut sess, &Cmd::Test, 500) {
                Some(RespLine::Ok) => log::info!("ble: 模组在位 @57600"),
                Some(RespLine::ErrCode(e)) => {
                    log::warn!("ble: 应答 +ERR={e}(已连接?拉低 MOD 再进配置)")
                }
                other => log::warn!("ble: 无握手应答({other:?})——先按书稿实验 0 用 USB-TTL@115200 发 AT+BAUD=8"),
            }

            // ---- 阶段 B:配置 7 步(逐条日志;失败不中断,现场可见)----
            let seq = xtask::ble::at::configure_gatt(SVR, CH_FROM, CH_TO, 160);
            for (i, cmd) in seq.iter().enumerate() {
                match sess.send(cmd) {
                    Ok(RespLine::Ok) => log::info!("ble: cfg[{}/7] {cmd:?}", i + 1),
                    Ok(r) => log::warn!("ble: cfg[{}/7] {cmd:?} -> {r:?}", i + 1),
                    Err(e) => log::warn!("ble: cfg[{}/7] {cmd:?} -> 传输 {e:?}", i + 1),
                }
            }

            // ---- 阶段 C:等 STA:wakeup(复位事件;3000ms 截止非死等)----
            let mut woke = false;
            for _ in 0..3000 {
                if let Some(RespLine::Sta(t)) = sess.poll() {
                    log::info!("ble: STA:{}", core::str::from_utf8(t.as_bytes()).unwrap_or("?"));
                    woke = true;
                    break;
                }
                xtask::sleep_ms(1);
            }
            if !woke {
                log::warn!("ble: 3000ms 未见 STA:wakeup(事件字样=真机核对点)");
            }

            // ---- 阶段 D:复位生效后开广播 + 打印 GATT 树两把 ----
            match sess.send(&Cmd::Adv(xtask::ble::at::AdvMode::Normal)) {
                Ok(_) => log::info!("ble: advertising @100ms"),
                Err(e) => log::warn!("ble: ADV 失败 {e:?}"),
            }
            log::info!("ble: 默认树\n{}", Service::default_tree());
            log::info!("ble: 自定义树\n{}", Service::custom(SVR, CH_FROM, CH_TO));

            // ---- 阶段 E:稳态泵(1ms)----
            let mut connected = false;
            let mut tick = 0u64;
            loop {
                // 事件/应答行:非阻塞排空 → 分类日志 + 连接状态翻转
                while let Some(line) = sess.poll() {
                    match line {
                        RespLine::Sta(t) => {
                            let now = is_connect(t.as_bytes());
                            if now != connected {
                                connected = now;
                                log::info!("ble: {}", if now { "已连接" } else { "已断开" });
                            }
                        }
                        other => log::info!("ble: {other:?}"),
                    }
                }
                // 透传载荷(手机经 FFF2 写来的裸字节):原样回写 → FFF1 notify
                let partial = sess.take_partial().to_vec();
                if !partial.is_empty() {
                    sess.reset_partial();
                    sess.write_raw(&partial); // 回显(FFF2→板→FFF1;经会话=同一写路径)
                    log::info!("ble: data {:?} (FFF2→FFF1 回显)", &partial);
                }
                tick += 1;
                // 心跳:仅连接时发(未连接写字节会被模组当非法 AT 回 +ERR)
                if connected && tick % 1000 == 0 {
                    let line = alloc::format!("ble_gatt: heartbeat #{tick}\r\n");
                    sess.write_raw(line.as_bytes());
                }
                xtask::sleep_ms(1);
            }
        });

    // 裸眼观测:绿 500ms 翻转
    TaskBuilder::new()
        .name("led-blink")
        .priority(1)
        .stack_size(256)
        .spawn(move || loop {
            green.toggle();
            xtask::sleep_ms(500);
        });

    xtask::start()
}
