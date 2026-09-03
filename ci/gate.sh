#!/usr/bin/env bash
# 内核门禁:host 回归测试 + 示例链接(弱符号 ISR 绑定)+ QEMU 执行级验证。
# 书稿(book/)已随书稿私有仓库 xtask-book 独立,本仓库仅保留内核自身门禁。
# 用法:bash ci/gate.sh   (Windows 也可用 Git Bash 执行)
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
# 全脚本统一构建目录 = 主 target:与各步硬编码的 ELF 探测路径始终一致。
export CARGO_TARGET_DIR="$ROOT_DIR/target"

# --- 预检:riscv32imac target(读者环境通常未装)---
if command -v rustup >/dev/null 2>&1     && ! rustup target list --installed | grep -q '^riscv32imac-unknown-none-elf$'; then
    echo "== 安装缺失的 riscv32imac-unknown-none-elf target =="
    rustup target add riscv32imac-unknown-none-elf
fi

echo "== [1/3] 内核 host 回归测试(阳性对照守卫)=="
HOST_TRIPLE="$(rustc -vV | sed -n "s/^host: //p")"
cargo test --manifest-path "$ROOT_DIR/Cargo.toml" --lib --target "$HOST_TRIPLE" \
    --no-default-features --features xtask_executor,xtask_scheduler,timer,fs,net,usb,ble

echo "== [2/3] 示例链接(弱符号 ISR 绑定守卫 + fatfs 全链链接)=="
# 用 build 不用 check:ch20 的 USART0 中断向量是 port.S 里的 .weak 符号,
# 应用层 #[no_mangle] 定义后由链接器绑定——只有链接才能验证机制生效
DRIVER_ELF="$ROOT_DIR/target/riscv32imac-unknown-none-elf/release/examples/driver"
cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --example driver \
    --features gd32vf103 --target riscv32imac-unknown-none-elf --release
if command -v python3 >/dev/null 2>&1 || command -v python >/dev/null 2>&1; then
    PY="$(command -v python3 || command -v python)"
    "$PY" "$ROOT_DIR/ci/check_isr_vec.py" "$DRIVER_ELF"
else
    echo "python 未安装,跳过向量表绑定校验(仅保留链接门禁)"
fi
# fatfs/net_echo/usb_cdc/ble_gatt:各协议栈全链(驱动层→适配器→栈→任务)的链接门禁,
# .cargo/config.toml 的 runner 是 per-target,这里只 build 不 run
cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --example fatfs \
    --features gd32vf103,fs --target riscv32imac-unknown-none-elf --release
cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --example net_echo \
    --features gd32vf103,net --target riscv32imac-unknown-none-elf --release
cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --example usb_cdc \
    --features gd32vf103,usb --target riscv32imac-unknown-none-elf --release
cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --example ble_gatt \
    --features gd32vf103,ble --target riscv32imac-unknown-none-elf --release

echo "== [3/3] QEMU 执行门禁(virt 机跑真内核——调度/切换/节拍执行级验证)=="
QEMU_BIN="$(command -v qemu-system-riscv32 || true)"
if [ -z "$QEMU_BIN" ] && [ -x "/c/Program Files/QEMU/qemu-system-riscv32.exe" ]; then
    QEMU_BIN="/c/Program Files/QEMU/qemu-system-riscv32.exe"
fi
# 仓库自带 xPack 静态版(.tools/,由 ci/get-qemu.sh 自动引导,读者零安装)
if [ -z "$QEMU_BIN" ] && [ -x "$ROOT_DIR/.tools/qemu/bin/qemu-system-riscv32" ]; then
    QEMU_BIN="$ROOT_DIR/.tools/qemu/bin/qemu-system-riscv32"
fi
if [ -z "$QEMU_BIN" ] && [ -x "$ROOT_DIR/.tools/qemu/bin/qemu-system-riscv32.exe" ]; then
    QEMU_BIN="$ROOT_DIR/.tools/qemu/bin/qemu-system-riscv32.exe"
fi
# 都没有:自动下载(约 90MB,仅首次);失败给出读者通道提示
if [ -z "$QEMU_BIN" ]; then
    echo "== 未找到 qemu-system-riscv32,尝试自动引导(ci/get-qemu.sh) =="
    if bash "$ROOT_DIR/ci/get-qemu.sh"; then
        if [ -x "$ROOT_DIR/.tools/qemu/bin/qemu-system-riscv32.exe" ]; then
            QEMU_BIN="$ROOT_DIR/.tools/qemu/bin/qemu-system-riscv32.exe"
        else
            QEMU_BIN="$ROOT_DIR/.tools/qemu/bin/qemu-system-riscv32"
        fi
    else
        echo "自动引导失败。可选通道:" >&2
        echo "  1. 手动安装 QEMU 后重跑" >&2
        echo "  2. GitHub Codespaces(.devcontainer.json,打开即跑)" >&2
        echo "  3. 直接查看 CI 门禁结果: https://github.com/gqf2008/Xtask/actions" >&2
        exit 1
    fi
