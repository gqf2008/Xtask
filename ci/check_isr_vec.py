#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""验证 driver 示例的弱符号 ISR 绑定（阳性对照守卫）。

port.S 的向量表对 USART0 声明 .weak 符号，应用层用 #[no_mangle] 定义后才由
链接器绑定进向量表第 56 项；若绑定失效（例如符号改名、LTO 配置回退到 fat），
链接本身仍会成功（weak 未定义时填 0），真机表现为"收不到字节、回显不工作"。
本脚本直接读 ELF：符号表里 USART0 的地址必须等于 .text 中向量表第 56 个字。
用法: python3 check_isr_vec.py <target/release/examples/driver>
"""
import struct
import sys


def main():
    path = sys.argv[1] if len(sys.argv) > 1 else "target/riscv32imac-unknown-none-elf/release/examples/driver"
    data = open(path, "rb").read()
    assert data[:4] == b"\x7fELF", "不是 ELF 文件"

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

    def find_sec(which):
        return secs[[i for i, v in enumerate(secs) if sname(i) == which][0]]

    symtab, strtab, text = find_sec(".symtab"), find_sec(".strtab"), find_sec(".text")
    symbols = {}
    for i in range(symtab[5] // symtab[9]):
        n, val, _sz, _info, _other, shndx = struct.unpack_from("<IIIBBH", data, symtab[4] + i * symtab[9])
        if n == 0 or n >= strtab[5]:
            continue
        end = data.index(b"\x00", strtab[4] + n)
        nm = data[strtab[4] + n:end].decode("latin-1")
        if nm not in symbols:
            symbols[nm] = (val, shndx)

    for sym in ("vectors", "USART0"):
        if sym not in symbols:
            print(f"FAIL: 符号 {sym} 不存在")
            sys.exit(1)

    vec_addr = symbols["vectors"][0]
    isr_addr = symbols["USART0"][0]
    if isr_addr == 0:
        print("FAIL: USART0 未定义——弱符号绑定失效（向量表将填 0）")
        sys.exit(1)
    # USART0 是中断号 56，向量表按 4 字节一组
    word = struct.unpack_from("<I", data, text[4] + (vec_addr - text[3]) + 56 * 4)[0]
    if word != isr_addr:
        print(f"FAIL: 向量表第 56 项 = 0x{word:08X}，不等于 USART0 地址 0x{isr_addr:08X}")
        sys.exit(1)
    print(f"PASS: 向量表第 56 项 -> USART0 @0x{isr_addr:08X}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
