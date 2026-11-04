# `T.ifv` IFV 扩展模式与乘客炮塔

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | — | partial | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（2）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `PassengerTurret` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:345 | no_use_found | — |
| `VoiceIFVRepair` | TechnoType | ValueableIdx<VocClass> | -1 | src/Ext/TechnoType/Body.cpp:315 | check_0A_use | src/rules/ruleset.rs |

## 钩子（1）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 7090A8 | `TechnoClass_SelectFiringVoice` | kept | 007090A0 `TechnoClass::TechnoClass__VoiceAttack` | decompiled/functions/1004B1D0.c | src/Ext/Techno/Hooks.cpp:530 | — |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1000ACD0 | `CRT::strcmp` | 被钩子调用 | decompiled/functions/1000ACD0.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
