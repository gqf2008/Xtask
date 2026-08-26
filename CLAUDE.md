# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`xtask` is a `#![no_std]` preemptive multitasking RTOS-style kernel for embedded targets, modeled on FreeRTOS. It compiles together with the application. Single physical hardware thread: priority + time-slice scheduling, high-priority preemption, fair round-robin among equal priorities. The design/rationale (including the assembly context-switch code) is documented in `Xtask.md` — read it before touching scheduling or porting code.

Rust **nightly** is required (`rust-toolchain` pins `nightly`; `lib.rs` uses many `#![feature(...)]` gates). Pure-logic unit tests (semaphore, bus, delay queue, software timers) run on the host via a `cfg(test)` `HostPorting` mock: `cargo test --lib --target x86_64-pc-windows-msvc` (any host triple works; do not omit `--lib`, examples only build for embedded targets). Everything else is verified by building and flashing examples to real boards.

## Build & run

Everything is selected with a **chip feature** + a **target triple**. You must pass both; the kernel does not build without a chip feature. `build.rs` copies the matching `src/chip/<chip>/memory.x` linker script into `OUT_DIR`.

```bash
# General form
cargo run --example <EXAMPLE> --features <CHIP> --target <TRIPLE> --release

# gd32vf103 (longan-nano)
cargo run --example led --features gd32vf103 --target riscv32imac-unknown-none-elf --release
# stm32f4 (greenpill; note multitask.rs itself is a gd32vf103-only example)
cargo run --example multitask_greenpill --features stm32f4 --target thumbv7em-none-eabihf --release
# stm32f1 (bluepill)
cargo run --example multitask_bluepill --features stm32f1 --target thumbv7m-none-eabi --release
# qemu_riscv (QEMU virt machine — the only port that EXECUTES, not just links;
# run under QEMU and self-exits via SiFive test device):
qemu-system-riscv32 -M virt -nographic -bios none -kernel     target/riscv32imac-unknown-none-elf/release/examples/qemu_pingpong
# rp2040 (revived 2026-08-23 on rp2040-hal 0.9): the old 0.5.0 pin depended on a yanked
# critical-section 0.2.x; 0.9+ uses 1.x. The board BSP is the repo's own bsp_pins! macro.
cargo run --example multitask_rp_pico --features rp2040 --target thumbv6m-none-eabi --release
```

`--release` is normal for flashing (both profiles use `opt-level = "z"`, `lto = true`). Some examples need extra features, e.g. the software-timer example: `--features gd32vf103,timer`.

A default target (`thumbv7em-none-eabihf`) and per-target `runner` (probe-run / gdb+openocd / elf2uf2-rs) are set in `.cargo/config.toml`; `cargo run` uses the runner to flash. OpenOCD configs and GDB scripts live in `debug/<chip>/`. Chip HALs are mostly crates, but `gd32vf103xx-hal` is a local path dep at `hal2/gd32vf103xx-hal`.

Chip features: `gd32vf103`, `stm32f1`, `stm32f4`, `stm32h7`, `cm32m4`, `rp2040`, `ch32v103`/`ch32v203`/`ch32v307`, `esp32c3`, plus `qemu_riscv` (QEMU virt; **execution-verified** — `check.sh` step 4 runs it; others build-verified 2026-08-23; real-board verification pending for f4/f1 constants, h7 timeline, cm32m4/rp2040). Non-chip features: `timer` (software timers), `tlsf` (global allocator backend swap to the hand-written mini TLSF, ch28), `debug_task`, `fs` (fatfs), `net` (smoltcp), `usb`, `ble`, `rtt_log` / `stdout_log`, board BSPs (`longan_nano`, `bluepill`, `greenpill`, `rp_pico`).

## Architecture

Layered, bottom-up:

