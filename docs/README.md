# 芯片与架构文档库(`docs/`)

移植层 `src/chip/<chip>/`（寄存器地址、CSR 行为、中断模型）与书稿论断的
一手依据。**版权归各厂商所有，此处仅为学习/开发对照用途的离线副本，不构成
再分发授权**；获取日期 2026-08-27，来源以下表中给出的官方站点或其公开镜像。

> 两处易混口径先钉死：本仓库 H7 口用的是 **STM32H7B0**（HAL feature
> `stm32h7b0`，对应手册 **RM0455**，不是网上最常见的 H743/RM0433 系）；
> CM32M4xxR 是**中移物联/芯昇科技**基于 **Nuclei N308** 核的 MCU，内核
> 行为（ECLIC/msubm/mtvt/0x7ED 嵌套）与 GD32VF103 的 Bumblebee(N2 系)
> 同族，两口的内核侧依据共用 `docs/core/nuclei_*` 手册。

## 架构 / 指令集（`docs/arch/`）

| 文档 | 作用 | 来源 |
|---|---|---|
| RISC-V_Privileged_Spec_20241101.pdf | M 模式 CSR/中断/CLINT 接口语义——qemu_riscv、esp32c3、ch32/cm32m4 各口的标准底座 | riscv.org 官方发布（egos.fun 教学镜像） |
| RISC-V_Spec_v2.1_CN_Unified.pdf | 用户级 RV32I/M/A/F/C/D 指令语义中文统一版(旧版, v2.1) | 公开中文译本镜像 |
| ST_PM0214_CortexM3_M4_Programming_Manual.pdf | ARMv7-M/ARMv7E-M 指令级与内核外设（thumbv7m/eabihf 三口） | st.com |
| ST_PM0215_CortexM0_Programming_Manual.pdf | ARMv6-M 指令级（thumbv6m = rp2040） | st.com |

⚠️ 已知缺口：ARM 官方《Architecture Reference Manual》DDI0419(v6-M)/
DDI0403(v7-M) 当前网络无法直连 developer.arm.com 获取，上表两份 ST 编程
手册覆盖同样的指令级内容作为替代；拿到正式 ARD 后可替换/补入。

## 内核 IP 手册（`docs/core/`）

| 文档 | 对应口 |
|---|---|
| Nuclei_N200_ISA_Architecture_Manual_CN / Nuclei_Bumblebee_ISA_Architecture_Manual_CN | gd32vf103 与 cm32m4 的 ECLIC、嵌套深度 CSR(0x7ED)、msubm(0x7C4) 等 —— `src/chip/{gd32vf103,cm32m4}/port.S` |
| Nuclei_N200_Brief_Datasheet_CN / Nuclei_Bumblebee_Brief_Datasheet_CN(+EN 架构手册) | 同上两口的内核数据面 |
| WCH_QingKe_V4_Processor_Manual_CN | ch32v203/ch32v307 的 PFIC/SysTick/Stk 模型 —— `src/chip/ch32v20{3,7}/port*` |

## 芯片手册（`docs/chip/<chip>/`）

| 目录 | 文档 | 对应口 |
|---|---|---|
| `gd32vf103/` | Datasheet EN（门户并存的两个版次,id287/id221）· User Manual CN+EN | `gd32vf103`（longan-nano；HAL 为本仓 `hal2/`）。注：VF 系列数据手册官方便未发布中文版,中文论述以 User Manual CN 为准 |
| `ch32v103/` | DS0 CN + CH32xRM 参考手册 CN/EN | `ch32v103`（青稞 V3A） |
| `ch32v20x/` | CH32V20xDS0 CN | `ch32v203` |
| `ch32v30x/` | CH32V30xDS0 EN | `ch32v307` |
| `stm32f1/` | RM0008 + DS5319(F103C8) | `stm32f1`（bluepill） |
| `stm32f4/` | RM0368 + DS8626(F401CC) | `stm32f4`（greenpill） |
| `stm32h7/` | RM0455 + DS(H7B0) | `stm32h7` |
| `rp2040/` | rp2040-datasheet（含 boot2 约定） | `rp2040` |
| `esp32c3/` | TRM + Datasheet | `esp32c3` |
| `cm32m4/` | CM32M4xxR Datasheet V1.0 CN + User Manual V1.0 CN | `cm32m4` |

## 已知缺口（2026-08-27 二轮补下载后）

- **ARM《Architecture Reference Manual》DDI0419(v6-M)/DDI0403(v7-M)**：
  唯一官方渠道 developer/support/static.arm.com 三域名在本网络全部不可达
  （超时/403/证书错），archive.org 存档与 gitee/GitHub 镜像检索均无逐字
  副本；已在 `docs/arch/` 以覆盖同样指令级的 ST PM0215(M0/M0+)、
  PM0214(M3/M4) 替代。日后网络可达时从 ARM 官方页免费下载替换。
- ~~CM32M4xxR 数据手册~~ 已补（rvmcu 官方镜像直链）。
- ~~GD32VF103 中文数据手册~~ 定性为官方便无中文版（门户两个版次均为
  EN）；中文对照走 User Manual CN，已入库。

### 其余说明

- qemu_riscv 口无独立芯片（QEMU virt 机），标准依据即上面两份 RISC-V spec。
