# `U.triggers` 新触发动作/事件（TAction/TEvent）

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M7 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（0）

无键（基础设施或无条件行为）。

## 钩子（17）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 6DD8D7 | `TActionClass_Execute` | kept ⚠改过 | 006DD8B0 `TActionClass::TActionClass__Execute` | decompiled/functions/10039A20.c | src/Ext/TAction/Hooks.cpp:8 | src/map/playfield.rs, src/map/resolved_terrain/cell_recalc.rs, src/sim/trigger_runtime.rs … |
| 6E1780 | `TActionClass_PlayAudioAtRandomWP` | kept | 006E1780 `TActionClass::TActionClass__PlayAudioAtRandomWP` | decompiled/functions/10039B60.c | src/Ext/TAction/Hooks.Replacements.cpp:11 | — |
| 6E20D8 | `TActionClass_DestroyAttached_Loop` | new_in_3.0 | 006E2050 `TActionClass::TActionClass__DestroyAttached` | decompiled/functions/1005D150.c | src/Recovered/TActionClass.cpp:88 | src/sim/trigger_runtime.rs |
| 6E3B60 | `TActionClass_GetMode` | new_in_3.0 | 006E3B60 `TActionClass__GetMode` | decompiled/functions/10039AD0.c | src/Recovered/TActionClass.cpp:67 | — |
| 6E3EE0 | `TActionClass_GetFlags` | new_in_3.0 | 006E3EE0 `TActionClass__GetFlags` | decompiled/functions/10039A90.c | src/Recovered/TActionClass.cpp:49 | — |
| 71E7F8 | `TEventClass_CTOR` | kept | 0071E6A0 `TEventClass::TEventClass__Constructor` | decompiled/functions/100500F0.c | src/Ext/TEvent/Body.cpp:83 | — |
| 71E949 | `TEventClass_HasOccured` | replaces_0.A:EventClass_HasOccured | 0071E940 `TEventClass::TEventClass__HasOccurred` | decompiled/functions/100503E0.c | src/Recovered/TEventClass.cpp:372 | src/sim/world/world_commands.rs |
| 71ED01 | `TEventClass_HasOccured_PlayerAtX2` | replaces_0.A:EventClass_HasOccured_PlayerAtX2 | 0071E940 `TEventClass::TEventClass__HasOccurred` | decompiled/functions/10050460.c | src/Recovered/TEventClass.cpp:415 | src/sim/world/world_commands.rs |
| 71EE79 | `TEventClass_HasOccured_PlayerAtX3` | replaces_0.A:EventClass_HasOccured_PlayerAtX3 | 0071E940 `TEventClass::TEventClass__HasOccurred` | decompiled/functions/10050480.c | src/Recovered/TEventClass.cpp:430 | src/sim/world/world_commands.rs |
| 71F06C | `TEventClass_HasOccured_PlayerAtX1` | replaces_0.A:EventClass_HasOccured_PlayerAtX1 | 0071E940 `TEventClass::TEventClass__HasOccurred` | decompiled/functions/10050430.c | src/Recovered/TEventClass.cpp:397 | src/sim/world/world_commands.rs |
| 71F39B | `TEventClass_SaveToINI` | new_in_3.0 | 0071F390 `TEventClass::TEventClass__PrepareSaveToINI` | decompiled/functions/10050500.c | src/Recovered/TEventClass.cpp:337 | — |
| 71F683 | `TEventClass_GetFlags` | new_in_3.0 | 0071F680 `TEventClass__GetFlags` | decompiled/functions/10050390.c | src/Recovered/TEventClass.cpp:449 | — |
| 71F8C0 | `TEventClass_SaveLoad_Prefix` | kept | 0071F930 `TEventClass__Save` | decompiled/functions/10050280.c | src/Ext/TEvent/Body.cpp:99 | — |
| 71F92B | `TEventClass_Load_Suffix` | kept | 0071F8C0 `TEventClass__Load` | decompiled/functions/10050150.c | src/Ext/TEvent/Body.cpp:110 | — |
| 71F94A | `TEventClass_Save_Suffix` | kept | 0071F930 `TEventClass__Save` | decompiled/functions/100502C0.c | src/Ext/TEvent/Body.cpp:54 | — |
| 71F9C0 | `TEventClass_Persistable` | new_in_3.0 | 0071F9C0 `TEventClass::TEventClass__GetStateB` | decompiled/functions/100504D0.c | src/Recovered/TEventClass.cpp:353 | src/sim/trigger_runtime.rs |
| 71FAA6 | `TEventClass_SDDTOR` | kept | 0071FA80 `TEventClass::TEventClass__ScalarDeletingDestructor` | decompiled/functions/10050250.c | src/Ext/TEvent/Body.cpp:91 | — |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 100396C0 | `TActionExt::Execute` | 被钩子调用 | decompiled/functions/100396C0.c |
| 10039930 | `TActionExt::GetFlags` | 被钩子调用 | decompiled/functions/10039930.c |
| 10039990 | `TActionExt::GetMode` | 被钩子调用 | decompiled/functions/10039990.c |
| 10039B40 | `ScenarioClass::IsDefinedWaypoint` | 被钩子调用 | decompiled/functions/10039B40.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 10039B20 | `ScenarioClass::GetWaypointCoords` | 被钩子调用 | decompiled/functions/10039B20.c |
| 10033BB0 | `VocClass::PlayIndexAtPos` | 被钩子调用 | decompiled/functions/10033BB0.c |
| 10061AC0 | `Debug::Log` | 被钩子调用 | decompiled/functions/10061AC0.c |
| 1004F6A0 | `TEventExt::GetFlags` | 被钩子调用 | decompiled/functions/1004F6A0.c |
| 1004F7A0 | `TEventExt::HasOccured` | 被钩子调用 | decompiled/functions/1004F7A0.c |
| 1004FE00 | `HouseClass::Index_IsMP` | 被钩子调用 | decompiled/functions/1004FE00.c |
| 1004F610 | `TEventExt::GetPersistable` | 被钩子调用 | decompiled/functions/1004F610.c |
| 1004F720 | `TEventExt::GetSaveParamKind` | 被钩子调用 | decompiled/functions/1004F720.c |

## 涉及的热点函数（2）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 006DD8B0 `TActionClass::TActionClass__Execute` | 2 | H.sidepresent, U.triggers |
| 0071E940 `TEventClass::TEventClass__HasOccurred` | 7 | U.triggers |
