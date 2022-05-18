mod port;
use super::{
    CPU_CLOCK_HZ, NVIC_INT_CTL_ADDR, NVIC_SYSTICK_CTL_ADDR, NVIC_SYSTICK_LOAD_ADDR,
    SYSTICK_CLOCK_HZ, TICK_CLOCK_HZ,
};
use crate::arch::cortex_m::delay::Delay;
use crate::arch::cortex_m::interrupt::Nr;
use crate::arch::cortex_m::peripheral::syst::SystClkSource;
use crate::arch::cortex_m::peripheral::{NVIC, SYST};
use crate::port::Portable;
use crate::task::Task;
use crate::CriticalSection;
use core::arch::asm;

pub struct STM32F4Porting;

unsafe fn setup_intrrupt() {
    let cp = crate::arch::cortex_m::Peripherals::take().unwrap();
    let mut syst = cp.SYST;
    syst.enable_interrupt();
    syst.enable_counter();
    syst.set_clock_source(SystClkSource::External);
    syst.set_reload(SYSTICK_CLOCK_HZ as u32 / TICK_CLOCK_HZ as u32 - 1);

    // let ptr = NVIC_SYSTICK_LOAD_ADDR as *mut u32;
    // ptr.write_volatile(SYSTICK_CLOCK_HZ as u32 / TICK_CLOCK_HZ as u32 - 1);
    // let ptr = NVIC_SYSTICK_CTL_ADDR as *mut u32;
    // ptr.write_volatile(1u32 << 2 | 1u32 << 1 | 1u32 << 0);
}
const SYST_COUNTER_MASK: u32 = 0x00ff_ffff;

const SYST_CSR_ENABLE: u32 = 1 << 0;
const SYST_CSR_TICKINT: u32 = 1 << 1;

trait Cm4Ext {
    fn enable_interrupt();
    fn disable_interrupt();
    fn enable_counter();
    fn disable_counter();
    fn clear_current();
    fn get_current() -> u32;
    fn get_reload() -> u32;
    fn set_reload(val: u32);
}

impl Cm4Ext for SYST {
    #[inline]
    fn enable_interrupt() {
        unsafe { (*Self::ptr()).csr.modify(|v| v | SYST_CSR_ENABLE) }
    }
    #[inline]
    fn disable_interrupt() {
        unsafe { (*Self::ptr()).csr.modify(|v| v | !SYST_CSR_TICKINT) }
    }
    #[inline]
    fn enable_counter() {
        unsafe { (*Self::ptr()).csr.modify(|v| v | SYST_CSR_ENABLE) }
    }

    #[inline]
    fn clear_current() {
        unsafe { (*Self::ptr()).cvr.write(0) }
    }

    #[inline]
    fn disable_counter() {
        unsafe { (*Self::ptr()).csr.modify(|v| v & !SYST_CSR_ENABLE) }
    }

    #[inline]
    fn get_current() -> u32 {
        unsafe { (*Self::ptr()).cvr.read() }
    }

    #[inline]
    fn get_reload() -> u32 {
        unsafe { (*Self::ptr()).rvr.read() }
    }
    #[inline]
    fn set_reload(value: u32) {
        unsafe { (*Self::ptr()).rvr.write(value) }
    }
}

impl Portable for STM32F4Porting {
    /// 完全内存屏障
    /// 保证在屏障之前的任何存储操作先于屏障之后的代码执行。
    fn barrier() {
        unimplemented!()
    }
    fn free<F, R>(f: F) -> R
    where
        F: FnOnce(&CriticalSection) -> R,
    {
        unsafe { crate::arch::cortex_m::interrupt::free(|_| f(&CriticalSection::new())) }
    }

    /// 开全局中断
    fn enable_interrupt() {
        unsafe { crate::arch::cortex_m::interrupt::enable() }
        unsafe {
            asm!(
                "
            cpsie i # 开中断
            cpsie f # 开异常
            dsb
            isb
            "
            );
        }
    }
    /// 关全局中断
    fn disable_interrupt() {
        // crate::arch::cortex_m::interrupt::disable()
        unsafe {
            asm!(
                "
        cpsid i # 关中断
        cpsid f # 关异常
        dsb
        isb
        "
            );
        }
    }
    /// 启动调度器
    fn start_scheduler() -> ! {
        //配置中断，这个函数就是定时中断和软中断使能

        //从任务栈恢复CPU状态，汇编实现
        unsafe {
            setup_intrrupt();
            asm!(
                "
        ldr r0, =0xE000ED08 // 向量表地址
        ldr r0, [r0]
        ldr r0, [r0]
        msr msp, r0
        cpsie i //使能全局中断
        cpsie f //使能全局异常
        dsb //数据同步
        isb //指令同步
        svc 11  //调用SVCall异常服务，在SVCall里恢复第一个任务
        nop
        nop
        "
            )
        };
        panic!("~!@#$%^&*()_");
    }
    /// 软中断
    fn irq() {
        let ptr = NVIC_INT_CTL_ADDR as *mut u32;
        unsafe {
            ptr.write_volatile(1 << 28);
            crate::arch::cortex_m::asm::dsb();
            crate::arch::cortex_m::asm::isb();
        }
    }

    fn disable_irq() {
        let ptr = NVIC_INT_CTL_ADDR as *mut u32;
        unsafe {
            ptr.write_volatile(1 << 27);
            crate::arch::cortex_m::asm::dsb();
            crate::arch::cortex_m::asm::isb();
        }
    }

    /// 获取rtc tick
    fn systick() -> u64 {
        unimplemented!()
    }
    /// 硬件延时，单位us
    fn delay_us(us: u64) {
        let clock = (us * (CPU_CLOCK_HZ as u64)) / 1_000_000;
        cortex_m::asm::delay(clock as u32);
    }
    /// 保存任务环境到任务栈
    fn save_context(_task: &mut Task) {
        unimplemented!()
    }
    /// 打印文本函数
    fn printf(_str: &str) {
        unimplemented!()
    }
}
