#!/usr/bin/env bash
# 内核门禁:host 回归测试 + 示例链接(弱符号 ISR 绑定)+ QEMU 执行级验证。
# 书稿(book/)已随书稿私有仓库 xtask-book 独立,本仓库仅保留内核自身门禁。
# 用法:bash ci/gate.sh   (Windows 也可用 Git Bash 执行)
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
# 全脚本统一构建目录 = 主 target:与各步硬编码的 ELF 探测路径始终一致。
# 外部目标盘可 export CARGO_TARGET_DIR 覆盖(内盘吃紧的机器必须覆盖:门禁产物几个 GB);
# 下面所有 ELF 探测路径都用 TARGET_DIR,别再写死 $ROOT_DIR/target。
TARGET_DIR="${CARGO_TARGET_DIR:-$ROOT_DIR/target}"
export CARGO_TARGET_DIR="$TARGET_DIR"

# --- 预检:门禁要用的 riscv target(读者环境通常未装)---
# 与下面的步骤一一对应:漏一个,读者跑"一条命令复现"时就会在编译期撞 E0463
# (本机/CI 恰好装过时不现形,只有冷环境才露)。
if command -v rustup >/dev/null 2>&1; then
    for t in riscv32imac-unknown-none-elf riscv32imc-unknown-none-elf; do
        if ! rustup target list --installed | grep -q "^${t}$"; then
            echo "== 安装缺失的 ${t} target =="
            rustup target add "$t"
        fi
    done
fi

echo "== [1/4] 内核 host 回归测试(阳性对照守卫)=="
HOST_TRIPLE="$(rustc -vV | sed -n "s/^host: //p")"
cargo test --manifest-path "$ROOT_DIR/Cargo.toml" --lib --target "$HOST_TRIPLE" \
    --no-default-features --features xtask_executor,xtask_scheduler,timer,fs,net,usb,ble

echo "== [2/4] 示例链接(弱符号 ISR 绑定守卫 + fatfs 全链链接)=="
# 用 build 不用 check:ch20 的 USART0 中断向量是 port.S 里的 .weak 符号,
# 应用层 #[no_mangle] 定义后由链接器绑定——只有链接才能验证机制生效
DRIVER_ELF="$TARGET_DIR/riscv32imac-unknown-none-elf/release/examples/driver"
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
# qemu_pingpong:第 4 步要跑它,但第 3 步的体积门禁先要对它比基线——冷目录
# (CARGO_TARGET_DIR 指向空目录、CI 首次构建)时若等到第 4 步才构建,第 3 步会
# 报"找不到产物"。构建前置到本步,让门禁与构建顺序无关。
cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --example qemu_pingpong \
    --features qemu_riscv --target riscv32imac-unknown-none-elf --release
# CH583(QingKe V4A)零 PAC 口:QEMU 没有对应的机器可跑(见第 3 步),门禁做
# 构建级 + 启动头产物级终检——WCH ROM 按向量表第 5 字 boot option 0xF3F9BDA9
# 识别有效用户程序,缺了它板上停在 ISP 而构建/门禁全绿(见 check_wch_boot.py)
cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --example multitask_ch583 \
    --features ch583,timer --target riscv32imac-unknown-none-elf --release
CH583_ELF="$TARGET_DIR/riscv32imac-unknown-none-elf/release/examples/multitask_ch583"
if [ -n "${PY:-}" ]; then
    "$PY" "$ROOT_DIR/ci/check_wch_boot.py" "$CH583_ELF"
else
    echo "python 未安装,跳过 CH583 启动头校验(仅保留构建门禁)"
fi
# 无 A 扩展目标(riscv32imc;CH572/青稞无 A 档位、esp32c3 真身):同一示例必须
# 也能编译+链接——内核的 Arc/信号量原子走 atomic-polyfill 的 critical-section
# 兜底(src/arch/riscv/critical.rs),缺那份实现时编译能过、链接缺符号
cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --example multitask_ch583 \
    --features ch583,timer --target riscv32imc-unknown-none-elf --release
IMC_ELF="$TARGET_DIR/riscv32imc-unknown-none-elf/release/examples/multitask_ch583"
if [ -n "${PY:-}" ]; then
    # 启动头布局与目标 ISA 无关,同一脚本顺带钉住 imc 产物
    "$PY" "$ROOT_DIR/ci/check_wch_boot.py" "$IMC_ELF"
fi
# CH572(QingKe **无 A 扩展**,12K SRAM):与 ch583 共用同一套启动头布局
# (官方 Link.ld 现场链接实测:魔数同样落在 flash 0x14),但目标是 riscv32imc;
# 示例按 12K 的实测内存账缩编(idle/软件定时器任务栈在 env.rs 里压到 256 字)
cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --example multitask_ch572 \
    --features ch572 --target riscv32imc-unknown-none-elf --release
