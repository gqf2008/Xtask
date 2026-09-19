//! riscv指令集架构，重新导出
//! https://github.com/rust-embedded/riscv-rt
//! https://github.com/rust-embedded/riscv

pub use riscv::*;
pub use riscv_rt as rt;

/// `critical-section` 的 RISC-V 实现:所有 riscv32 目标都要提供——详见本文件内注释。
///
/// 门控**不能**写成 `not(target_has_atomic = "ptr")`:`atomic-polyfill` 的表是
/// (≤32 位/指针档, u64/i64 档) 二元组,`riscv32imac-*` 是 `(Native, Polyfill)`
/// ——有 A 扩展的口上 `AtomicU64`/`AtomicI64` 仍走 critical-section
/// (2026-09-19 实测:imac 用一次 `AtomicU64::fetch_add` 即链接缺符号)。
#[cfg(any(feature = "esp32c3", target_arch = "riscv32"))]
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
