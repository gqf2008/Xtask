//! riscv指令集架构，重新导出
//! https://github.com/rust-embedded/riscv-rt
//! https://github.com/rust-embedded/riscv

pub use riscv::*;
pub use riscv_rt as rt;

/// `critical-section` 的 RISC-V 实现:无硬件原子的目标(riscv32imc/CH572、
/// esp32c3)靠它让 `atomic-polyfill` 的兜底路径链得上——详见本文件内注释。
#[cfg(any(feature = "esp32c3", not(target_has_atomic = "ptr")))]
mod critical;

/// 异常处理函数
#[allow(non_snake_case)]
#[no_mangle]
fn ExceptionHandler(_: &rt::TrapFrame) {
    let _cause = register::mcause::Exception::from(register::mcause::read().code() & 0xFFF);

    unsafe {
        asm::nop();
    }
}

/// 默认异常处理函数
#[allow(non_snake_case)]
#[no_mangle]
fn DefaultHandler() {
    let code = register::mcause::read().code() & 0xFFF;
    let _cause = register::mcause::Exception::from(code);

    unsafe {
        asm::wfi();
    }
}