CH572_ELF="$TARGET_DIR/riscv32imc-unknown-none-elf/release/examples/multitask_ch572"
if [ -n "${PY:-}" ]; then
    "$PY" "$ROOT_DIR/ci/check_wch_boot.py" "$CH572_ELF"
fi
# ESP32-C3 真身是 **RV32IMC**(无 A 扩展):必须按 riscv32imc 构建——用 riscv32imac
# 编出的同名产物实测含 12 条 A 扩展指令(amo/lr/sc,来自 Arc/信号量的读改写),
# 真芯片执行到就是非法指令;按真 ISA 构建后为 0 条(原子走 critical-section 垫片)
cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --example multitask_esp32c3 \
    --features esp32c3,timer --target riscv32imc-unknown-none-elf --release
# 上板自检例程(ch57x 家族共用,依赖 src/bsp/ch57x 的轮询控制台):
# 把 issue #14 里"只能上板回答"的核对点收敛成一次烧写——先扫主频假设,
# 再打印启动头/STK/PFIC/INTSYSCR 回读与 SWI pending 归宿,最后两任务活性测试
cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --example board_check_ch57x \
    --features ch583,timer --target riscv32imac-unknown-none-elf --release
cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --example board_check_ch57x \
    --features ch572 --target riscv32imc-unknown-none-elf --release

echo "== [3/4] 体积门禁(flash/RAM 回归守卫)=="
# 定位:设备的存储有限,抽象层/依赖只要默默胖起来,板子就烧不下。
# 口径:ci/check_size.py 直接解 ELF 段表(不依赖 size/objdump 的具体实现,
#       arm-none-eabi-size 读不了 riscv 产物);flash = 所有可载入段(含 .data 的
#       初始化镜像),RAM = .data+.bss(链接脚本预留的 .stack 单独报出,不算门禁——
#       否则调栈会误报)。
# 阈值:flash +5%/+256B、RAM +4%/+128B;真正的功能增量才更新基线:
#       python3 ci/check_size.py <名字> <elf> --update
SIZE_PY="${PY:-$(command -v python3 || command -v python || true)}"
if [ -z "$SIZE_PY" ]; then
    echo "python 未安装,跳过体积门禁"
else
    RIMAC="$TARGET_DIR/riscv32imac-unknown-none-elf/release/examples"
    RIMC="$TARGET_DIR/riscv32imc-unknown-none-elf/release/examples"
    # 覆盖四种典型剖面:驱动能力面 / FS 全链 / 协议栈 / 小 flash 板 / 无 A 扩展
    "$SIZE_PY" "$ROOT_DIR/ci/check_size.py" driver           "$DRIVER_ELF"
    "$SIZE_PY" "$ROOT_DIR/ci/check_size.py" fatfs            "$RIMAC/fatfs"
    "$SIZE_PY" "$ROOT_DIR/ci/check_size.py" net_echo         "$RIMAC/net_echo"
    "$SIZE_PY" "$ROOT_DIR/ci/check_size.py" qemu_pingpong    "$RIMAC/qemu_pingpong"
    "$SIZE_PY" "$ROOT_DIR/ci/check_size.py" multitask_ch583  "$RIMAC/multitask_ch583"
    "$SIZE_PY" "$ROOT_DIR/ci/check_size.py" multitask_ch572  "$RIMC/multitask_ch572"
fi

