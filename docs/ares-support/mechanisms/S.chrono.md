# `S.chrono` ChronoSphere/ChronoWarp

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M5 | S.core | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（9）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Chronosphere.AffectsIronCurtain` | SWType | — | — | src/Misc/SWTypes/ChronoSphere.cpp:68 | unscanned | — |
| `Chronosphere.AffectsUndeployable` | SWType | — | — | src/Misc/SWTypes/ChronoSphere.cpp:70 | unscanned | — |
| `Chronosphere.AffectsUnwarpable` | SWType | — | — | src/Misc/SWTypes/ChronoSphere.cpp:69 | unscanned | — |
| `Chronosphere.BlastDest` | SWType | — | — | src/Misc/SWTypes/ChronoSphere.cpp:75 | unscanned | — |
| `Chronosphere.BlastSrc` | SWType | — | — | src/Misc/SWTypes/ChronoSphere.cpp:74 | unscanned | — |
| `Chronosphere.BlowUnplaceable` | SWType | — | — | src/Misc/SWTypes/ChronoSphere.cpp:71 | unscanned | — |
| `Chronosphere.KillOrganic` | SWType | — | — | src/Misc/SWTypes/ChronoSphere.cpp:66 | unscanned | — |
| `Chronosphere.KillTeleporters` | SWType | — | — | src/Misc/SWTypes/ChronoSphere.cpp:67 | unscanned | — |
| `Chronosphere.ReconsiderBuildings` | SWType | — | — | src/Misc/SWTypes/ChronoSphere.cpp:72 | unscanned | — |

## 钩子（0）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

