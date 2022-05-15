//! riscv指令集架构，重新导出
//! https://github.com/rust-embedded/riscv-rt
//! https://github.com/rust-embedded/riscv

pub use riscv::*;
pub use riscv_rt as rt;

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