echo "== [4/4] QEMU 执行门禁(virt 机跑真内核——调度/切换/节拍执行级验证)=="
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
    QEMU_ELF="$TARGET_DIR/riscv32imac-unknown-none-elf/release/examples/qemu_pingpong"
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
    # qemu_kernel_tests:28 项内核全功能自测;测试 22/23/24 需 python 经
    # stdin 喂字节(读内核串口握手标记,零时序假设),缺失则无法执行
    PYF="$(command -v python3 || command -v python || true)"
    if [ -z "$PYF" ]; then
        echo "python 未安装——qemu_kernel_tests 的 UART RX 喂字节握手无法执行;安装 python3 后重跑"
        exit 1
    fi
    KTESTS_ELF="$TARGET_DIR/riscv32imac-unknown-none-elf/release/examples/qemu_kernel_tests"
    cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --example qemu_kernel_tests \
        --features qemu_riscv,timer --target riscv32imac-unknown-none-elf --release
    KOUT="$( "$PYF" "$ROOT_DIR/ci/feed_qemu.py" 180 "$QEMU_BIN" "$KTESTS_ELF" | grep -v "GLib-GIO" || true )"
    # 已知 tickless 时序 flake(本机/CI 负载下 ~8%,master 同样复现):失败重试一次,
    # 两次都失败才算真回归——避免下一个 PR 作者被假红误导。
    if ! echo "$KOUT" | grep -q "28/28 passed"; then
        echo "qemu_kernel_tests 首次未全过(已知 tickless 时序 flake),重试一次"
        KOUT="$( "$PYF" "$ROOT_DIR/ci/feed_qemu.py" 180 "$QEMU_BIN" "$KTESTS_ELF" | grep -v "GLib-GIO" || true )"
    fi
    echo "$KOUT" | grep -q "28/28 passed" \
        || { echo "qemu_kernel_tests 未全过;输出:"; echo "$KOUT" | tail -15; exit 1; }
    echo "$KOUT" | grep -q "byte B @ T23-SLEEPING" \
        || { echo "qemu_kernel_tests 未握手到早醒弹字节(test 23 未喂到);输出:"; echo "$KOUT" | tail -8; exit 1; }
    echo "$KOUT" | grep -q "byte N5 @ T24-SLEEPING" \
        || { echo "qemu_kernel_tests 未握手到噪声风暴第 5 字节(test 24 未喂全);输出:"; echo "$KOUT" | tail -8; exit 1; }
    echo "qemu_kernel_tests: 28/28 passed(全内核机制执行级验证)"
    # -smp 2 起跑:hart1 由 riscv-rt 默认停泊,hart0-only 语义应逐字不变
    SMP_OUT="$( "$PYF" "$ROOT_DIR/ci/feed_qemu.py" 180 "$QEMU_BIN" "$KTESTS_ELF" -smp 2 | grep -v "GLib-GIO" || true )"
    if ! echo "$SMP_OUT" | grep -q "28/28 passed"; then
        echo "qemu_kernel_tests -smp 2 首次未全过(已知 tickless 时序 flake),重试一次"
        SMP_OUT="$( "$PYF" "$ROOT_DIR/ci/feed_qemu.py" 180 "$QEMU_BIN" "$KTESTS_ELF" -smp 2 | grep -v "GLib-GIO" || true )"
    fi
    echo "$SMP_OUT" | grep -q "28/28 passed" \
        || { echo "qemu_kernel_tests -smp 2 未全过(hart1 停泊失效?);输出:"; echo "$SMP_OUT" | tail -15; exit 1; }
    echo "$SMP_OUT" | grep -q "boot hart: 0" \
        || { echo "-smp 2 下启动核不是 hart0;输出:"; echo "$SMP_OUT" | tail -5; exit 1; }
    echo "qemu_kernel_tests -smp 2: 28/28 passed(双核起跑,hart1 停泊,hart0-only 语义不变)"
    # TLSF 全局后端:整个套件换 tlsf feature 重跑,28/28 不变即后端透明
    cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --example qemu_kernel_tests \
        --features qemu_riscv,timer,tlsf --target riscv32imac-unknown-none-elf --release
    TLSF_OUT="$( "$PYF" "$ROOT_DIR/ci/feed_qemu.py" 180 "$QEMU_BIN" "$KTESTS_ELF" | grep -v "GLib-GIO" || true )"
    echo "$TLSF_OUT" | grep -q "28/28 passed" \
        || { echo "qemu_kernel_tests(tlsf 全局后端)未全过;输出:"; echo "$TLSF_OUT" | tail -15; exit 1; }
    echo "qemu_kernel_tests(tlsf 全局后端): 28/28 passed(分配器换引擎对内核透明)"
    # SMP 双核执行门禁:应用显式 smp::enable() 后 hart1 真正参与调度
    SMP_ELF="$TARGET_DIR/riscv32imac-unknown-none-elf/release/examples/qemu_smp"
    cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --example qemu_smp \
        --features qemu_riscv,timer --target riscv32imac-unknown-none-elf --release
    SMP2_OUT="$( "$QEMU_BIN" -M virt -smp 2 -nographic -bios none -kernel "$SMP_ELF" </dev/null 2>&1 | grep -v "GLib-GIO" || true )"
    # 既有 SMP 抢分配 flake(见 issue):失败重试一次。
    if ! echo "$SMP2_OUT" | grep -q "smp PASS: 9/9"; then
        echo "qemu_smp 首次未全过(既有 SMP 时序/分配 flake),重试一次"
        SMP2_OUT="$( "$QEMU_BIN" -M virt -smp 2 -nographic -bios none -kernel "$SMP_ELF" </dev/null 2>&1 | grep -v "GLib-GIO" || true )"
    fi
    echo "$SMP2_OUT" | grep -q "smp PASS: 9/9" \
        || { echo "qemu_smp 未全过;输出:"; echo "$SMP2_OUT" | tail -15; exit 1; }
    echo "$SMP2_OUT" | grep -q "harts online: 2" \
        || { echo "qemu_smp 未识别双核;输出:"; echo "$SMP2_OUT" | tail -5; exit 1; }
    echo "qemu_smp -smp 2: 9/9 passed(双核调度/跨核 IPI/锁堆压力/绑核/定时器跨核 执行级验证)"
else
    echo "qemu-system-riscv32 未安装,跳过执行门禁(安装:winget install SoftwareFreedomConservancy.QEMU)"
fi

echo "== 全部通过 =="