- **`src/port.rs` — the portability seam.** Trait `Portable` defines the ~10 primitives the kernel needs (memory barrier, critical section `free`, enable/disable interrupt, `start_scheduler`, software `irq`, `systick`, `delay_us`, `save_context`; plus six defaulted tickless methods — `tickless_supported`/`tickless_arm_delta`/`tickless_stop_timer`/`tickless_wait`, the leave-idle boundary hook `tickless_leave_idle` (measured catch-up + periodic restore, called by `do_schedule` when leaving idle), and `tickless_resume_periodic` (self-checking re-enable before the fallback spin), all default to constant-tick no-ops, so the 12 ports compile unchanged, ch29). The type alias `Porting` is bound to exactly one chip implementation via `#[cfg(feature = ...)]`. **All kernel code calls `Porting::...` — never a chip directly.** To port to a new chip: implement `Portable` + provide `memory.x` under `src/chip/<chip>/`, then add the feature wiring in `port.rs`, `chip/mod.rs`, `build.rs`, and `Cargo.toml`.
- **`src/chip/<chip>/`** — per-chip `Portable` impl. `port.rs` (interrupt/systick glue), `port.S` + `restore_ctx.S` (assembly context save/restore), `stdout.rs`, `memory.x`. The context switch is in assembly — read the in-code comments and `Xtask.md`.
- **`src/arch/`** — re-exports of the Rust embedded-wg runtime crates (`riscv-rt`, `cortex-m-rt`, etc.) chosen by `target_arch`. Examples get their `#[entry]` and interrupt macros from here.
- **`src/task/`** — the kernel core. `task.rs` defines `Task` (state machine: `Ready/Running/Suspended/Blocked/Terminated`), `TaskBuilder`/`spawn`, `sleep_ms`, `yield_now`. `task/scheduler.rs` defines the `Scheduler` trait and a global `schedulee`; `task/scheduler/xtask.rs` is the default scheduler. `task/executor.rs` (`xworker`) runs the current task. `task/scheduler/idle.rs` runs the idle task — under tickless (ch29) it replaces the hot-spin `loop {}` with a three-state engine (sleep-forever / sleep-until-nearest-deadline / process-now; the whole decide+act+wfi round runs in one critical section), one-shot `tickless_arm_delta`, and a measured `TICKS += el` jump in the tick ISR; gated on `Porting::tickless_supported() && !smp::enabled() && crate::tickless::enabled()` (runtime toggle `xtask::tickless::set_enabled`, defaults on). Tasks, stacks, and closures are heap-allocated then deliberately `mem::forget`-ed to escape ownership (freed manually on exit); each stack has a `STACK_FENCE` canary checked at context-switch time for overflow.
- **`src/sync/`** — IPC primitives built on the task state machine: `semaphore` (binary + counting), `queue` (MPMC), `broadcast`, `notify`, `mutex`, `reentrant_mutex` (recursive lock: gate semaphore + owner/depth ledger), `arc`, `free_queue`.
- **`src/bus.rs`** — PubSub message bus.
- **`src/timer.rs`** (`timer` feature) — software timers.
- **`src/allocator.rs`** — global heap allocator (`linked_list_allocator`); `pub mod allocator` re-exports the engine for examples — the `tlsf` feature swaps `XTaskAllocer`'s inner engine to the hand-written mini TLSF (`allocator/tlsf.rs`, ch28; `FirstFit` is the same-shape first-fit wrapper for A/B experiments); apps call `xtask::init_heap(start_addr, size)` before spawning. `src/logger.rs` — `log` facade over RTT or stdout.
- **`src/bsp/<board>/`** — board support (LED, stdout/UART, LCD, sensors). **`src/fs` / `src/net`** — optional fatfs / smoltcp integration.

`src/prelude.rs` re-exports the public API (`spawn`, `TaskBuilder`, `sleep_ms`, `yield_now`, sync primitives, `start`, `sprint/sprintln`, etc.) — examples and apps `use xtask::prelude::*`.

### Typical app flow (see `examples/led.rs`)

1. `init()`: `xtask::init_heap(_sheap, size)` → configure clocks/GPIO/UART → spawn tasks with `TaskBuilder::new().name(..).priority(..).stack_size(..).spawn(closure)`.
2. `xtask::start()` → starts the (optional) timer task, then `schedulee.start()` → `Porting::start_scheduler()` never returns.

## Conventions

- Comments and docs are in Chinese; keep new comments in Chinese to match.
- Cross-chip changes must go through `Portable`/`Porting`; keep chip-specific `#[cfg]` out of the generic kernel.
- Priorities run 1–16 and **smaller number = higher priority** (Q1 is highest; the idle task is 16). `do_systick` preempts when a ready task's priority number is `<=` the current task's. `priority(0)` is invalid (`TaskBuilder` asserts `> 0`). Default stack size is 256 words; assert requires `> 64`.
