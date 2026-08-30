# 芯片与架构文档索引(`docs/`)

移植层 `src/chip/<chip>/`（寄存器地址、CSR 行为、中断模型）与书稿论断的
一手依据。**版权归各厂商所有，本目录不存放任何 PDF 副本**（2026-08-30
起从仓库与历史中整体摘除：~173MB 二进制 + Git LFS 免费额度推不动），
改为下表「来源链接」直接指向各厂商官方下载入口或可达镜像——**链接可能
随厂商改版漂移，取不到时按「可达源地图」（见下）换路重试**。

> 本机网络可达性实测（2026-08-27）：`st.com`、`riscv.org`（大文件 GET 会
> 被 WAF 截断）、`gd32mcu.com`（JS 门户但 `/download/down/document_id/N/
> path_type/1(2=CN)` id 端点稳定）、`rvmcu.com`、`egos.fun`、
> `ch32-riscv-ug` GitHub 组织镜像可达；`developer.arm.com` / `wch.cn`
> SPA 拿不到直链（走镜像）。校验铁律：真 GET 验 `%PDF` 头 + 尾 `%%EOF`
> 防截断（HEAD 200 可能是 SPA 兜底假阳性）。

> 两处易混口径先钉死：本仓库 H7 口用的是 **STM32H7B0**（HAL feature
> `stm32h7b0`，对应手册 **RM0455**，不是网上最常见的 H743/RM0433 系）；
> CM32M4xxR 是**中移物联/芯昇科技**基于 **Nuclei N308** 核的 MCU，内核
> 行为（ECLIC/msubm/mtvt/0x7ED 嵌套）与 GD32VF103 的 Bumblebee(N2 系)
> 同族，两口的内核侧依据共用 `docs/core/nuclei_*` 手册。

## 架构 / 指令集

