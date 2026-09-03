#!/usr/bin/env bash
# 自动引导 QEMU(riscv32)静态二进制到 <repo>/.tools/qemu —— 读者零手工安装。
# 用法: bash ci/get-qemu.sh
# 来源: xPack qemu-riscv 静态发布(免管理员,免系统包管理器)
# 可覆盖版本: QEMU_XPACK_VERSION=9.2.4-1 bash ci/get-qemu.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VER="${QEMU_XPACK_VERSION:-9.2.4-1}"
BASE_URL="https://github.com/xpack-dev-tools/qemu-riscv-xpack/releases/download/v${VER}"

# --- 平台探测 ---
case "$(uname -s)" in
  Linux)                         OS="linux"  ;;
  Darwin)                        OS="darwin" ;;
  MINGW*|MSYS*|CYGWIN*)          OS="win32"  ;;
  *) echo "不支持的 OS: $(uname -s)"; exit 1 ;;
esac
case "$(uname -m)" in
  x86_64|amd64) ARCH="x64" ;;
  arm64|aarch64) ARCH="arm64" ;;
  *) echo "不支持的架构: $(uname -m)"; exit 1 ;;
esac
# darwin-x64 在 Apple Silicon 上经 Rosetta 亦可;此处按真实内核取
EXT="tar.gz"; [ "$OS" = "win32" ] && EXT="zip"
NAME="xpack-qemu-riscv-${VER}-${OS}-${ARCH}"
EXE="qemu-system-riscv32"; [ "$OS" = "win32" ] && EXE="${EXE}.exe"

DIR="$ROOT/.tools/qemu"
if [ -x "$DIR/bin/$EXE" ]; then
  echo "QEMU 已就绪: $DIR/bin/$EXE"
  "$DIR/bin/$EXE" --version | head -1
  exit 0
fi

echo "== 未找到 qemu-system-riscv32,自动下载 xPack 静态包 =="
echo "   $NAME.$EXT (~90MB, 首次仅一次)"
mkdir -p "$ROOT/.tools" && cd "$ROOT/.tools"
curl -fL --retry 3 -o "$NAME.$EXT" "$BASE_URL/$NAME.$EXT" \
  || { echo "下载失败: $BASE_URL/$NAME.$EXT"; exit 1; }

if [ "$EXT" = "zip" ]; then
  if command -v unzip >/dev/null 2>&1; then
    unzip -q "$NAME.$EXT"
  elif command -v powershell >/dev/null 2>&1; then
    powershell -NoProfile -Command "Expand-Archive -Force '$NAME.$EXT' ."
  else
    echo "需要 unzip 或 powershell 来解压"; exit 1
  fi
else
  tar xzf "$NAME.$EXT"
fi
rm -f "$NAME.$EXT"

# 统一目录名(解压目录可能含版本号)
if [ -d "$ROOT/.tools/$NAME" ]; then mv "$ROOT/.tools/$NAME" "$DIR"; fi
[ -x "$DIR/bin/$EXE" ] || { echo "解压后未找到 $DIR/bin/$EXE"; ls "$ROOT/.tools"; exit 1; }
echo "OK: $DIR/bin/$EXE"
"$DIR/bin/$EXE" --version | head -1
echo "提示: ci/gate.sh 会自动使用 .tools/qemu/bin 下的 QEMU,无需手动加 PATH"
