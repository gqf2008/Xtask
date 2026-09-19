//! `critical-section` 1.x 的 RISC-V 裸机实现——本仓 RISC-V 侧**唯一一份**。
//!
//! # 谁需要它
//!
//! - **无硬件原子**的 RISC-V 目标:`atomic-polyfill` 的 `build.rs` 对
//!   `riscv32*`(除 `riscv32imac-*` / `riscv32gc-*` / `*-espidf`)一律启用
//!   polyfill —— 用 `critical-section` 把读改写包起来。缺这份实现时
//!   **编译能过、链接报** `undefined symbol: _critical_section_1_0_acquire`
//!   (2026-09-19 在 `riscv32imc-unknown-none-elf` 上实测复现;
//!   CH572/无 A 档位、以及 esp32c3 真身都是这一类目标)。
//! - **esp32c3**:其 PAC 自身引用 `critical-section`,故该 feature 下无论目标
//!   有没有 A 扩展都要提供(ARM 侧不走这里,由 `cortex-m` 的
//!   `critical-section-single-core` feature 提供)。
//!
//! # 语义
//!
//! 存 `mstatus.MIE` → 关中断 → 临界区 → 按旧值恢复,与 `Porting::free` 同款。
//! **单核假设**:多核口必须换成自旋锁版本,只关本核中断挡不住别核。
//! 有 A 扩展的 RISC-V 口不需要它(`atomic-polyfill` 直接复用 `core::sync::atomic`)。

/// 进临界区:返回"进入前中断是否开着"
#[no_mangle]
unsafe extern "C" fn _critical_section_1_0_acquire() -> bool {
    let was_enabled = riscv::register::mstatus::read().mie();
    riscv::interrupt::disable();
    was_enabled
}

/// 出临界区:按进入前的状态恢复
#[no_mangle]
unsafe extern "C" fn _critical_section_1_0_release(was_enabled: bool) {
    if was_enabled {
        riscv::interrupt::enable();
    }
}
