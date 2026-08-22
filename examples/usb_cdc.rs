#![no_std]
#![no_main]

extern crate alloc;

use alloc::boxed::Box;
use alloc::format;

use gd32vf103xx_hal as hal;
use hal::{gpio::GpioExt, pac, prelude::*, rcu::RcuExt};

use xtask::arch::riscv::rt;
use xtask::bsp::longan_nano::led::{rgb, GREEN, Led};
use xtask::bsp::longan_nano::usb::Gd32UsbBus;
use xtask::prelude::*;
use xtask::usb::device::{UsbDeviceBuilder, UsbVidPid};
use xtask::usb::bus::UsbBusAllocator;
use xtask::usb::{LangID, SerialPort};

// USB CDC 示例(第 23 章):板子插上电脑即枚举出虚拟串口,日志从 UART 搬到 USB。
// 链路:Gd32UsbBus(OTG_FS 寄存器 ↔ 包,chip 层)→ usb-device(枚举/描述符/
// 请求,crate 内部)→ usbd-serial(CDC-ACM 类)→ usb-pump 任务。
// 任务:
// - usb-pump(prio 2):**唯一**驱动点——1ms 轮询 `dev.poll`(≪10ms 合规),
//   读→回显,每 1000 tick 经 CDC 写一行"日志"(书稿:日志从 UART 搬到 USB);
// - led-blink(prio 1):500ms 翻绿——无中断轮询方案在跑的裸眼观测点;
// 关键前提:**sysclk 必须是 96MHz**(USBFSPSC=/2 → 48MHz;108MHz 分不出 48M,
// usbclk_valid() 查询背书——示例运行环境不准,先看这条断言)。
// 日志走默认 RTT(不占任何串口);真机:插 USB 线 → 设备管理器/dmesg 出
// COM/ttyACM → 打开终端(9600 8N1)看到心跳行,输入字符回显。
// 注意:env.rs 的 CPU_CLOCK_HZ 仍按 108MHz 编译,本示例的 tick 实际约
// 1.125ms(96/108)——对"≥10ms 轮询合规"无影响;这是"频率假设要显式"
// 的现场,书稿踩坑记录条目。

fn init() -> GREEN {
    extern "C" {
        /// 堆内存开始地址,在 riscv-rt link.x 文件里定义
        static _sheap: u8;
    }
    let start_addr = unsafe { &_sheap as *const u8 as usize };
    xtask::init_heap(start_addr, 32 * 1024);

    let dp = pac::Peripherals::take().unwrap();
    // 配置时钟:96MHz —— 48MHz USB 时钟的唯一组合(108M 分不出)
    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(96.mhz())
        .freeze();
    debug_assert!(
        rcu.clocks.usbclk_valid(),
        "USB 需要 48MHz 时钟:sysclk 必须是 48/72/96MHz 之一"
    );

    // USART0 留空(日志走 RTT;USB 才是本示例的"串口")
    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpioc = dp.GPIOC.split(&mut rcu);
    let (mut red, mut green, _blue) = rgb(gpioc.pc13, gpioa.pa1, gpioa.pa2);
    red.off();
    green.off();

    // 绿 LED 交给 blink 任务(move 走;红/蓝直接熄灭)
    // PA11(DM)/PA12(DP) 是专用脚,无需 GPIO 配置、无 remap、无板级冲突
    log::info!("usb_cdc: base clock 96MHz (usbclk_valid={})", rcu.clocks.usbclk_valid());
    green
}

#[rt::entry]
fn main() -> ! {
    let green = init();

    // 协议栈挂载(与 ch21/22 同款:调度器启动前完成)。
    // allocator 泄漏成 'static:dev/serial 借用它,要进任务闭包
    let alloc: &'static UsbBusAllocator<Gd32UsbBus> =
        Box::leak(Box::new(UsbBusAllocator::new(Gd32UsbBus::new())));
    let mut serial = SerialPort::new(alloc);
    let strings = [xtask::usb::device::StringDescriptors::new(LangID::EN_US)
        .manufacturer("gqf")
        .product("Xtask CDC")];
    let mut dev = UsbDeviceBuilder::new(alloc, UsbVidPid(0x0512, 0xCECE))
        .strings(&strings)
        .expect("strings")
        .device_class(xtask::usb::USB_CLASS_CDC)
        .build();

    // USB 泵:唯一时间持有者(与 net_echo 的泵同构;区别是节奏 1ms)
    TaskBuilder::new()
        .name("usb-pump")
        .priority(2)
        .stack_size(1024)
        .spawn(move || {
            let mut buf = [0u8; 128];
            let mut tick = 0u64;
            loop {
                // 协议状态机单点驱动(注意:绝不能在 log() 里 poll——书稿练习)
                let _ = dev.poll(&mut [&mut serial]);
                // 回显:读到即写(WouldBlock → 0,不阻塞)
                if let Ok(n) = serial.read(&mut buf) {
                    if n > 0 {
                        let _ = serial.write(&buf[..n]);
                    }
                }
                tick += 1;
                if tick % 1000 == 0 {
                    // "UART 日志"搬进 USB:一行心跳,经 CDC 流到电脑串口
                    let line = format!("usb_cdc: heartbeat #{tick}, state={:?}\r\n", dev.state());
                    let _ = serial.write(line.as_bytes());
                }
                xtask::sleep_ms(1); // 1 tick = 1.125ms(96MHz 偏差,见头部注释)
            }
        });

    // 裸眼观测:绿 LED 500ms 翻转——"USB 轮询在跑 + 系统没死"的物理证据
    let mut green = green;
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
