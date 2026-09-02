#!/usr/bin/env python3
"""qemu_kernel_tests 的 stdin 喂字节器——ch29 章末练习 1 的执行级验证器。

用法:feed_qemu.py <超时秒> <qemu> <elf> [附加 qemu 参数...]

内核经串口打出握手标记(见 examples/qemu_kernel_tests.rs 测试 22/23/24):
  T22-FROZEN     → 冻眠已就绪,喂字节 A(UART RX 中断唤醒)
  T23-SLEEPING   → 2000ms 睡眠已开始,喂字节 B(早醒弹)
  T24-SLEEPING   → 2000ms 睡眠已开始,喂 5 个噪声字节 N(间隔 150ms;
                   回调不唤醒任何任务,专打"停留 idle 重武装"路径)
本器读到标记才写字节——**早到不存在**:字节只可能落在"内核说它睡了/
睡了多久"之后。字节 A 立即写;字节 B 前置 150ms **有界延迟**(远小于
2000ms 睡眠窗口)——判别力要求"弹的提前量 > 拍账上界"(150 拍 ≈ 3.75×
40 拍环境容差),不是时序竞态,见下方 pump 内注释。噪声字节 N 同理:
150ms 间隔 ×5,无补账时每颗把期限推远一段(累计 ≫ 40 拍容差)。
输出原样透传(门禁后续 grep PASS 即靠它),自己的喂字节记录也走
stdout(便于确认握手发生)。

退出码 = qemu 退出码(0 = 内核 SiFive test PASS 自退出);超时(内核死
等字节/挂死)→ 杀 qemu 退出 2,门禁判红。
"""
import subprocess
import sys
import threading
import time

if len(sys.argv) < 4:
    print(__doc__)
    sys.exit(2)

timeout_s = float(sys.argv[1])
qemu = sys.argv[2]
elf = sys.argv[3]
extra = sys.argv[4:]

p = subprocess.Popen(
    [qemu, "-M", "virt", "-nographic", "-bios", "none", "-kernel", elf] + extra,
    stdin=subprocess.PIPE,
    stdout=subprocess.PIPE,
    stderr=subprocess.STDOUT,
)

fed = {"A": False, "B": False, "N": False}


def feed(byte: bytes, tag: str):
    """写字节并记录;qemu 已退出(内核自退出/崩溃)时管道断裂不算错误——
    握手标记都打出来了内核却先死,失败会由退出码/grep 兜住"""
    try:
        p.stdin.write(byte)
        p.stdin.flush()
        print(f"[feeder] byte {tag}", flush=True)
    except (BrokenPipeError, OSError) as e:
        print(f"[feeder] {tag} 写入失败(qemu 已退出?): {e}", flush=True)


def pump():
    for line in iter(p.stdout.readline, b""):
        sys.stdout.buffer.write(line)
        sys.stdout.buffer.flush()
        if b"T22-FROZEN" in line and not fed["A"]:
            fed["A"] = True
            feed(b"A", "A @ T22-FROZEN")
        if b"T23-SLEEPING" in line and not fed["B"]:
            fed["B"] = True
            # 早醒弹要落在"武装之后 ≥ 若干毫秒":无 leave_idle 补账时
            # 拖后量 = 弹的提前量,若 < 拍账上界(40ms)会被容差吃掉,
            # 测试失去判别力(踩坑 5 的守卫红线)。150ms 远小于 2000ms
            # 睡眠窗口且 ≈ 3.75× 上界,是有界延迟不是时序竞态
            time.sleep(0.15)
            feed(b"B", "B @ T23-SLEEPING")
        if b"T24-SLEEPING" in line and not fed["N"]:
            fed["N"] = True
            # 噪声风暴:5 字节 ×150ms。内核侧回调只计数不唤醒——每次
            # 早醒"停留 idle"。无停留补账时,每颗字节把期限推远"距上次
            # 武装已流逝的墙钟"一段,2000ms 睡眠被推到 ≈2750ms+;
            # 有补账时墙钟期限不动(踩坑 5 下半场的专项守卫)
            for i in range(5):
                time.sleep(0.15)
                feed(b"N", f"N{i + 1} @ T24-SLEEPING")


t = threading.Thread(target=pump, daemon=True)
t.start()
try:
    p.wait(timeout=timeout_s)
except subprocess.TimeoutExpired:
    p.kill()
    print(f"[feeder] TIMEOUT {timeout_s}s — 内核未自退出(死等字节?)", flush=True)
    sys.exit(2)
t.join()
sys.exit(p.returncode)
