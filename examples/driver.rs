#![no_std]
#![no_main]

extern crate alloc;

use alloc::boxed::Box;
use gd32vf103xx_hal as hal;
use hal::{gpio::GpioExt, pac, prelude::*, rcu::RcuExt};

use xtask::arch::riscv::rt;
use xtask::bsp::longan_nano::drv_led::{DrvLed, LED_ON, LED_TOGGLE};
use xtask::bsp::longan_nano::drv_uart::uart0_isr;
use xtask::bsp::longan_nano::led::{rgb, Led};
use xtask::device::table::{self, DeviceSlot};
use xtask::prelude::*;

// 驱动抽象层示例：注册→查找→使用 全流程
// - 表只认识 `&'static dyn Device`；能力分发走 `as_*` 查询——LED 是纯控制设备
//   （Control），UART 是流设备（StreamDevice）+ 事件能力（EventDevice）
// - LED：编译期设备清单（device::table）——"red"/"green"/"blue" 的名字集合在编译期
//   定死（const 数组），实例在 init 里构造后填进静态槽（RAM）；清单里有名而槽未填 =
//   未就绪（find 返回 None，对应 Zephyr device_is_ready 语义）
// - UART0：运行期注册表（device::register）——实例是运行期独占构造（Serial::new），
//   按名即挂即查。两种形态共用同一种设备句柄，只换"表"的实现
// - UART0：中断驱动接收 + 环形缓冲；echo 任务用内核通用适配器 read_blocking
//   （StreamDevice + EventDevice 组合，等待即 Blocked），终端键入什么回什么，
//   每收一字节蓝灯翻转一下；绿灯 500ms 常亮常灭
// - 不配置 stdout：USART0 已被 Uart0 占用（日志走默认 RTT）

// 编译期设备清单：名字 → 槽（槽在下面声明为静态，init 里 fill）
xtask::device_list! { BOARD_LEDS {
    "red" => &RED_SLOT,
    "green" => &GREEN_SLOT,
    "blue" => &BLUE_SLOT,
} }

static RED_SLOT: DeviceSlot = DeviceSlot::new();
static GREEN_SLOT: DeviceSlot = DeviceSlot::new();
static BLUE_SLOT: DeviceSlot = DeviceSlot::new();

// USART0 中断向量：port.S 的 vectors 表（#56）里的 weak 符号，
// 应用层定义同名符号后由链接器绑定进向量表
#[no_mangle]
extern "C" fn USART0() {
    uart0_isr();
}

fn init() {
    extern "C" {
        /// 堆内存开始地址，在 riscv-rt link.x 文件里定义
        static _sheap: u8;
    }
    // 堆必须先于一切 Box::leak（设备实例靠堆泄漏成 'static）
    let start_addr = unsafe { &_sheap as *const u8 as usize };
    xtask::init_heap(start_addr, 32 * 1024);
    xtask::init_logger();

    let dp = pac::Peripherals::take().unwrap();
    // 配置时钟
    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();

    let gpioa = dp.GPIOA.split(&mut rcu);
    let gpioc = dp.GPIOC.split(&mut rcu);

    let (mut red, mut green, mut blue) = rgb(gpioc.pc13, gpioa.pa1, gpioa.pa2);
    red.off();
    green.off();
    blue.off();

    // 清单三件套：构造实例 → 填槽（每槽一次，重复填会 panic）→ attach 清单。
    // 清单本体（const 数组）在 ROM，槽（一个 Option<&'static dyn Device>）在 RAM——
    // 对应 Zephyr 的 const struct device 数组 + RAM 里的 device_state。
    RED_SLOT.fill(Box::leak(Box::new(DrvLed::new(red))) as &'static dyn Device);
    GREEN_SLOT.fill(Box::leak(Box::new(DrvLed::new(green))) as &'static dyn Device);
    BLUE_SLOT.fill(Box::leak(Box::new(DrvLed::new(blue))) as &'static dyn Device);
    table::attach(BOARD_LEDS);

    let mut afio = dp.AFIO.constrain(&mut rcu);

    // Uart0::new 内部"先存转发锚点、最后开中断"，返回 'static 设备句柄
    let uart0 = xtask::bsp::longan_nano::drv_uart::Uart0::new(
        dp.USART0,
        gpioa.pa9,
        gpioa.pa10,
        57600.bps(), // 这块板子 PCB 设计有瑕疵，uart 速率只能到 57600
        &mut afio,
        &mut rcu,
    );

    // 注册：把设备按名登记（注册表不拥有设备，只登记"谁是谁"；句柄已是 'static）。
    // 静态名字 + 空注册表，注册失败只可能是名字撞车——启动期直接暴露。
    register("uart0", uart0).unwrap();

    log::info!("driver: leds via device list + uart0 via registry");
    // 开机先亮一下红灯：调度器启动前的 find→use 也走通
    table::find_control("red")
        .unwrap()
        .control(LED_ON, 0)
        .unwrap();
}

#[rt::entry]
fn main() -> ! {
    init();

    // 绿灯 500ms 翻转：应用不认识具体 LED，只按名字拿控制面发命令
    TaskBuilder::new().name("blink").priority(3).spawn(|| loop {
        table::find_control("green")
            .unwrap()
            .control(LED_TOGGLE, 0)
            .unwrap();
        xtask::sleep_ms(500);
    });

    // echo：read_blocking 没字节时任务挂起（Blocked），UART 中断来了才被唤醒
    TaskBuilder::new().name("echo").priority(2).spawn(|| loop {
        let dev = find("uart0").unwrap();
        let mut b = [0u8; 1];
        read_blocking(dev, &mut b).unwrap();
        find_stream("uart0").unwrap().write(&b).unwrap();
        table::find_control("blue")
            .unwrap()
            .control(LED_TOGGLE, 0)
            .unwrap();
    });

    xtask::start()
}