| 文档 | 作用 | 来源链接 |
|---|---|---|
| RISC-V Privileged Spec（20241101 版, M 模式 CSR/中断/CLINT） | qemu_riscv、esp32c3、ch32/cm32m4 各口的标准底座 | [riscv.org/specifications](https://riscv.org/technical/specifications/)（大文件被 WAF 截断时走 [egos.fun 教学镜像](https://egos.fun/) 检索） |
| RISC-V v2.1 中文统一版（用户级 RV32I/M/A/F/C/D, 旧版） | 用户级指令语义中文对照 | 公开中文译本镜像（按 `RISC-V 中文统一版 v2.1` 检索；本机 egos.fun 曾可达） |
| ST PM0214（Cortex-M3/M4 编程手册, ARMv7-M/E-M 指令级） | thumbv7m/eabihf 三口 | [st.com PM0214](https://www.st.com/resource/en/programming_manual/pm0214-stm32-cortexm3-mcus-programming-manual-stmicroelectronics.pdf) |
| ST PM0215（Cortex-M0 编程手册, ARMv6-M 指令级） | thumbv6m = rp2040 | [st.com PM0215](https://www.st.com/resource/en/programming_manual/pm0215-stm32f0xxx-cortex-m0-programming-manual-stmicroelectronics.pdf) |

⚠️ 已知缺口：ARM 官方《Architecture Reference Manual》DDI0419(v6-M)/
DDI0403(v7-M) 当前网络无法直连 developer.arm.com 获取，上表两份 ST 编程
手册覆盖同样的指令级内容作为替代；拿到正式 ARD 后可替换/补入。

## 内核 IP 手册

| 文档 | 对应口 | 来源链接 |
|---|---|---|
| Nuclei N200 ISA 架构手册 CN / Bumblebee ISA 架构手册 CN | gd32vf103 与 cm32m4 的 ECLIC、嵌套深度 CSR(0x7ED)、msubm(0x7C4) —— `src/chip/{gd32vf103,cm32m4}/port.S` | [nucleisys.com 文档页](https://www.nucleisys.com/)（导航 Developer→Document；Nuclei 官网） |
| Nuclei N200 Brief Datasheet CN / Bumblebee Brief Datasheet CN(+EN 架构手册) | 同上两口的内核数据面 | 同上（Nuclei 官网文档页） |
| WCH 青稞 V4 处理器手册 CN | ch32v203/ch32v307 的 PFIC/SysTick/Stk 模型 —— `src/chip/ch32v20{3,7}/port*` | [ch32-riscv-ug GitHub 组织](https://github.com/ch32-riscv-ug)（镜像收全套装；wch.cn 官网为 SPA 拿不到直链） |

## 芯片手册

| 芯片 | 文档 | 对应口 | 来源链接 |
|---|---|---|---|
| gd32vf103 | Datasheet EN（门户并存 id287/id221 两个版次）· User Manual CN+EN | `gd32vf103`（longan-nano；HAL 为本仓 `hal2/`） | [gd32mcu.com 下载中心](https://www.gd32mcu.com/en/download/)（id 端点 `/download/down/document_id/N/path_type/1`；VF 系列官方便无中文 Datasheet） |
| ch32v103 | CH32V103 DS0 CN + CH32xRM 参考手册 CN/EN | `ch32v103`（青稞 V3A） | [ch32-riscv-ug/CH32V103 镜像](https://github.com/ch32-riscv-ug/CH32V103) |
| ch32v20x | CH32V20x DS0 CN | `ch32v203` | [ch32-riscv-ug 组织](https://github.com/ch32-riscv-ug) |
| ch32v30x | CH32V30x DS0 EN | `ch32v307` | [ch32-riscv-ug 组织](https://github.com/ch32-riscv-ug) |
| stm32f1 | RM0008（参考手册）+ DS5319（F103C8 数据手册） | `stm32f1`（bluepill） | [st.com RM0008](https://www.st.com/resource/en/reference_manual/rm0008-stm32f101xx-stm32f102xx-stm32f103xx-stm32f105xx-and-stm32f107xx-advanced-armbased-32bit-mcus-stmicroelectronics.pdf) / [DS5319](https://www.st.com/resource/en/datasheet/stm32f103c8.pdf) |
| stm32f4 | RM0368（F401 参考手册）+ DS8626（F401CC 数据手册） | `stm32f4`（greenpill） | [st.com RM0368](https://www.st.com/resource/en/reference_manual/rm0368-stm32f401xbc-and-stm32f401xde-advanced-armbased-32bit-mcus-stmicroelectronics.pdf) / [DS8626](https://www.st.com/resource/en/datasheet/stm32f401cc.pdf) |
| stm32h7 | RM0455（H7B0 参考手册）+ H7B0 DS | `stm32h7`（注意是 H7B0, 非常见 H743） | [st.com RM0455](https://www.st.com/resource/en/reference_manual/rm0455-stm32h7b0xx-and-stm32h7b1xx-arm-cortex-m7-32bit-mcu-stmicroelectronics.pdf) |
| rp2040 | rp2040-datasheet（含 boot2 约定） | `rp2040` | [raspberrypi.com 官方直链](https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf) |
| esp32c3 | TRM + Datasheet | `esp32c3` | [espressif.com esp32-c3 页](https://www.espressif.com/en/products/socs/esp32-c3)（文档/技术参考手册链接在页面文档区） |
| cm32m4 | CM32M4xxR Datasheet V1.0 CN + User Manual V1.0 CN | `cm32m4` | [rvmcu.com 官方镜像](https://www.rvmcu.com/)（检索 CM32M4xxR；本机实测可达） |

## 已知缺口（2026-08-27 二轮补下载后）

- **ARM《Architecture Reference Manual》DDI0419(v6-M)/DDI0403(v7-M)**：
  唯一官方渠道 developer/support/static.arm.com 三域名在本网络全部不可达
  （超时/403/证书错），archive.org 存档与 gitee/GitHub 镜像检索均无逐字
  副本；以覆盖同样指令级的 ST PM0215(M0/M0+)、PM0214(M3/M4) 替代。日后
  网络可达时从 ARM 官方页免费下载替换。
- ~~CM32M4xxR 数据手册~~ 已补（rvmcu 官方镜像直链）。
- ~~GD32VF103 中文数据手册~~ 定性为官方便无中文版（门户两个版次均为
  EN）；中文对照走 User Manual CN。

### 其余说明

- qemu_riscv 口无独立芯片（QEMU virt 机），标准依据即上面两份 RISC-V spec。
- 2026-08-30 变更：docs/ 下全部 PDF 从仓库与历史摘除（Git LFS 免费额度
  推不动 173MB），本文件升级为纯链接索引；原 PDF 可从上述链接重新获取。
