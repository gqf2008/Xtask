#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""校验 CH583 启动头字节布局(WCH ROM 认的硬要求)。

WCH ROM 从 flash 0x0 取指,并按"向量表第 5 个字 = boot option"识别有效用户
程序:官方 EVT/EXAM/SRC/Startup/startup_CH583.S 的 `.vector` 段第 5 个字固定
0xF3F9BDA9(注释 "boot option, can't modify"),官方 Ld/Link.ld 把 `.vector`
的 flash LMA 紧跟在 `.init` 的 `j handle_reset` 之后 → 魔数落在 flash 0x14。

本口用 `.bootvec` 段复刻同一布局(src/chip/ch583/port.S 内容 + memory.x 摆位,
另有链接期 ASSERT);本脚本做**产物级**终检:段地址/大小、入口跳转、魔数字节。
少了它,板上表现是"停在 ISP、不进用户程序",而构建与门禁全绿。

用法: python3 ci/check_wch_boot.py <target/.../examples/multitask_ch583>
"""
import struct
import sys

MAGIC = 0xF3F9BDA9
BOOT_LEN = 0x18


def main():
    path = sys.argv[1] if len(sys.argv) > 1 else \
        "target/riscv32imac-unknown-none-elf/release/examples/multitask_ch583"
    data = open(path, "rb").read()
    # 显式判断而非 assert:-O 下 assert 会被关掉,校验不能依赖它
    if len(data) < 0x34 or data[:4] != b"\x7fELF":
        print("FAIL: 不是 ELF 文件")
        return 1
    if data[4] != 1 or data[5] != 1:
        print(f"FAIL: 期望 32 位小端 ELF,实测 class={data[4]} endian={data[5]}")
        return 1

    e_shoff = struct.unpack_from("<I", data, 0x20)[0]
    e_shentsize = struct.unpack_from("<H", data, 0x2E)[0]
    e_shnum = struct.unpack_from("<H", data, 0x30)[0]
    e_shstrndx = struct.unpack_from("<H", data, 0x32)[0]
    secs = [struct.unpack_from("<IIIIIIIIII", data, e_shoff + i * e_shentsize) for i in range(e_shnum)]
    shstr_off = secs[e_shstrndx][4]

    def sname(i):
        n = secs[i][0]
        end = data.index(b"\x00", shstr_off + n)
        return data[shstr_off + n:end].decode("latin-1")

    boot = [s for i, s in enumerate(secs) if sname(i) == ".bootvec"]
    if not boot:
        print("FAIL: 没有 .bootvec 段——ROM 认的启动头缺失(见 src/chip/ch583/port.S)")
        return 1
    addr, size = boot[0][3], boot[0][5]
    if addr != 0 or size != BOOT_LEN:
        print(f"FAIL: .bootvec 应落在 flash 0x0 起 0x{BOOT_LEN:X} 字节,实测 0x{addr:X} + 0x{size:X}")
        return 1

    e_phoff = struct.unpack_from("<I", data, 0x1C)[0]
    e_phentsize = struct.unpack_from("<H", data, 0x2A)[0]
    e_phnum = struct.unpack_from("<H", data, 0x2C)[0]
    seg = None
    for i in range(e_phnum):
        p_type, p_offset, p_vaddr, _pa, p_filesz, _ms, _fl, _al = struct.unpack_from(
            "<IIIIIIII", data, e_phoff + i * e_phentsize)
        if p_type == 1 and p_vaddr == 0:
            seg = (p_offset, p_filesz)
    if seg is None:
        print("FAIL: 没有 vaddr=0 的 LOAD 段——启动头不会被烧进 flash")
        return 1

    def word(a):
        if a + 4 > seg[1]:
            print(f"FAIL: flash 0x{a:X} 超出 LOAD 段(0x{seg[1]:X} 字节)")
            sys.exit(1)
        return struct.unpack_from("<I", data, seg[0] + a)[0]

    w0 = word(0)
    # ROM 从 0x0 取指:首字必须是跳转(JAL,或 2 字节 c.j —— 两种都合法;
    # 真正钉死布局的是上面的 .bootvec 段地址/长度与下面的魔数偏移)
    is_jal = w0 & 0x7F == 0x6F
    is_cj = w0 & 0xE003 == 0xA001
    if not (is_jal or is_cj):
        print(f"FAIL: flash 0x0 不是跳转指令(JAL/c.j),实测 0x{w0:08X}——ROM 取指入口失效")
        return 1
    for a in (0x04, 0x08):
        if word(a) != 0:
            print(f"FAIL: 向量表[{a // 4}] 应为 0(官方同款),实测 0x{word(a):08X}")
            return 1
    for a, what in ((0x0C, "NMI"), (0x10, "HardFault")):
        if word(a) == 0:
            print(f"FAIL: 向量表[{a // 4}]({what}) 是空槽")
            return 1
    if word(0x14) != MAGIC:
        print(f"FAIL: boot option 应为 0x{MAGIC:08X}(flash 0x14),实测 0x{word(0x14):08X}")
        return 1
    print(f"PASS: 启动头 @0x0..0x{BOOT_LEN:02X} — 入口跳转 + boot option 0x{MAGIC:08X} @0x14")
    return 0


if __name__ == "__main__":
    sys.exit(main())
