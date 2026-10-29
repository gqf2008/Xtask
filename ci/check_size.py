#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""体积门禁:直接从 ELF 段表统计 flash / RAM 占用,并与基线比对。

为什么不用 size/objdump:
  - `arm-none-eabi-size` 的 BFD 只认 arm,读不了 riscv 产物;
  - host 的 `size` 在 macOS / CI 上实现不一(LLVM vs GNU),输出列含义也有差异。
段表是 ELF 规范的一部分,解析它只依赖 python 标准库,跨工具链/跨平台一致。

计量口径(与烧写占用一致):
  flash = 所有 SHF_ALLOC 且 SHT_PROGBITS 的段(.text/.rodata/.data 载入镜像/.isr_vector)
  ram   = 所有 SHF_ALLOC 且(SHT_NOBITS 或 SHF_WRITE)的段(.data + .bss)
  (.data 同时计入两边:载入镜像占 flash,运行时初始化数据占 RAM——与烧写/RAM 的真实
   占用一致;.rodata 与 .data 之间搬家不会虚报 flash 变化)

用法:
  python3 ci/check_size.py --baseline ci/size-baseline.txt driver target/.../driver
  python3 ci/check_size.py --baseline ci/size-baseline.txt --update driver <elf>   # 重新记基线
"""
import argparse
import os
import struct
import subprocess
import sys

SHF_WRITE = 0x1
SHF_ALLOC = 0x2
SHT_NOBITS = 8


def read_sections(path):
    with open(path, "rb") as f:
        data = f.read()
    if len(data) < 52 or data[:4] != b"\x7fELF":
        raise SystemExit("%s: 不是 ELF 文件" % path)
    is64 = data[4] == 2
    endc = "<" if data[5] == 1 else ">"
    if is64:
        e_shoff, = struct.unpack_from(endc + "Q", data, 0x28)
        e_shentsize, e_shnum, e_shstrndx = struct.unpack_from(endc + "HHH", data, 0x3A)
        sh_fmt = endc + "IIQQQQIIQQ"
        name_off = 0
    else:
        e_shoff, = struct.unpack_from(endc + "I", data, 0x20)
        e_shentsize, e_shnum, e_shstrndx = struct.unpack_from(endc + "HHH", data, 0x2E)
        sh_fmt = endc + "IIIIIIIIII"
        name_off = 0
    shs = []
    for i in range(e_shnum):
        base = e_shoff + i * e_shentsize
        if base + struct.calcsize(sh_fmt) > len(data):
            break
        f = struct.unpack_from(sh_fmt, data, base)
        shs.append({
            "name": f[name_off], "type": f[1], "flags": f[2],
            "addr": f[3], "offset": f[4], "size": f[5],
        })
    # 段名表
    if e_shstrndx < len(shs):
        strtab = shs[e_shstrndx]
        base = strtab["offset"]
        end = base + strtab["size"]
        blob = data[base:end]
        for s in shs:
            o = s["name"]
            if 0 <= o < len(blob):
                z = blob.find(b"\x00", o)
                s["name"] = blob[o:z if z >= 0 else len(blob)].decode("utf-8", "replace")
            else:
                s["name"] = ""
    else:
        for s in shs:
            s["name"] = ""
    return shs


def measure(path):
    flash = 0
    ram = 0
    reserved = 0
    detail = []
    for s in read_sections(path):
        if not (s["flags"] & SHF_ALLOC) or s["size"] == 0:
            continue
        # .stack 是链接脚本预留的栈区(NOBITS), 属配置常量而非代码增长;
        # 单独计作 reserved, 只报不进门禁, 否则调栈会持续误报。
        if s["name"].startswith(".stack"):
            reserved += s["size"]
            detail.append(("stack", s["name"], s["size"]))
            continue
        # flash = 烧写镜像里的所有可载入段:.text/.rodata/.isr_vector,以及
        # .data 的初始化镜像(PROGBITS,LMA 在 flash)。NOBITS(.bss)不占 flash。
        if s["type"] != SHT_NOBITS:
            flash += s["size"]
            detail.append(("flash", s["name"], s["size"]))
        # RAM = .bss + .data(初始化数据运行时同样占 RAM)
        if s["type"] == SHT_NOBITS or (s["flags"] & SHF_WRITE):
            ram += s["size"]
            detail.append(("ram", s["name"], s["size"]))
    return flash, ram, reserved, detail


def rustc_version():
    try:
        out = subprocess.check_output(["rustc", "-vV"], stderr=subprocess.DEVNULL)
        for line in out.decode("utf-8", "replace").splitlines():
            if line.startswith("release: "):
                return line[len("release: "):].strip()
    except Exception:
        pass
    return "unknown"


def load_baseline(path):
    entries = {}
    meta = {}
    if not os.path.exists(path):
        return entries, meta
    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            if line.startswith("#"):
                if ":" in line:
                    k, v = line[1:].split(":", 1)
                    meta[k.strip()] = v.strip()
                continue
            parts = line.split()
            if len(parts) == 3:
                entries[parts[0]] = (int(parts[1]), int(parts[2]))
    return entries, meta


def save_baseline(path, entries, rustc):
    names = sorted(entries)
    with open(path, "w", encoding="utf-8") as f:
        f.write("# xtask 体积基线:flash / RAM 字节数(ci/check_size.py --update 生成)\n")
        f.write("# rustc: %s\n" % rustc)
        f.write("# 只有升级工具链或有意增加功能时才更新;无意的增长应视为回归。\n")
        for n in names:
            f.write("%s %d %d\n" % (n, entries[n][0], entries[n][1]))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("name", help="基线里的条目名(如 driver)")
    ap.add_argument("elf", help="ELF 产物路径")
    ap.add_argument("--baseline", default=os.path.join(os.path.dirname(__file__), "size-baseline.txt"))
    ap.add_argument("--update", action="store_true", help="把当前体积写回基线")
    ap.add_argument("--pct", type=float, default=5.0, help="允许的相对增长 %% (默认 5)")
    ap.add_argument("--min", type=int, default=256, help="允许的绝对增长字节下限 (默认 256)")
    ap.add_argument("--ram-pct", type=float, default=4.0, help="RAM 允许的相对增长 %% (默认 4)")
    ap.add_argument("--ram-min", type=int, default=128, help="RAM 允许的绝对增长字节下限 (默认 128)")
    ap.add_argument("--verbose", action="store_true")
    args = ap.parse_args()

    if not os.path.exists(args.elf):
        raise SystemExit("体积门禁:找不到产物 %s(先构建该示例)" % args.elf)

    flash, ram, reserved, detail = measure(args.elf)
    rustc = rustc_version()
    entries, meta = load_baseline(args.baseline)

    if args.verbose:
        for kind, name, size in sorted(detail, key=lambda x: -x[2]):
            print("   %-5s %-18s %7d" % (kind, name, size))

    if args.update:
        entries[args.name] = (flash, ram)
        save_baseline(args.baseline, entries, rustc)
        print("体积基线已更新: %-22s flash=%d ram=%d (reserved .stack=%d 不计入门禁)"
              % (args.name, flash, ram, reserved))
        return 0

    old = entries.get(args.name)
    if old is None:
        print("体积门禁: %-22s flash=%d ram=%d (基线缺失,未比对)" % (args.name, flash, ram))
        print("        记基线: python3 ci/check_size.py %s <elf> --update" % args.name)
        return 0

    base_flash, base_ram = old
    d_flash = flash - base_flash
    d_ram = ram - base_ram
    lim_flash = int(base_flash * args.pct / 100.0)
    if lim_flash < args.min:
        lim_flash = args.min
    lim_ram = int(base_ram * args.ram_pct / 100.0)
    if lim_ram < args.ram_min:
        lim_ram = args.ram_min

    ver_note = ""
    if meta.get("rustc") not in (None, rustc):
        ver_note = " [rustc %s -> %s,基线版本不同]" % (meta.get("rustc"), rustc)

    status = "OK"
    bad = []
    if d_flash > lim_flash:
        bad.append("flash %+d B > 容许 %d B" % (d_flash, lim_flash))
    if d_ram > lim_ram:
        bad.append("RAM %+d B > 容许 %d B" % (d_ram, lim_ram))
    if bad:
        status = "FAIL"

    print("体积门禁: %-22s flash=%6d(%+d) ram=%5d(%+d) reserved=%6d  %s%s"
          % (args.name, flash, d_flash, ram, d_ram, reserved, status, ver_note))

    if bad:
        print("   超出容许增长: " + "; ".join(bad))
        print("   当前最大段(排查方向):")
        for kind, name, size in sorted(detail, key=lambda x: -x[2])[:6]:
            print("     %-5s %-18s %7d" % (kind, name, size))
        print("   若增长是有意的: python3 ci/check_size.py %s %s --update" % (args.name, args.elf))
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
