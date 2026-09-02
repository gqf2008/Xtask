#!/usr/bin/env bash
# R5 口执行级门禁:补丁版 QEMU 跑 qemu_arm_r52_pingpong(200 轮乒乓)与
# qemu_arm_r52_fp(VFP 帧跨切换验证),全部 PASS 才退出 0。
# 用法:
#   QEMU_R52_BIN=/path/to/qemu-system-aarch64 bash ci/check_r52.sh
# 未给 QEMU_R52_BIN 时按常见路径探测(见下);CI 里由 workflow 先构建
# 补丁版 QEMU(ci/qemu-gic-r52.patch 打到 qemu-11.1.0 源码)再传入。
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
TARGET=armv7r-none-eabi
FEATURES="qemu_arm_r52,stdout_log,xtask_executor,xtask_scheduler"

# ---- 定位补丁版 QEMU(上游版 GIC CPU 接口地址重叠,IRQ 永不投递) ----
QEMU_BIN="${QEMU_R52_BIN:-}"
if [ -z "$QEMU_BIN" ]; then
    for cand in \
        "$ROOT_DIR/target/qemu-r52/bin/qemu-system-aarch64" \
        "$ROOT_DIR/qemu-r52/qemu-system-aarch64" \
        /tmp/qemu-r52/qemu-system-aarch64; do
        if [ -x "$cand" ]; then QEMU_BIN="$cand"; break; fi
    done
fi
if [ -z "$QEMU_BIN" ] || [ ! -f "$QEMU_BIN" ]; then
    echo "错误: 找不到补丁版 QEMU。请先构建(ci/qemu-gic-r52.patch 打到" \
         "qemu-11.1.0)并用 QEMU_R52_BIN 传入。" >&2
    exit 1
fi
echo "== 补丁版 QEMU: $QEMU_BIN"
# 可执行性:Git Bash 下 -x 对 Windows .exe 判定不稳,用 -f + 直接调用验证。
# Windows 上手编 QEMU 的 DLL 依赖官方安装目录(或 msys2),尝试补 PATH
case "$(uname -s)" in
    MINGW*|MSYS*|CYGWIN*)
        for d in /c/Program\ Files/qemu /d/toolchains/msys64/mingw64/bin; do
            [ -d "$d" ] && PATH="$d:$PATH"
        done
        ;;
esac
"$QEMU_BIN" --version >/dev/null 2>&1 \
    || { echo "错误: QEMU 不可执行: $QEMU_BIN" >&2; exit 1; }

# ---- 构建 R5 例程 ----
echo "== 构建 qemu_arm_r52_pingpong / qemu_arm_r52_fp"
cargo build --manifest-path "$ROOT_DIR/Cargo.toml" \
    --example qemu_arm_r52_pingpong --no-default-features \
    --features "$FEATURES" --target "$TARGET" --release
cargo build --manifest-path "$ROOT_DIR/Cargo.toml" \
    --example qemu_arm_r52_fp --no-default-features \
    --features "$FEATURES" --target "$TARGET" --release

ELF="$ROOT_DIR/target/$TARGET/release/examples"
QEMU_ARGS=(-M xlnx-zcu102 -smp 5 -m 1G -nographic
    -global xlnx-zynqmp.boot-cpu=rpu-cpu[0]
    -semihosting-config enable=on,target=native)

# ---- 门禁 1:200 轮乒乓 + tick 心跳 + 自退出 ----
echo "== 运行 qemu_arm_r52_pingpong"
P1=$("$QEMU_BIN" "${QEMU_ARGS[@]}" -device loader,file="$ELF/qemu_arm_r52_pingpong" 2>&1)
RC1=$?
echo "$P1" | grep -q "PASS: ping-pong done" \
    || { echo "pingpong 未达 PASS;输出尾部:"; echo "$P1" | tail -5; exit 1; }
echo "$P1" | grep -qE "tick [0-9]+" \
    || { echo "pingpong 未见 tick 心跳;输出尾部:"; echo "$P1" | tail -5; exit 1; }
[ "$RC1" -eq 0 ] || { echo "pingpong 退出码非 0: $RC1"; exit 1; }
echo "qemu_arm_r52_pingpong: PASS(200 轮乒乓 + tick 心跳)"

# ---- 门禁 2:VFP 帧跨切换验证 ----
echo "== 运行 qemu_arm_r52_fp"
P2=$("$QEMU_BIN" "${QEMU_ARGS[@]}" -device loader,file="$ELF/qemu_arm_r52_fp" 2>&1)
RC2=$?
echo "$P2" | grep -q "fpA: 100 rounds s3 kept" \
    || { echo "fp 未达 PASS;输出尾部:"; echo "$P2" | tail -5; exit 1; }
[ "$RC2" -eq 0 ] || { echo "fp 退出码非 0: $RC2"; exit 1; }
echo "qemu_arm_r52_fp: PASS(VFP 状态 100 轮跨切换保持)"

echo "== R5 门禁全部通过 =="