fi
if [ -n "$QEMU_BIN" ]; then
    # qemu_pingpong:两任务 200 轮乒乓 + tick 心跳;跑满写 SiFive test 自退出
    QEMU_ELF="$ROOT_DIR/target/riscv32imac-unknown-none-elf/release/examples/qemu_pingpong"
    cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --example qemu_pingpong \
        --features qemu_riscv --target riscv32imac-unknown-none-elf --release
    QEMU_OUT="$( "$QEMU_BIN" -M virt -nographic -bios none -kernel "$QEMU_ELF" </dev/null 2>&1 || true )"
    echo "$QEMU_OUT" | grep -q "PASS: 200 rounds ping-pong" \
        || { echo "qemu_pingpong 未达 PASS;输出尾部:"; echo "$QEMU_OUT" | tail -5; exit 1; }
    # 计数剥 \r(CI 的 QEMU 输出行尾可能带 CR,^A$ 不匹配)且 || true
    # 防 grep -c 零匹配的退出码在 set -e 下直接炸掉
    A_CNT="$(echo "$QEMU_OUT" | tr -d '\r' | grep -c '^A$' || true)"
    B_CNT="$(echo "$QEMU_OUT" | tr -d '\r' | grep -c '^B$' || true)"
    [ "$A_CNT" -eq 200 ] && [ "$B_CNT" -eq 200 ] \
        || { echo "乒乓计数异常 A=$A_CNT B=$B_CNT(应各 200)"; exit 1; }
    echo "qemu_pingpong: PASS(A×$A_CNT B×$B_CNT,调度/切换/节拍执行级验证)"
    # qemu_kernel_tests:24 项内核全功能自测;测试 22/23/24 需 python 经
    # stdin 喂字节(读内核串口握手标记,零时序假设),缺失则无法执行
    PYF="$(command -v python3 || command -v python || true)"
    if [ -z "$PYF" ]; then
        echo "python 未安装——qemu_kernel_tests 的 UART RX 喂字节握手无法执行;安装 python3 后重跑"
        exit 1
    fi
    KTESTS_ELF="$ROOT_DIR/target/riscv32imac-unknown-none-elf/release/examples/qemu_kernel_tests"
    cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --example qemu_kernel_tests \
        --features qemu_riscv,timer --target riscv32imac-unknown-none-elf --release
    KOUT="$( "$PYF" "$ROOT_DIR/ci/feed_qemu.py" 180 "$QEMU_BIN" "$KTESTS_ELF" | grep -v "GLib-GIO" || true )"
    echo "$KOUT" | grep -q "24/24 passed" \
        || { echo "qemu_kernel_tests 未全过;输出:"; echo "$KOUT" | tail -15; exit 1; }
    echo "$KOUT" | grep -q "byte B @ T23-SLEEPING" \
        || { echo "qemu_kernel_tests 未握手到早醒弹字节(test 23 未喂到);输出:"; echo "$KOUT" | tail -8; exit 1; }
    echo "$KOUT" | grep -q "byte N5 @ T24-SLEEPING" \
        || { echo "qemu_kernel_tests 未握手到噪声风暴第 5 字节(test 24 未喂全);输出:"; echo "$KOUT" | tail -8; exit 1; }
    echo "qemu_kernel_tests: 24/24 passed(全内核机制执行级验证)"
    # -smp 2 起跑:hart1 由 riscv-rt 默认停泊,hart0-only 语义应逐字不变
    SMP_OUT="$( "$PYF" "$ROOT_DIR/ci/feed_qemu.py" 180 "$QEMU_BIN" "$KTESTS_ELF" -smp 2 | grep -v "GLib-GIO" || true )"
    echo "$SMP_OUT" | grep -q "24/24 passed" \
        || { echo "qemu_kernel_tests -smp 2 未全过(hart1 停泊失效?);输出:"; echo "$SMP_OUT" | tail -15; exit 1; }
    echo "$SMP_OUT" | grep -q "boot hart: 0" \
        || { echo "-smp 2 下启动核不是 hart0;输出:"; echo "$SMP_OUT" | tail -5; exit 1; }
    echo "qemu_kernel_tests -smp 2: 24/24 passed(双核起跑,hart1 停泊,hart0-only 语义不变)"
    # TLSF 全局后端:整个套件换 tlsf feature 重跑,24/24 不变即后端透明
    cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --example qemu_kernel_tests \
        --features qemu_riscv,timer,tlsf --target riscv32imac-unknown-none-elf --release
    TLSF_OUT="$( "$PYF" "$ROOT_DIR/ci/feed_qemu.py" 180 "$QEMU_BIN" "$KTESTS_ELF" | grep -v "GLib-GIO" || true )"
    echo "$TLSF_OUT" | grep -q "24/24 passed" \
        || { echo "qemu_kernel_tests(tlsf 全局后端)未全过;输出:"; echo "$TLSF_OUT" | tail -15; exit 1; }
    echo "qemu_kernel_tests(tlsf 全局后端): 24/24 passed(分配器换引擎对内核透明)"
    # SMP 双核执行门禁:应用显式 smp::enable() 后 hart1 真正参与调度
    SMP_ELF="$ROOT_DIR/target/riscv32imac-unknown-none-elf/release/examples/qemu_smp"
    cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --example qemu_smp \
        --features qemu_riscv,timer --target riscv32imac-unknown-none-elf --release
    SMP2_OUT="$( "$QEMU_BIN" -M virt -smp 2 -nographic -bios none -kernel "$SMP_ELF" </dev/null 2>&1 | grep -v "GLib-GIO" || true )"
    echo "$SMP2_OUT" | grep -q "smp PASS: 9/9" \
        || { echo "qemu_smp 未全过;输出:"; echo "$SMP2_OUT" | tail -15; exit 1; }
    echo "$SMP2_OUT" | grep -q "harts online: 2" \
        || { echo "qemu_smp 未识别双核;输出:"; echo "$SMP2_OUT" | tail -5; exit 1; }
    echo "qemu_smp -smp 2: 9/9 passed(双核调度/跨核 IPI/锁堆压力/绑核/定时器跨核 执行级验证)"
else
    echo "qemu-system-riscv32 未安装,跳过执行门禁(安装:winget install SoftwareFreedomConservancy.QEMU)"
fi

echo "== 全部通过 =="
