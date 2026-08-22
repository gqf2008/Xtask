#![no_std]
#![no_main]

extern crate alloc;

use alloc::boxed::Box;
use gd32vf103xx_hal as hal;
use hal::{gpio::GpioExt, pac, prelude::*, rcu::RcuExt};

use xtask::arch::riscv::rt;
use xtask::bsp::longan_nano::drv_led::DrvLed;
use xtask::bsp::longan_nano::drv_uart::uart0_isr;
use xtask::bsp::longan_nano::led::{rgb, Led};
use xtask::prelude::*;

// 驱动抽象层示例：注册→查找→使用 全流程
// - LED：DrvLed 设备按名注册（"red"/"green"/"blue"），任务通过 find_led 拿到驱动
// - UART0：中断驱动接收 + 环形缓冲，echo 任务 read_byte 挂进状态机（等待即 Blocked），
//   终端键入什么回什么，每收一字节蓝灯翻转一下；绿灯 500ms 常亮常灭
// - 不配置 stdout：USART0 已被 Uart0 占用（日志走默认 RTT）

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

    let mut afio = dp.AFIO.constrain(&mut rcu);

    // 注册：把设备泄漏成 'static 后按名登记（注册表不拥有设备，只登记"谁是谁"）。
    // 静态名字 + 空注册表，注册失败只可能是名字撞车——启动期直接暴露。
    register_led("red", Box::leak(Box::new(DrvLed::new(red)))).expect("red 注册失败");
    register_led("green", Box::leak(Box::new(DrvLed::new(green)))).expect("green 注册失败");
    register_led("blue", Box::leak(Box::new(DrvLed::new(blue)))).expect("blue 注册失败");

    // Uart0::new 内部"先存转发锚点、最后开中断"，返回 'static 设备句柄
    let uart0 = xtask::bsp::longan_nano::drv_uart::Uart0::new(
        dp.USART0,
        gpioa.pa9,
        gpioa.pa10,
        57600.bps(), // 这块板子 PCB 设计有瑕疵，uart 速率只能到 57600
        &mut afio,
        &mut rcu,
    );
    register_uart("uart0", uart0).unwrap();

    log::info!("driver: led x3 + uart0 registered");
    // 开机先亮一下红灯：调度器启动前的 find→use 也走通
    find_led("red").unwrap().on();
}

#[rt::entry]
fn main() -> ! {
    init();

    // 绿灯 500ms 翻转：应用不认识具体 LED，只按名字拿驱动
    TaskBuilder::new()
        .name("blink")
        .priority(3)
        .spawn(|| loop {
            find_led("green").unwrap().toggle();
            xtask::sleep_ms(500);
        });

    // echo：read_byte 没字节时任务挂起（Blocked），UART 中断来了才被唤醒
    TaskBuilder::new()
        .name("echo")
        .priority(2)
        .spawn(|| loop {
            let b = find_uart("uart0").unwrap().read_byte();
            find_uart("uart0").unwrap().write_all(&[b]);
            find_led("blue").unwrap().toggle();
        });

    xtask::start()
}
