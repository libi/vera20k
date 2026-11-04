# `F.prereq` [GenericPrerequisites] 与 Prerequisite.Lists/Theaters/StolenTechs

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M1 | F.readers | partial | docs/ares-prerequisites.md |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（3）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Prerequisite.Lists` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:164 | unscanned | — |
| `Prerequisite.RequiredTheaters` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:180 | unscanned | — |
| `Prerequisite.StolenTechs` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:262 | unscanned | — |

## 钩子（6）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 4140EB | `AircraftClass_DTOR_Prereqs` | kept | 00414080 `AircraftClass::AircraftClass__Destructor` | decompiled/functions/1004E750.c | src/Ext/Techno/Hooks.Prerequisites.cpp:8 | — |
| 4D7221 | `FootClass_Put_Prereqs` | kept | 004D7170 `FootClass::FootClass__Unlimbo` | decompiled/functions/1004E780.c | src/Ext/Techno/Hooks.Prerequisites.cpp:50 | src/sim/movement/slope_transition.rs, src/sim/sensor_lifecycle.rs, src/sim/world/lifecycle.rs |
| 517DF2 | `InfantryClass_DTOR_Prereqs` | kept | 00517D90 `InfantryClass::InfantryClass__Destructor` | decompiled/functions/1004E7C0.c | src/Ext/Techno/Hooks.Prerequisites.cpp:22 | — |
| 6F4A1D | `TechnoClass_DiscoveredBy_Prereqs` | kept | 006F4960 `TechnoClass::TechnoClass__DiscoveredBy` | decompiled/functions/1004E840.c | src/Ext/Techno/Hooks.Prerequisites.cpp:64 | src/sim/movement/foot_mark.rs, src/sim/world/lifecycle.rs |
| 7015EB | `TechnoClass_ChangeOwnership_Prereqs` | kept | 007014A0 `TechnoClass::TechnoClass__ChangeOwner` | decompiled/functions/1004E7F0.c | src/Ext/Techno/Hooks.Prerequisites.cpp:79 | src/sim/game_entity.rs, src/sim/house_threat.rs, src/sim/sensor_lifecycle.rs … |
| 7357F6 | `UnitClass_DTOR_Prereqs` | kept | 00735780 `UnitClass::UnitClass__Destructor` | decompiled/functions/1004E890.c | src/Ext/Techno/Hooks.Prerequisites.cpp:36 | — |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1003DD00 | `TechnoTypeExt::ExtData::IsGenericPrerequisite` | 被钩子调用 | decompiled/functions/1003DD00.c |

## 涉及的热点函数（2）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 004D7170 `FootClass::FootClass__Unlimbo` | 2 | F.prereq, ext:Techno |
| 007014A0 `TechnoClass::TechnoClass__ChangeOwner` | 2 | F.prereq, ext:Techno |
