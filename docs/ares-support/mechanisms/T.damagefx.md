# `T.damagefx` 受损烟雾/火花粒子

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（6）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `DamageSmokeParticleSystems` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:448 | unscanned | — |
| `DamageSparks` | TechnoType | Nullable<bool> | (empty) | src/Ext/TechnoType/Body.cpp:446 | check_0A_use | — |
| `DamageSparksParticleSystems` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:449 | unscanned | — |
| `Smoke.Anim` | TechnoType | Valueable<AnimTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:431 | check_0A_use | — |
| `Smoke.ChanceDead` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:433 | check_0A_use | — |
| `Smoke.ChanceRed` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:432 | check_0A_use | — |

## 钩子（2）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 415085 | `AircraftClass_Update_DamageSmoke` | kept | 00414BB0 `AircraftClass::AircraftClass__AI` | decompiled/functions/10047B60.c | src/Ext/Techno/Hooks.Art.cpp:156 | src/sim/aircraft/mod.rs, src/sim/world/techno_ai.rs |
| 6FACD9 | `TechnoClass_Update_DamageSparks` | kept ⚠改过 | 006F9E50 `TechnoClass::TechnoClass__AI_Update` | decompiled/functions/1004B760.c | src/Ext/Techno/Hooks.cpp:1451 | src/sim/combat/greatest_threat.rs, src/sim/shield.rs, src/sim/world/techno_ai.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1003DEC0 | TechnoTypeExt::ExtMap.LoadFromINI [source_0.A,inferred] | 使用键 | decompiled/functions/1003DEC0.c |
| 1000ACD0 | `CRT::strcmp` | 被钩子调用 | decompiled/functions/1000ACD0.c |
| 1000D3E0 | `AnimTypeClass::Find` | 被钩子调用 | decompiled/functions/1000D3E0.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
